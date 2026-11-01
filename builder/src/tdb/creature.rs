use std::fmt::Write as _;

use rusqlite::{Connection, Statement, params};

use crate::{BuildError, dbc::Context, dump::Row, names};

pub(super) const TABLES: &[(&str, &[&str])] = &[
    (
        "creature_template",
        &[
            "entry",
            "modelid1",
            "modelid2",
            "modelid3",
            "modelid4",
            "name",
            "subname",
            "minlevel",
            "maxlevel",
            "exp",
            "faction",
            "npcflag",
            "speed_walk",
            "speed_run",
            "scale",
            "rank",
            "unit_class",
            "family",
            "type",
            "type_flags",
            "lootid",
            "pickpocketloot",
            "skinloot",
            "VehicleId",
            "mingold",
            "maxgold",
            "AIName",
            "HealthModifier",
            "ManaModifier",
            "ArmorModifier",
            "DamageModifier",
            "mechanic_immune_mask",
            "ScriptName",
        ],
    ),
    ("creature_template_spell", &["CreatureID", "Index", "Spell"]),
    (
        "creature",
        &[
            "guid",
            "id",
            "map",
            "zoneId",
            "areaId",
            "position_x",
            "position_y",
            "position_z",
            "orientation",
            "spawntimesecs",
        ],
    ),
    (
        "gameobject_template",
        &[
            "entry",
            "type",
            "displayId",
            "name",
            "castBarCaption",
            "size",
            "Data0",
            "Data1",
            "Data2",
            "Data3",
            "Data4",
            "Data5",
            "Data6",
            "Data7",
            "Data8",
            "Data9",
            "Data10",
            "Data11",
            "Data12",
            "Data13",
            "Data14",
            "Data15",
            "Data16",
            "Data17",
            "Data18",
            "Data19",
            "Data20",
            "Data21",
            "Data22",
            "Data23",
            "AIName",
            "ScriptName",
        ],
    ),
    (
        "gameobject",
        &[
            "guid",
            "id",
            "map",
            "zoneId",
            "areaId",
            "position_x",
            "position_y",
            "position_z",
            "orientation",
            "spawntimesecs",
        ],
    ),
];

const DATA_COLUMNS: [&str; 24] = [
    "Data0", "Data1", "Data2", "Data3", "Data4", "Data5", "Data6", "Data7", "Data8", "Data9", "Data10", "Data11",
    "Data12", "Data13", "Data14", "Data15", "Data16", "Data17", "Data18", "Data19", "Data20", "Data21", "Data22",
    "Data23",
];

fn immune_mechanics(mask: u32) -> Option<String> {
    if mask == 0 {
        return None;
    }
    let mut result = String::new();
    for bit in 0..32 {
        let flag = 1_u32 << bit;
        if mask & flag == 0 {
            continue;
        }
        if !result.is_empty() {
            result.push('|');
        }
        if let Some((_, name)) = names::MECHANICS.iter().find(|(id, _)| *id == i64::from(bit) + 1) {
            result.push_str(name);
        } else {
            let _ = write!(result, "0x{flag:08X}");
        }
    }
    Some(result)
}

fn nonzero(value: i64) -> Option<i64> {
    (value != 0).then_some(value)
}

fn flags(row: &Row<'_>, column: &str) -> Result<u32, BuildError> {
    let value = row.int(column)?;
    u32::try_from(value).map_err(|_| BuildError::Dump {
        line: row.line(),
        message: format!("invalid unsigned 32-bit flags in {column}: {value}"),
    })
}

#[expect(
    clippy::float_cmp,
    reason = "Dump defaults are exact decimal values, not computed measurements"
)]
fn nondefault(value: f64, default: f64) -> Option<f64> {
    (value != default).then_some(value)
}

fn nonempty(value: Option<&str>) -> Option<&str> {
    value.filter(|text| !text.is_empty())
}

fn optional_enum(table: &[(i64, &str)], value: i64) -> Option<String> {
    nonzero(value).and_then(|value| names::enum_name(table, value))
}

pub(super) struct Importer<'conn> {
    creature: Statement<'conn>,
    creature_spell: Statement<'conn>,
    creature_spawn: Statement<'conn>,
    gameobject: Statement<'conn>,
    gameobject_spawn: Statement<'conn>,
}

impl<'conn> Importer<'conn> {
    pub(super) fn new(conn: &'conn Connection) -> Result<Self, BuildError> {
        Ok(Self {
            creature: conn.prepare("INSERT INTO creature (id,name,subname,min_level,max_level,rank,type,family,faction_template,faction_id,faction,npc_flags,unit_class,type_flags,expansion,loot_id,pickpocket_loot,skin_loot,min_gold,max_gold,health_modifier,mana_modifier,armor_modifier,damage_modifier,speed_walk,speed_run,scale,models,mechanic_immune,vehicle_id,ai_name,script_name) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)")?,
            creature_spell: conn.prepare("INSERT INTO creature_spell (creature_id,slot,spell_id) VALUES (?,?,?)")?,
            creature_spawn: conn.prepare("INSERT INTO creature_spawn (guid,creature_id,map,zone,area,x,y,z,orientation,spawn_time) VALUES (?,?,?,?,?,?,?,?,?,?)")?,
            gameobject: conn.prepare("INSERT INTO gameobject (id,name,type,display_id,size,loot_id,data,cast_bar_caption,ai_name,script_name) VALUES (?,?,?,?,?,?,?,?,?,?)")?,
            gameobject_spawn: conn.prepare("INSERT INTO gameobject_spawn (guid,gameobject_id,map,zone,area,x,y,z,orientation,spawn_time) VALUES (?,?,?,?,?,?,?,?,?,?)")?,
        })
    }

