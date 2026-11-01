//! Verified, atomic installation of the versioned database cache.

use std::{
    error::Error,
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

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
pub fn fetch_database(path: &Path, progress: impl FnMut(u8)) -> io::Result<()> {
    let provisioner = Provisioner::new(path);
    provisioner
        .install(progress)
        .map(drop)
        .map_err(|error| io::Error::other(provisioner.failure(&error)))
}

#[derive(Debug, Clone)]
pub(crate) struct Provisioner {
    pub(crate) path: PathBuf,
    pub(crate) url: String,
    pub(crate) sha256: Option<String>,
}

impl Provisioner {
    pub(crate) fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
            url: format!("{RELEASE_URL}/db-schema{SCHEMA_VERSION}/wotlk-schema{SCHEMA_VERSION}.sqlite.zst"),
            sha256: option_env!("WOTLK_DB_SHA256").map(str::to_owned),
        }
    }

    pub(crate) fn failure(&self, error: &dyn Error) -> String {
        let mut cause = error.to_string();
        let mut source = error.source();
        while let Some(error) = source {
            cause.push_str(": ");
            cause.push_str(&error.to_string());
            source = error.source();
        }
        format!(
            "Database provisioning failed: {cause}. Asset: {}. Retry with `npx wotlk-database-mcp fetch`, or download and decompress the asset manually and set WOTLK_DB to the SQLite file.",
            self.url
        )
    }

    pub(crate) fn install(&self, mut progress: impl FnMut(u8)) -> io::Result<Database> {
        let directory = self
            .path
            .parent()
            .ok_or_else(|| io::Error::other("cache path has no parent"))?;
        fs::create_dir_all(directory)?;
        let lock = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(directory.join(".lock"))?;
        lock.lock()?;
        let result = self.install_locked(directory, &mut progress);
        lock.unlock()?;
        result
    }

    fn install_locked(&self, directory: &Path, progress: &mut impl FnMut(u8)) -> io::Result<Database> {
        // Another process may have installed the database while we waited for its lock.
        if let Ok(database) = Database::open(&self.path) {
            return Ok(database);
        }
        // The exclusive lock proves no active installer owns these partial files.
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            if entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.starts_with("wotlk.sqlite.tmp-"))
            {
                fs::remove_file(entry.path())?;
            }
        }
        let expected = self.sha256.as_deref().ok_or_else(|| {
            io::Error::other("this build has no embedded database checksum; use --db <path> or WOTLK_DB")
        })?;
        if expected.len() != 64
            || !expected
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(io::Error::other(
                "embedded database checksum must be 64 lowercase hexadecimal characters",
            ));
        }

        let response = reqwest::blocking::Client::builder()
            .build()
            .map_err(io::Error::other)?
            .get(&self.url)
            .send()
            .and_then(reqwest::blocking::Response::error_for_status)
            .map_err(io::Error::other)?;
        let total = response.content_length();
        progress(0);
        let source = ProgressReader {
            source: response,
            total,
            received: 0,
            percent: 0,
            progress,
        };
        let temporary = TemporaryFile(
            self.path
                .with_file_name(format!("wotlk.sqlite.tmp-{}", std::process::id())),
        );
        let mut destination = HashingWriter {
            file: File::create(&temporary.0)?,
            hash: Sha256::new(),
        };
        zstd::stream::copy_decode(source, &mut destination)?;
        let actual = hex_digest(&destination.hash.finalize());
        if actual != expected {
            return Err(io::Error::other(format!(
                "database checksum mismatch: expected {expected}, found {actual}"
            )));
        }
        destination.file.sync_all()?;
        drop(destination.file);
        // Check the schema before exposing a new final file, including on Windows.
        drop(Database::open(&temporary.0).map_err(io::Error::other)?);
        if self.path.exists() {
            fs::remove_file(&self.path)?;
        }
        fs::rename(&temporary.0, &self.path)?;
        let database = Database::open(&self.path).map_err(io::Error::other)?;
        progress(100);
        Ok(database)
    }
}

struct TemporaryFile(PathBuf);

impl Drop for TemporaryFile {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_file(&self.0)
            && error.kind() != io::ErrorKind::NotFound
        {
            tracing::warn!(path = %self.0.display(), %error, "cannot remove temporary database");
        }
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
        if let Some(total) = self.total.filter(|total| *total > 0) {
            let percent = u8::try_from((u128::from(self.received) * 100 / u128::from(total)).min(100))
                .map_err(io::Error::other)?;
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

fn hex_digest(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 15)]));
    }
    output
}

#[cfg(test)]
pub(crate) mod tests;
