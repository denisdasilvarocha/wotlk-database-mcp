use std::{fs, path::Path};

use wow_dbc::{DbcTable, ExtendedLocalizedString};
use wow_world_base::wrath::AuraMod;

use wow_dbc::wrath_tables::achievement::{Achievement, AchievementRow};
use wow_dbc::wrath_tables::achievement_category::{Achievement_Category, Achievement_CategoryRow};
use wow_dbc::wrath_tables::achievement_criteria::{Achievement_Criteria, Achievement_CriteriaRow};
use wow_dbc::wrath_tables::area_table::{AreaTable, AreaTableRow};
use wow_dbc::wrath_tables::faction::{Faction, FactionRow};
use wow_dbc::wrath_tables::faction_template::{FactionTemplate, FactionTemplateRow};
use wow_dbc::wrath_tables::glyph_properties::{GlyphProperties, GlyphPropertiesRow};
use wow_dbc::wrath_tables::item::{Item, ItemRow};
use wow_dbc::wrath_tables::item_display_info::{ItemDisplayInfo, ItemDisplayInfoRow};
use wow_dbc::wrath_tables::item_extended_cost::{ItemExtendedCost, ItemExtendedCostRow};
use wow_dbc::wrath_tables::item_set::{ItemSet, ItemSetRow};
use wow_dbc::wrath_tables::map::{Map, MapRow};
use wow_dbc::wrath_tables::skill_line::{SkillLine, SkillLineRow};
use wow_dbc::wrath_tables::skill_line_ability::{SkillLineAbility, SkillLineAbilityRow};
use wow_dbc::wrath_tables::spell::{Spell, SpellRow};
use wow_dbc::wrath_tables::spell_cast_times::{SpellCastTimes, SpellCastTimesRow};
use wow_dbc::wrath_tables::spell_duration::{SpellDuration, SpellDurationRow};
use wow_dbc::wrath_tables::spell_icon::{SpellIcon, SpellIconRow};
use wow_dbc::wrath_tables::spell_radius::{SpellRadius, SpellRadiusRow};
use wow_dbc::wrath_tables::spell_range::{SpellRange, SpellRangeRow};
use wow_dbc::wrath_tables::talent::{Talent, TalentRow};
use wow_dbc::wrath_tables::talent_tab::{TalentTab, TalentTabRow};

/// Writable DBC tables used by integration tests.
pub(crate) struct Fixtures {
    /// Fixture rows for Spell.
    pub spell: Spell,
    /// Fixture rows for `SpellDuration`.
    pub spell_duration: SpellDuration,
    /// Fixture rows for `SpellRadius`.
    pub spell_radius: SpellRadius,
    /// Fixture rows for `SpellRange`.
    pub spell_range: SpellRange,
    /// Fixture rows for `SpellCastTimes`.
    pub spell_cast_times: SpellCastTimes,
    /// Fixture rows for `SpellIcon`.
    pub spell_icon: SpellIcon,
    /// Fixture rows for Item.
    pub item: Item,
    /// Fixture rows for `ItemSet`.
    pub item_set: ItemSet,
    /// Fixture rows for `ItemDisplayInfo`.
    pub item_display_info: ItemDisplayInfo,
    /// Fixture rows for `ItemExtendedCost`.
    pub item_extended_cost: ItemExtendedCost,
    /// Fixture rows for Map.
    pub map: Map,
    /// Fixture rows for `AreaTable`.
    pub area_table: AreaTable,
    /// Fixture rows for Faction.
    pub faction: Faction,
    /// Fixture rows for `FactionTemplate`.
    pub faction_template: FactionTemplate,
    /// Fixture rows for Talent.
    pub talent: Talent,
    /// Fixture rows for `TalentTab`.
    pub talent_tab: TalentTab,
    /// Fixture rows for `SkillLine`.
    pub skill_line: SkillLine,
    /// Fixture rows for `SkillLineAbility`.
    pub skill_line_ability: SkillLineAbility,
    /// Fixture rows for `GlyphProperties`.
    pub glyph_properties: GlyphProperties,
    /// Fixture rows for Achievement.
    pub achievement: Achievement,
    /// Fixture rows for `Achievement_Criteria`.
    pub achievement_criteria: Achievement_Criteria,
    /// Fixture rows for `Achievement_Category`.
    pub achievement_category: Achievement_Category,
}

