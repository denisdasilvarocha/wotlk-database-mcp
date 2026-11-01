//! Curated tables and indexes shared by the builder and server.

pub(crate) const CREATE: &str = r"
CREATE TABLE item (
  id INTEGER PRIMARY KEY, -- Item entry ID.
  name TEXT, -- Item name.
  quality TEXT, -- Decoded item quality.
  item_level INTEGER, -- Item level.
  required_level INTEGER, -- Minimum character level.
  class TEXT, -- Decoded item class.
  subclass TEXT, -- Decoded item subclass.
  slot TEXT, -- Decoded inventory slot; NULL for none.
  bonding TEXT, -- Decoded binding type; NULL for none.
  flags TEXT, -- Decoded item flags separated by |.
  flags2 TEXT, -- Decoded extra item flags separated by |.
  display_id INTEGER, -- Item display ID.
  icon TEXT, -- Inventory icon from the item display.
  description TEXT, -- Item flavor text.
  buy_price INTEGER, -- Purchase price in copper.
  buy_count INTEGER, -- Number bought per purchase; NULL for one.
  sell_price INTEGER, -- Sale price in copper.
  max_count INTEGER, -- Maximum owned count.
  stackable INTEGER, -- Maximum stack size; NULL for one.
  container_slots INTEGER, -- Number of container slots.
  allowed_classes TEXT, -- Allowed class names separated by |; NULL for all.
  allowed_races TEXT, -- Allowed race names separated by |; NULL for all.
  required_skill INTEGER, -- Required skill ID.
  required_skill_rank INTEGER, -- Required skill rank.
  required_spell INTEGER, -- Required spell ID.
  required_faction INTEGER, -- Required reputation faction ID.
  required_reputation TEXT, -- Required decoded reputation rank.
  stats TEXT, -- Nonzero stat names and signed values separated by commas.
  armor INTEGER, -- Armor value.
  damage TEXT, -- Damage ranges and schools separated by semicolons.
  speed REAL, -- Weapon delay in seconds.
  block INTEGER, -- Block value.
  resistances TEXT, -- School resistance names and signed values separated by commas.
  spells TEXT, -- Trigger, spell ID and optional name separated by semicolons.
  sockets TEXT, -- Decoded socket colors separated by commas.
  socket_bonus INTEGER, -- Socket bonus enchantment ID.
  gem_properties INTEGER, -- Gem properties ID.
  itemset INTEGER, -- Item set ID.
  start_quest INTEGER, -- Quest started by the item.
  lock_id INTEGER, -- Lock ID.
  max_durability INTEGER, -- Maximum durability.
  duration INTEGER, -- Duration in seconds.
  area INTEGER, -- Required area ID.
  map INTEGER, -- Required map ID.
  bag_family INTEGER, -- Bag family mask.
  totem_category INTEGER, -- Totem category ID.
  random_property INTEGER, -- Random property ID.
  random_suffix INTEGER, -- Random suffix ID.
  disenchant_id INTEGER, -- Disenchant loot entry ID.
  required_disenchant_skill INTEGER, -- Required disenchant skill; NULL for default -1.
  item_limit_category INTEGER, -- Item limit category ID.
  page_text INTEGER -- Page text ID.
);
CREATE TABLE creature (
  id INTEGER PRIMARY KEY, -- Creature entry ID.
  name TEXT, -- Creature name.
  subname TEXT, -- Creature subtitle.
  min_level INTEGER, -- Minimum level.
  max_level INTEGER, -- Maximum level.
  rank TEXT, -- Decoded elite rank; NULL for normal.
  type TEXT, -- Decoded creature type.
  family TEXT, -- Decoded creature family.
  faction_template INTEGER, -- Faction template ID.
  faction_id INTEGER, -- Faction ID resolved from the faction template.
  faction TEXT, -- Resolved faction name.
  npc_flags TEXT, -- Decoded NPC flags separated by |.
  unit_class TEXT, -- Decoded unit class.
  type_flags TEXT, -- Decoded creature type flags separated by |.
  expansion INTEGER, -- Expansion number.
  loot_id INTEGER, -- Creature loot entry ID.
  pickpocket_loot INTEGER, -- Pickpocket loot entry ID.
  skin_loot INTEGER, -- Skinning loot entry ID.
  min_gold INTEGER, -- Minimum money in copper.
  max_gold INTEGER, -- Maximum money in copper.
  health_modifier REAL, -- Health multiplier; NULL for one.
  mana_modifier REAL, -- Mana multiplier; NULL for one.
  armor_modifier REAL, -- Armor multiplier; NULL for one.
  damage_modifier REAL, -- Damage multiplier; NULL for one.
  speed_walk REAL, -- Walk speed multiplier; NULL for one.
  speed_run REAL, -- Run speed multiplier; NULL for default 1.14286.
  scale REAL, -- Model scale; NULL for one.
  models TEXT, -- Model IDs separated by commas.
  mechanic_immune TEXT, -- Immune mechanic names separated by |.
  vehicle_id INTEGER, -- Vehicle ID.
  ai_name TEXT, -- AI implementation name.
  script_name TEXT -- Script implementation name.
);
CREATE TABLE spell (
  id INTEGER PRIMARY KEY, -- Spell ID.
  name TEXT, -- Spell name.
  rank TEXT, -- Rank or name subtext.
  description TEXT, -- Description with supported tokens resolved.
  description_raw TEXT, -- Original description; NULL if unchanged.
  aura_text TEXT, -- Aura description with supported tokens resolved.
  aura_text_raw TEXT, -- Original aura description; NULL if unchanged.
  school TEXT, -- Decoded spell schools separated by |.
  power_type TEXT, -- Decoded power type when a power cost exists.
  mana_cost INTEGER, -- Base power cost.
  mana_cost_pct INTEGER, -- Power cost as a percentage.
  mana_per_second INTEGER, -- Power consumed per second.
  cast_time INTEGER, -- Base cast time in milliseconds.
  range_min REAL, -- Minimum range in yards.
  range_max REAL, -- Maximum range in yards.
  range_name TEXT, -- Range display name.
  cooldown INTEGER, -- Recovery time in milliseconds.
  category_cooldown INTEGER, -- Category recovery time in milliseconds.
  category INTEGER, -- Spell category ID.
  gcd INTEGER, -- Global cooldown in milliseconds.
  duration INTEGER, -- Duration in milliseconds; -1 means permanent.
  spell_level INTEGER, -- Spell level.
  base_level INTEGER, -- Base spell level.
  max_level INTEGER, -- Maximum scaling level.
  max_targets INTEGER, -- Maximum target count.
  proc_chance INTEGER, -- Proc chance percentage.
  proc_charges INTEGER, -- Proc charge count.
  max_stack INTEGER, -- Maximum aura stacks.
  icon TEXT, -- Spell icon texture path.
  attributes TEXT, -- Decoded attribute flags separated by |.
  mechanic TEXT, -- Decoded mechanic name.
  dispel TEXT, -- Decoded dispel type.
  damage_class TEXT, -- Decoded damage class.
  family TEXT, -- Decoded spell family.
  family_flags TEXT, -- Three hexadecimal spell family masks separated by commas.
  effect1 TEXT, -- Rendered first spell effect.
  effect2 TEXT, -- Rendered second spell effect.
  effect3 TEXT, -- Rendered third spell effect.
  reagents TEXT, -- Reagent item ID and count pairs separated by commas.
  totems TEXT, -- Totem item IDs separated by commas.
  equipped_item_class INTEGER, -- Required equipped item class; NULL for -1.
  speed REAL -- Missile speed.
);
CREATE TABLE itemset (
  id INTEGER PRIMARY KEY, -- Item set ID.
  name TEXT, -- Item set name.
  items TEXT, -- Item IDs and names in slot order, separated by commas.
  bonuses TEXT, -- Threshold, spell ID and name separated by semicolons.
  required_skill INTEGER, -- Required skill ID.
  required_skill_rank INTEGER -- Required skill rank.
);
CREATE TABLE quest (
  id INTEGER PRIMARY KEY, -- Quest ID.
  name TEXT, -- Quest log title.
  level INTEGER, -- Quest level.
  min_level INTEGER, -- Minimum character level.
  max_level INTEGER, -- Maximum character level.
  zone_id INTEGER, -- Area ID when the quest sort ID is positive.
  zone TEXT, -- Resolved zone name.
  sort TEXT, -- Decoded quest sort for a negative quest sort ID.
  type TEXT, -- Decoded quest type.
  suggested_players INTEGER, -- Suggested group size.
  flags TEXT, -- Decoded quest flags separated by |.
  special_flags INTEGER, -- Quest special flags mask.
  objectives TEXT, -- Quest log description.
  details TEXT, -- Quest description.
  area_description TEXT, -- Area description text.
  completion_log TEXT, -- Completion log text.
  objective_texts TEXT, -- Objective texts separated by |.
  allowed_races TEXT, -- Allowed race names separated by |; NULL for all.
  allowed_classes TEXT, -- Allowed class names separated by |; NULL for all.
  time_allowed INTEGER, -- Time allowed in seconds.
  prev_quest INTEGER, -- Previous quest ID; negative requires it to be active.
  next_quest INTEGER, -- Next quest ID.
  exclusive_group INTEGER, -- Exclusive quest group ID.
  breadcrumb_for INTEGER, -- Quest ID this breadcrumb points to.
  reward_next_quest INTEGER, -- Next quest offered on reward.
  start_item INTEGER, -- Starting item ID.
  source_spell INTEGER, -- Source spell ID.
  required_skill INTEGER, -- Required skill ID.
  required_skill_points INTEGER, -- Required skill points.
  required_min_rep_faction INTEGER, -- Minimum reputation faction ID.
  required_min_rep_value INTEGER, -- Minimum reputation value.
  required_max_rep_faction INTEGER, -- Maximum reputation faction ID.
  required_max_rep_value INTEGER, -- Maximum reputation value.
  required_player_kills INTEGER, -- Required player kills.
  reward_money INTEGER, -- Reward money in copper.
  reward_bonus_money INTEGER, -- Bonus reward money in copper.
  reward_xp_difficulty INTEGER, -- Reward XP difficulty index.
  reward_spell INTEGER, -- Reward spell ID.
  reward_display_spell INTEGER, -- Displayed reward spell ID.
  reward_honor INTEGER, -- Reward honor points.
  reward_title INTEGER, -- Reward title ID.
  reward_talents INTEGER, -- Reward talent points.
  reward_arena_points INTEGER -- Reward arena points.
);
CREATE TABLE gameobject (
  id INTEGER PRIMARY KEY, -- Gameobject entry ID.
  name TEXT, -- Gameobject name.
  type TEXT, -- Decoded gameobject type; DOOR is retained.
  display_id INTEGER, -- Display ID.
  size REAL, -- Model scale; NULL for one.
  loot_id INTEGER, -- Loot entry for chests and fishing holes.
  data TEXT, -- Nonzero data fields as index=value pairs separated by spaces.
  cast_bar_caption TEXT, -- Cast bar caption.
  ai_name TEXT, -- AI implementation name.
  script_name TEXT -- Script implementation name.
);
CREATE TABLE map (
  id INTEGER PRIMARY KEY, -- Map ID.
  name TEXT, -- Map name.
  directory TEXT, -- Client map directory.
  type TEXT, -- Decoded map type; COMMON is retained.
  expansion INTEGER, -- Expansion number.
  max_players INTEGER, -- Maximum players.
  area_id INTEGER, -- Associated area ID.
  description_alliance TEXT, -- Alliance map description.
  description_horde TEXT -- Horde map description.
);
CREATE TABLE area (
  id INTEGER PRIMARY KEY, -- Area ID.
  name TEXT, -- Area name.
  map_id INTEGER, -- Continent map ID.
  map TEXT, -- Resolved map name.
  parent_id INTEGER, -- Parent area ID.
  parent TEXT, -- Resolved parent area name.
  level INTEGER, -- Exploration level.
  flags TEXT, -- Decoded area flags separated by |.
  area_bit INTEGER, -- Area exploration bit.
  team TEXT -- Decoded faction team.
);
CREATE TABLE faction (
  id INTEGER PRIMARY KEY, -- Faction ID.
  name TEXT, -- Faction name.
  description TEXT, -- Faction description.
  reputation_index INTEGER, -- Reputation index; NULL for -1.
  parent_id INTEGER, -- Parent faction ID.
  parent TEXT -- Resolved parent faction name.
);
CREATE TABLE talent (
  id INTEGER PRIMARY KEY, -- Talent ID.
  name TEXT, -- Name of the first rank spell.
  tab_id INTEGER, -- Talent tab ID.
  tab TEXT, -- Talent tab name.
  class TEXT, -- Decoded class names separated by |.
  tier INTEGER, -- Zero-based talent tier.
  column_index INTEGER, -- Zero-based talent column.
  max_rank INTEGER, -- Number of nonzero spell ranks.
  ranks TEXT, -- Rank spell IDs separated by commas.
  prereq_talent INTEGER, -- Prerequisite talent ID.
  prereq_rank INTEGER, -- One-based prerequisite rank.
  required_spell INTEGER -- Required spell ID.
);
CREATE TABLE skill (
  id INTEGER PRIMARY KEY, -- Skill line ID.
  name TEXT, -- Skill display name.
  category TEXT, -- Decoded skill category.
  description TEXT, -- Skill description.
  icon TEXT, -- Skill icon texture path.
  can_link INTEGER -- Whether the skill can be linked.
);
CREATE TABLE glyph (
  id INTEGER PRIMARY KEY, -- Glyph properties ID.
  name TEXT, -- Glyph spell name.
  spell_id INTEGER, -- Glyph spell ID.
  type TEXT, -- Glyph type: MINOR or MAJOR.
  icon TEXT -- Glyph icon texture path.
);
CREATE TABLE achievement (
  id INTEGER PRIMARY KEY, -- Achievement ID.
  name TEXT, -- Achievement title.
  description TEXT, -- Achievement description.
  category_id INTEGER, -- Achievement category ID.
  category TEXT, -- Achievement category name.
  points INTEGER, -- Achievement points.
  faction TEXT, -- HORDE or ALLIANCE; NULL for either.
  map_id INTEGER, -- Instance map ID; NULL for -1.
  flags TEXT, -- Decoded achievement flags separated by |.
  reward TEXT, -- Achievement reward text.
  icon TEXT, -- Achievement icon texture path.
  previous INTEGER, -- Previous achievement ID.
  min_criteria INTEGER -- Minimum criteria count.
);
CREATE TABLE offsets (
  name TEXT PRIMARY KEY, -- Fully qualified catalog name.
  domain TEXT, -- First namespace under WoWOffsets.
  platform TEXT, -- Platform subfolder name.
  va TEXT, -- Preferred-image virtual address as eight hexadecimal digits.
  rva TEXT, -- Relative virtual address as eight hexadecimal digits.
  status TEXT, -- Verification status derived from the catalog comment.
  comment TEXT -- Original catalog comment including caveats.
);
CREATE TABLE lua_function (
  name TEXT PRIMARY KEY, -- Function name.
  signature TEXT, -- Function signature.
  description TEXT, -- Function description.
  parameters TEXT, -- Formatted parameters separated by |.
  returns TEXT, -- Formatted return values separated by |.
  category TEXT, -- Function category.
  subcategory TEXT, -- Function subcategory.
  tags TEXT, -- Tags separated by commas.
  protected INTEGER, -- 1 when protected; NULL otherwise.
  hw_event INTEGER, -- 1 when a hardware event is required; NULL otherwise.
  address TEXT, -- Documented memory address.
  related TEXT, -- Related names separated by commas.
  patch TEXT, -- Patch information.
  url TEXT, -- Documentation URL.
  examples TEXT, -- Titled examples separated by blank lines.
  book_page TEXT -- Source book page.
);
CREATE TABLE lua_event (
  name TEXT PRIMARY KEY, -- Event name.
  description TEXT, -- Event description.
  parameters TEXT, -- Formatted event payload separated by |.
  category TEXT, -- Event category.
  related TEXT, -- Related names separated by commas.
  patch TEXT, -- Patch information.
  url TEXT, -- Documentation URL.
  book_page TEXT -- Source book page.
);
CREATE TABLE widget (
  name TEXT PRIMARY KEY, -- Widget type name.
  description TEXT, -- Widget description.
  category TEXT, -- Widget category.
  inherits TEXT, -- Parent types separated by commas.
  methods TEXT, -- Method signatures separated by |.
  inherited_methods TEXT, -- Inherited method signatures separated by |.
  script_handlers TEXT, -- Script handler signatures separated by |.
  book_page TEXT -- Source book page.
);
CREATE TABLE cvar (
  name TEXT PRIMARY KEY, -- CVar name.
  default_value TEXT, -- Default value as text; literal 0 is retained.
  description TEXT, -- CVar description.
  category TEXT -- CVar category.
);
CREATE TABLE lua_type (
  name TEXT PRIMARY KEY, -- Lua type name.
  description TEXT, -- Type description.
  enum_values TEXT, -- Value and description pairs separated by |.
  related TEXT, -- Related names separated by commas.
  book_page TEXT -- Source book page.
);
CREATE TABLE combatlog_event (
  name TEXT PRIMARY KEY, -- Combat log event or prefix name.
  type TEXT, -- Event type, or prefix for prefix records.
  prefixes TEXT, -- Applicable prefixes separated by commas.
  description TEXT, -- Event description.
  arguments TEXT, -- Formatted arguments separated by |.
  book_page TEXT -- Source book page.
);
CREATE TABLE secure_template (
  name TEXT PRIMARY KEY, -- Secure template name.
  description TEXT, -- Template description.
  attributes TEXT, -- Attribute names, allowed values and descriptions separated by |.
  examples TEXT -- Template examples.
);
CREATE TABLE meta (
  name TEXT PRIMARY KEY, -- Provenance key.
  value TEXT -- Provenance value.
);
CREATE TABLE item_spell (
  item_id INTEGER, -- Item ID.
  slot INTEGER, -- Source item spell slot.
  spell_id INTEGER, -- Spell ID.
  trigger TEXT, -- Decoded item spell trigger.
  charges INTEGER, -- Spell charges.
  cooldown INTEGER -- Cooldown in milliseconds; NULL for -1.
);
CREATE TABLE creature_spell (
  creature_id INTEGER, -- Creature entry ID.
  slot INTEGER, -- Source spell slot.
  spell_id INTEGER -- Spell ID.
);
CREATE TABLE creature_spawn (
  guid INTEGER PRIMARY KEY, -- Spawn GUID.
  creature_id INTEGER, -- Creature entry ID.
  map INTEGER, -- Map ID.
  zone INTEGER, -- Zone area ID.
  area INTEGER, -- Area ID.
  x REAL, -- World X coordinate.
  y REAL, -- World Y coordinate.
  z REAL, -- World Z coordinate.
  orientation REAL, -- Orientation in radians.
  spawn_time INTEGER -- Respawn time in seconds.
);
CREATE TABLE gameobject_spawn (
  guid INTEGER PRIMARY KEY, -- Spawn GUID.
  gameobject_id INTEGER, -- Gameobject entry ID.
  map INTEGER, -- Map ID.
  zone INTEGER, -- Zone area ID.
  area INTEGER, -- Area ID.
  x REAL, -- World X coordinate.
  y REAL, -- World Y coordinate.
  z REAL, -- World Z coordinate.
  orientation REAL, -- Orientation in radians.
  spawn_time INTEGER -- Respawn time in seconds.
);
CREATE TABLE loot (
  source TEXT, -- Loot source: creature, gameobject, item, reference or disenchant.
  entry INTEGER, -- Source loot entry ID.
  item INTEGER, -- Item ID; NULL for reference loot.
  reference INTEGER, -- Reference loot entry ID.
  chance REAL, -- Drop chance percentage.
  quest_required INTEGER, -- Whether a quest is required.
  group_id INTEGER, -- Loot group ID.
  min_count INTEGER, -- Minimum item count.
  max_count INTEGER -- Maximum item count.
);
CREATE TABLE vendor (
  creature_id INTEGER, -- Vendor creature entry ID.
  slot INTEGER, -- Vendor slot.
  item_id INTEGER, -- Sold item ID.
  max_count INTEGER, -- Maximum stock count.
  restock_time INTEGER, -- Restock time in seconds.
  extended_cost INTEGER, -- Extended cost ID.
  cost TEXT -- Honor, arena, rating and item costs separated by commas.
);
CREATE TABLE trainer (
  id INTEGER PRIMARY KEY, -- Trainer ID.
  type TEXT, -- Trainer type: CLASS, MOUNT, TRADESKILL or PET.
  requirement INTEGER, -- Trainer requirement ID.
  greeting TEXT -- Trainer greeting text.
);
CREATE TABLE trainer_spell (
  trainer_id INTEGER, -- Trainer ID.
  spell_id INTEGER, -- Taught spell ID.
  cost INTEGER, -- Training cost in copper.
  req_skill INTEGER, -- Required skill line ID.
  req_skill_rank INTEGER, -- Required skill rank.
  req_level INTEGER, -- Required character level.
  req_abilities TEXT -- Required ability spell IDs separated by commas.
);
CREATE TABLE creature_trainer (
  creature_id INTEGER, -- Creature entry ID.
  trainer_id INTEGER -- Trainer ID.
);
CREATE TABLE quest_relation (
  quest_id INTEGER, -- Quest ID.
  role TEXT, -- Relation role: start or end.
  source_kind TEXT, -- Source kind: creature or gameobject.
  source_id INTEGER -- Source entry ID.
);
CREATE TABLE quest_item (
  quest_id INTEGER, -- Quest ID.
  role TEXT, -- Item role: reward, choice, required or source.
  item_id INTEGER, -- Item ID.
  count INTEGER -- Item count.
);
CREATE TABLE quest_objective (
  quest_id INTEGER, -- Quest ID.
  idx INTEGER, -- Objective index.
  target_kind TEXT, -- Target kind: creature or gameobject.
  target_id INTEGER, -- Target entry ID.
  count INTEGER -- Required target count.
);
CREATE TABLE quest_reward_faction (
  quest_id INTEGER, -- Quest ID.
  idx INTEGER, -- Reward faction index.
  faction_id INTEGER, -- Reward faction ID.
  value_id INTEGER, -- Reputation reward value ID.
  override INTEGER -- Override reputation value.
);
CREATE TABLE spell_rank (
  first_spell_id INTEGER, -- First spell ID in the rank chain.
  spell_id INTEGER, -- Spell ID.
  rank INTEGER -- One-based rank.
);
CREATE TABLE spell_effect (
  spell_id INTEGER, -- Spell ID.
  idx INTEGER, -- One-based effect index.
  effect TEXT, -- Decoded effect type.
  aura TEXT, -- Decoded aura type.
  value TEXT, -- Rendered effect value or range.
  base_points INTEGER, -- Effect base points.
  die_sides INTEGER, -- Effect die sides.
  period INTEGER, -- Effect period in milliseconds.
  radius REAL, -- Effect radius in yards.
  chain_targets INTEGER, -- Number of chain targets.
  trigger_spell INTEGER, -- Triggered spell ID.
  misc_value INTEGER, -- Effect-specific value; zero can be a valid enum (for example mana or strength).
  misc_value_b INTEGER, -- Second effect-specific value; zero is preserved.
  item_type INTEGER, -- Effect item ID.
  target_a TEXT, -- Decoded first implicit target.
  target_b TEXT, -- Decoded second implicit target.
  mechanic TEXT -- Decoded effect mechanic.
);
CREATE TABLE spell_icon (
  id INTEGER PRIMARY KEY, -- Spell icon ID.
  texture TEXT -- Texture path.
);
CREATE TABLE item_display (
  id INTEGER PRIMARY KEY, -- Item display ID.
  icon TEXT, -- First inventory icon.
  model TEXT, -- First model name.
  model_texture TEXT -- First model texture.
);
CREATE TABLE itemset_item (
  itemset_id INTEGER, -- Item set ID.
  slot INTEGER, -- Item set slot.
  item_id INTEGER -- Item ID.
);
CREATE TABLE itemset_bonus (
  itemset_id INTEGER, -- Item set ID.
  threshold INTEGER, -- Number of pieces needed.
  spell_id INTEGER -- Bonus spell ID.
);
CREATE TABLE talent_tab (
  id INTEGER PRIMARY KEY, -- Talent tab ID.
  name TEXT, -- Talent tab name.
  class TEXT, -- Decoded class names separated by |.
  order_index INTEGER, -- Talent tab display order.
  icon TEXT -- Talent tab icon texture.
);
CREATE TABLE talent_spell (
  talent_id INTEGER, -- Talent ID.
  rank INTEGER, -- One-based talent rank.
  spell_id INTEGER -- Rank spell ID.
);
CREATE TABLE skill_ability (
  id INTEGER PRIMARY KEY, -- Skill line ability ID.
  skill_id INTEGER, -- Skill line ID.
  spell_id INTEGER, -- Ability spell ID.
  min_skill_rank INTEGER, -- Minimum skill rank.
  superseded_by INTEGER, -- Superseding spell ID.
  acquire_method INTEGER, -- Ability acquisition method.
  trivial_high INTEGER, -- High trivial skill threshold.
  trivial_low INTEGER, -- Low trivial skill threshold.
  classes TEXT, -- Allowed class names separated by |.
  races TEXT -- Allowed race names separated by |.
);
CREATE TABLE achievement_criteria (
  id INTEGER PRIMARY KEY, -- Criteria ID.
  achievement_id INTEGER, -- Achievement ID.
  type TEXT, -- Decoded achievement criteria type.
  asset_id INTEGER, -- Criteria-specific asset ID.
  quantity INTEGER, -- Required quantity.
  description TEXT, -- Criteria description.
  flags INTEGER, -- Criteria flags mask.
  timer_time INTEGER, -- Criteria time limit in seconds.
  ui_order INTEGER -- Criteria display order.
);
";

