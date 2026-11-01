//! Builds the WotLK reference database from local TDB and DBC inputs.

use anyhow::{Result, bail};
use std::path::PathBuf;
use tracing_subscriber::{EnvFilter, filter::LevelFilter};
use wotlk_db_builder::{Inputs, build};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();
    let usage = "usage: wotlk-db-builder --tdb <dump.sql> --dbc <dbc dir>";
    let mut args = std::env::args_os().skip(1);
    let mut tdb = None;
    let mut dbc = None;
    while let Some(arg) = args.next() {
        let Some(value) = args.next() else { bail!("{usage}") };
        match arg.to_str() {
            Some("--tdb") if tdb.is_none() => tdb = Some(PathBuf::from(value)),
            Some("--dbc") if dbc.is_none() => dbc = Some(PathBuf::from(value)),
            _ => bail!("{usage}"),
        }
    }
    let (Some(tdb), Some(dbc)) = (tdb, dbc) else {
        bail!("{usage}")
    };
    let inputs = Inputs {
        tdb,
        dbc,
        offsets: concat!(env!("CARGO_MANIFEST_DIR"), "/vendor/offsets").into(),
        codex: concat!(env!("CARGO_MANIFEST_DIR"), "/vendor/api").into(),
    };
    build(
        &inputs,
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/wotlk.sqlite")),
    )?;
    Ok(())
}