impl Fixtures {
    /// Constructs the complete representative dataset.
    #[expect(
        clippy::too_many_lines,
        reason = "the complete representative dataset is clearest together"
    )]
    pub(crate) fn new() -> Self {
        Self {
            spell: Spell {
                rows: vec![
                    spell_row(133, |r| {
                        r.name_lang = localized("Fireball");
                        r.name_subtext_lang = localized("Rank 1");
                        r.school_mask = 4;
                        r.mana_cost = 30;
                        r.casting_time_index = wow_dbc::wrath_tables::spell_cast_times::SpellCastTimesKey::new(4);
                        r.range_index = 4;
                        r.duration_index = wow_dbc::wrath_tables::spell_duration::SpellDurationKey::new(2);
                        r.spell_icon_id = wow_dbc::wrath_tables::spell_icon::SpellIconKey::new(1);
                        r.attributes = 0x10;
                        r.effect = [2, 6, 0];
                        r.effect_base_points = [13, 0, 0];
                        r.effect_die_sides = [9, 1, 0];
                        r.implicit_target_a = [6, 6, 0];
                        r.effect_aura = [AuraMod::None, AuraMod::PeriodicDamage, AuraMod::None];
                        r.effect_aura_period = [0, 2000, 0];
                        r.description_lang = localized(
                            "Hurls a fiery ball that causes $s1 Fire damage and an additional $o2 Fire damage over $d. Rank 2 deals $143s1. ${$m1*2} $lpoint:points;",
                        );
                    }),
                    spell_row(143, |r| {
                        r.name_lang = localized("Fireball");
                        r.name_subtext_lang = localized("Rank 2");
                        r.effect = [2, 0, 0];
                        r.effect_base_points = [30, 0, 0];
                        r.effect_die_sides = [1, 0, 0];
                    }),
                    spell_row(2136, |r| {
                        r.name_lang = localized("Fire Blast");
                        r.name_subtext_lang = localized("Rank 1");
                        r.effect = [64, 0, 0];
                        r.effect_trigger_spell = [133, 0, 0];
                        r.effect_radius_index = [8, 0, 0];
                        r.description_lang = localized("Blasts within $a1 yards.");
                    }),
                    spell_row(14147, |r| {
                        r.name_lang = localized("Signed Dice");
                        r.duration_index = wow_dbc::wrath_tables::spell_duration::SpellDurationKey::new(2);
                        r.effect = [2, 6, 0];
                        r.effect_base_points = [1, -761, 0];
                        r.effect_die_sides = [-3, -79, 0];
                        r.effect_aura = [AuraMod::None, AuraMod::PeriodicDamage, AuraMod::None];
                        r.effect_aura_period = [0, 2000, 0];
                        r.description_lang = localized("Range $s2 minimum $m2 total $o2; crossing $s1 minimum $m1.");
                        r.aura_description_lang = localized("Range $s2 minimum $m2 total $o2.");
                    }),
                    spell_row(11366, |r| {
                        r.name_lang = localized("Pyroblast");
                        r.name_subtext_lang = localized("Rank 1");
                    }),
                    spell_row(56368, |r| {
                        r.name_lang = localized("Glyph of Fireball");
                        r.name_subtext_lang = localized("");
                    }),
                    spell_row(56390, |r| {
                        r.name_lang = localized("Glyph of Fireball");
                        r.name_subtext_lang = localized("");
                        r.effect = [74, 0, 0];
                        r.effect_misc_value = [1, 0, 0];
                    }),
                    spell_row(483, |r| {
                        r.name_lang = localized("Learning");
                        r.name_subtext_lang = localized("");
                    }),
                    spell_row(4001, |r| {
                        r.name_lang = localized("Fiery Brew");
                        r.name_subtext_lang = localized("");
                        r.effect = [24, 0, 0];
                        r.effect_item_type = [4002, 0, 0];
                        r.effect_die_sides = [1, 0, 0];
                    }),
                    spell_row(4003, |r| {
                        r.name_lang = localized("Teach Fiery Brew");
                        r.name_subtext_lang = localized("");
                        r.effect = [36, 0, 0];
                        r.effect_trigger_spell = [4001, 0, 0];
                    }),
                    spell_row(4006, |r| {
                        r.name_lang = localized("Fiery Regalia Bonus");
                        r.name_subtext_lang = localized("");
                    }),
                    // NPC copy of Fireball has no player acquisition entry.
                    spell_row(9053, |r| {
                        r.name_lang = localized("Fireball");
                    }),
                    // Ranked triggered copies likewise have no player acquisition entry.
                    spell_row(50001, |r| {
                        r.name_lang = localized("Fireball");
                        r.name_subtext_lang = localized("Rank 1");
                    }),
                    spell_row(50002, |r| {
                        r.name_lang = localized("Fireball");
                        r.name_subtext_lang = localized("Rank 2");
                    }),
                ],
            },
            spell_duration: SpellDuration {
                rows: vec![spell_duration_row(2, |r| {
                    r.duration = 4000;
                })],
            },
            spell_radius: SpellRadius {
                rows: vec![spell_radius_row(8, |r| {
                    r.radius = 10.0;
                })],
            },
            spell_range: SpellRange {
                rows: vec![
                    spell_range_row(1, |r| {
                        r.display_name_lang = localized("Self Only");
                    }),
                    spell_range_row(4, |r| {
                        r.display_name_lang = localized("Long Range");
                        r.range_max = [35.0, 35.0];
                    }),
                ],
            },
            spell_cast_times: SpellCastTimes {
                rows: vec![
                    spell_cast_times_row(1, |_| {}),
                    spell_cast_times_row(4, |r| {
                        r.base = 1500;
                    }),
                ],
            },
            spell_icon: SpellIcon {
                rows: vec![spell_icon_row(1, |r| {
                    r.texture_filename = r"Interface\Icons\Spell_Fire_FlameBolt".into();
                })],
            },
            item: Item {
                rows: vec![
                    item_row(19019, |r| {
                        r.class_id = 4;
                        r.display_info_id = 30606;
                    }),
                    item_row(50000, |r| {
                        r.class_id = 4;
                        r.display_info_id = 1234;
                    }),
                ],
            },
            item_set: ItemSet {
                rows: vec![item_set_row(1, |r| {
                    r.name_lang = localized("Fiery Regalia");
                    r.item_id[0] = 4004;
                    r.item_id[1] = 4005;
                    r.set_spell_id[0] = 4006;
                    r.set_threshold[0] = 2;
                })],
            },
            item_display_info: ItemDisplayInfo {
                rows: vec![item_display_info_row(30606, |r| {
                    r.inventory_icon[0] = "INV_Sword_39".into();
                })],
            },
            item_extended_cost: ItemExtendedCost {
                rows: vec![item_extended_cost_row(1, |r| {
                    r.honor_points = 100;
                    r.item_id[0] = 4002;
                    r.item_count[0] = 2;
                })],
            },
            map: Map {
                rows: vec![
                    map_row(0, |r| {
                        r.map_name_lang = localized("Eastern Kingdoms");
                    }),
                    map_row(409, |r| {
                        r.map_name_lang = localized("Molten Core");
                        r.instance_type = 2;
                        r.max_players = 40;
                    }),
                ],
            },
            area_table: AreaTable {
                rows: vec![
                    area_table_row(12, |r| {
                        r.area_name_lang = localized("Elwynn Forest");
                    }),
                    area_table_row(87, |r| {
                        r.area_name_lang = localized("Goldshire");
                        r.parent_area_id = wow_dbc::wrath_tables::area_table::AreaTableKey::new(12);
                    }),
                    area_table_row(2717, |r| {
                        r.area_name_lang = localized("Molten Core");
                        r.continent_id = wow_dbc::wrath_tables::map::MapKey::new(409);
                    }),
                ],
            },
            faction: Faction {
                rows: vec![faction_row(72, |r| {
                    r.name_lang = localized("Stormwind");
                })],
            },
            faction_template: FactionTemplate {
                rows: vec![faction_template_row(11, |r| {
                    r.faction = wow_dbc::wrath_tables::faction::FactionKey::new(72);
                })],
            },
            talent: Talent {
                rows: vec![talent_row(36, |r| {
                    r.tab_id = 41;
                    r.column_index = 1;
                    r.spell_rank[0] = 11366;
                })],
            },
            talent_tab: TalentTab {
                rows: vec![talent_tab_row(41, |r| {
                    r.name_lang = localized("Fire");
                    r.class_mask = 128;
                })],
            },
            skill_line: SkillLine {
                rows: vec![skill_line_row(8, |r| {
                    r.display_name_lang = localized("Fire");
                    r.category_id = wow_dbc::wrath_tables::skill_line_category::SkillLineCategoryKey::new(7);
                })],
            },
            skill_line_ability: SkillLineAbility {
                rows: vec![skill_line_ability_row(1, |r| {
                    r.skill_line = wow_dbc::wrath_tables::skill_line::SkillLineKey::new(8);
                    r.spell = wow_dbc::wrath_tables::spell::SpellKey::new(133);
                    r.superceded_by_spell = wow_dbc::wrath_tables::spell::SpellKey::new(143);
                })],
            },
            glyph_properties: GlyphProperties {
                rows: vec![glyph_properties_row(1, |r| {
                    r.spell_id = wow_dbc::wrath_tables::spell::SpellKey::new(56368);
                    r.spell_icon_id = wow_dbc::wrath_tables::spell_icon::SpellIconKey::new(1);
                })],
            },
            achievement: Achievement {
                rows: vec![achievement_row(6, |r| {
                    r.title_lang = localized("Level 10");
                    r.description_lang = localized("Reach level 10.");
                    r.category = wow_dbc::wrath_tables::achievement_category::Achievement_CategoryKey::new(92);
                    r.points = 10;
                    r.faction = wow_dbc::wrath_tables::faction::FactionKey::new(-1);
                    r.instance_id = wow_dbc::wrath_tables::map::MapKey::new(-1);
                })],
            },
            achievement_criteria: Achievement_Criteria {
                rows: vec![achievement_criteria_row(1, |r| {
                    r.achievement_id = wow_dbc::wrath_tables::achievement::AchievementKey::new(6);
                    r.ty = 5;
                    r.quantity = 10;
                    r.description_lang = localized("Reach level 10");
                })],
            },
            achievement_category: Achievement_Category {
                rows: vec![achievement_category_row(92, |r| {
                    r.name_lang = localized("General");
                })],
            },
        }
    }

    /// Writes each table with its real WDBC layout.
    #[expect(
        clippy::expect_used,
        reason = "fixture setup must fail the test if its directory cannot be created"
    )]
    pub(crate) fn write(&self, dir: &Path) {
        fs::create_dir_all(dir).expect("create DBC fixture directory");
        write_table(dir, &self.spell);
        write_table(dir, &self.spell_duration);
        write_table(dir, &self.spell_radius);
        write_table(dir, &self.spell_range);
        write_table(dir, &self.spell_cast_times);
        write_table(dir, &self.spell_icon);
        write_table(dir, &self.item);
        write_table(dir, &self.item_set);
        write_table(dir, &self.item_display_info);
        write_table(dir, &self.item_extended_cost);
        write_table(dir, &self.map);
        write_table(dir, &self.area_table);
        write_table(dir, &self.faction);
        write_table(dir, &self.faction_template);
        write_table(dir, &self.talent);
        write_table(dir, &self.talent_tab);
        write_table(dir, &self.skill_line);
        write_table(dir, &self.skill_line_ability);
        write_table(dir, &self.glyph_properties);
        write_table(dir, &self.achievement);
        write_table(dir, &self.achievement_criteria);
        write_table(dir, &self.achievement_category);
    }
}

