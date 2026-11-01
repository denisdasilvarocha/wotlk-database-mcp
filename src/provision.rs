//! Verified, atomic installation of the versioned database cache.

use std::{
    fmt::Write as _,
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, anyhow, ensure};
use sha2::{Digest, Sha256};

use crate::{SCHEMA_VERSION, db::Database};

const RELEASE_URL: &str = "https://github.com/denisdasilvarocha/wotlk-database-mcp/releases/download";

/// Downloads and verifies the database at the final cache path, or uses a valid cache.
///
/// `progress` receives compressed download percentages. This function blocks;
/// async callers must run it on a blocking worker.
///
/// # Errors
/// Returns an error if no valid cache exists and the build has no embedded checksum,
/// or if locking, downloading, verification, or installation fails.
pub fn fetch_database(path: &Path, progress: impl FnMut(u8)) -> anyhow::Result<()> {
    Provisioner::new(path.to_owned()).install(progress).map(drop)
}

#[derive(Debug)]
pub(crate) struct Provisioner {
    pub(crate) path: PathBuf,
    pub(crate) url: String,
    pub(crate) sha256: Option<String>,
}

impl Provisioner {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self {
            path,
            url: format!("{RELEASE_URL}/db-schema{SCHEMA_VERSION}/wotlk-schema{SCHEMA_VERSION}.sqlite.zst"),
            sha256: option_env!("WOTLK_DB_SHA256").map(str::to_owned),
        }
    }

    /// Returns the cached database, downloading and verifying it first when needed. Blocks.
    pub(crate) fn install(&self, mut progress: impl FnMut(u8)) -> anyhow::Result<Database> {
        self.try_install(&mut progress).map_err(|error| anyhow!(
            "Database provisioning failed: {error:#}. Asset: {}. Retry with `npx wotlk-database-mcp fetch`, or download and decompress the asset manually and set WOTLK_DB to the SQLite file.",
            self.url
        ))
    }

    fn try_install(&self, progress: &mut impl FnMut(u8)) -> anyhow::Result<Database> {
        let directory = self.path.parent().context("cache path has no parent directory")?;
        fs::create_dir_all(directory)?;
        // All installers serialize on this lock; dropping `lock` releases it.
        let lock = File::options()
            .write(true)
            .create(true)
            .truncate(false)
            .open(directory.join(".lock"))?;
        lock.lock()?;
        // Another process may have installed the database while this one waited.
        if let Ok(database) = Database::open(&self.path) {
            return Ok(database);
        }
        let expected = self
            .sha256
            .as_deref()
            .context("this build has no embedded database checksum; use --db <path> or WOTLK_DB")?;
        // The lock makes this name exclusive; File::create truncates an interrupted download.
        let partial = self.path.with_added_extension("partial");
        let downloaded = self.download(&partial, expected, progress);
        if downloaded.is_err() {
            // Best effort: the next attempt truncates the file anyway.
            let _ = fs::remove_file(&partial);
        }
        downloaded?;
        fs::rename(&partial, &self.path)?;
        let database = Database::open(&self.path)?;
        progress(100);
        Ok(database)
    }

    /// Streams the asset through zstd into `partial` and checks the sha256 of the decoded bytes.
    fn download(&self, partial: &Path, expected: &str, progress: &mut impl FnMut(u8)) -> anyhow::Result<()> {
        let response = reqwest::blocking::get(&self.url)?.error_for_status()?;
        let total = response.content_length().filter(|&length| length > 0);
        progress(0);
        let source = ProgressReader {
            source: response,
            total,
            received: 0,
            percent: 0,
            progress,
        };
        let mut destination = HashingWriter {
            file: File::create(partial)?,
            hash: Sha256::new(),
        };
        zstd::stream::copy_decode(source, &mut destination)?;
        let actual = hex(&destination.hash.finalize());
        ensure!(
            actual == expected,
            "database checksum mismatch: expected {expected}, found {actual}"
        );
        destination.file.sync_all()?;
        Ok(())
    }
}

struct ProgressReader<'a, R, F> {
    source: R,
    total: Option<u64>,
    received: u64,
    percent: u8,
    progress: &'a mut F,
}

impl<R: Read, F: FnMut(u8)> Read for ProgressReader<'_, R, F> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let count = self.source.read(buffer)?;
        self.received += count as u64;
        if let Some(total) = self.total {
            let percent = u8::try_from(self.received * 100 / total).unwrap_or(100).min(100);
            if percent != self.percent {
                self.percent = percent;
                (self.progress)(percent);
            }
        }
        Ok(count)
    }
}

struct HashingWriter {
    file: File,
    hash: Sha256,
}

impl Write for HashingWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let count = self.file.write(buffer)?;
        self.hash.update(&buffer[..count]);
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut hex, byte| {
            let _ = write!(hex, "{byte:02x}");
            hex
        })
}

#[cfg(test)]
pub(crate) mod tests;
