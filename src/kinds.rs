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

// An enum that limits a tool parameter to a subset of `Kind`; variants map onto `Kind`.
macro_rules! kind_enum {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
        #[serde(rename_all = "snake_case")]
        pub(crate) enum $name { $($variant),+ }

        impl $name {
            pub(crate) const fn kind(self) -> Kind {
                match self { $(Self::$variant => Kind::$variant),+ }
            }
        }
    };
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub(crate) enum RecordId {
    Number(i64),
    Text(String),
}

// Every searchable kind; `meta` is excluded because it has no records to find.
kind_enum!(SearchKind {
    Item,
    Creature,
    Spell,
    Itemset,
    Quest,
    Gameobject,
    Map,
    Area,
    Faction,
    Talent,
    Skill,
    Glyph,
    Achievement,
    Offset,
    LuaFunction,
    LuaEvent,
    Widget,
    Cvar,
    LuaType,
    CombatlogEvent,
    SecureTemplate
});

// Only the kinds with relations, so `related` cannot be called with an unusable kind.
kind_enum!(RelatedKind {
    Item,
    Creature,
    Quest,
    Spell,
    Gameobject,
    Area,
    Faction,
    Achievement
});

macro_rules! loot_cols {
    () => {
        "l.chance, CASE WHEN l.min_count = l.max_count THEN l.min_count ELSE l.min_count || '-' || l.max_count END AS count, l.quest_required AS quest, l.group_id AS \"group\""
    };
}

// Raw loot columns for a CTE body; the loot table is always aliased `l`.
macro_rules! loot_fields {
    () => {
        "l.chance, l.min_count, l.max_count, l.quest_required, l.group_id"
    };
}

// Kind, id and name of a loot row: an item, or the reference table that holds it.
macro_rules! loot_entry {
    () => {
        "CASE WHEN l.reference IS NULL THEN 'item' ELSE 'reference' END, coalesce(l.item, l.reference), i.name"
    };
}

// A loot table plus every item of the reference tables it points at, nested up to five
// levels. Each expanded row carries `via` = the reference table that contains it, and
// `chance` = the item's chance inside that table.
macro_rules! loot_tree {
    ($filter:literal) => {
        concat!(
            "WITH RECURSIVE base(path, kind, id, name, chance, min_count, max_count, quest_required, group_id) AS (",
            "SELECT printf('%06d', row_number() OVER (ORDER BY l.group_id, l.chance DESC, coalesce(l.item, l.reference))), ",
            loot_entry!(),
            ", ",
            loot_fields!(),
            " FROM loot l LEFT JOIN item i ON i.id = l.item WHERE ",
            $filter,
            "), tree(path, depth, visited, kind, id, name, chance, min_count, max_count, quest_required, group_id, via) AS (",
            "SELECT path, 0, ',', kind, id, name, chance, min_count, max_count, quest_required, group_id, NULL FROM base",
            " UNION ALL SELECT t.path || printf('|%04d%09d%010d', l.group_id, 1000000 - CAST(round(l.chance * 100) AS INTEGER), coalesce(l.item, l.reference)), t.depth + 1, t.visited || t.id || ',', ",
            loot_entry!(),
            ", ",
            loot_fields!(),
            ", t.id FROM tree t JOIN loot l ON l.source = 'reference' AND l.entry = t.id",
            " LEFT JOIN item i ON i.id = l.item",
            " WHERE t.kind = 'reference' AND t.depth < 5 AND instr(t.visited, ',' || t.id || ',') = 0) ",
            "SELECT kind, id, name, ",
            loot_cols!(),
            ", via FROM tree l ORDER BY l.path"
        )
    };
}

// Faction name and the zone of the lowest-GUID spawn of creature `c`. Most TDB 335 spawns
// have no zone id, so the zone falls back to the map name, as in the spawns relation.
macro_rules! creature_zone {
    () => {
        ", c.faction, (SELECT coalesce(a.name, m.name) FROM creature_spawn s LEFT JOIN area a ON a.id = s.zone LEFT JOIN map m ON m.id = s.map WHERE s.creature_id = c.id ORDER BY s.guid LIMIT 1) AS zone"
    };
}