pub(crate) const SECONDARY_INDEXES: &str = r"
CREATE INDEX creature_spawn_creature_id ON creature_spawn(creature_id);
CREATE INDEX gameobject_spawn_gameobject_id ON gameobject_spawn(gameobject_id);
CREATE INDEX loot_source_entry ON loot(source, entry);
CREATE INDEX loot_item ON loot(item);
CREATE INDEX loot_reference ON loot(reference);
CREATE INDEX vendor_item_id ON vendor(item_id);
CREATE INDEX vendor_creature_id ON vendor(creature_id);
CREATE INDEX quest_relation_quest_id ON quest_relation(quest_id);
CREATE INDEX quest_relation_source ON quest_relation(source_kind, source_id);
CREATE INDEX quest_item_item_id ON quest_item(item_id);
CREATE INDEX quest_item_quest_id ON quest_item(quest_id);
CREATE INDEX quest_objective_quest_id ON quest_objective(quest_id);
CREATE INDEX quest_reward_faction_quest_id ON quest_reward_faction(quest_id);
CREATE INDEX item_start_quest ON item(start_quest);
CREATE INDEX item_itemset ON item(itemset);
CREATE INDEX item_disenchant_id ON item(disenchant_id);
CREATE INDEX creature_loot_id ON creature(loot_id);
CREATE INDEX creature_faction_id ON creature(faction_id);
CREATE INDEX gameobject_loot_id ON gameobject(loot_id);
CREATE INDEX item_spell_spell_id ON item_spell(spell_id);
CREATE INDEX item_spell_item_id ON item_spell(item_id);
CREATE INDEX spell_effect_spell_id ON spell_effect(spell_id);
CREATE INDEX spell_effect_trigger_spell ON spell_effect(trigger_spell);
CREATE INDEX spell_effect_item_type ON spell_effect(item_type);
CREATE INDEX spell_effect_effect_misc ON spell_effect(effect, misc_value);
CREATE INDEX spell_rank_first_spell_id ON spell_rank(first_spell_id);
CREATE INDEX spell_rank_spell_id ON spell_rank(spell_id);
CREATE INDEX trainer_spell_spell_id ON trainer_spell(spell_id);
CREATE INDEX trainer_spell_trainer_id ON trainer_spell(trainer_id);
CREATE INDEX creature_trainer_creature_id ON creature_trainer(creature_id);
CREATE INDEX creature_trainer_trainer_id ON creature_trainer(trainer_id);
CREATE INDEX creature_spell_creature_id ON creature_spell(creature_id);
CREATE INDEX creature_spell_spell_id ON creature_spell(spell_id);
CREATE INDEX talent_spell_spell_id ON talent_spell(spell_id);
CREATE INDEX glyph_spell_id ON glyph(spell_id);
CREATE INDEX area_parent_id ON area(parent_id);
CREATE INDEX achievement_criteria_achievement_id ON achievement_criteria(achievement_id);
CREATE INDEX itemset_item_itemset_id ON itemset_item(itemset_id);
CREATE INDEX itemset_bonus_itemset_id ON itemset_bonus(itemset_id);
CREATE INDEX quest_prev_quest ON quest(prev_quest);
CREATE INDEX quest_exclusive_group ON quest(exclusive_group);
CREATE INDEX quest_breadcrumb_for ON quest(breadcrumb_for);
CREATE INDEX quest_reward_spell ON quest(reward_spell);
CREATE INDEX offsets_domain ON offsets(domain);
CREATE INDEX skill_ability_skill_id ON skill_ability(skill_id);
CREATE INDEX skill_ability_spell_id ON skill_ability(spell_id);
";