#[expect(
    clippy::expect_used,
    reason = "fixture setup must fail the test if a DBC file cannot be written"
)]
fn write_table<T: DbcTable>(dir: &Path, table: &T) {
    let mut file = fs::File::create(dir.join(T::FILENAME)).expect("create DBC fixture");
    table.write(&mut file).expect("write DBC fixture");
}

fn localized(value: &str) -> ExtendedLocalizedString {
    ExtendedLocalizedString {
        en_gb: value.into(),
        ..Default::default()
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "all fields of the generated DBC row require explicit initialization"
)]
fn spell_row(id: i32, edit: impl FnOnce(&mut SpellRow)) -> SpellRow {
    let mut row = SpellRow {
        id: wow_dbc::wrath_tables::spell::SpellKey::new(id),
        category: wow_dbc::wrath_tables::spell_category::SpellCategoryKey::new(0),
        dispel_type: wow_dbc::wrath_tables::spell_dispel_type::SpellDispelTypeKey::new(0),
        mechanic: wow_dbc::wrath_tables::spell_mechanic::SpellMechanicKey::new(0),
        attributes: 0,
        attributes_ex: 0,
        attributes_ex_b: 0,
        attributes_ex_c: 0,
        attributes_ex_d: 0,
        attributes_ex_e: 0,
        attributes_ex_f: 0,
        attributes_ex_g: 0,
        shapeshift_mask: [0; 2],
        shapeshift_exclude: [0; 2],
        targets: 0,
        target_creature_type: 0,
        requires_spell_focus: wow_dbc::wrath_tables::spell_focus_object::SpellFocusObjectKey::new(0),
        facing_caster_flags: 0,
        caster_aura_state: 0,
        target_aura_state: 0,
        exclude_caster_aura_state: 0,
        exclude_target_aura_state: 0,
        caster_aura_spell: 0,
        target_aura_spell: 0,
        exclude_caster_aura_spell: 0,
        exclude_target_aura_spell: 0,
        casting_time_index: wow_dbc::wrath_tables::spell_cast_times::SpellCastTimesKey::new(0),
        recovery_time: 0,
        category_recovery_time: 0,
        interrupt_flags: 0,
        aura_interrupt_flags: 0,
        channel_interrupt_flags: 0,
        proc_type_mask: 0,
        proc_chance: 0,
        proc_charges: 0,
        max_level: 0,
        base_level: 0,
        spell_level: 0,
        duration_index: wow_dbc::wrath_tables::spell_duration::SpellDurationKey::new(0),
        power_type: 0,
        mana_cost: 0,
        mana_cost_per_level: 0,
        mana_per_second: 0,
        mana_per_second_per_level: 0,
        range_index: 0,
        speed: 0.0,
        modal_next_spell: 0,
        cumulative_aura: 0,
        totem: [0; 2],
        reagent: [0; 8],
        reagent_count: [0; 8],
        equipped_item_class: 0,
        equipped_item_subclass: 0,
        equipped_item_inv_types: 0,
        effect: [0; 3],
        effect_die_sides: [0; 3],
        effect_real_points_per_level: [0.0; 3],
        effect_base_points: [0; 3],
        effect_mechanic: [0; 3],
        implicit_target_a: [0; 3],
        implicit_target_b: [0; 3],
        effect_radius_index: [0; 3],
        effect_aura: [AuraMod::default(); 3],
        effect_aura_period: [0; 3],
        effect_amplitude: [0.0; 3],
        effect_chain_targets: [0; 3],
        effect_item_type: [0; 3],
        effect_misc_value: [0; 3],
        effect_misc_value_b: [0; 3],
        effect_trigger_spell: [0; 3],
        effect_points_per_combo: [0.0; 3],
        effect_spell_class_mask_a: [0; 3],
        effect_spell_class_mask_b: [0; 3],
        effect_spell_class_mask_c: [0; 3],
        spell_visual_id: [0; 2],
        spell_icon_id: wow_dbc::wrath_tables::spell_icon::SpellIconKey::new(0),
        active_icon_id: wow_dbc::wrath_tables::spell_icon::SpellIconKey::new(0),
        spell_priority: 0,
        name_lang: ExtendedLocalizedString::default(),
        name_subtext_lang: ExtendedLocalizedString::default(),
        description_lang: ExtendedLocalizedString::default(),
        aura_description_lang: ExtendedLocalizedString::default(),
        mana_cost_pct: 0,
        start_recovery_category: 0,
        start_recovery_time: 0,
        max_target_level: 0,
        spell_class_set: 0,
        spell_class_mask: [0; 3],
        max_targets: 0,
        defense_type: 0,
        prevention_type: 0,
        stance_bar_order: 0,
        effect_chain_amplitude: [0.0; 3],
        min_faction_id: wow_dbc::wrath_tables::faction::FactionKey::new(0),
        min_reputation: 0,
        required_aura_vision: 0,
        required_totem_category_id: [0; 2],
        required_areas_id: wow_dbc::wrath_tables::area_group::AreaGroupKey::new(0),
        school_mask: 0,
        rune_cost_id: wow_dbc::wrath_tables::spell_rune_cost::SpellRuneCostKey::new(0),
        spell_missile_id: wow_dbc::wrath_tables::spell_missile::SpellMissileKey::new(0),
        power_display_id: wow_dbc::wrath_tables::power_display::PowerDisplayKey::new(0),
        effect_bonus_coefficient: [0.0; 3],
        description_variables_id: wow_dbc::wrath_tables::spell_description_variables::SpellDescriptionVariablesKey::new(
            0,
        ),
        difficulty: 0,
    };
    edit(&mut row);
    row
}

