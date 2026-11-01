use rusqlite::{Connection, Statement, params};

use crate::{BuildError, dbc::Context, dump::Row, names};

pub(super) const TABLES: &[(&str, &[&str])] = &[
    (
        "quest_template",
        &[
            "ID",
            "QuestLevel",
            "MinLevel",
            "QuestSortID",
            "QuestInfoID",
            "SuggestedGroupNum",
            "RewardNextQuest",
            "RewardXPDifficulty",
            "RewardMoney",
            "RewardBonusMoney",
            "RewardDisplaySpell",
            "RewardSpell",
            "RewardHonor",
            "StartItem",
            "Flags",
            "RequiredPlayerKills",
            "RewardItem1",
            "RewardAmount1",
            "RewardItem2",
            "RewardAmount2",
            "RewardItem3",
            "RewardAmount3",
            "RewardItem4",
            "RewardAmount4",
            "ItemDrop1",
            "ItemDropQuantity1",
            "ItemDrop2",
            "ItemDropQuantity2",
            "ItemDrop3",
            "ItemDropQuantity3",
            "ItemDrop4",
            "ItemDropQuantity4",
            "RewardChoiceItemID1",
            "RewardChoiceItemQuantity1",
            "RewardChoiceItemID2",
            "RewardChoiceItemQuantity2",
            "RewardChoiceItemID3",
            "RewardChoiceItemQuantity3",
            "RewardChoiceItemID4",
            "RewardChoiceItemQuantity4",
            "RewardChoiceItemID5",
            "RewardChoiceItemQuantity5",
            "RewardChoiceItemID6",
            "RewardChoiceItemQuantity6",
            "RewardTitle",
            "RewardTalents",
            "RewardArenaPoints",
            "RewardFactionID1",
            "RewardFactionValue1",
            "RewardFactionOverride1",
            "RewardFactionID2",
            "RewardFactionValue2",
            "RewardFactionOverride2",
            "RewardFactionID3",
            "RewardFactionValue3",
            "RewardFactionOverride3",
            "RewardFactionID4",
            "RewardFactionValue4",
            "RewardFactionOverride4",
            "RewardFactionID5",
            "RewardFactionValue5",
            "RewardFactionOverride5",
            "TimeAllowed",
            "AllowableRaces",
            "LogTitle",
            "LogDescription",
            "QuestDescription",
            "AreaDescription",
            "QuestCompletionLog",
            "RequiredNpcOrGo1",
            "RequiredNpcOrGo2",
            "RequiredNpcOrGo3",
            "RequiredNpcOrGo4",
            "RequiredNpcOrGoCount1",
            "RequiredNpcOrGoCount2",
            "RequiredNpcOrGoCount3",
            "RequiredNpcOrGoCount4",
            "RequiredItemId1",
            "RequiredItemId2",
            "RequiredItemId3",
            "RequiredItemId4",
            "RequiredItemId5",
            "RequiredItemId6",
            "RequiredItemCount1",
            "RequiredItemCount2",
            "RequiredItemCount3",
            "RequiredItemCount4",
            "RequiredItemCount5",
            "RequiredItemCount6",
            "ObjectiveText1",
            "ObjectiveText2",
            "ObjectiveText3",
            "ObjectiveText4",
        ],
    ),
    (
        "quest_template_addon",
        &[
            "ID",
            "MaxLevel",
            "AllowableClasses",
            "SourceSpellID",
            "PrevQuestID",
            "NextQuestID",
            "ExclusiveGroup",
            "BreadcrumbForQuestId",
            "RequiredSkillID",
            "RequiredSkillPoints",
            "RequiredMinRepFaction",
            "RequiredMaxRepFaction",
            "RequiredMinRepValue",
            "RequiredMaxRepValue",
            "SpecialFlags",
        ],
    ),
    ("creature_queststarter", &["id", "quest"]),
    ("creature_questender", &["id", "quest"]),
    ("gameobject_queststarter", &["id", "quest"]),
    ("gameobject_questender", &["id", "quest"]),
];

