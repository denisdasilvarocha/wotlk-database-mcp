//! Import the client DBC tables and retain lookups used by the world importer.

use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use rusqlite::{Connection, params};
use wow_dbc::{
    DbcTable,
    wrath_tables::{
        achievement::Achievement, achievement_category::Achievement_Category,
        achievement_criteria::Achievement_Criteria, area_table::AreaTable, faction::Faction,
        faction_template::FactionTemplate, glyph_properties::GlyphProperties, item::Item,
        item_display_info::ItemDisplayInfo, item_extended_cost::ItemExtendedCost, item_set::ItemSet, map::Map,
        skill_line::SkillLine, skill_line_ability::SkillLineAbility, spell::Spell, spell_cast_times::SpellCastTimes,
        spell_duration::SpellDuration, spell_icon::SpellIcon, spell_radius::SpellRadius, spell_range::SpellRange,
        talent::Talent, talent_tab::TalentTab,
    },
};

use crate::{BuildError, names, text::real};

pub(crate) const FILES: &[&str] = &[
    Spell::FILENAME,
    SpellDuration::FILENAME,
    SpellRadius::FILENAME,
    SpellRange::FILENAME,
    SpellCastTimes::FILENAME,
    SpellIcon::FILENAME,
    Item::FILENAME,
    ItemSet::FILENAME,
    ItemDisplayInfo::FILENAME,
    ItemExtendedCost::FILENAME,
    Map::FILENAME,
    AreaTable::FILENAME,
    Faction::FILENAME,
    FactionTemplate::FILENAME,
    Talent::FILENAME,
    TalentTab::FILENAME,
    SkillLine::FILENAME,
    SkillLineAbility::FILENAME,
    GlyphProperties::FILENAME,
    Achievement::FILENAME,
    Achievement_Criteria::FILENAME,
    Achievement_Category::FILENAME,
];

pub(crate) struct ItemFallback {
    pub(crate) id: i32,
    pub(crate) class_id: i32,
    pub(crate) display_id: i32,
}

pub(crate) struct Context {
    pub(crate) spell_names: HashMap<i32, String>,
    pub(crate) display_icons: HashMap<i32, String>,
    pub(crate) faction_templates: HashMap<i32, i32>,
    pub(crate) faction_names: HashMap<i32, String>,
    pub(crate) area_names: HashMap<i32, String>,
    pub(crate) extended_costs: HashMap<i32, String>,
    pub(crate) items: Vec<ItemFallback>,
}

struct Tables {
    spell: Spell,
    duration: SpellDuration,
    radius: SpellRadius,
    range: SpellRange,
    cast_times: SpellCastTimes,
    icon: SpellIcon,
    item: Item,
    item_set: ItemSet,
    item_display: ItemDisplayInfo,
    extended_cost: ItemExtendedCost,
    map: Map,
    area: AreaTable,
    faction: Faction,
    faction_template: FactionTemplate,
    talent: Talent,
    talent_tab: TalentTab,
    skill: SkillLine,
    skill_ability: SkillLineAbility,
    glyph: GlyphProperties,
    achievement: Achievement,
    achievement_criteria: Achievement_Criteria,
    achievement_category: Achievement_Category,
}

fn read_table<T: DbcTable>(dir: &Path) -> Result<T, BuildError> {
    let path = dir.join(T::FILENAME);
    let file = File::open(&path).map_err(|source| BuildError::Io { path, source })?;
    T::read(&mut BufReader::new(file)).map_err(|err| BuildError::Dbc {
        file: T::FILENAME.to_owned(),
        message: format!("does not match the 3.3.5a (12340) layout or is unreadable: {err}"),
    })
}

fn nz(value: i32) -> Option<i32> {
    (value != 0).then_some(value)
}
fn txt(value: &str) -> Option<&str> {
    (!value.is_empty()).then_some(value)
}

