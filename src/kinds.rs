//! Kind metadata and the fixed, parameterized relation queries.

use rmcp::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

macro_rules! named_enum {
    ($name:ident { $($variant:ident => $text:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
        #[serde(rename_all = "snake_case")]
        pub(crate) enum $name { $($variant),+ }

        impl $name {
            pub(crate) const fn as_str(self) -> &'static str {
                match self { $(Self::$variant => $text),+ }
            }
        }
    };
}

named_enum!(Kind {
    Item => "item", Creature => "creature", Spell => "spell", Itemset => "itemset",
    Quest => "quest", Gameobject => "gameobject", Map => "map", Area => "area",
    Faction => "faction", Talent => "talent", Skill => "skill", Glyph => "glyph",
    Achievement => "achievement", Offset => "offset", LuaFunction => "lua_function",
    LuaEvent => "lua_event", Widget => "widget", Cvar => "cvar", LuaType => "lua_type",
    CombatlogEvent => "combatlog_event", SecureTemplate => "secure_template", Meta => "meta"
});

named_enum!(Relation {
    DroppedBy => "dropped_by", SoldBy => "sold_by", RewardOf => "reward_of", Set => "set",
    CreatedBy => "created_by", Disenchant => "disenchant", Loot => "loot", Sells => "sells",
    Trains => "trains", StartsQuests => "starts_quests", EndsQuests => "ends_quests",
    Spawns => "spawns", Spells => "spells", Givers => "givers", Enders => "enders",
    Chain => "chain", Rewards => "rewards", Requirements => "requirements", Ranks => "ranks",
    TaughtBy => "taught_by", UsedBy => "used_by", Talent => "talent", Glyph => "glyph",
    Triggered => "triggered", Children => "children", Parent => "parent", Members => "members",
    Criteria => "criteria"
});

macro_rules! loot_cols {
    () => {
        "l.chance, CASE WHEN l.min_count = l.max_count THEN l.min_count ELSE l.min_count || '-' || l.max_count END AS count, l.quest_required AS quest, l.group_id AS \"group\""
    };
}

macro_rules! loot_rows {
    () => {
        concat!("SELECT CASE WHEN l.reference IS NULL THEN 'item' ELSE 'reference' END AS kind, coalesce(l.item, l.reference) AS id, i.name, ", loot_cols!())
    };
}

impl Kind {
    pub(crate) const fn table(self) -> &'static str {
        match self {
            Self::Offset => "offsets",
            _ => self.as_str(),
        }
    }

    pub(crate) const fn key(self) -> &'static str {
        match self {
            Self::Offset
            | Self::LuaFunction
            | Self::LuaEvent
            | Self::Widget
            | Self::Cvar
            | Self::LuaType
            | Self::CombatlogEvent
            | Self::SecureTemplate
            | Self::Meta => "name",
            _ => "id",
        }
    }

    pub(crate) const fn summary(self) -> &'static [&'static str] {
        match self {
            Self::Item => &[
                "name",
                "quality",
                "item_level",
                "required_level",
                "class",
                "subclass",
                "slot",
                "bonding",
                "stats",
                "armor",
                "damage",
                "speed",
                "spells",
                "sockets",
                "itemset",
                "icon",
            ],
            Self::Creature => &[
                "name",
                "subname",
                "min_level",
                "max_level",
                "rank",
                "type",
                "faction",
                "npc_flags",
            ],
            Self::Spell => &[
                "name",
                "rank",
                "school",
                "power_type",
                "mana_cost",
                "mana_cost_pct",
                "cast_time",
                "range_min",
                "range_max",
                "cooldown",
                "duration",
                "description",
                "icon",
            ],
            Self::Itemset => &["name", "items", "bonuses", "required_skill", "required_skill_rank"],
            Self::Quest => &[
                "name",
                "level",
                "min_level",
                "zone",
                "sort",
                "type",
                "objectives",
                "prev_quest",
                "next_quest",
                "exclusive_group",
                "reward_money",
                "reward_spell",
            ],
            Self::Gameobject => &["name", "type", "display_id", "loot_id"],
            Self::Map => &["name", "type", "expansion", "max_players"],
            Self::Area => &["name", "map", "parent", "level", "team"],
            Self::Faction => &["name", "description", "parent"],
            Self::Talent => &[
                "name",
                "tab_id",
                "tab",
                "class",
                "tier",
                "column_index",
                "max_rank",
                "ranks",
                "prereq_talent",
                "prereq_rank",
                "required_spell",
            ],
            Self::Skill => &["name", "category", "description"],
            Self::Glyph => &["name", "spell_id", "type", "icon"],
            Self::Achievement => &["name", "description", "category", "points", "faction", "reward"],
            Self::Offset => &["domain", "platform", "va", "rva", "status", "comment"],
            Self::LuaFunction => &[
                "signature",
                "description",
                "parameters",
                "returns",
                "protected",
                "hw_event",
            ],
            Self::LuaEvent => &["description", "parameters", "category"],
            Self::Widget => &["description", "inherits", "methods", "script_handlers"],
            Self::Cvar => &["default_value", "description", "category"],
            Self::LuaType => &["description", "enum_values"],
            Self::CombatlogEvent => &["type", "prefixes", "description", "arguments"],
            Self::SecureTemplate => &["description", "attributes", "examples"],
            Self::Meta => &["value"],
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "keep the static SQL registry together for lookup by kind"
    )]
    pub(crate) const fn relations(self) -> &'static [(Relation, &'static str)] {
        use Relation as R;
        match self {
            Self::Item => &[
                (
                    R::DroppedBy,
                    concat!(
                        "SELECT * FROM (SELECT 'creature' AS kind, c.id, c.name, ",
                        loot_cols!(),
                        " FROM loot l JOIN creature c ON c.loot_id = l.entry WHERE l.source = 'creature' AND l.item = ?1 UNION ALL SELECT 'gameobject', g.id, g.name, ",
                        loot_cols!(),
                        " FROM loot l JOIN gameobject g ON g.loot_id = l.entry WHERE l.source = 'gameobject' AND l.item = ?1 UNION ALL SELECT 'item', x.id, x.name, ",
                        loot_cols!(),
                        " FROM loot l JOIN item x ON x.id = l.entry WHERE l.source = 'item' AND l.item = ?1 UNION ALL SELECT 'reference', l.entry, NULL, ",
                        loot_cols!(),
                        " FROM loot l WHERE l.source = 'reference' AND l.item = ?1) ORDER BY chance DESC, kind, id"
                    ),
                ),
                (
                    R::SoldBy,
                    "SELECT 'creature', c.id, c.name, i.buy_price AS price, v.cost, v.max_count, v.restock_time FROM vendor v JOIN creature c ON c.id = v.creature_id LEFT JOIN item i ON i.id = v.item_id WHERE v.item_id = ?1 ORDER BY c.id",
                ),
                (
                    R::RewardOf,
                    "SELECT 'quest', q.id, q.name, qi.role, qi.count FROM quest_item qi JOIN quest q ON q.id = qi.quest_id WHERE qi.item_id = ?1 AND qi.role IN ('reward', 'choice') ORDER BY q.id",
                ),
                (
                    R::Set,
                    "SELECT kind, id, name, pieces FROM (SELECT 0 AS o, 'itemset' AS kind, s.id, s.name, NULL AS pieces, 0 AS k FROM item i JOIN itemset s ON s.id = i.itemset WHERE i.id = ?1 UNION ALL SELECT 1, 'item', si.item_id, x.name, NULL, si.slot FROM item i JOIN itemset_item si ON si.itemset_id = i.itemset LEFT JOIN item x ON x.id = si.item_id WHERE i.id = ?1 UNION ALL SELECT 2, 'spell', b.spell_id, sp.name, b.threshold, b.threshold FROM item i JOIN itemset_bonus b ON b.itemset_id = i.itemset LEFT JOIN spell sp ON sp.id = b.spell_id WHERE i.id = ?1) ORDER BY o, k, id",
                ),
                (
                    R::CreatedBy,
                    "SELECT 'spell', s.id, s.name, e.value AS count FROM spell_effect e JOIN spell s ON s.id = e.spell_id WHERE e.effect IN ('CREATE_ITEM', 'CREATE_ITEM_2') AND e.item_type = ?1 ORDER BY s.id",
                ),
                (
                    R::Disenchant,
                    concat!(
                        loot_rows!(),
                        " FROM item d JOIN loot l ON l.source = 'disenchant' AND l.entry = d.disenchant_id LEFT JOIN item i ON i.id = l.item WHERE d.id = ?1 ORDER BY l.chance DESC, id"
                    ),
                ),
            ],
            Self::Creature => &[
                (
                    R::Loot,
                    concat!(
                        loot_rows!(),
                        " FROM creature c JOIN loot l ON l.source = 'creature' AND l.entry = c.loot_id LEFT JOIN item i ON i.id = l.item WHERE c.id = ?1 ORDER BY l.group_id, l.chance DESC, id"
                    ),
                ),
                (
                    R::Sells,
                    "SELECT 'item', v.item_id, i.name, i.buy_price AS price, v.cost, v.max_count, v.restock_time FROM vendor v LEFT JOIN item i ON i.id = v.item_id WHERE v.creature_id = ?1 ORDER BY v.slot, v.item_id",
                ),
                (
                    R::Trains,
                    "SELECT 'spell', ts.spell_id, s.name, s.rank, ts.cost, ts.req_level, ts.req_skill, ts.req_skill_rank FROM creature_trainer ct JOIN trainer_spell ts ON ts.trainer_id = ct.trainer_id LEFT JOIN spell s ON s.id = ts.spell_id WHERE ct.creature_id = ?1 ORDER BY ts.req_level, ts.spell_id",
                ),
                (
                    R::StartsQuests,
                    "SELECT 'quest', q.id, q.name, q.level FROM quest_relation r JOIN quest q ON q.id = r.quest_id WHERE r.source_kind = 'creature' AND r.source_id = ?1 AND r.role = 'start' ORDER BY q.id",
                ),
                (
                    R::EndsQuests,
                    "SELECT 'quest', q.id, q.name, q.level FROM quest_relation r JOIN quest q ON q.id = r.quest_id WHERE r.source_kind = 'creature' AND r.source_id = ?1 AND r.role = 'end' ORDER BY q.id",
                ),
                (
                    R::Spawns,
                    "SELECT 'spawn', s.guid, coalesce(z.name, m.name), s.map, s.zone, s.area, s.x, s.y, s.z, s.orientation AS o, s.spawn_time FROM creature_spawn s LEFT JOIN area z ON z.id = s.zone LEFT JOIN map m ON m.id = s.map WHERE s.creature_id = ?1 ORDER BY s.guid",
                ),
                (
                    R::Spells,
                    "SELECT 'spell', cs.spell_id, s.name, s.rank FROM creature_spell cs LEFT JOIN spell s ON s.id = cs.spell_id WHERE cs.creature_id = ?1 ORDER BY cs.slot",
                ),
            ],
            Self::Quest => &[
                (
                    R::Givers,
                    "SELECT * FROM (SELECT r.source_kind AS kind, r.source_id AS id, coalesce(c.name, g.name) AS name FROM quest_relation r LEFT JOIN creature c ON r.source_kind = 'creature' AND c.id = r.source_id LEFT JOIN gameobject g ON r.source_kind = 'gameobject' AND g.id = r.source_id WHERE r.quest_id = ?1 AND r.role = 'start' UNION ALL SELECT 'item', i.id, i.name FROM item i WHERE i.start_quest = ?1) ORDER BY kind, id",
                ),
                (
                    R::Enders,
                    "SELECT r.source_kind AS kind, r.source_id AS id, coalesce(c.name, g.name) AS name FROM quest_relation r LEFT JOIN creature c ON r.source_kind = 'creature' AND c.id = r.source_id LEFT JOIN gameobject g ON r.source_kind = 'gameobject' AND g.id = r.source_id WHERE r.quest_id = ?1 AND r.role = 'end' ORDER BY kind, id",
                ),
                (
                    R::Chain,
                    "SELECT * FROM (SELECT 'quest' AS kind, p.id, p.name, 'previous' AS link, CASE WHEN q.prev_quest < 0 THEN 1 END AS must_be_active FROM quest q JOIN quest p ON p.id = abs(q.prev_quest) WHERE q.id = ?1 UNION ALL SELECT 'quest', n.id, n.name, 'next', NULL FROM quest q JOIN quest n ON n.id = q.next_quest WHERE q.id = ?1 UNION ALL SELECT 'quest', n.id, n.name, 'reward_next', NULL FROM quest q JOIN quest n ON n.id = q.reward_next_quest WHERE q.id = ?1 UNION ALL SELECT 'quest', f.id, f.name, 'follow_up', CASE WHEN f.prev_quest < 0 THEN 1 END FROM quest f WHERE abs(f.prev_quest) = ?1 UNION ALL SELECT 'quest', e.id, e.name, CASE WHEN q.exclusive_group < 0 THEN 'group_all' ELSE 'exclusive' END, NULL FROM quest q JOIN quest e ON e.exclusive_group = q.exclusive_group AND e.id <> q.id WHERE q.id = ?1 AND q.exclusive_group IS NOT NULL UNION ALL SELECT 'quest', b.id, b.name, 'breadcrumb_for', NULL FROM quest q JOIN quest b ON b.id = q.breadcrumb_for WHERE q.id = ?1 UNION ALL SELECT 'quest', b.id, b.name, 'breadcrumb', NULL FROM quest b WHERE b.breadcrumb_for = ?1) ORDER BY link, id",
                ),
                (
                    R::Rewards,
                    "SELECT * FROM (SELECT 'item' AS kind, qi.item_id AS id, i.name, qi.count, CASE WHEN qi.role = 'choice' THEN 1 END AS choice, NULL AS value_id, NULL AS override FROM quest_item qi LEFT JOIN item i ON i.id = qi.item_id WHERE qi.quest_id = ?1 AND qi.role IN ('reward', 'choice') UNION ALL SELECT 'spell', s.id, s.name, NULL, NULL, NULL, NULL FROM quest q JOIN spell s ON s.id IN (q.reward_spell, q.reward_display_spell) WHERE q.id = ?1 UNION ALL SELECT 'faction', f.faction_id, fa.name, NULL, NULL, f.value_id, f.override FROM quest_reward_faction f LEFT JOIN faction fa ON fa.id = f.faction_id WHERE f.quest_id = ?1) ORDER BY kind, id",
                ),
                (
                    R::Requirements,
                    "SELECT * FROM (SELECT 'item' AS kind, qi.item_id AS id, i.name, qi.count, CASE WHEN qi.role = 'source' THEN 1 END AS provided FROM quest_item qi LEFT JOIN item i ON i.id = qi.item_id WHERE qi.quest_id = ?1 AND qi.role IN ('required', 'source') UNION ALL SELECT o.target_kind, o.target_id, coalesce(c.name, g.name), o.count, NULL FROM quest_objective o LEFT JOIN creature c ON o.target_kind = 'creature' AND c.id = o.target_id LEFT JOIN gameobject g ON o.target_kind = 'gameobject' AND g.id = o.target_id WHERE o.quest_id = ?1) ORDER BY kind, id",
                ),
            ],
            Self::Spell => &[
                (
                    R::Ranks,
                    "SELECT 'spell', s.id, s.name, r.rank FROM spell_rank x JOIN spell_rank r ON r.first_spell_id = x.first_spell_id JOIN spell s ON s.id = r.spell_id WHERE x.spell_id = ?1 ORDER BY r.rank",
                ),
                (
                    R::TaughtBy,
                    "SELECT * FROM (SELECT 'creature' AS kind, c.id, c.name, ts.cost, ts.req_level FROM trainer_spell ts JOIN creature_trainer ct ON ct.trainer_id = ts.trainer_id JOIN creature c ON c.id = ct.creature_id WHERE ts.spell_id = ?1 UNION ALL SELECT 'item', i.id, i.name, NULL, NULL FROM item_spell x JOIN item i ON i.id = x.item_id WHERE x.spell_id = ?1 AND x.trigger = 'LEARN_SPELL_ID' UNION ALL SELECT 'spell', s.id, s.name, NULL, NULL FROM spell_effect e JOIN spell s ON s.id = e.spell_id WHERE e.effect IN ('LEARN_SPELL', 'LEARN_PET_SPELL') AND e.trigger_spell = ?1 UNION ALL SELECT 'quest', q.id, q.name, NULL, NULL FROM quest q WHERE q.reward_spell = ?1 OR q.reward_display_spell = ?1) ORDER BY kind, id",
                ),
                (
                    R::UsedBy,
                    "SELECT * FROM (SELECT 'item' AS kind, i.id, i.name, x.trigger, x.charges FROM item_spell x JOIN item i ON i.id = x.item_id WHERE x.spell_id = ?1 AND x.trigger <> 'LEARN_SPELL_ID' UNION ALL SELECT 'creature', c.id, c.name, NULL, NULL FROM creature_spell cs JOIN creature c ON c.id = cs.creature_id WHERE cs.spell_id = ?1) ORDER BY kind, id",
                ),
                (
                    R::Talent,
                    "SELECT 'talent', t.id, t.name, ts.rank, t.tab, t.class, t.tier, t.column_index FROM talent_spell ts JOIN talent t ON t.id = ts.talent_id WHERE ts.spell_id = ?1",
                ),
                (
                    R::Glyph,
                    "SELECT * FROM (SELECT 'glyph' AS kind, g.id, g.name, g.type FROM glyph g WHERE g.spell_id = ?1 UNION ALL SELECT 'item', i.id, i.name, NULL FROM glyph g JOIN spell_effect e ON e.effect = 'APPLY_GLYPH' AND e.misc_value = g.id JOIN item_spell x ON x.spell_id = e.spell_id JOIN item i ON i.id = x.item_id WHERE g.spell_id = ?1) ORDER BY kind, id",
                ),
                (
                    R::Triggered,
                    "SELECT 'spell', e.trigger_spell, s.name, e.idx AS effect_index, e.effect FROM spell_effect e LEFT JOIN spell s ON s.id = e.trigger_spell WHERE e.spell_id = ?1 AND e.trigger_spell IS NOT NULL ORDER BY e.idx",
                ),
            ],
            Self::Gameobject => &[
                (
                    R::Loot,
                    concat!(
                        loot_rows!(),
                        " FROM gameobject g JOIN loot l ON l.source = 'gameobject' AND l.entry = g.loot_id LEFT JOIN item i ON i.id = l.item WHERE g.id = ?1 ORDER BY l.group_id, l.chance DESC, id"
                    ),
                ),
                (
                    R::Spawns,
                    "SELECT 'spawn', s.guid, coalesce(z.name, m.name), s.map, s.zone, s.area, s.x, s.y, s.z, s.orientation AS o, s.spawn_time FROM gameobject_spawn s LEFT JOIN area z ON z.id = s.zone LEFT JOIN map m ON m.id = s.map WHERE s.gameobject_id = ?1 ORDER BY s.guid",
                ),
            ],
            Self::Area => &[
                (
                    R::Children,
                    "SELECT 'area', id, name, level FROM area WHERE parent_id = ?1 ORDER BY name, id",
                ),
                (
                    R::Parent,
                    "SELECT * FROM (SELECT 'area' AS kind, p.id, p.name FROM area a JOIN area p ON p.id = a.parent_id WHERE a.id = ?1 UNION ALL SELECT 'map', m.id, m.name FROM area a JOIN map m ON m.id = a.map_id WHERE a.id = ?1) ORDER BY kind",
                ),
            ],
            Self::Faction => &[(
                R::Members,
                "SELECT 'creature', id, name, min_level, max_level FROM creature WHERE faction_id = ?1 ORDER BY id",
            )],
            Self::Achievement => &[(
                R::Criteria,
                "SELECT 'criteria', id, description, type, asset_id, quantity FROM achievement_criteria WHERE achievement_id = ?1 ORDER BY ui_order, id",
            )],
            _ => &[],
        }
    }
}
