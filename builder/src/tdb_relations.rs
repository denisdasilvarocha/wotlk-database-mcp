use std::fmt::Write;

use rusqlite::{Connection, Statement, params};

use crate::{BuildError, dbc::Context, dump::Row, names};

const LOOT_COLUMNS: &[&str] = &[
    "Entry",
    "Item",
    "Reference",
    "Chance",
    "QuestRequired",
    "GroupId",
    "MinCount",
    "MaxCount",
];

pub(super) const TABLES: &[(&str, &[&str])] = &[
    ("creature_loot_template", LOOT_COLUMNS),
    ("gameobject_loot_template", LOOT_COLUMNS),
    ("item_loot_template", LOOT_COLUMNS),
    ("reference_loot_template", LOOT_COLUMNS),
    ("disenchant_loot_template", LOOT_COLUMNS),
    (
        "npc_vendor",
        &["entry", "slot", "item", "maxcount", "incrtime", "ExtendedCost"],
    ),
    ("trainer", &["Id", "Type", "Requirement", "Greeting"]),
    (
        "trainer_spell",
        &[
            "TrainerId",
            "SpellId",
            "MoneyCost",
            "ReqSkillLine",
            "ReqSkillRank",
            "ReqAbility1",
            "ReqAbility2",
            "ReqAbility3",
            "ReqLevel",
        ],
    ),
    ("creature_default_trainer", &["CreatureId", "TrainerId"]),
    ("spell_ranks", &["first_spell_id", "spell_id", "rank"]),
];

fn nonzero(value: i64) -> Option<i64> {
    (value != 0).then_some(value)
}

pub(super) struct Importer<'conn> {
    loot: Statement<'conn>,
    vendor: Statement<'conn>,
    trainer: Statement<'conn>,
    trainer_spell: Statement<'conn>,
    creature_trainer: Statement<'conn>,
    spell_rank: Statement<'conn>,
}

impl<'conn> Importer<'conn> {
    pub(super) fn new(conn: &'conn Connection) -> Result<Self, BuildError> {
        Ok(Self {
            loot: conn.prepare("INSERT INTO loot (source, entry, item, reference, chance, quest_required, group_id, min_count, max_count) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")?,
            vendor: conn.prepare("INSERT INTO vendor_import (entry, slot, item, max_count, restock_time, extended_cost, cost, line) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?,
            trainer: conn.prepare("INSERT INTO trainer (id, type, requirement, greeting) VALUES (?1, ?2, ?3, ?4)")?,
            trainer_spell: conn.prepare("INSERT INTO trainer_spell (trainer_id, spell_id, cost, req_skill, req_skill_rank, req_level, req_abilities) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?,
            creature_trainer: conn.prepare("INSERT INTO creature_trainer (creature_id, trainer_id) VALUES (?1, ?2)")?,
            spell_rank: conn.prepare("INSERT INTO spell_rank (first_spell_id, spell_id, rank) VALUES (?1, ?2, ?3)")?,
        })
    }

    #[expect(
        clippy::too_many_lines,
        reason = "Keep each source relation table's field mapping beside its prepared insert"
    )]
    pub(super) fn import(&mut self, table: &str, row: &Row<'_>, dbc: &Context) -> Result<(), BuildError> {
        let source = match table {
            "creature_loot_template" => Some("creature"),
            "gameobject_loot_template" => Some("gameobject"),
            "item_loot_template" => Some("item"),
            "reference_loot_template" => Some("reference"),
            "disenchant_loot_template" => Some("disenchant"),
            _ => None,
        };
        if let Some(source) = source {
            let reference = row.int("Reference")?;
            let item = row.int("Item")?;
            self.loot.execute(params![
                source,
                row.int("Entry")?,
                if reference != 0 { None } else { nonzero(item) },
                nonzero(reference),
                row.real("Chance")?,
                nonzero(row.int("QuestRequired")?),
                nonzero(row.int("GroupId")?),
                nonzero(row.int("MinCount")?),
                nonzero(row.int("MaxCount")?)
            ])?;
            return Ok(());
        }
        match table {
            "npc_vendor" => {
                let item = row.int("item")?;
                if item == i64::MIN {
                    return Err(BuildError::Dump {
                        line: row.line(),
                        message: "vendor reference out of range".into(),
                    });
                }
                let extended_cost = row.int("ExtendedCost")?;
                let cost = nonzero(extended_cost)
                    .and_then(|id| i32::try_from(id).ok())
                    .and_then(|id| dbc.extended_costs.get(&id));
                let line = i64::try_from(row.line()).map_err(|_| BuildError::Dump {
                    line: row.line(),
                    message: "vendor line number out of range".into(),
                })?;
                self.vendor.execute(params![
                    row.int("entry")?,
                    row.int("slot")?,
                    item,
                    nonzero(row.int("maxcount")?),
                    nonzero(row.int("incrtime")?),
                    nonzero(extended_cost),
                    cost,
                    line
                ])?;
            }
            "trainer" => {
                const TYPES: &[(i64, &str)] = &[(0, "CLASS"), (1, "MOUNT"), (2, "TRADESKILL"), (3, "PET")];
                self.trainer.execute(params![
                    row.int("Id")?,
                    names::enum_name(TYPES, row.int("Type")?),
                    nonzero(row.int("Requirement")?),
                    row.text("Greeting")?
                ])?;
            }
            "trainer_spell" => {
                let mut abilities = String::new();
                for column in ["ReqAbility1", "ReqAbility2", "ReqAbility3"] {
                    let ability = row.int(column)?;
                    if ability != 0 {
                        if !abilities.is_empty() {
                            abilities.push(',');
                        }
                        write!(&mut abilities, "{ability}").map_err(|_| BuildError::Dump {
                            line: 0,
                            message: "could not format trainer abilities".into(),
                        })?;
                    }
                }
                self.trainer_spell.execute(params![
                    row.int("TrainerId")?,
                    row.int("SpellId")?,
                    nonzero(row.int("MoneyCost")?),
                    nonzero(row.int("ReqSkillLine")?),
                    nonzero(row.int("ReqSkillRank")?),
                    nonzero(row.int("ReqLevel")?),
                    (!abilities.is_empty()).then_some(abilities.as_str())
                ])?;
            }
            "creature_default_trainer" => {
                self.creature_trainer
                    .execute(params![row.int("CreatureId")?, row.int("TrainerId")?])?;
            }
            "spell_ranks" => {
                self.spell_rank.execute(params![
                    row.int("first_spell_id")?,
                    row.int("spell_id")?,
                    row.int("rank")?
                ])?;
            }
            _ => {
                return Err(BuildError::Dump {
                    line: 0,
                    message: format!("unexpected relation table {table}"),
                });
            }
        }
        Ok(())
    }
}