fn nz(value: i64) -> Option<i64> {
    (value != 0).then_some(value)
}
fn mask_names(
    table: &[(u32, &str)],
    value: i64,
    all: u32,
    column: &str,
    line: u64,
) -> Result<Option<String>, BuildError> {
    if value == -1 || value == 0 {
        return Ok(None);
    }
    let mask = checked_mask(value, column, line)?;
    Ok(if mask & all == all {
        None
    } else {
        names::flag_names(table, mask)
    })
}

fn checked_mask(value: i64, column: &str, line: u64) -> Result<u32, BuildError> {
    u32::try_from(value).map_err(|_| BuildError::Dump {
        line,
        message: format!("{column} mask out of range: {value}"),
    })
}

const ADDON_TABLE: &str = "CREATE TEMP TABLE IF NOT EXISTS quest_addon_import (
    id INTEGER PRIMARY KEY, max_level INTEGER, allowed_classes TEXT, source_spell INTEGER,
    prev_quest INTEGER, next_quest INTEGER, exclusive_group INTEGER, breadcrumb_for INTEGER,
    required_skill INTEGER, required_skill_points INTEGER, required_min_rep_faction INTEGER,
    required_max_rep_faction INTEGER, required_min_rep_value INTEGER, required_max_rep_value INTEGER,
    special_flags INTEGER
)";

pub(super) fn initialize(conn: &Connection) -> Result<(), BuildError> {
    conn.execute_batch(ADDON_TABLE)?;
    Ok(())
}

pub(super) struct Importer<'conn> {
    quest: Statement<'conn>,
    addon: Statement<'conn>,
    relation: Statement<'conn>,
    item: Statement<'conn>,
    objective: Statement<'conn>,
    faction: Statement<'conn>,
}

impl<'conn> Importer<'conn> {
    pub(super) fn new(conn: &'conn Connection) -> Result<Self, BuildError> {
        Ok(Self {
            quest: conn.prepare("INSERT INTO quest (id,name,level,min_level,zone_id,zone,sort,type,suggested_players,flags,objectives,details,area_description,completion_log,objective_texts,allowed_races,time_allowed,reward_next_quest,start_item,required_player_kills,reward_money,reward_bonus_money,reward_xp_difficulty,reward_spell,reward_display_spell,reward_honor,reward_title,reward_talents,reward_arena_points) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)")?,
            addon: conn.prepare("INSERT INTO quest_addon_import VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)")?,
            relation: conn.prepare("INSERT INTO quest_relation (quest_id, role, source_kind, source_id) VALUES (?,?,?,?)")?,
            item: conn.prepare("INSERT INTO quest_item (quest_id,role,item_id,count) VALUES (?,?,?,?)")?,
            objective: conn.prepare("INSERT INTO quest_objective (quest_id,idx,target_kind,target_id,count) VALUES (?,?,?,?,?)")?,
            faction: conn.prepare("INSERT INTO quest_reward_faction (quest_id,idx,faction_id,value_id,override) VALUES (?,?,?,?,?)")?,
        })
    }