fn spell_duration_row(id: i32, edit: impl FnOnce(&mut SpellDurationRow)) -> SpellDurationRow {
    let mut row = SpellDurationRow {
        id: wow_dbc::wrath_tables::spell_duration::SpellDurationKey::new(id),
        duration: 0,
        duration_per_level: 0,
        max_duration: 0,
    };
    edit(&mut row);
    row
}

fn spell_radius_row(id: i32, edit: impl FnOnce(&mut SpellRadiusRow)) -> SpellRadiusRow {
    let mut row = SpellRadiusRow {
        id: wow_dbc::wrath_tables::spell_radius::SpellRadiusKey::new(id),
        radius: 0.0,
        radius_per_level: 0.0,
        radius_max: 0.0,
    };
    edit(&mut row);
    row
}

fn spell_range_row(id: i32, edit: impl FnOnce(&mut SpellRangeRow)) -> SpellRangeRow {
    let mut row = SpellRangeRow {
        id: wow_dbc::wrath_tables::spell_range::SpellRangeKey::new(id),
        range_min: [0.0; 2],
        range_max: [0.0; 2],
        flags: 0,
        display_name_lang: ExtendedLocalizedString::default(),
        display_name_short_lang: ExtendedLocalizedString::default(),
    };
    edit(&mut row);
    row
}