const VENDOR_WALK: &str = "
    WITH RECURSIVE walk(root, slot, item, max_count, restock_time, extended_cost, cost, line, path, cycle) AS (
        SELECT entry, slot, item, max_count, restock_time, extended_cost, cost, line,
               ',' || entry || ',',
               item < 0 AND -item = entry
        FROM vendor_import
        UNION ALL
        SELECT walk.root, walk.slot, child.item, child.max_count, child.restock_time,
               child.extended_cost, child.cost, child.line,
               walk.path || child.entry || ',',
               child.item < 0 AND instr(walk.path || child.entry || ',', ',' || -child.item || ',') > 0
        FROM walk JOIN vendor_import child ON child.entry = -walk.item
        WHERE walk.item < 0 AND NOT walk.cycle
    )
";

pub(super) fn initialize(conn: &Connection) -> Result<(), BuildError> {
    conn.execute_batch(
        "CREATE TEMP TABLE vendor_import (
            entry INTEGER, slot INTEGER, item INTEGER, max_count INTEGER, restock_time INTEGER,
            extended_cost INTEGER, cost TEXT, line INTEGER
        );
        CREATE INDEX vendor_import_entry ON vendor_import(entry);",
    )?;
    Ok(())
}

pub(super) fn finalize(conn: &Connection) -> Result<(), BuildError> {
    use rusqlite::OptionalExtension;

    let mut cycles = conn.prepare(&format!("{VENDOR_WALK} SELECT line FROM walk WHERE cycle LIMIT 1"))?;
    if let Some(line) = cycles.query_row([], |row| row.get::<_, i64>(0)).optional()? {
        let line = u64::try_from(line).map_err(|_| BuildError::Dump {
            line: 0,
            message: "vendor line number out of range".into(),
        })?;
        return Err(BuildError::Dump {
            line,
            message: "cyclic vendor reference (cycle detected)".into(),
        });
    }
    let mut insert = conn.prepare(&format!(
        "{VENDOR_WALK}
         INSERT INTO vendor (creature_id, slot, item_id, max_count, restock_time, extended_cost, cost)
         SELECT root, slot, item, max_count, restock_time, extended_cost, cost FROM walk WHERE item > 0"
    ))?;
    insert.execute([])?;
    conn.execute_batch("DROP TABLE vendor_import")?;
    Ok(())
}