    pub(super) fn import(&mut self, table: &str, row: &Row<'_>, dbc: &Context) -> Result<(), BuildError> {
        match table {
            "quest_template" => self.import_quest(row, dbc),
            "quest_template_addon" => {
                self.addon.execute(params![
                    row.int("ID")?,
                    nz(row.int("MaxLevel")?),
                    mask_names(
                        names::CLASSES,
                        row.int("AllowableClasses")?,
                        0x5ff,
                        "AllowableClasses",
                        row.line()
                    )?,
                    nz(row.int("SourceSpellID")?),
                    nz(row.int("PrevQuestID")?),
                    nz(row.int("NextQuestID")?),
                    nz(row.int("ExclusiveGroup")?),
                    nz(row.int("BreadcrumbForQuestId")?),
                    nz(row.int("RequiredSkillID")?),
                    nz(row.int("RequiredSkillPoints")?),
                    nz(row.int("RequiredMinRepFaction")?),
                    nz(row.int("RequiredMaxRepFaction")?),
                    nz(row.int("RequiredMinRepValue")?),
                    nz(row.int("RequiredMaxRepValue")?),
                    nz(row.int("SpecialFlags")?),
                ])?;
                Ok(())
            }
            "creature_queststarter" | "creature_questender" | "gameobject_queststarter" | "gameobject_questender" => {
                let role = if table.ends_with("starter") { "start" } else { "end" };
                let kind = if table.starts_with("creature_") {
                    "creature"
                } else {
                    "gameobject"
                };
                self.relation
                    .execute(params![row.int("quest")?, role, kind, row.int("id")?])?;
                Ok(())
            }
            _ => Err(BuildError::Dump {
                line: 0,
                message: format!("unexpected quest table {table}"),
            }),
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "The quest row and its related records are mapped together"
    )]
    fn import_quest(&mut self, row: &Row<'_>, dbc: &Context) -> Result<(), BuildError> {
        let id = row.int("ID")?;
        let sort = row.int("QuestSortID")?;
        let info = row.int("QuestInfoID")?;
        let zone_id = (sort > 0).then_some(sort);
        let zone = zone_id
            .and_then(|id| i32::try_from(id).ok())
            .and_then(|id| dbc.area_names.get(&id));
        let mut objective_texts = String::new();
        for column in ["ObjectiveText1", "ObjectiveText2", "ObjectiveText3", "ObjectiveText4"] {
            if let Some(value) = row.text(column)? {
                if !objective_texts.is_empty() {
                    objective_texts.push_str(" | ");
                }
                objective_texts.push_str(value);
            }
        }
        self.quest.execute(params![
            id,
            row.text("LogTitle")?,
            nz(row.int("QuestLevel")?),
            nz(row.int("MinLevel")?),
            zone_id,
            zone,
            if sort < 0 {
                names::enum_name(names::QUEST_SORT, -sort)
            } else {
                None
            },
            if info != 0 {
                names::enum_name(names::QUEST_INFO, info)
            } else {
                None
            },
            nz(row.int("SuggestedGroupNum")?),
            names::flag_names(
                names::QUEST_FLAGS,
                checked_mask(row.int("Flags")?, "Flags", row.line())?
            ),
            row.text("LogDescription")?,
            row.text("QuestDescription")?,
            row.text("AreaDescription")?,
            row.text("QuestCompletionLog")?,
            (!objective_texts.is_empty()).then_some(objective_texts.as_str()),
            mask_names(
                names::RACES,
                row.int("AllowableRaces")?,
                0x6ff,
                "AllowableRaces",
                row.line()
            )?,
            nz(row.int("TimeAllowed")?),
            nz(row.int("RewardNextQuest")?),
            nz(row.int("StartItem")?),
            nz(row.int("RequiredPlayerKills")?),
            nz(row.int("RewardMoney")?),
            nz(row.int("RewardBonusMoney")?),
            nz(row.int("RewardXPDifficulty")?),
            nz(row.int("RewardSpell")?),
            nz(row.int("RewardDisplaySpell")?),
            nz(row.int("RewardHonor")?),
            nz(row.int("RewardTitle")?),
            nz(row.int("RewardTalents")?),
            nz(row.int("RewardArenaPoints")?),
        ])?;
        for (role, ids, counts) in [
            (
                "reward",
                &["RewardItem1", "RewardItem2", "RewardItem3", "RewardItem4"][..],
                &["RewardAmount1", "RewardAmount2", "RewardAmount3", "RewardAmount4"][..],
            ),
            (
                "choice",
                &[
                    "RewardChoiceItemID1",
                    "RewardChoiceItemID2",
                    "RewardChoiceItemID3",
                    "RewardChoiceItemID4",
                    "RewardChoiceItemID5",
                    "RewardChoiceItemID6",
                ][..],
                &[
                    "RewardChoiceItemQuantity1",
                    "RewardChoiceItemQuantity2",
                    "RewardChoiceItemQuantity3",
                    "RewardChoiceItemQuantity4",
                    "RewardChoiceItemQuantity5",
                    "RewardChoiceItemQuantity6",
                ][..],
            ),
            (
                "required",
                &[
                    "RequiredItemId1",
                    "RequiredItemId2",
                    "RequiredItemId3",
                    "RequiredItemId4",
                    "RequiredItemId5",
                    "RequiredItemId6",
                ][..],
                &[
                    "RequiredItemCount1",
                    "RequiredItemCount2",
                    "RequiredItemCount3",
                    "RequiredItemCount4",
                    "RequiredItemCount5",
                    "RequiredItemCount6",
                ][..],
            ),
            (
                "source",
                &["ItemDrop1", "ItemDrop2", "ItemDrop3", "ItemDrop4"][..],
                &[
                    "ItemDropQuantity1",
                    "ItemDropQuantity2",
                    "ItemDropQuantity3",
                    "ItemDropQuantity4",
                ][..],
            ),
        ] {
            for (id_col, count_col) in ids.iter().zip(counts) {
                let item_id = row.int(id_col)?;
                if item_id != 0 {
                    self.item.execute(params![id, role, item_id, nz(row.int(count_col)?)])?;
                }
            }
        }
        for (index, (target_col, count_col)) in (1_i64..).zip(
            [
                "RequiredNpcOrGo1",
                "RequiredNpcOrGo2",
                "RequiredNpcOrGo3",
                "RequiredNpcOrGo4",
            ]
            .iter()
            .zip([
                "RequiredNpcOrGoCount1",
                "RequiredNpcOrGoCount2",
                "RequiredNpcOrGoCount3",
                "RequiredNpcOrGoCount4",
            ]),
        ) {
            let target = row.int(target_col)?;
            if target != 0 {
                self.objective.execute(params![
                    id,
                    index,
                    if target < 0 { "gameobject" } else { "creature" },
                    target.abs(),
                    nz(row.int(count_col)?)
                ])?;
            }
        }
        for (index, (faction_col, value_col, override_col)) in (1_i64..).zip(
            [
                ("RewardFactionID1", "RewardFactionValue1", "RewardFactionOverride1"),
                ("RewardFactionID2", "RewardFactionValue2", "RewardFactionOverride2"),
                ("RewardFactionID3", "RewardFactionValue3", "RewardFactionOverride3"),
                ("RewardFactionID4", "RewardFactionValue4", "RewardFactionOverride4"),
                ("RewardFactionID5", "RewardFactionValue5", "RewardFactionOverride5"),
            ]
            .iter(),
        ) {
            let faction_id = row.int(faction_col)?;
            if faction_id != 0 {
                self.faction.execute(params![
                    id,
                    index,
                    faction_id,
                    nz(row.int(value_col)?),
                    nz(row.int(override_col)?)
                ])?;
            }
        }
        Ok(())
    }
}

pub(super) fn finalize(conn: &Connection) -> Result<(), BuildError> {
    conn.execute_batch(ADDON_TABLE)?;
    conn.execute_batch(
        "UPDATE quest SET
        (max_level, allowed_classes, source_spell, prev_quest, next_quest, exclusive_group,
         breadcrumb_for, required_skill, required_skill_points, required_min_rep_faction,
         required_max_rep_faction, required_min_rep_value, required_max_rep_value, special_flags) =
        (SELECT max_level, allowed_classes, source_spell, prev_quest, next_quest, exclusive_group,
         breadcrumb_for, required_skill, required_skill_points, required_min_rep_faction,
         required_max_rep_faction, required_min_rep_value, required_max_rep_value, special_flags
         FROM quest_addon_import WHERE quest_addon_import.id = quest.id)
        WHERE EXISTS (SELECT 1 FROM quest_addon_import WHERE quest_addon_import.id = quest.id);
        DROP TABLE quest_addon_import;",
    )?;
    Ok(())
}
