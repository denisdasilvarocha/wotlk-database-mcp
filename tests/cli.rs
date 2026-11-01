//! Exercises foreground CLI behavior without an MCP client.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use wotlk_database_mcp::SCHEMA_VERSION;

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> std::io::Result<Self> {
        let path = std::env::temp_dir().join(format!(
            "wotlk-cli-{}-{}",
            std::process::id(),
            NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&path)?;
        Ok(Self(path))
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct ChildGuard(std::process::Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn binary() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_wotlk-database-mcp"));
    command.env_remove("WOTLK_DB");
    command
}

#[cfg(not(windows))]
fn isolated_cache(command: &mut Command, home: &Path) -> PathBuf {
    #[cfg(target_os = "macos")]
    let root = {
        command.env("HOME", home);
        home.join("Library/Caches")
    };
    #[cfg(not(target_os = "macos"))]
    let root = {
        command.env("XDG_CACHE_HOME", home);
        home.to_path_buf()
    };
    root.join("wotlk-database-mcp").join(format!("schema-{SCHEMA_VERSION}"))
}

fn missing_explicit_path(path: &Path, environment: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = binary();
    if environment {
        command.env("WOTLK_DB", path);
    } else {
        command.arg("--db").arg(path);
    }
    let output = command.output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains(&*path.to_string_lossy()));
    assert_eq!(output.stdout, b"");
    assert!(!path.exists());
    Ok(())
}

#[test]
fn missing_argument_database_exits_with_path() -> Result<(), Box<dyn std::error::Error>> {
    let directory = TestDirectory::new()?;
    missing_explicit_path(&directory.0.join("missing.sqlite"), false)
}

#[test]
fn missing_environment_database_exits_with_path() -> Result<(), Box<dyn std::error::Error>> {
    let directory = TestDirectory::new()?;
    missing_explicit_path(&directory.0.join("missing.sqlite"), true)
}

// Windows resolves its Known Folder through the OS, not LOCALAPPDATA; avoid the user's cache.
#[cfg(not(windows))]
#[test]
fn fetch_uses_valid_cache_without_stdout_or_download() -> Result<(), Box<dyn std::error::Error>> {
    let directory = TestDirectory::new()?;
    let mut command = binary();
    let path = isolated_cache(&mut command, &directory.0).join("wotlk.sqlite");
    std::fs::create_dir_all(path.parent().ok_or("database has no parent")?)?;
    let connection = rusqlite::Connection::open(&path)?;
    connection.execute_batch("CREATE TABLE meta(name TEXT PRIMARY KEY, value TEXT);")?;
    connection.execute(
        "INSERT INTO meta VALUES ('schema_version', ?1)",
        [SCHEMA_VERSION.to_string()],
    )?;
    connection.execute("INSERT INTO meta VALUES ('client_build', '12340')", [])?;
    drop(connection);
    // Any accidental network request fails rather than leaving the test dependent on GitHub.
    command.env("HTTPS_PROXY", "http://127.0.0.1:1");
    command.env("https_proxy", "http://127.0.0.1:1");
    command.env("NO_PROXY", "");
    command.env("no_proxy", "");
    let output = command.arg("fetch").output()?;
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(output.stdout, b"");
    Ok(())
}

#[cfg(not(windows))]
#[test]
fn stdio_shutdown_does_not_wait_for_another_installers_lock() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{BufRead, Write};
    use std::process::Stdio;
    use std::time::{Duration, Instant};

    let directory = TestDirectory::new()?;
    let mut command = binary();
    let cache = isolated_cache(&mut command, &directory.0);
    std::fs::create_dir_all(&cache)?;
    let lock = std::fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(cache.join(".lock"))?;
    lock.lock()?;

    let mut child = ChildGuard(
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?,
    );
    let mut stdin = child.0.stdin.take().ok_or("missing child stdin")?;
    let stdout = child.0.stdout.take().ok_or("missing child stdout")?;
    let (sender, receiver) = std::sync::mpsc::channel();
    let reader = std::thread::spawn(move || {
        for line in std::io::BufReader::new(stdout).lines() {
            if sender.send(line).is_err() {
                break;
            }
        }
    });
    let timeout = Duration::from_secs(5);
    stdin.write_all(concat!(
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"shutdown-test","version":"1"}}}"#,
        "\n"
    ).as_bytes())?;
    stdin.flush()?;
    let initialize: rmcp::serde_json::Value = rmcp::serde_json::from_str(&receiver.recv_timeout(timeout)??)?;
    assert_eq!(initialize["id"], 1);
    assert!(initialize.get("result").is_some());
    stdin.write_all(
        concat!(
            r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
            "\n",
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get","arguments":{"kind":"meta"}}}"#,
            "\n"
        )
        .as_bytes(),
    )?;
    stdin.flush()?;
    let downloading = receiver.recv_timeout(timeout)??;
    assert!(downloading.contains("downloading"), "{downloading}");
    drop(stdin);

    let deadline = Instant::now() + timeout;
    let status = loop {
        if let Some(status) = child.0.try_wait()? {
            break status;
        }
        if Instant::now() >= deadline {
            return Err("server did not exit while another installer held the cache lock".into());
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    assert!(status.success(), "{status}");
    reader.join().map_err(|_| "stdout reader panicked")?;
    lock.unlock()?;
    Ok(())
}

#[test]
fn stdio_shutdown_does_not_wait_for_queued_sql_work() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{BufRead, Write};
    use std::process::Stdio;
    use std::time::{Duration, Instant};

    let directory = TestDirectory::new()?;
    let path = directory.0.join("wotlk.sqlite");
    let connection = rusqlite::Connection::open(&path)?;
    connection.execute_batch("CREATE TABLE meta(name TEXT PRIMARY KEY, value TEXT);")?;
    connection.execute(
        "INSERT INTO meta VALUES ('schema_version', ?1)",
        [SCHEMA_VERSION.to_string()],
    )?;
    connection.execute("INSERT INTO meta VALUES ('client_build', '12340')", [])?;
    drop(connection);

    let mut child = ChildGuard(
        binary()
            .arg("--db")
            .arg(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?,
    );
    let mut stdin = child.0.stdin.take().ok_or("missing child stdin")?;
    let stdout = child.0.stdout.take().ok_or("missing child stdout")?;
    let (sender, receiver) = std::sync::mpsc::channel();
    let reader = std::thread::spawn(move || {
        for line in std::io::BufReader::new(stdout).lines() {
            if sender.send(line).is_err() {
                break;
            }
        }
    });
    stdin.write_all(concat!(
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"shutdown-test","version":"1"}}}"#,
        "\n"
    ).as_bytes())?;
    stdin.flush()?;
    let initialize: rmcp::serde_json::Value =
        rmcp::serde_json::from_str(&receiver.recv_timeout(Duration::from_secs(10))??)?;
    assert_eq!(initialize["id"], 1);
    assert!(initialize.get("result").is_some());

    stdin.write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n")?;
    let started = Instant::now();
    for id in 2..8 {
        writeln!(
            stdin,
            r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"sql","arguments":{{"query":"WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL SELECT x + 1 FROM c) SELECT count(*) FROM c"}}}}}}"#
        )?;
    }
    stdin.flush()?;
    let response: rmcp::serde_json::Value =
        rmcp::serde_json::from_str(&receiver.recv_timeout(Duration::from_secs(10))??)?;
    assert!((2..8).contains(&response["id"].as_u64().ok_or("missing SQL response id")?));
    assert_eq!(response["result"]["isError"], true);
    assert!(
        started.elapsed() >= Duration::from_secs(1),
        "SQL returned before executing"
    );
    drop(stdin);

    // The MCP drain is bounded to five seconds; queued blocking queries must not extend it.
    let deadline = Instant::now() + Duration::from_secs(7);
    let status = loop {
        if let Some(status) = child.0.try_wait()? {
            break status;
        }
        if Instant::now() >= deadline {
            return Err("server did not exit promptly with queued SQL work".into());
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    assert!(status.success(), "{status}");
    reader.join().map_err(|_| "stdout reader panicked")?;
    Ok(())
}
