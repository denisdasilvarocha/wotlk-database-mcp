//! Import server data from the pinned `TrinityCore` world dump.

use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};

use rusqlite::{Connection, params};
use sha2::{Digest, Sha256};

use crate::{BuildError, dbc::Context, dump, names};

#[path = "tdb_creature.rs"]
mod creature;
#[path = "tdb_item.rs"]
mod item;
#[path = "tdb_quest.rs"]
mod quest;
#[path = "tdb_relations.rs"]
mod relations;

const HEX: &[u8; 16] = b"0123456789abcdef";

pub(crate) struct Provenance {
    pub(crate) release: String,
    pub(crate) sha256: String,
}

struct HashReader<'a, R> {
    inner: R,
    hash: &'a mut Sha256,
}

impl<R: Read> Read for HashReader<'_, R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let count = self.inner.read(buffer)?;
        self.hash.update(&buffer[..count]);
        Ok(count)
    }
}

pub(crate) fn import(conn: &Connection, path: &Path, dbc: &Context) -> Result<Provenance, BuildError> {
    let file = File::open(path).map_err(|source| BuildError::Io {
        path: path.to_owned(),
        source,
    })?;
    let mut hash = Sha256::new();
    let reader = BufReader::new(HashReader {
        inner: file,
        hash: &mut hash,
    });
    let mut wanted = vec![("version", &["db_version"][..])];
    wanted.extend_from_slice(item::TABLES);
    wanted.extend_from_slice(creature::TABLES);
    wanted.extend_from_slice(quest::TABLES);
    quest::initialize(conn)?;
    wanted.extend_from_slice(relations::TABLES);
    relations::initialize(conn)?;
    let mut release = None;
    {
        let mut items = item::Importer::new(conn)?;
        let mut creatures = creature::Importer::new(conn)?;
        let mut quests = quest::Importer::new(conn)?;
        let mut relations = relations::Importer::new(conn)?;
        dump::read_dump(reader, &wanted, |table, row| {
            if table == "version" {
                release = row.text("db_version")?.map(str::to_owned);
            } else if item::TABLES.iter().any(|(name, _)| *name == table) {
                items.import(table, row, dbc)?;
            } else if creature::TABLES.iter().any(|(name, _)| *name == table) {
                creatures.import(table, row, dbc)?;
            } else if quest::TABLES.iter().any(|(name, _)| *name == table) {
                quests.import(table, row, dbc)?;
            } else {
                relations.import(table, row, dbc)?;
            }
            Ok(())
        })?;
    }
    let release = release.unwrap_or_else(|| "none".to_owned());
    let release = release.trim().to_owned();
    if !release.contains("335.26091") {
        return Err(BuildError::Dump {
            line: 0,
            message: format!("expected TDB 335.26091, found {release}"),
        });
    }
    quest::finalize(conn)?;
    relations::finalize(conn)?;
    {
        let mut insert =
            conn.prepare("INSERT OR IGNORE INTO item (id, class, display_id, icon) VALUES (?1, ?2, ?3, ?4)")?;
        for row in &dbc.items {
            let icon = dbc.display_icons.get(&row.display_id);
            insert.execute(params![
                row.id,
                names::enum_name(names::ITEM_CLASS, i64::from(row.class_id)),
                (row.display_id != 0).then_some(row.display_id),
                icon
            ])?;
        }
    }
    conn.execute_batch("UPDATE itemset SET items = (SELECT group_concat(piece, ', ') FROM (SELECT CAST(si.item_id AS TEXT) || CASE WHEN i.name IS NOT NULL AND i.name <> '' THEN ' ' || i.name ELSE '' END AS piece FROM itemset_item si LEFT JOIN item i ON i.id = si.item_id WHERE si.itemset_id = itemset.id ORDER BY si.slot))")?;
    let mut sha256 = String::with_capacity(64);
    for byte in hash.finalize() {
        sha256.push(char::from(HEX[usize::from(byte >> 4)]));
        sha256.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    Ok(Provenance { release, sha256 })
}