fn spell_cast_times_row(id: i32, edit: impl FnOnce(&mut SpellCastTimesRow)) -> SpellCastTimesRow {
    let mut row = SpellCastTimesRow {
        id: wow_dbc::wrath_tables::spell_cast_times::SpellCastTimesKey::new(id),
        base: 0,
        per_level: 0,
        minimum: 0,
    };
    edit(&mut row);
    row
}

fn spell_icon_row(id: i32, edit: impl FnOnce(&mut SpellIconRow)) -> SpellIconRow {
    let mut row = SpellIconRow {
        id: wow_dbc::wrath_tables::spell_icon::SpellIconKey::new(id),
        texture_filename: String::new(),
    };
    edit(&mut row);
    row
}

fn item_row(id: i32, edit: impl FnOnce(&mut ItemRow)) -> ItemRow {
    let mut row = ItemRow {
        id: wow_dbc::wrath_tables::item::ItemKey::new(id),
        class_id: 0,
        subclass_id: 0,
        sound_override_subclass_id: 0,
        material: wow_dbc::wrath_tables::material::MaterialKey::new(0),
        display_info_id: 0,
        inventory_type: 0,
        sheathe_type: 0,
    };
    edit(&mut row);
    row
}

fn item_set_row(id: i32, edit: impl FnOnce(&mut ItemSetRow)) -> ItemSetRow {
    let mut row = ItemSetRow {
        id: wow_dbc::wrath_tables::item_set::ItemSetKey::new(id),
        name_lang: ExtendedLocalizedString::default(),
        item_id: [0; 17],
        set_spell_id: [0; 8],
        set_threshold: [0; 8],
        required_skill: wow_dbc::wrath_tables::skill_line::SkillLineKey::new(0),
        required_skill_rank: 0,
    };
    edit(&mut row);
    row
}

