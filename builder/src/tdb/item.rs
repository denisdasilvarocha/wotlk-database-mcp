use std::fmt::Write;

use rusqlite::{Connection, Statement, params};

use crate::{BuildError, dbc::Context, dump::Row, names};

pub(super) const TABLES: &[(&str, &[&str])] = &[(
    "item_template",
    &[
        "entry",
        "class",
        "subclass",
        "name",
        "displayid",
        "Quality",
        "Flags",
        "FlagsExtra",
        "BuyCount",
        "BuyPrice",
        "SellPrice",
        "InventoryType",
        "AllowableClass",
        "AllowableRace",
        "ItemLevel",
        "RequiredLevel",
        "RequiredSkill",
        "RequiredSkillRank",
        "requiredspell",
        "RequiredReputationFaction",
        "RequiredReputationRank",
        "maxcount",
        "stackable",
        "ContainerSlots",
        "stat_type1",
        "stat_value1",
        "stat_type2",
        "stat_value2",
        "stat_type3",
        "stat_value3",
        "stat_type4",
        "stat_value4",
        "stat_type5",
        "stat_value5",
        "stat_type6",
        "stat_value6",
        "stat_type7",
        "stat_value7",
        "stat_type8",
        "stat_value8",
        "stat_type9",
        "stat_value9",
        "stat_type10",
        "stat_value10",
        "dmg_min1",
        "dmg_max1",
        "dmg_type1",
        "dmg_min2",
        "dmg_max2",
        "dmg_type2",
        "armor",
        "holy_res",
        "fire_res",
        "nature_res",
        "frost_res",
        "shadow_res",
        "arcane_res",
        "delay",
        "spellid_1",
        "spelltrigger_1",
        "spellcharges_1",
        "spellcooldown_1",
        "spellid_2",
        "spelltrigger_2",
        "spellcharges_2",
        "spellcooldown_2",
        "spellid_3",
        "spelltrigger_3",
        "spellcharges_3",
        "spellcooldown_3",
        "spellid_4",
        "spelltrigger_4",
        "spellcharges_4",
        "spellcooldown_4",
        "spellid_5",
        "spelltrigger_5",
        "spellcharges_5",
        "spellcooldown_5",
        "bonding",
        "description",
        "PageText",
        "startquest",
        "lockid",
        "RandomProperty",
        "RandomSuffix",
        "block",
        "itemset",
        "MaxDurability",
        "area",
        "Map",
        "BagFamily",
        "TotemCategory",
        "socketColor_1",
        "socketColor_2",
        "socketColor_3",
        "socketBonus",
        "GemProperties",
        "RequiredDisenchantSkill",
        "duration",
        "ItemLimitCategory",
        "DisenchantID",
    ],
)];

fn nonzero(row: &Row<'_>, column: &str) -> Result<Option<i64>, BuildError> {
    let value = row.int(column)?;
    Ok((value != 0).then_some(value))
}

fn joined(values: &[String], separator: &str) -> Option<String> {
    if values.is_empty() {
        None
    } else {
        Some(values.join(separator))
    }
}

fn checked_u32(value: i64, column: &str, line: u64) -> Result<u32, BuildError> {
    u32::try_from(value).map_err(|_| BuildError::Dump {
        line,
        message: format!("{column} out of range for u32: {value}"),
    })
}

fn checked_i32(value: i64, column: &str, line: u64) -> Result<i32, BuildError> {
    i32::try_from(value).map_err(|_| BuildError::Dump {
        line,
        message: format!("{column} out of range for i32: {value}"),
    })
}

pub(super) struct Importer<'conn> {
    item: Statement<'conn>,
    item_spell: Statement<'conn>,
}

impl<'conn> Importer<'conn> {
    pub(super) fn new(conn: &'conn Connection) -> Result<Self, BuildError> {
        Ok(Self {
            item: conn.prepare("INSERT INTO item (id, name, quality, item_level, required_level, class, subclass, slot, bonding, flags, flags2, display_id, icon, description, buy_price, buy_count, sell_price, max_count, stackable, container_slots, allowed_classes, allowed_races, required_skill, required_skill_rank, required_spell, required_faction, required_reputation, stats, armor, damage, speed, block, resistances, spells, sockets, socket_bonus, gem_properties, itemset, start_quest, lock_id, max_durability, duration, area, map, bag_family, totem_category, random_property, random_suffix, disenchant_id, required_disenchant_skill, item_limit_category, page_text) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")?,
            item_spell: conn.prepare(
                "INSERT INTO item_spell (item_id, slot, spell_id, trigger, charges, cooldown) VALUES (?, ?, ?, ?, ?, ?)",
            )?,
        })
    }

