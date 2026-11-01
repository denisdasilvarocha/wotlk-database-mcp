use std::{
    fmt::Write as _,
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    path::PathBuf,
    sync::{
        Arc, Barrier,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};

use sha2::{Digest, Sha256};

use super::Provisioner;

static NEXT_DIR: AtomicUsize = AtomicUsize::new(0);

pub(crate) struct Fixture {
    dir: PathBuf,
    pub(crate) path: PathBuf,
    pub(crate) compressed: Vec<u8>,
    pub(crate) hash: String,
}

impl Fixture {
    pub(crate) fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "wotlk-provision-{}-{}",
            std::process::id(),
            NEXT_DIR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).expect("create fixture directory");
        let source = dir.join("source.sqlite");
        let conn = rusqlite::Connection::open(&source).expect("create fixture database");
        conn.execute_batch(
            "CREATE TABLE meta(name TEXT PRIMARY KEY, value TEXT);
             INSERT INTO meta VALUES('schema_version', '1'), ('client_build', '12340');",
        )
        .expect("write fixture metadata");
        drop(conn);
        let bytes = fs::read(source).expect("read fixture database");
        let mut hash = String::new();
        for byte in Sha256::digest(&bytes) {
            write!(&mut hash, "{byte:02x}").expect("format fixture checksum");
        }
        Self {
            path: dir.join("cache/wotlk.sqlite"),
            compressed: zstd::stream::encode_all(bytes.as_slice(), 1).expect("compress fixture database"),
            hash,
            dir,
        }
    }

    pub(crate) fn provisioner(&self, url: String) -> Provisioner {
        Provisioner {
            path: self.path.clone(),
            url,
            sha256: Some(self.hash.clone()),
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.dir).expect("remove fixture directory");
    }
}

pub(crate) fn serve_once(bytes: Vec<u8>, release: Option<mpsc::Receiver<()>>) -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind fixture HTTP server");
    let url = format!("http://{}/database.zst", listener.local_addr().expect("HTTP address"));
    let task = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept download");
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("set read timeout");
        {
            let mut request = BufReader::new(&mut stream);
            let mut headers = String::new();
            while !headers.ends_with("\r\n\r\n") {
                assert!(request.read_line(&mut headers).expect("read download request") > 0);
            }
        }
        write!(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            bytes.len()
        )
        .expect("write headers");
        let prefix = if let Some(release) = release {
            let prefix = (bytes.len() * 40).div_ceil(100);
            stream.write_all(&bytes[..prefix]).expect("write partial body");
            stream.flush().expect("flush partial body");
            release.recv_timeout(Duration::from_secs(5)).expect("release download");
            prefix
        } else {
            0
        };
        // Small chunks make compressed-byte progress observable before completion.
        for chunk in bytes[prefix..].chunks(16) {
            stream.write_all(chunk).expect("write response body");
            stream.flush().expect("flush response body");
            thread::sleep(Duration::from_millis(2));
        }
    });
    (url, task)
}

#[test]
fn installs_verified_database_and_reports_compressed_byte_progress() {
    let fixture = Fixture::new();
    let (url, http) = serve_once(fixture.compressed.clone(), None);
    let mut progress = Vec::new();
    drop(
        fixture
            .provisioner(url)
            .install(|percent| progress.push(percent))
            .expect("install database"),
    );
    http.join().expect("join HTTP server");
    assert!(fixture.path.is_file());
    assert!(progress.contains(&0), "{progress:?}");
    assert!(progress.iter().any(|p| (1..100).contains(p)), "{progress:?}");
    assert_eq!(progress.last(), Some(&100));
}

#[test]
fn successful_install_removes_temporary_files_from_interrupted_processes() {
    let fixture = Fixture::new();
    fs::create_dir_all(fixture.path.parent().expect("cache directory")).expect("create cache directory");
    let stale = fixture.path.with_file_name("wotlk.sqlite.tmp-999999999");
    fs::write(&stale, b"interrupted database").expect("seed interrupted download");
    let (url, http) = serve_once(fixture.compressed.clone(), None);
    drop(fixture.provisioner(url).install(|_| {}).expect("install database"));
    http.join().expect("join HTTP server");
    assert!(!stale.exists());
}

#[test]
fn wrong_hash_removes_partial_file_and_never_installs_database() {
    let fixture = Fixture::new();
    let (url, http) = serve_once(fixture.compressed.clone(), None);
    let mut provisioner = fixture.provisioner(url);
    provisioner.sha256 = Some("0".repeat(64));
    let error = provisioner.install(|_| {}).expect_err("reject wrong checksum");
    http.join().expect("join HTTP server");
    assert!(error.to_string().contains("checksum mismatch"), "{error}");
    assert!(!fixture.path.exists());
    assert!(
        !fixture
            .path
            .with_file_name(format!("wotlk.sqlite.tmp-{}", std::process::id()))
            .exists()
    );
}

#[test]
fn concurrent_installs_request_database_only_once() {
    let fixture = Fixture::new();
    // The listener serves exactly one request; a second request fails the caller.
    let (url, http) = serve_once(fixture.compressed.clone(), None);
    let first = fixture.provisioner(url.clone());
    let second = fixture.provisioner(url);
    let start = Arc::new(Barrier::new(2));
    let other_start = Arc::clone(&start);
    let task = thread::spawn(move || {
        other_start.wait();
        first.install(|_| {}).expect("first install")
    });
    start.wait();
    drop(second.install(|_| {}).expect("second install"));
    drop(task.join().expect("join first install"));
    http.join().expect("join HTTP server");
}

#[test]
fn valid_cache_needs_neither_network_nor_embedded_checksum() {
    let fixture = Fixture::new();
    fs::create_dir_all(fixture.path.parent().expect("cache parent")).expect("create cache directory");
    fs::copy(fixture.dir.join("source.sqlite"), &fixture.path).expect("seed valid cache");
    let mut provisioner = fixture.provisioner("http://127.0.0.1:1/unreachable".to_owned());
    provisioner.sha256 = None;
    drop(provisioner.install(|_| {}).expect("use existing cache"));
}

#[test]
fn missing_checksum_does_not_request_or_create_a_database() {
    let fixture = Fixture::new();
    let mut provisioner = fixture.provisioner("http://127.0.0.1:1/unreachable".to_owned());
    provisioner.sha256 = None;
    let error = provisioner.install(|_| {}).expect_err("require embedded checksum");
    assert!(error.to_string().contains("no embedded database checksum"), "{error}");
    assert!(error.to_string().contains("--db"), "{error}");
    assert!(error.to_string().contains("WOTLK_DB"), "{error}");
    assert!(!fixture.path.exists());
}