fn item_display_info_row(id: i32, edit: impl FnOnce(&mut ItemDisplayInfoRow)) -> ItemDisplayInfoRow {
    let mut row = ItemDisplayInfoRow {
        id: wow_dbc::wrath_tables::item_display_info::ItemDisplayInfoKey::new(id),
        model_name: std::array::from_fn(|_| String::new()),
        model_texture: std::array::from_fn(|_| String::new()),
        inventory_icon: std::array::from_fn(|_| String::new()),
        geoset_group: [0; 3],
        flags: 0,
        spell_visual_id: wow_dbc::wrath_tables::spell_visual::SpellVisualKey::new(0),
        group_sound_index: 0,
        helmet_geoset_vis_id: [0; 2],
        texture: std::array::from_fn(|_| String::new()),
        item_visual: 0,
        particle_color_id: wow_dbc::wrath_tables::particle_color::ParticleColorKey::new(0),
    };
    edit(&mut row);
    row
}

fn item_extended_cost_row(id: i32, edit: impl FnOnce(&mut ItemExtendedCostRow)) -> ItemExtendedCostRow {
    let mut row = ItemExtendedCostRow {
        id: wow_dbc::wrath_tables::item_extended_cost::ItemExtendedCostKey::new(id),
        honor_points: 0,
        arena_points: 0,
        arena_bracket: 0,
        item_id: [0; 5],
        item_count: [0; 5],
        required_arena_rating: 0,
        item_purchase_group: wow_dbc::wrath_tables::item_purchase_group::ItemPurchaseGroupKey::new(0),
    };
    edit(&mut row);
    row
}

fn map_row(id: i32, edit: impl FnOnce(&mut MapRow)) -> MapRow {
    let mut row = MapRow {
        id: wow_dbc::wrath_tables::map::MapKey::new(id),
        directory: String::new(),
        instance_type: 0,
        flags: 0,
        p_v_p: 0,
        map_name_lang: ExtendedLocalizedString::default(),
        area_table_id: wow_dbc::wrath_tables::area_table::AreaTableKey::new(0),
        map_description0_lang: ExtendedLocalizedString::default(),
        map_description1_lang: ExtendedLocalizedString::default(),
        loading_screen_id: wow_dbc::wrath_tables::loading_screens::LoadingScreensKey::new(0),
        minimap_icon_scale: 0.0,
        corpse_map_id: wow_dbc::wrath_tables::map::MapKey::new(0),
        corpse: [0.0; 2],
        time_of_day_override: 0,
        expansion_id: 0,
        raid_offset: 0,
        max_players: 0,
    };
    edit(&mut row);
    row
}