    #[expect(
        clippy::too_many_lines,
        reason = "The related creature and gameobject row mappings are kept together"
    )]
    pub(super) fn import(&mut self, table: &str, row: &Row<'_>, dbc: &Context) -> Result<(), BuildError> {
        match table {
            "creature_template" => {
                let faction_template = row.int("faction")?;
                let faction_id = i32::try_from(faction_template)
                    .ok()
                    .and_then(|id| dbc.faction_templates.get(&id))
                    .copied()
                    .filter(|id| *id != 0);
                let faction = faction_id.and_then(|id| dbc.faction_names.get(&id));
                let mut models = String::new();
                for column in ["modelid1", "modelid2", "modelid3", "modelid4"] {
                    let model = row.int(column)?;
                    if model != 0 {
                        if !models.is_empty() {
                            models.push_str(", ");
                        }
                        let _ = write!(models, "{model}");
                    }
                }
                self.creature.execute(params![
                    row.int("entry")?,
                    row.text("name")?,
                    row.text("subname")?,
                    nonzero(row.int("minlevel")?),
                    nonzero(row.int("maxlevel")?),
                    optional_enum(names::CREATURE_ELITE_TYPE, row.int("rank")?),
                    optional_enum(names::CREATURE_TYPE, row.int("type")?),
                    optional_enum(names::CREATURE_FAMILY, row.int("family")?),
                    nonzero(faction_template),
                    faction_id,
                    faction,
                    names::flag_names(names::NPC_FLAGS, flags(row, "npcflag")?),
                    optional_enum(names::UNIT_CLASS, row.int("unit_class")?),
                    names::flag_names(names::CREATURE_TYPE_FLAGS, flags(row, "type_flags")?),
                    nonzero(row.int("exp")?),
                    nonzero(row.int("lootid")?),
                    nonzero(row.int("pickpocketloot")?),
                    nonzero(row.int("skinloot")?),
                    nonzero(row.int("mingold")?),
                    nonzero(row.int("maxgold")?),
                    nondefault(row.real("HealthModifier")?, 1.0),
                    nondefault(row.real("ManaModifier")?, 1.0),
                    nondefault(row.real("ArmorModifier")?, 1.0),
                    nondefault(row.real("DamageModifier")?, 1.0),
                    nondefault(row.real("speed_walk")?, 1.0),
                    nondefault(row.real("speed_run")?, 1.14286),
                    nondefault(row.real("scale")?, 1.0),
                    nonempty(Some(&models)),
                    immune_mechanics(flags(row, "mechanic_immune_mask")?),
                    nonzero(row.int("VehicleId")?),
                    row.text("AIName")?,
                    row.text("ScriptName")?,
                ])?;
            }
            "creature_template_spell" => {
                self.creature_spell.execute(params![
                    row.int("CreatureID")?,
                    row.int("Index")?,
                    nonzero(row.int("Spell")?)
                ])?;
            }
            "creature" | "gameobject" => {
                let statement = if table == "creature" {
                    &mut self.creature_spawn
                } else {
                    &mut self.gameobject_spawn
                };
                statement.execute(params![
                    row.int("guid")?,
                    row.int("id")?,
                    row.int("map")?,
                    nonzero(row.int("zoneId")?),
                    nonzero(row.int("areaId")?),
                    nondefault(row.real("position_x")?, 0.0),
                    nondefault(row.real("position_y")?, 0.0),
                    nondefault(row.real("position_z")?, 0.0),
                    nondefault(row.real("orientation")?, 0.0),
                    nonzero(row.int("spawntimesecs")?),
                ])?;
            }
            "gameobject_template" => {
                let kind = row.int("type")?;
                let mut data = String::new();
                let mut loot = None;
                for (index, column) in DATA_COLUMNS.iter().enumerate() {
                    let value = row.int(column)?;
                    if value != 0 {
                        if !data.is_empty() {
                            data.push(' ');
                        }
                        let _ = write!(data, "{index}={value}");
                        if (kind == 3 || kind == 25) && index == 1 {
                            loot = Some(value);
                        }
                    }
                }
                self.gameobject.execute(params![
                    row.int("entry")?,
                    row.text("name")?,
                    names::enum_name(names::GAMEOBJECT_TYPES, kind),
                    nonzero(row.int("displayId")?),
                    nondefault(row.real("size")?, 1.0),
                    loot,
                    nonempty(Some(&data)),
                    row.text("castBarCaption")?,
                    row.text("AIName")?,
                    row.text("ScriptName")?
                ])?;
            }
            _ => {
                return Err(BuildError::Dump {
                    line: 0,
                    message: format!("unknown creature import table: {table}"),
                });
            }
        }
        Ok(())
    }
}