// The whole quest chain around one quest: ancestors, the quest itself, descendants.
macro_rules! quest_chain {
    () => {
        concat!(
            "WITH RECURSIVE chain(id, step, link, must_be_active, up, down) AS (",
            "SELECT ?1, 0, 'self', NULL, 0, 0",
            " UNION SELECT p.id, c.step - 1, 'previous', CASE WHEN q.prev_quest < 0 THEN 1 END, c.up + 1, 0",
            " FROM chain c JOIN quest q ON q.id = c.id JOIN quest p ON p.id = abs(q.prev_quest)",
            " WHERE c.down = 0 AND c.up < 50 AND q.prev_quest <> 0",
            " UNION SELECT n.id, c.step + 1, 'next', NULL, 0, c.down + 1",
            " FROM chain c JOIN quest q ON q.id = c.id JOIN quest n ON n.id = q.next_quest",
            " WHERE c.up = 0 AND c.down < 50 AND q.next_quest <> 0",
            " UNION SELECT n.id, c.step + 1, 'reward_next', NULL, 0, c.down + 1",
            " FROM chain c JOIN quest q ON q.id = c.id JOIN quest n ON n.id = q.reward_next_quest",
            " WHERE c.up = 0 AND c.down < 50 AND q.reward_next_quest <> 0",
            " UNION SELECT f.id, c.step + 1, 'follow_up', CASE WHEN f.prev_quest < 0 THEN 1 END, 0, c.down + 1",
            " FROM chain c JOIN quest f ON f.prev_quest = c.id OR f.prev_quest = -c.id",
            " WHERE c.up = 0 AND c.down < 50), ",
            "ranked(id, step, link, must_be_active, rank) AS (",
            "SELECT id, step, link, must_be_active, row_number() OVER (PARTITION BY id",
            " ORDER BY abs(step), CASE link WHEN 'previous' THEN 0 WHEN 'next' THEN 1 WHEN 'reward_next' THEN 2 ELSE 3 END, step) AS rank",
            " FROM chain) ",
            // `step` is printed as text: the formatter drops numeric zeros, but step 0 marks
            // the queried quest itself.
            "SELECT kind, id, name, link, must_be_active, printf('%d', step) AS step FROM (",
            "SELECT 'quest' AS kind, r.id, q.name, r.link, r.must_be_active, r.step",
            " FROM ranked r JOIN quest q ON q.id = r.id WHERE r.rank = 1",
            " UNION ALL SELECT 'quest', e.id, e.name, CASE WHEN q.exclusive_group < 0 THEN 'group_all' ELSE 'exclusive' END, NULL, 0",
            " FROM quest q JOIN quest e ON e.exclusive_group = q.exclusive_group AND e.id <> q.id",
            " WHERE q.id = ?1 AND q.exclusive_group IS NOT NULL",
            " UNION ALL SELECT 'quest', b.id, b.name, 'breadcrumb_for', NULL, 1",
            " FROM quest q JOIN quest b ON b.id = q.breadcrumb_for WHERE q.id = ?1",
            " UNION ALL SELECT 'quest', b.id, b.name, 'breadcrumb', NULL, -1 FROM quest b WHERE b.breadcrumb_for = ?1)",
            " ORDER BY CAST(step AS INTEGER), CASE link WHEN 'self' THEN 0 WHEN 'breadcrumb' THEN 1",
            " WHEN 'previous' THEN 2 WHEN 'next' THEN 3 WHEN 'reward_next' THEN 4 ELSE 5 END, id"
        )
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
                    // Direct drops, plus every source that uses a reference table holding
                    // the item, nested up to five levels; `via` names the innermost
                    // reference table and `chance` is the item's chance inside it.
                    concat!(
                        "WITH RECURSIVE refs(via, ref, depth, chance, min_count, max_count, quest_required, group_id) AS (",
                        "SELECT l.entry, l.entry, 0, ",
                        loot_fields!(),
                        " FROM loot l WHERE l.source = 'reference' AND l.item = ?1",
                        " UNION SELECT r.via, l.entry, r.depth + 1, r.chance, r.min_count, r.max_count, r.quest_required, r.group_id",
                        " FROM refs r CROSS JOIN loot l INDEXED BY loot_reference",
                        " WHERE l.reference = r.ref AND l.source = 'reference' AND r.depth < 5), ",
                        "rows(kind, id, name, chance, min_count, max_count, quest_required, group_id, via) AS (",
                        "SELECT 'creature', c.id, c.name, ",
                        loot_fields!(),
                        ", NULL FROM loot l JOIN creature c ON c.loot_id = l.entry WHERE l.source = 'creature' AND l.item = ?1",
                        " UNION SELECT 'gameobject', g.id, g.name, ",
                        loot_fields!(),
                        ", NULL FROM loot l JOIN gameobject g ON g.loot_id = l.entry WHERE l.source = 'gameobject' AND l.item = ?1",
                        " UNION SELECT 'item', x.id, x.name, ",
                        loot_fields!(),
                        ", NULL FROM loot l JOIN item x ON x.id = l.entry WHERE l.source = 'item' AND l.item = ?1",
                        " UNION SELECT 'creature', c.id, c.name, r.chance, r.min_count, r.max_count, r.quest_required, r.group_id, r.via",
                        " FROM refs r CROSS JOIN loot l INDEXED BY loot_reference CROSS JOIN creature c",
                        " WHERE l.reference = r.ref AND l.source = 'creature' AND c.loot_id = l.entry",
                        " UNION SELECT 'gameobject', g.id, g.name, r.chance, r.min_count, r.max_count, r.quest_required, r.group_id, r.via",
                        " FROM refs r CROSS JOIN loot l INDEXED BY loot_reference CROSS JOIN gameobject g",
                        " WHERE l.reference = r.ref AND l.source = 'gameobject' AND g.loot_id = l.entry",
                        " UNION SELECT 'item', x.id, x.name, r.chance, r.min_count, r.max_count, r.quest_required, r.group_id, r.via",
                        " FROM refs r CROSS JOIN loot l INDEXED BY loot_reference CROSS JOIN item x",
                        " WHERE l.reference = r.ref AND l.source = 'item' AND x.id = l.entry)",
                        " SELECT kind, id, name, ",
                        loot_cols!(),
                        ", via FROM rows l ORDER BY l.chance DESC, l.kind, l.id"
                    ),
                ),
                (
                    R::SoldBy,
                    concat!(
                        "SELECT 'creature', c.id, c.name, i.buy_price AS price, v.cost, v.max_count, v.restock_time",
                        creature_zone!(),
                        " FROM vendor v JOIN creature c ON c.id = v.creature_id LEFT JOIN item i ON i.id = v.item_id WHERE v.item_id = ?1 ORDER BY c.id"
                    ),
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
                    loot_tree!("l.source = 'disenchant' AND l.entry = (SELECT disenchant_id FROM item WHERE id = ?1)"),
                ),
            ],
            Self::Creature => &[
                (
                    R::Loot,
                    loot_tree!("l.source = 'creature' AND l.entry = (SELECT loot_id FROM creature WHERE id = ?1)"),
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
                    concat!(
                        "SELECT * FROM (SELECT r.source_kind AS kind, r.source_id AS id, coalesce(c.name, g.name) AS name",
                        creature_zone!(),
                        " FROM quest_relation r LEFT JOIN creature c ON r.source_kind = 'creature' AND c.id = r.source_id LEFT JOIN gameobject g ON r.source_kind = 'gameobject' AND g.id = r.source_id WHERE r.quest_id = ?1 AND r.role = 'start'",
                        " UNION ALL SELECT 'item', i.id, i.name, NULL, NULL FROM item i WHERE i.start_quest = ?1) ORDER BY kind, id"
                    ),
                ),
                (
                    R::Enders,
                    concat!(
                        "SELECT r.source_kind AS kind, r.source_id AS id, coalesce(c.name, g.name) AS name",
                        creature_zone!(),
                        " FROM quest_relation r LEFT JOIN creature c ON r.source_kind = 'creature' AND c.id = r.source_id LEFT JOIN gameobject g ON r.source_kind = 'gameobject' AND g.id = r.source_id WHERE r.quest_id = ?1 AND r.role = 'end' ORDER BY kind, id"
                    ),
                ),
                (R::Chain, quest_chain!()),
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
                    concat!(
                        "SELECT * FROM (SELECT 'creature' AS kind, c.id, c.name, ts.cost, ts.req_level",
                        creature_zone!(),
                        " FROM trainer_spell ts JOIN creature_trainer ct ON ct.trainer_id = ts.trainer_id JOIN creature c ON c.id = ct.creature_id WHERE ts.spell_id = ?1",
                        " UNION ALL SELECT 'item', i.id, i.name, NULL, NULL, NULL, NULL FROM item_spell x JOIN item i ON i.id = x.item_id WHERE x.spell_id = ?1 AND x.trigger = 'LEARN_SPELL_ID'",
                        " UNION ALL SELECT 'spell', s.id, s.name, NULL, NULL, NULL, NULL FROM spell_effect e JOIN spell s ON s.id = e.spell_id WHERE e.effect IN ('LEARN_SPELL', 'LEARN_PET_SPELL') AND e.trigger_spell = ?1",
                        " UNION ALL SELECT 'quest', q.id, q.name, NULL, NULL, NULL, NULL FROM quest q WHERE q.reward_spell = ?1 OR q.reward_display_spell = ?1) ORDER BY kind, id"
                    ),
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
                    loot_tree!("l.source = 'gameobject' AND l.entry = (SELECT loot_id FROM gameobject WHERE id = ?1)"),
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