fn area_table_row(id: i32, edit: impl FnOnce(&mut AreaTableRow)) -> AreaTableRow {
    let mut row = AreaTableRow {
        id: wow_dbc::wrath_tables::area_table::AreaTableKey::new(id),
        continent_id: wow_dbc::wrath_tables::map::MapKey::new(0),
        parent_area_id: wow_dbc::wrath_tables::area_table::AreaTableKey::new(0),
        area_bit: 0,
        flags: 0,
        sound_provider_pref: wow_dbc::wrath_tables::sound_provider_preferences::SoundProviderPreferencesKey::new(0),
        sound_provider_pref_underwater:
            wow_dbc::wrath_tables::sound_provider_preferences::SoundProviderPreferencesKey::new(0),
        ambience_id: wow_dbc::wrath_tables::sound_ambience::SoundAmbienceKey::new(0),
        zone_music: wow_dbc::wrath_tables::zone_music::ZoneMusicKey::new(0),
        intro_sound: wow_dbc::wrath_tables::zone_intro_music_table::ZoneIntroMusicTableKey::new(0),
        exploration_level: 0,
        area_name_lang: ExtendedLocalizedString::default(),
        faction_group_mask: 0,
        liquid_type_id: [0; 4],
        min_elevation: 0.0,
        ambient_multiplier: 0.0,
        light_id: wow_dbc::wrath_tables::light::LightKey::new(0),
    };
    edit(&mut row);
    row
}

fn faction_row(id: i32, edit: impl FnOnce(&mut FactionRow)) -> FactionRow {
    let mut row = FactionRow {
        id: wow_dbc::wrath_tables::faction::FactionKey::new(id),
        reputation_index: 0,
        reputation_race_mask: [0; 4],
        reputation_class_mask: [0; 4],
        reputation_base: [0; 4],
        reputation_flags: [0; 4],
        parent_faction_id: wow_dbc::wrath_tables::faction::FactionKey::new(0),
        parent_faction_mod: [0.0; 2],
        parent_faction_cap: [0; 2],
        name_lang: ExtendedLocalizedString::default(),
        description_lang: ExtendedLocalizedString::default(),
    };
    edit(&mut row);
    row
}

fn faction_template_row(id: i32, edit: impl FnOnce(&mut FactionTemplateRow)) -> FactionTemplateRow {
    let mut row = FactionTemplateRow {
        id: wow_dbc::wrath_tables::faction_template::FactionTemplateKey::new(id),
        faction: wow_dbc::wrath_tables::faction::FactionKey::new(0),
        flags: 0,
        faction_group: wow_dbc::wrath_tables::faction_group::FactionGroupKey::new(0),
        friend_group: 0,
        enemy_group: 0,
        enemies: [0; 4],
        friend: [0; 4],
    };
    edit(&mut row);
    row
}

fn talent_row(id: i32, edit: impl FnOnce(&mut TalentRow)) -> TalentRow {
    let mut row = TalentRow {
        id: wow_dbc::wrath_tables::talent::TalentKey::new(id),
        tab_id: 0,
        tier_id: 0,
        column_index: 0,
        spell_rank: [0; 9],
        prereq_talent: [0; 3],
        prereq_rank: [0; 3],
        flags: 0,
        required_spell_id: wow_dbc::wrath_tables::spell::SpellKey::new(0),
        category_mask: [0; 2],
    };
    edit(&mut row);
    row
}

fn talent_tab_row(id: i32, edit: impl FnOnce(&mut TalentTabRow)) -> TalentTabRow {
    let mut row = TalentTabRow {
        id: wow_dbc::wrath_tables::talent_tab::TalentTabKey::new(id),
        name_lang: ExtendedLocalizedString::default(),
        spell_icon_id: wow_dbc::wrath_tables::spell_icon::SpellIconKey::new(0),
        race_mask: 0,
        class_mask: 0,
        category_enum_id: 0,
        order_index: 0,
        background_file: String::new(),
    };
    edit(&mut row);
    row
}