pub(crate) fn import(conn: &Connection, dir: &Path) -> Result<Context, BuildError> {
    let spell: Spell = read_table(dir)?;
    if !spell
        .rows
        .iter()
        .any(|row| row.id.id == 133 && row.name_lang.en_gb == "Fireball")
    {
        return Err(BuildError::Dbc {
            file: "Spell.dbc".into(),
            message: "not an enUS 12340 client: record 133 is missing or its enUS name is not \"Fireball\"".into(),
        });
    }
    let tables = Tables {
        spell,
        duration: read_table(dir)?,
        radius: read_table(dir)?,
        range: read_table(dir)?,
        cast_times: read_table(dir)?,
        icon: read_table(dir)?,
        item: read_table(dir)?,
        item_set: read_table(dir)?,
        item_display: read_table(dir)?,
        extended_cost: read_table(dir)?,
        map: read_table(dir)?,
        area: read_table(dir)?,
        faction: read_table(dir)?,
        faction_template: read_table(dir)?,
        talent: read_table(dir)?,
        talent_tab: read_table(dir)?,
        skill: read_table(dir)?,
        skill_ability: read_table(dir)?,
        glyph: read_table(dir)?,
        achievement: read_table(dir)?,
        achievement_criteria: read_table(dir)?,
        achievement_category: read_table(dir)?,
    };
    let context = context_from_tables(&tables);
    import_spells(conn, &tables)?;
    import_world(conn, &tables, &context.spell_names)?;
    Ok(context)
}