    pub(super) fn import(&mut self, _table: &str, row: &Row<'_>, dbc: &Context) -> Result<(), BuildError> {
        import(&mut self.item, &mut self.item_spell, row, dbc)
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "The item row mapping is kept together with its SQL insert"
)]
fn import(
    insert: &mut Statement<'_>,
    insert_spell: &mut Statement<'_>,
    row: &Row<'_>,
    dbc: &Context,
) -> Result<(), BuildError> {
    let id = row.int("entry")?;
    let class = row.int("class")?;
    let subclass = row.int("subclass")?;
    let display = row.int("displayid")?;
    let display_id = checked_i32(display, "displayid", row.line())?;
    let slot = row.int("InventoryType")?;
    let bonding = row.int("bonding")?;
    let allowed_classes = row.int("AllowableClass")?;
    let allowed_races = row.int("AllowableRace")?;
    let class_mask = if allowed_classes == -1 {
        0
    } else {
        checked_u32(allowed_classes, "AllowableClass", row.line())?
    };
    let race_mask = if allowed_races == -1 {
        0
    } else {
        checked_u32(allowed_races, "AllowableRace", row.line())?
    };
    let faction = row.int("RequiredReputationFaction")?;
    let buy_count = row.int("BuyCount")?;
    let stackable = row.int("stackable")?;
    let disenchant_skill = row.int("RequiredDisenchantSkill")?;
    let mut stats = Vec::new();
    for (kind, value) in [
        ("stat_type1", "stat_value1"),
        ("stat_type2", "stat_value2"),
        ("stat_type3", "stat_value3"),
        ("stat_type4", "stat_value4"),
        ("stat_type5", "stat_value5"),
        ("stat_type6", "stat_value6"),
        ("stat_type7", "stat_value7"),
        ("stat_type8", "stat_value8"),
        ("stat_type9", "stat_value9"),
        ("stat_type10", "stat_value10"),
    ] {
        let amount = row.int(value)?;
        if amount != 0 {
            let name = names::enum_name(names::ITEM_MOD_TYPE, row.int(kind)?).unwrap_or_default();
            stats.push(format!("{name} {amount:+}"));
        }
    }
    let mut damage = Vec::new();
    for (min, max, school) in [
        ("dmg_min1", "dmg_max1", "dmg_type1"),
        ("dmg_min2", "dmg_max2", "dmg_type2"),
    ] {
        let min = row.real(min)?;
        let max = row.real(max)?;
        if max > 0.0 {
            let school = names::enum_name(names::SPELL_SCHOOLS, row.int(school)?).unwrap_or_default();
            damage.push(format!("{min}-{max} {school}"));
        }
    }
    let mut resistances = Vec::new();
    for (column, name) in [
        ("holy_res", "HOLY"),
        ("fire_res", "FIRE"),
        ("nature_res", "NATURE"),
        ("frost_res", "FROST"),
        ("shadow_res", "SHADOW"),
        ("arcane_res", "ARCANE"),
    ] {
        let value = row.int(column)?;
        if value != 0 {
            resistances.push(format!("{name} {value:+}"));
        }
    }
    let mut spells = Vec::new();
    for ((spell, trigger, charges, cooldown), slot) in [
        ("spellid_1", "spelltrigger_1", "spellcharges_1", "spellcooldown_1"),
        ("spellid_2", "spelltrigger_2", "spellcharges_2", "spellcooldown_2"),
        ("spellid_3", "spelltrigger_3", "spellcharges_3", "spellcooldown_3"),
        ("spellid_4", "spelltrigger_4", "spellcharges_4", "spellcooldown_4"),
        ("spellid_5", "spelltrigger_5", "spellcharges_5", "spellcooldown_5"),
    ]
    .into_iter()
    .zip(1_i64..)
    {
        let spell_value = row.int(spell)?;
        if spell_value <= 0 {
            continue;
        }
        let spell_id = checked_i32(spell_value, spell, row.line())?;
        let spell = spell_value;
        let trigger = names::enum_name(names::ITEM_SPELLTRIGGER_TYPE, row.int(trigger)?);
        let charges = row.int(charges)?;
        let cooldown = row.int(cooldown)?;
        insert_spell.execute(params![
            id,
            slot,
            spell,
            trigger,
            (charges != 0).then_some(charges),
            (cooldown != -1).then_some(cooldown)
        ])?;
        let mut text = format!("{} {spell}", trigger.as_deref().unwrap_or_default());
        if let Some(name) = dbc.spell_names.get(&spell_id).filter(|s| !s.is_empty()) {
            let _ = write!(text, " {name}");
        }
        spells.push(text);
    }
    let mut sockets = Vec::new();
    for column in ["socketColor_1", "socketColor_2", "socketColor_3"] {
        if let Some(color) = names::flag_names(names::SOCKET_COLOR, checked_u32(row.int(column)?, column, row.line())?)
        {
            sockets.push(color);
        }
    }
    let delay = checked_u32(row.int("delay")?, "delay", row.line())?;
    insert.execute(params![
        id,
        row.text("name")?,
        names::enum_name(names::ITEM_QUALITIES, row.int("Quality")?),
        nonzero(row, "ItemLevel")?,
        nonzero(row, "RequiredLevel")?,
        names::enum_name(names::ITEM_CLASS, class),
        names::item_subclass(class, subclass),
        if slot == 0 {
            None
        } else {
            names::enum_name(names::INVENTORY_TYPE, slot)
        },
        if bonding == 0 {
            None
        } else {
            names::enum_name(names::ITEM_BONDING_TYPE, bonding)
        },
        names::flag_names(names::ITEM_FLAGS, checked_u32(row.int("Flags")?, "Flags", row.line())?),
        names::flag_names(
            names::ITEM_FLAGS2,
            checked_u32(row.int("FlagsExtra")?, "FlagsExtra", row.line())?
        ),
        (display != 0).then_some(display),
        dbc.display_icons.get(&display_id),
        row.text("description")?,
        nonzero(row, "BuyPrice")?,
        (buy_count != 0 && buy_count != 1).then_some(buy_count),
        nonzero(row, "SellPrice")?,
        nonzero(row, "maxcount")?,
        (stackable != 0 && stackable != 1).then_some(stackable),
        nonzero(row, "ContainerSlots")?,
        if class_mask == 0 || class_mask & 0x5ff == 0x5ff {
            None
        } else {
            names::flag_names(names::CLASSES, class_mask)
        },
        if race_mask == 0 || race_mask & 0x6ff == 0x6ff {
            None
        } else {
            names::flag_names(names::RACES, race_mask)
        },
        nonzero(row, "RequiredSkill")?,
        nonzero(row, "RequiredSkillRank")?,
        nonzero(row, "requiredspell")?,
        (faction != 0).then_some(faction),
        if faction == 0 {
            None
        } else {
            names::enum_name(names::REPUTATION_RANK, row.int("RequiredReputationRank")?)
        },
        joined(&stats, ", "),
        nonzero(row, "armor")?,
        joined(&damage, "; "),
        (class == 2 && delay != 0).then_some(f64::from(delay) / 1000.0),
        nonzero(row, "block")?,
        joined(&resistances, ", "),
        joined(&spells, "; "),
        joined(&sockets, ", "),
        nonzero(row, "socketBonus")?,
        nonzero(row, "GemProperties")?,
        nonzero(row, "itemset")?,
        nonzero(row, "startquest")?,
        nonzero(row, "lockid")?,
        nonzero(row, "MaxDurability")?,
        nonzero(row, "duration")?,
        nonzero(row, "area")?,
        nonzero(row, "Map")?,
        nonzero(row, "BagFamily")?,
        nonzero(row, "TotemCategory")?,
        nonzero(row, "RandomProperty")?,
        nonzero(row, "RandomSuffix")?,
        nonzero(row, "DisenchantID")?,
        (disenchant_skill != -1).then_some(disenchant_skill),
        nonzero(row, "ItemLimitCategory")?,
        nonzero(row, "PageText")?
    ])?;
    Ok(())
}