fn skill_line_row(id: i32, edit: impl FnOnce(&mut SkillLineRow)) -> SkillLineRow {
    let mut row = SkillLineRow {
        id: wow_dbc::wrath_tables::skill_line::SkillLineKey::new(id),
        category_id: wow_dbc::wrath_tables::skill_line_category::SkillLineCategoryKey::new(0),
        skill_costs_id: 0,
        display_name_lang: ExtendedLocalizedString::default(),
        description_lang: ExtendedLocalizedString::default(),
        spell_icon_id: wow_dbc::wrath_tables::spell_icon::SpellIconKey::new(0),
        alternate_verb_lang: ExtendedLocalizedString::default(),
        can_link: 0,
    };
    edit(&mut row);
    row
}

fn skill_line_ability_row(id: i32, edit: impl FnOnce(&mut SkillLineAbilityRow)) -> SkillLineAbilityRow {
    let mut row = SkillLineAbilityRow {
        id: wow_dbc::wrath_tables::skill_line_ability::SkillLineAbilityKey::new(id),
        skill_line: wow_dbc::wrath_tables::skill_line::SkillLineKey::new(0),
        spell: wow_dbc::wrath_tables::spell::SpellKey::new(0),
        race_mask: 0,
        class_mask: 0,
        exclude_race: 0,
        exclude_class: 0,
        min_skill_line_rank: 0,
        superceded_by_spell: wow_dbc::wrath_tables::spell::SpellKey::new(0),
        acquire_method: 0,
        trivial_skill_line_rank_high: 0,
        trivial_skill_line_rank_low: 0,
        character_points: [0; 2],
    };
    edit(&mut row);
    row
}

fn glyph_properties_row(id: i32, edit: impl FnOnce(&mut GlyphPropertiesRow)) -> GlyphPropertiesRow {
    let mut row = GlyphPropertiesRow {
        id: wow_dbc::wrath_tables::glyph_properties::GlyphPropertiesKey::new(id),
        spell_id: wow_dbc::wrath_tables::spell::SpellKey::new(0),
        glyph_slot_flags: 0,
        spell_icon_id: wow_dbc::wrath_tables::spell_icon::SpellIconKey::new(0),
    };
    edit(&mut row);
    row
}

fn achievement_row(id: i32, edit: impl FnOnce(&mut AchievementRow)) -> AchievementRow {
    let mut row = AchievementRow {
        id: wow_dbc::wrath_tables::achievement::AchievementKey::new(id),
        faction: wow_dbc::wrath_tables::faction::FactionKey::new(0),
        instance_id: wow_dbc::wrath_tables::map::MapKey::new(0),
        supercedes: wow_dbc::wrath_tables::achievement::AchievementKey::new(0),
        title_lang: ExtendedLocalizedString::default(),
        description_lang: ExtendedLocalizedString::default(),
        category: wow_dbc::wrath_tables::achievement_category::Achievement_CategoryKey::new(0),
        points: 0,
        ui_order: 0,
        flags: 0,
        icon_id: wow_dbc::wrath_tables::spell_icon::SpellIconKey::new(0),
        reward_lang: ExtendedLocalizedString::default(),
        minimum_criteria: 0,
        shares_criteria: wow_dbc::wrath_tables::achievement::AchievementKey::new(0),
    };
    edit(&mut row);
    row
}

fn achievement_criteria_row(id: i32, edit: impl FnOnce(&mut Achievement_CriteriaRow)) -> Achievement_CriteriaRow {
    let mut row = Achievement_CriteriaRow {
        id: wow_dbc::wrath_tables::achievement_criteria::Achievement_CriteriaKey::new(id),
        achievement_id: wow_dbc::wrath_tables::achievement::AchievementKey::new(0),
        ty: 0,
        asset_id: 0,
        quantity: 0,
        start_event: 0,
        start_asset: 0,
        fail_event: 0,
        fail_asset: 0,
        description_lang: ExtendedLocalizedString::default(),
        flags: 0,
        timer_start_event: 0,
        timer_asset_id: 0,
        timer_time: 0,
        ui_order: 0,
    };
    edit(&mut row);
    row
}

fn achievement_category_row(id: i32, edit: impl FnOnce(&mut Achievement_CategoryRow)) -> Achievement_CategoryRow {
    let mut row = Achievement_CategoryRow {
        id: wow_dbc::wrath_tables::achievement_category::Achievement_CategoryKey::new(id),
        parent: wow_dbc::wrath_tables::achievement_category::Achievement_CategoryKey::new(0),
        name_lang: ExtendedLocalizedString::default(),
        ui_order: 0,
    };
    edit(&mut row);
    row
}