fn context_from_tables(tables: &Tables) -> Context {
    let spell_names = tables
        .spell
        .rows
        .iter()
        .filter_map(|row| txt(&row.name_lang.en_gb).map(|name| (row.id.id, name.to_owned())))
        .collect();
    let display_icons = tables
        .item_display
        .rows
        .iter()
        .filter_map(|row| txt(&row.inventory_icon[0]).map(|icon| (row.id.id, icon.to_owned())))
        .collect();
    let faction_templates = tables
        .faction_template
        .rows
        .iter()
        .map(|row| (row.id.id, row.faction.id))
        .collect();
    let faction_names = tables
        .faction
        .rows
        .iter()
        .filter_map(|row| txt(&row.name_lang.en_gb).map(|name| (row.id.id, name.to_owned())))
        .collect();
    let area_names = tables
        .area
        .rows
        .iter()
        .filter_map(|row| txt(&row.area_name_lang.en_gb).map(|name| (row.id.id, name.to_owned())))
        .collect();
    let extended_costs = tables
        .extended_cost
        .rows
        .iter()
        .filter_map(|row| {
            let mut parts = Vec::new();
            for (name, value) in [
                ("honor", row.honor_points),
                ("arena", row.arena_points),
                ("rating", row.required_arena_rating),
            ] {
                if value != 0 {
                    parts.push(format!("{name}:{value}"));
                }
            }
            for (&id, &count) in row.item_id.iter().zip(&row.item_count) {
                if id != 0 {
                    parts.push(format!("{id}x{count}"));
                }
            }
            (!parts.is_empty()).then(|| (row.id.id, parts.join(",")))
        })
        .collect();
    let items = tables
        .item
        .rows
        .iter()
        .map(|row| ItemFallback {
            id: row.id.id,
            class_id: row.class_id,
            display_id: row.display_info_id,
        })
        .collect();
    Context {
        spell_names,
        display_icons,
        faction_templates,
        faction_names,
        area_names,
        extended_costs,
        items,
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "Keeping the fixed DBC-to-SQL mappings together makes the import order and shared lookups clear"
)]
fn import_world(conn: &Connection, tables: &Tables, spell_names: &HashMap<i32, String>) -> Result<(), BuildError> {
    let maps: HashMap<_, _> = tables
        .map
        .rows
        .iter()
        .map(|r| (r.id.id, r.map_name_lang.en_gb.as_str()))
        .collect();
    let areas: HashMap<_, _> = tables
        .area
        .rows
        .iter()
        .map(|r| (r.id.id, r.area_name_lang.en_gb.as_str()))
        .collect();
    let factions: HashMap<_, _> = tables
        .faction
        .rows
        .iter()
        .map(|r| (r.id.id, r.name_lang.en_gb.as_str()))
        .collect();
    let tabs: HashMap<_, _> = tables.talent_tab.rows.iter().map(|r| (r.id.id, r)).collect();
    let icons: HashMap<_, _> = tables
        .icon
        .rows
        .iter()
        .map(|r| (r.id.id, r.texture_filename.as_str()))
        .collect();
    let categories: HashMap<_, _> = tables
        .achievement_category
        .rows
        .iter()
        .map(|r| (r.id.id, r.name_lang.en_gb.as_str()))
        .collect();
    let icon = |id: i32| icons.get(&id).and_then(|s| txt(s));
    let spell_name = |id: i32| spell_names.get(&id).and_then(|s| txt(s));

    {
        let mut stmt = conn.prepare("INSERT INTO map (id,name,directory,type,expansion,max_players,area_id,description_alliance,description_horde) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)")?;
        for r in &tables.map.rows {
            stmt.execute(params![
                r.id.id,
                txt(&r.map_name_lang.en_gb),
                txt(&r.directory),
                names::enum_name(names::MAP_TYPES, r.instance_type.into()),
                nz(r.expansion_id),
                nz(r.max_players),
                nz(r.area_table_id.id),
                txt(&r.map_description0_lang.en_gb),
                txt(&r.map_description1_lang.en_gb)
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare("INSERT INTO area (id,name,map_id,map,parent_id,parent,level,flags,area_bit,team) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)")?;
        for r in &tables.area.rows {
            stmt.execute(params![
                r.id.id,
                txt(&r.area_name_lang.en_gb),
                r.continent_id.id,
                maps.get(&r.continent_id.id).and_then(|s| txt(s)),
                nz(r.parent_area_id.id),
                nz(r.parent_area_id.id)
                    .and_then(|id| areas.get(&id))
                    .and_then(|s| txt(s)),
                nz(r.exploration_level),
                names::flag_names(names::AREA_FLAGS, r.flags.cast_unsigned()),
                r.area_bit,
                (r.faction_group_mask != 0)
                    .then(|| names::enum_name(names::AREA_TEAMS, r.faction_group_mask.into()))
                    .flatten()
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare(
            "INSERT INTO faction (id,name,description,reputation_index,parent_id,parent) VALUES (?1,?2,?3,?4,?5,?6)",
        )?;
        for r in &tables.faction.rows {
            stmt.execute(params![
                r.id.id,
                txt(&r.name_lang.en_gb),
                txt(&r.description_lang.en_gb),
                (r.reputation_index != -1).then_some(r.reputation_index),
                nz(r.parent_faction_id.id),
                nz(r.parent_faction_id.id)
                    .and_then(|id| factions.get(&id))
                    .and_then(|s| txt(s))
            ])?;
        }
    }
    {
        let mut stmt =
            conn.prepare("INSERT INTO talent_tab (id,name,class,order_index,icon) VALUES (?1,?2,?3,?4,?5)")?;
        for r in &tables.talent_tab.rows {
            stmt.execute(params![
                r.id.id,
                txt(&r.name_lang.en_gb),
                names::flag_names(names::CLASSES, r.class_mask.cast_unsigned()),
                r.order_index,
                icon(r.spell_icon_id.id)
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare("INSERT INTO talent (id,name,tab_id,tab,class,tier,column_index,max_rank,ranks,prereq_talent,prereq_rank,required_spell) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)")?;
        let mut rank_stmt = conn.prepare("INSERT INTO talent_spell (talent_id,rank,spell_id) VALUES (?1,?2,?3)")?;
        for r in &tables.talent.rows {
            let tab = tabs.get(&r.tab_id);
            let ranks: Vec<_> = r.spell_rank.iter().filter(|&&id| id != 0).map(i32::to_string).collect();
            for (index, &id) in (1_i32..).zip(&r.spell_rank).filter(|(_, id)| **id != 0) {
                rank_stmt.execute(params![r.id.id, index, id])?;
            }
            let ranks_text = ranks.join(",");
            stmt.execute(params![
                r.id.id,
                spell_name(r.spell_rank[0]),
                nz(r.tab_id),
                tab.and_then(|t| txt(&t.name_lang.en_gb)),
                tab.and_then(|t| names::flag_names(names::CLASSES, t.class_mask.cast_unsigned())),
                r.tier_id,
                r.column_index,
                r.spell_rank.iter().map(|&id| i32::from(id != 0)).sum::<i32>(),
                txt(&ranks_text),
                nz(r.prereq_talent[0]),
                (r.prereq_talent[0] != 0).then_some(r.prereq_rank[0] + 1),
                nz(r.required_spell_id.id)
            ])?;
        }
    }
    {
        let mut stmt =
            conn.prepare("INSERT INTO skill (id,name,category,description,icon,can_link) VALUES (?1,?2,?3,?4,?5,?6)")?;
        for r in &tables.skill.rows {
            stmt.execute(params![
                r.id.id,
                txt(&r.display_name_lang.en_gb),
                names::enum_name(names::SKILL_CATEGORY, r.category_id.id.into()),
                txt(&r.description_lang.en_gb),
                icon(r.spell_icon_id.id),
                nz(r.can_link)
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare("INSERT INTO skill_ability (id,skill_id,spell_id,min_skill_rank,superseded_by,acquire_method,trivial_high,trivial_low,classes,races) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)")?;
        for r in &tables.skill_ability.rows {
            stmt.execute(params![
                r.id.id,
                nz(r.skill_line.id),
                nz(r.spell.id),
                nz(r.min_skill_line_rank),
                nz(r.superceded_by_spell.id),
                nz(r.acquire_method),
                nz(r.trivial_skill_line_rank_high),
                nz(r.trivial_skill_line_rank_low),
                names::flag_names(names::CLASSES, r.class_mask.cast_unsigned()),
                names::flag_names(names::RACES, r.race_mask.cast_unsigned())
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare("INSERT INTO glyph (id,name,spell_id,type,icon) VALUES (?1,?2,?3,?4,?5)")?;
        for r in &tables.glyph.rows {
            stmt.execute(params![
                r.id.id,
                spell_name(r.spell_id.id),
                nz(r.spell_id.id),
                if r.glyph_slot_flags & 1 != 0 { "MINOR" } else { "MAJOR" },
                icon(r.spell_icon_id.id)
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare("INSERT INTO achievement (id,name,description,category_id,category,points,faction,map_id,flags,reward,icon,previous,min_criteria) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)")?;
        for r in &tables.achievement.rows {
            let faction = match r.faction.id {
                -1 => None,
                0 => Some("HORDE".to_owned()),
                1 => Some("ALLIANCE".to_owned()),
                n => Some(n.to_string()),
            };
            stmt.execute(params![
                r.id.id,
                txt(&r.title_lang.en_gb),
                txt(&r.description_lang.en_gb),
                nz(r.category.id),
                categories.get(&r.category.id).and_then(|s| txt(s)),
                nz(r.points),
                faction,
                (r.instance_id.id != -1).then_some(r.instance_id.id),
                names::flag_names(names::ACHIEVEMENT_FLAGS, r.flags.cast_unsigned()),
                txt(&r.reward_lang.en_gb),
                icon(r.icon_id.id),
                nz(r.supercedes.id),
                nz(r.minimum_criteria)
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare("INSERT INTO achievement_criteria (id,achievement_id,type,asset_id,quantity,description,flags,timer_time,ui_order) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)")?;
        for r in &tables.achievement_criteria.rows {
            stmt.execute(params![
                r.id.id,
                nz(r.achievement_id.id),
                names::enum_name(names::ACHIEVEMENT_CRITERIA_TYPES, r.ty.into()),
                nz(r.asset_id),
                nz(r.quantity),
                txt(&r.description_lang.en_gb),
                nz(r.flags),
                nz(r.timer_time),
                nz(r.ui_order)
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare("INSERT INTO itemset (id,name,items,bonuses,required_skill,required_skill_rank) VALUES (?1,?2,NULL,?3,?4,?5)")?;
        let mut item_stmt = conn.prepare("INSERT INTO itemset_item (itemset_id,slot,item_id) VALUES (?1,?2,?3)")?;
        let mut bonus_stmt =
            conn.prepare("INSERT INTO itemset_bonus (itemset_id,threshold,spell_id) VALUES (?1,?2,?3)")?;
        for r in &tables.item_set.rows {
            for (slot, &item) in (0_i32..).zip(&r.item_id).filter(|(_, id)| **id != 0) {
                item_stmt.execute(params![r.id.id, slot, item])?;
            }
            let mut bonuses: Vec<_> = r
                .set_spell_id
                .iter()
                .copied()
                .zip(r.set_threshold.iter().copied())
                .filter(|(spell, _)| *spell != 0)
                .collect();
            bonuses.sort_by_key(|&(_, threshold)| threshold);
            let mut rendered = Vec::with_capacity(bonuses.len());
            for (spell, threshold) in bonuses {
                bonus_stmt.execute(params![r.id.id, threshold, spell])?;
                let mut text = format!("{threshold}: {spell}");
                if let Some(name) = spell_name(spell) {
                    text.push(' ');
                    text.push_str(name);
                }
                rendered.push(text);
            }
            let text = rendered.join("; ");
            stmt.execute(params![
                r.id.id,
                txt(&r.name_lang.en_gb),
                txt(&text),
                nz(r.required_skill.id),
                nz(r.required_skill_rank)
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare("INSERT INTO item_display (id,icon,model,model_texture) VALUES (?1,?2,?3,?4)")?;
        for r in &tables.item_display.rows {
            stmt.execute(params![
                r.id.id,
                txt(&r.inventory_icon[0]),
                txt(&r.model_name[0]),
                txt(&r.model_texture[0])
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare("INSERT INTO spell_icon (id,texture) VALUES (?1,?2)")?;
        for r in &tables.icon.rows {
            stmt.execute(params![r.id.id, txt(&r.texture_filename)])?;
        }
    }
    Ok(())
}
#[expect(
    clippy::too_many_lines,
    reason = "Keeping the fixed database column mapping together makes it easier to audit"
)]
fn import_spells(conn: &Connection, tables: &Tables) -> Result<(), BuildError> {
    use crate::spell_text::{SpellNumbers, resolve};
    use rusqlite::params;
    use std::collections::HashMap;
    use std::fmt::Write as _;

    let durations: HashMap<_, _> = tables.duration.rows.iter().map(|r| (r.id.id, r)).collect();
    let radii: HashMap<_, _> = tables.radius.rows.iter().map(|r| (r.id.id, r)).collect();
    let ranges: HashMap<_, _> = tables.range.rows.iter().map(|r| (r.id.id, r)).collect();
    let casts: HashMap<_, _> = tables.cast_times.rows.iter().map(|r| (r.id.id, r)).collect();
    let icons: HashMap<_, _> = tables.icon.rows.iter().map(|r| (r.id.id, r)).collect();
    let numbers: HashMap<i32, SpellNumbers> = tables
        .spell
        .rows
        .iter()
        .map(|s| {
            let duration_ms = durations.get(&s.duration_index.id).map_or(0, |d| d.duration);
            let range_max = ranges.get(&s.range_index).map_or(0.0, |r| real(r.range_max[0]));
            (
                s.id.id,
                SpellNumbers {
                    base_points: s.effect_base_points,
                    die_sides: s.effect_die_sides,
                    period: s.effect_aura_period,
                    radius: std::array::from_fn(|i| radii.get(&s.effect_radius_index[i]).map(|r| real(r.radius))),
                    chain_targets: s.effect_chain_targets,
                    duration_ms,
                    range_max,
                },
            )
        })
        .collect();
    let lookup = |id| numbers.get(&id);
    let school_bits: Vec<_> = names::SPELL_SCHOOLS
        .iter()
        .map(|&(bit, name)| (1u32 << bit, name))
        .collect();

    let mut insert_spell = conn.prepare("INSERT INTO spell (id, name, rank, description, description_raw, aura_text, aura_text_raw, school, power_type, mana_cost, mana_cost_pct, mana_per_second, cast_time, range_min, range_max, range_name, cooldown, category_cooldown, category, gcd, duration, spell_level, base_level, max_level, max_targets, proc_chance, proc_charges, max_stack, icon, attributes, mechanic, dispel, damage_class, family, family_flags, effect1, effect2, effect3, reagents, totems, equipped_item_class, speed) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,?25,?26,?27,?28,?29,?30,?31,?32,?33,?34,?35,?36,?37,?38,?39,?40,?41,?42)")?;
    let mut insert_effect = conn.prepare("INSERT INTO spell_effect (spell_id, idx, effect, aura, value, base_points, die_sides, period, radius, chain_targets, trigger_spell, misc_value, misc_value_b, item_type, target_a, target_b, mechanic) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17)")?;
    for s in &tables.spell.rows {
        let Some(n) = numbers.get(&s.id.id) else { continue };
        let raw_description = &s.description_lang.en_gb;
        let raw_aura = &s.aura_description_lang.en_gb;
        let description = resolve(raw_description, n, &lookup);
        let aura_text = resolve(raw_aura, n, &lookup);
        let school = names::flag_names(&school_bits, s.school_mask.cast_unsigned());
        let attr_tables = [
            names::SPELL_ATTR0,
            names::SPELL_ATTR1,
            names::SPELL_ATTR2,
            names::SPELL_ATTR3,
            names::SPELL_ATTR4,
            names::SPELL_ATTR5,
            names::SPELL_ATTR6,
            names::SPELL_ATTR7,
        ];
        let attr_values = [
            s.attributes,
            s.attributes_ex,
            s.attributes_ex_b,
            s.attributes_ex_c,
            s.attributes_ex_d,
            s.attributes_ex_e,
            s.attributes_ex_f,
            s.attributes_ex_g,
        ];
        let attrs: Vec<_> = attr_tables
            .into_iter()
            .zip(attr_values)
            .filter_map(|(t, v)| names::flag_names(t, v.cast_unsigned()))
            .collect();
        let attributes = (!attrs.is_empty()).then(|| attrs.join("|"));
        let range = ranges.get(&s.range_index);
        let duration = durations.get(&s.duration_index.id).map(|d| d.duration);
        let cast = casts.get(&s.casting_time_index.id).map(|c| c.base);
        let icon = icons.get(&s.spell_icon_id.id).and_then(|i| txt(&i.texture_filename));
        let power = (s.mana_cost > 0 || s.mana_cost_pct > 0 || s.mana_per_second > 0)
            .then(|| names::enum_name(names::POWERS, i64::from(s.power_type)))
            .flatten();
        let family_flags = s.spell_class_mask.iter().any(|&v| v != 0).then(|| {
            format!(
                "0x{:08X},0x{:08X},0x{:08X}",
                s.spell_class_mask[0].cast_unsigned(),
                s.spell_class_mask[1].cast_unsigned(),
                s.spell_class_mask[2].cast_unsigned()
            )
        });
        let reagents: Vec<_> = s
            .reagent
            .iter()
            .zip(s.reagent_count)
            .filter(|(id, count)| **id > 0 && *count > 0)
            .map(|(id, count)| format!("{id}x{count}"))
            .collect();
        let totems: Vec<_> = s.totem.iter().filter(|&&id| id != 0).map(ToString::to_string).collect();
        let mut rendered: [Option<String>; 3] = [None, None, None];
        for ((i, rendered_effect), effect_index) in rendered.iter_mut().enumerate().zip(1_i32..=3) {
            if s.effect[i] == 0 {
                continue;
            }
            let effect = names::enum_name(names::SPELL_EFFECTS, i64::from(s.effect[i]));
            let aura_id = s.effect_aura[i].as_int();
            let aura = (aura_id != 0)
                .then(|| names::enum_name(names::AURA_TYPE, i64::from(aura_id)))
                .flatten();
            let base = i64::from(s.effect_base_points[i]);
            let sides = i64::from(s.effect_die_sides[i]);
            let value = (base != 0 || sides != 0).then(|| {
                if sides <= 1 {
                    (base + sides).to_string()
                } else {
                    format!("{}-{}", base + 1, base + sides)
                }
            });
            let target_a = (s.implicit_target_a[i] != 0)
                .then(|| names::enum_name(names::TARGETS, i64::from(s.implicit_target_a[i])))
                .flatten();
            let target_b = (s.implicit_target_b[i] != 0)
                .then(|| names::enum_name(names::TARGETS, i64::from(s.implicit_target_b[i])))
                .flatten();
            let mut text = effect.clone().unwrap_or_default();
            if let Some(ref aura) = aura {
                text.push(' ');
                text.push_str(aura);
            }
            if let Some(ref value) = value {
                let _ = write!(text, " value={value}");
            }
            if s.effect_aura_period[i] != 0 {
                let _ = write!(text, " period={}", s.effect_aura_period[i]);
            }
            if let Some(radius) = n.radius[i] {
                let _ = write!(text, " radius={radius}");
            }
            if s.effect_chain_targets[i] > 0 {
                let _ = write!(text, " chain={}", s.effect_chain_targets[i]);
            }
            if s.effect_trigger_spell[i] != 0 {
                let _ = write!(text, " trigger={}", s.effect_trigger_spell[i]);
            }
            if s.effect_misc_value[i] != 0 {
                let _ = write!(text, " misc={}", s.effect_misc_value[i]);
            }
            if s.effect_item_type[i] != 0 {
                let _ = write!(text, " item={}", s.effect_item_type[i]);
            }
            if let Some(ref target) = target_a {
                let _ = write!(text, " target={target}");
                if let Some(ref target) = target_b {
                    let _ = write!(text, "/{target}");
                }
            } else if let Some(ref target) = target_b {
                let _ = write!(text, " target={target}");
            }
            *rendered_effect = Some(text);
            insert_effect.execute(params![
                s.id.id,
                effect_index,
                effect,
                aura,
                value,
                s.effect_base_points[i],
                s.effect_die_sides[i],
                nz(s.effect_aura_period[i]),
                n.radius[i],
                nz(s.effect_chain_targets[i]),
                nz(s.effect_trigger_spell[i]),
                s.effect_misc_value[i],
                s.effect_misc_value_b[i],
                nz(s.effect_item_type[i]),
                target_a,
                target_b,
                (s.effect_mechanic[i] != 0)
                    .then(|| names::enum_name(names::MECHANICS, i64::from(s.effect_mechanic[i])))
                    .flatten(),
            ])?;
        }
        insert_spell.execute(params![
            s.id.id,
            txt(&s.name_lang.en_gb),
            txt(&s.name_subtext_lang.en_gb),
            txt(&description),
            (description != *raw_description).then_some(raw_description.as_str()),
            txt(&aura_text),
            (aura_text != *raw_aura).then_some(raw_aura.as_str()),
            school,
            power,
            nz(s.mana_cost),
            nz(s.mana_cost_pct),
            nz(s.mana_per_second),
            cast,
            range.map(|r| real(r.range_min[0])),
            range.map(|r| real(r.range_max[0])),
            range.and_then(|r| txt(&r.display_name_lang.en_gb)),
            nz(s.recovery_time),
            nz(s.category_recovery_time),
            nz(s.category.id),
            nz(s.start_recovery_time),
            duration,
            nz(s.spell_level),
            nz(s.base_level),
            nz(s.max_level),
            nz(s.max_targets),
            nz(s.proc_chance),
            nz(s.proc_charges),
            nz(s.cumulative_aura),
            icon,
            attributes,
            (s.mechanic.id != 0)
                .then(|| names::enum_name(names::MECHANICS, i64::from(s.mechanic.id)))
                .flatten(),
            (s.dispel_type.id != 0)
                .then(|| names::enum_name(names::DISPEL_TYPE, i64::from(s.dispel_type.id)))
                .flatten(),
            (s.defense_type != 0)
                .then(|| names::enum_name(names::SPELL_DMG_CLASS, i64::from(s.defense_type)))
                .flatten(),
            (s.spell_class_set != 0)
                .then(|| names::enum_name(names::SPELL_FAMILY_NAMES, i64::from(s.spell_class_set)))
                .flatten(),
            family_flags,
            rendered[0],
            rendered[1],
            rendered[2],
            (!reagents.is_empty()).then(|| reagents.join(",")),
            (!totems.is_empty()).then(|| totems.join(",")),
            (s.equipped_item_class != -1).then_some(s.equipped_item_class),
            (s.speed != 0.0).then(|| real(s.speed)),
        ])?;
    }
    Ok(())
}
