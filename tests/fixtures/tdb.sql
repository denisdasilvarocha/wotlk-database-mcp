-- MySQL dump fixture for TrinityCore TDB 335.26091.
/*!40101 SET NAMES utf8mb4 */;

-- Table structure for table `version`
DROP TABLE IF EXISTS `version`;
CREATE TABLE `version` (
  `core_version` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT 'Core revision dumped at startup.',
  `core_revision` varchar(120) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `db_version` varchar(120) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT 'Version of world DB.',
  `cache_id` int DEFAULT '0'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `version` WRITE;
INSERT INTO `version` VALUES ('TrinityCore fixture','fixture','TDB 335.26091',26091);
UNLOCK TABLES;

-- Table structure for table `item_template`
DROP TABLE IF EXISTS `item_template`;
CREATE TABLE `item_template` (
  `entry` int unsigned NOT NULL DEFAULT '0',
  `class` tinyint unsigned NOT NULL DEFAULT '0',
  `subclass` tinyint unsigned NOT NULL DEFAULT '0',
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `displayid` int unsigned NOT NULL DEFAULT '0',
  `Quality` tinyint unsigned NOT NULL DEFAULT '0',
  `Flags` int unsigned NOT NULL DEFAULT '0',
  `FlagsExtra` int unsigned NOT NULL DEFAULT '0',
  `BuyCount` tinyint unsigned NOT NULL DEFAULT '1',
  `BuyPrice` bigint NOT NULL DEFAULT '0',
  `SellPrice` int unsigned NOT NULL DEFAULT '0',
  `InventoryType` tinyint unsigned NOT NULL DEFAULT '0',
  `AllowableClass` int NOT NULL DEFAULT '-1',
  `AllowableRace` int NOT NULL DEFAULT '-1',
  `ItemLevel` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredLevel` tinyint unsigned NOT NULL DEFAULT '0',
  `RequiredSkill` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredSkillRank` smallint unsigned NOT NULL DEFAULT '0',
  `requiredspell` int unsigned NOT NULL DEFAULT '0',
  `RequiredReputationFaction` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredReputationRank` smallint unsigned NOT NULL DEFAULT '0',
  `maxcount` int NOT NULL DEFAULT '0',
  `stackable` int DEFAULT '1',
  `ContainerSlots` tinyint unsigned NOT NULL DEFAULT '0',
  `stat_type1` tinyint unsigned NOT NULL DEFAULT '0',
  `stat_value1` smallint NOT NULL DEFAULT '0',
  `stat_type2` tinyint unsigned NOT NULL DEFAULT '0',
  `stat_value2` smallint NOT NULL DEFAULT '0',
  `stat_type3` tinyint unsigned NOT NULL DEFAULT '0',
  `stat_value3` smallint NOT NULL DEFAULT '0',
  `stat_type4` tinyint unsigned NOT NULL DEFAULT '0',
  `stat_value4` smallint NOT NULL DEFAULT '0',
  `stat_type5` tinyint unsigned NOT NULL DEFAULT '0',
  `stat_value5` smallint NOT NULL DEFAULT '0',
  `stat_type6` tinyint unsigned NOT NULL DEFAULT '0',
  `stat_value6` smallint NOT NULL DEFAULT '0',
  `stat_type7` tinyint unsigned NOT NULL DEFAULT '0',
  `stat_value7` smallint NOT NULL DEFAULT '0',
  `stat_type8` tinyint unsigned NOT NULL DEFAULT '0',
  `stat_value8` smallint NOT NULL DEFAULT '0',
  `stat_type9` tinyint unsigned NOT NULL DEFAULT '0',
  `stat_value9` smallint NOT NULL DEFAULT '0',
  `stat_type10` tinyint unsigned NOT NULL DEFAULT '0',
  `stat_value10` smallint NOT NULL DEFAULT '0',
  `dmg_min1` float NOT NULL DEFAULT '0',
  `dmg_max1` float NOT NULL DEFAULT '0',
  `dmg_type1` tinyint unsigned NOT NULL DEFAULT '0',
  `dmg_min2` float NOT NULL DEFAULT '0',
  `dmg_max2` float NOT NULL DEFAULT '0',
  `dmg_type2` tinyint unsigned NOT NULL DEFAULT '0',
  `armor` smallint unsigned NOT NULL DEFAULT '0',
  `holy_res` tinyint unsigned NOT NULL DEFAULT '0',
  `fire_res` tinyint unsigned NOT NULL DEFAULT '0',
  `nature_res` tinyint unsigned NOT NULL DEFAULT '0',
  `frost_res` tinyint unsigned NOT NULL DEFAULT '0',
  `shadow_res` tinyint unsigned NOT NULL DEFAULT '0',
  `arcane_res` tinyint unsigned NOT NULL DEFAULT '0',
  `delay` smallint unsigned NOT NULL DEFAULT '1000',
  `spellid_1` int NOT NULL DEFAULT '0',
  `spelltrigger_1` tinyint unsigned NOT NULL DEFAULT '0',
  `spellcharges_1` smallint NOT NULL DEFAULT '0',
  `spellcooldown_1` int NOT NULL DEFAULT '-1',
  `spellid_2` int NOT NULL DEFAULT '0',
  `spelltrigger_2` tinyint unsigned NOT NULL DEFAULT '0',
  `spellcharges_2` smallint NOT NULL DEFAULT '0',
  `spellcooldown_2` int NOT NULL DEFAULT '-1',
  `spellid_3` int NOT NULL DEFAULT '0',
  `spelltrigger_3` tinyint unsigned NOT NULL DEFAULT '0',
  `spellcharges_3` smallint NOT NULL DEFAULT '0',
  `spellcooldown_3` int NOT NULL DEFAULT '-1',
  `spellid_4` int NOT NULL DEFAULT '0',
  `spelltrigger_4` tinyint unsigned NOT NULL DEFAULT '0',
  `spellcharges_4` smallint NOT NULL DEFAULT '0',
  `spellcooldown_4` int NOT NULL DEFAULT '-1',
  `spellid_5` int NOT NULL DEFAULT '0',
  `spelltrigger_5` tinyint unsigned NOT NULL DEFAULT '0',
  `spellcharges_5` smallint NOT NULL DEFAULT '0',
  `spellcooldown_5` int NOT NULL DEFAULT '-1',
  `bonding` tinyint unsigned NOT NULL DEFAULT '0',
  `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `PageText` int unsigned NOT NULL DEFAULT '0',
  `startquest` int unsigned NOT NULL DEFAULT '0',
  `lockid` int unsigned NOT NULL DEFAULT '0',
  `RandomProperty` int NOT NULL DEFAULT '0',
  `RandomSuffix` int unsigned NOT NULL DEFAULT '0',
  `block` int unsigned NOT NULL DEFAULT '0',
  `itemset` int unsigned NOT NULL DEFAULT '0',
  `MaxDurability` smallint unsigned NOT NULL DEFAULT '0',
  `area` int unsigned NOT NULL DEFAULT '0',
  `Map` smallint NOT NULL DEFAULT '0',
  `BagFamily` int NOT NULL DEFAULT '0',
  `TotemCategory` int NOT NULL DEFAULT '0',
  `socketColor_1` tinyint NOT NULL DEFAULT '0',
  `socketColor_2` tinyint NOT NULL DEFAULT '0',
  `socketColor_3` tinyint NOT NULL DEFAULT '0',
  `socketBonus` int NOT NULL DEFAULT '0',
  `GemProperties` int NOT NULL DEFAULT '0',
  `RequiredDisenchantSkill` smallint NOT NULL DEFAULT '-1',
  `duration` int unsigned NOT NULL DEFAULT '0',
  `ItemLimitCategory` smallint NOT NULL DEFAULT '0',
  `DisenchantID` int unsigned NOT NULL DEFAULT '0'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `item_template` WRITE;
INSERT INTO `item_template` VALUES (19019,2,7,'Thunderfury, Blessed Blade of the Windseeker',30606,5,2,0,1,0,0,13,-1,-1,80,60,0,0,0,0,0,0,1,0,7,16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,44,82,0,0,0,0,0,0,0,0,0,0,0,1900,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,1,'',0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,1),(19020,0,0,'Tyrael\'s Hilt',0,0,0,0,1,0,0,0,-1,-1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,'',0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0),(19021,0,0,'Back\\slash Blade',0,0,0,0,1,0,0,0,-1,-1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,NULL,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0),(19022,0,0,'O''Reilly''s Ring',0,0,0,0,1,0,0,0,-1,-1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,'',0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0),(4000,0,0,'Recipe: Fiery Brew',0,0,0,0,1,0,0,0,-1,-1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,483,0,0,-1,4001,6,0,-1,-1,0,0,-1,0,0,0,-1,0,0,0,-1,0,'',0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0),(4002,0,0,'Fiery Brew Potion',0,0,0,0,1,0,0,0,-1,-1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,'',0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0),(4004,0,0,'Fiery Cowl',0,0,0,0,1,0,0,0,-1,-1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,'',0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,-1,0,0,0),(4005,0,0,'Fiery Robe',0,0,0,0,1,0,0,0,-1,-1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,'',0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,-1,0,0,0),(4007,0,0,'Mysterious Letter',0,0,0,0,1,0,0,0,-1,-1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,'',0,1001,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0),(4008,0,0,'Bag of Goodies',0,0,0,0,1,0,0,0,-1,-1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,'',0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0),(20725,0,0,'Nexus Crystal',0,0,0,0,1,0,0,0,-1,-1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,'',0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0),(42739,0,0,'Glyph of Fireball',0,0,0,0,1,0,0,0,-1,-1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,56390,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,'',0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0);
UNLOCK TABLES;

-- Table structure for table `creature_template`
DROP TABLE IF EXISTS `creature_template`;
CREATE TABLE `creature_template` (
  `entry` int unsigned NOT NULL DEFAULT '0',
  `modelid1` int unsigned NOT NULL DEFAULT '0',
  `modelid2` int unsigned NOT NULL DEFAULT '0',
  `modelid3` int unsigned NOT NULL DEFAULT '0',
  `modelid4` int unsigned NOT NULL DEFAULT '0',
  `name` char(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '0',
  `subname` char(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `minlevel` tinyint unsigned NOT NULL DEFAULT '1',
  `maxlevel` tinyint unsigned NOT NULL DEFAULT '1',
  `exp` smallint NOT NULL DEFAULT '0',
  `faction` smallint unsigned NOT NULL DEFAULT '0',
  `npcflag` int unsigned NOT NULL DEFAULT '0',
  `speed_walk` float NOT NULL DEFAULT '1' COMMENT 'Result of 2.5/2.5, most common value',
  `speed_run` float NOT NULL DEFAULT '1.14286' COMMENT 'Result of 8.0/7.0, most common value',
  `scale` float NOT NULL DEFAULT '1',
  `rank` tinyint unsigned NOT NULL DEFAULT '0',
  `unit_class` tinyint unsigned NOT NULL DEFAULT '0',
  `family` tinyint NOT NULL DEFAULT '0',
  `type` tinyint unsigned NOT NULL DEFAULT '0',
  `type_flags` int unsigned NOT NULL DEFAULT '0',
  `lootid` int unsigned NOT NULL DEFAULT '0',
  `pickpocketloot` int unsigned NOT NULL DEFAULT '0',
  `skinloot` int unsigned NOT NULL DEFAULT '0',
  `VehicleId` int unsigned NOT NULL DEFAULT '0',
  `mingold` int unsigned NOT NULL DEFAULT '0',
  `maxgold` int unsigned NOT NULL DEFAULT '0',
  `AIName` char(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `HealthModifier` float NOT NULL DEFAULT '1',
  `ManaModifier` float NOT NULL DEFAULT '1',
  `ArmorModifier` float NOT NULL DEFAULT '1',
  `DamageModifier` float NOT NULL DEFAULT '1',
  `mechanic_immune_mask` int unsigned NOT NULL DEFAULT '0',
  `ScriptName` char(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT ''
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `creature_template` WRITE;
INSERT INTO `creature_template` VALUES (12056,0,0,0,0,'Baron Geddon','',0,0,0,14,0,1,1.14286,1,3,0,0,0,0,12056,0,0,0,0,0,'',1,1,1,1,0,''),
(5000,0,0,0,0,'Brew Vendor','Potion Vendor',0,0,0,11,128,1,1.14286,1,0,0,0,0,0,0,0,0,0,0,0,'',1,1,1,1,0,''),
(5001,0,0,0,0,'Fire Trainer','Mage Trainer',0,0,0,11,48,1,1.14286,1,0,0,0,0,0,0,0,0,0,0,0,'',1,1,1,1,0,''),
(5002,0,0,0,0,'Marshal Dughan','',0,0,0,11,2,1,1.14286,1,0,0,0,0,0,0,0,0,0,0,0,'',1,1,1,1,0,'');
UNLOCK TABLES;

-- Table structure for table `creature_template_spell`
DROP TABLE IF EXISTS `creature_template_spell`;
CREATE TABLE `creature_template_spell` (
  `CreatureID` int unsigned NOT NULL,
  `Index` tinyint unsigned NOT NULL DEFAULT '0',
  `Spell` int unsigned NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `creature_template_spell` WRITE;
INSERT INTO `creature_template_spell` VALUES (12056,0,133);
UNLOCK TABLES;

-- Table structure for table `creature`
DROP TABLE IF EXISTS `creature`;
CREATE TABLE `creature` (
  `guid` int unsigned NOT NULL AUTO_INCREMENT COMMENT 'Global Unique Identifier',
  `id` int unsigned NOT NULL DEFAULT '0' COMMENT 'Creature Identifier',
  `map` smallint unsigned NOT NULL DEFAULT '0' COMMENT 'Map Identifier',
  `zoneId` smallint unsigned NOT NULL DEFAULT '0' COMMENT 'Zone Identifier',
  `areaId` smallint unsigned NOT NULL DEFAULT '0' COMMENT 'Area Identifier',
  `position_x` float NOT NULL DEFAULT '0',
  `position_y` float NOT NULL DEFAULT '0',
  `position_z` float NOT NULL DEFAULT '0',
  `orientation` float NOT NULL DEFAULT '0',
  `spawntimesecs` int unsigned NOT NULL DEFAULT '120'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `creature` WRITE;
INSERT INTO `creature` VALUES (1,12056,409,2717,2717,759.5,-1040.25,-133.5,2,604800),
(2,5002,0,12,87,0,0,0,0,0);
UNLOCK TABLES;

-- Table structure for table `gameobject_template`
DROP TABLE IF EXISTS `gameobject_template`;
CREATE TABLE `gameobject_template` (
  `entry` int unsigned NOT NULL DEFAULT '0',
  `type` tinyint unsigned NOT NULL DEFAULT '0',
  `displayId` int unsigned NOT NULL DEFAULT '0',
  `name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `castBarCaption` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `size` float NOT NULL DEFAULT '1',
  `Data0` int NOT NULL DEFAULT '0',
  `Data1` int NOT NULL DEFAULT '0',
  `Data2` int NOT NULL DEFAULT '0',
  `Data3` int NOT NULL DEFAULT '0',
  `Data4` int NOT NULL DEFAULT '0',
  `Data5` int NOT NULL DEFAULT '0',
  `Data6` int NOT NULL DEFAULT '0',
  `Data7` int NOT NULL DEFAULT '0',
  `Data8` int NOT NULL DEFAULT '0',
  `Data9` int NOT NULL DEFAULT '0',
  `Data10` int NOT NULL DEFAULT '0',
  `Data11` int NOT NULL DEFAULT '0',
  `Data12` int NOT NULL DEFAULT '0',
  `Data13` int NOT NULL DEFAULT '0',
  `Data14` int NOT NULL DEFAULT '0',
  `Data15` int NOT NULL DEFAULT '0',
  `Data16` int NOT NULL DEFAULT '0',
  `Data17` int NOT NULL DEFAULT '0',
  `Data18` int NOT NULL DEFAULT '0',
  `Data19` int NOT NULL DEFAULT '0',
  `Data20` int NOT NULL DEFAULT '0',
  `Data21` int NOT NULL DEFAULT '0',
  `Data22` int NOT NULL DEFAULT '0',
  `Data23` int NOT NULL DEFAULT '0',
  `AIName` char(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `ScriptName` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT ''
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `gameobject_template` WRITE;
INSERT INTO `gameobject_template` VALUES (2843,3,0,'Battered Chest','',1,0,2843,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'','');
UNLOCK TABLES;

-- Table structure for table `gameobject`
DROP TABLE IF EXISTS `gameobject`;
CREATE TABLE `gameobject` (
  `guid` int unsigned NOT NULL AUTO_INCREMENT COMMENT 'Global Unique Identifier',
  `id` int unsigned NOT NULL DEFAULT '0' COMMENT 'Gameobject Identifier',
  `map` smallint unsigned NOT NULL DEFAULT '0' COMMENT 'Map Identifier',
  `zoneId` smallint unsigned NOT NULL DEFAULT '0' COMMENT 'Zone Identifier',
  `areaId` smallint unsigned NOT NULL DEFAULT '0' COMMENT 'Area Identifier',
  `position_x` float NOT NULL DEFAULT '0',
  `position_y` float NOT NULL DEFAULT '0',
  `position_z` float NOT NULL DEFAULT '0',
  `orientation` float NOT NULL DEFAULT '0',
  `spawntimesecs` int NOT NULL DEFAULT '0'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `gameobject` WRITE;
INSERT INTO `gameobject` VALUES (1,2843,0,12,87,0,0,0,0,0);
UNLOCK TABLES;

-- Table structure for table `quest_template`
DROP TABLE IF EXISTS `quest_template`;
CREATE TABLE `quest_template` (
  `ID` int unsigned NOT NULL DEFAULT '0',
  `QuestLevel` smallint NOT NULL DEFAULT '1',
  `MinLevel` tinyint unsigned NOT NULL DEFAULT '0',
  `QuestSortID` smallint NOT NULL DEFAULT '0',
  `QuestInfoID` smallint unsigned NOT NULL DEFAULT '0',
  `SuggestedGroupNum` tinyint unsigned NOT NULL DEFAULT '0',
  `RewardNextQuest` int unsigned NOT NULL DEFAULT '0',
  `RewardXPDifficulty` tinyint unsigned NOT NULL DEFAULT '0',
  `RewardMoney` int NOT NULL DEFAULT '0',
  `RewardBonusMoney` int unsigned NOT NULL DEFAULT '0',
  `RewardDisplaySpell` int unsigned NOT NULL DEFAULT '0',
  `RewardSpell` int NOT NULL DEFAULT '0',
  `RewardHonor` int NOT NULL DEFAULT '0',
  `StartItem` int unsigned NOT NULL DEFAULT '0',
  `Flags` int unsigned NOT NULL DEFAULT '0',
  `RequiredPlayerKills` tinyint unsigned NOT NULL DEFAULT '0',
  `RewardItem1` int unsigned NOT NULL DEFAULT '0',
  `RewardAmount1` smallint unsigned NOT NULL DEFAULT '0',
  `RewardItem2` int unsigned NOT NULL DEFAULT '0',
  `RewardAmount2` smallint unsigned NOT NULL DEFAULT '0',
  `RewardItem3` int unsigned NOT NULL DEFAULT '0',
  `RewardAmount3` smallint unsigned NOT NULL DEFAULT '0',
  `RewardItem4` int unsigned NOT NULL DEFAULT '0',
  `RewardAmount4` smallint unsigned NOT NULL DEFAULT '0',
  `ItemDrop1` int unsigned NOT NULL DEFAULT '0',
  `ItemDropQuantity1` smallint unsigned NOT NULL DEFAULT '0',
  `ItemDrop2` int unsigned NOT NULL DEFAULT '0',
  `ItemDropQuantity2` smallint unsigned NOT NULL DEFAULT '0',
  `ItemDrop3` int unsigned NOT NULL DEFAULT '0',
  `ItemDropQuantity3` smallint unsigned NOT NULL DEFAULT '0',
  `ItemDrop4` int unsigned NOT NULL DEFAULT '0',
  `ItemDropQuantity4` smallint unsigned NOT NULL DEFAULT '0',
  `RewardChoiceItemID1` int unsigned NOT NULL DEFAULT '0',
  `RewardChoiceItemQuantity1` smallint unsigned NOT NULL DEFAULT '0',
  `RewardChoiceItemID2` int unsigned NOT NULL DEFAULT '0',
  `RewardChoiceItemQuantity2` smallint unsigned NOT NULL DEFAULT '0',
  `RewardChoiceItemID3` int unsigned NOT NULL DEFAULT '0',
  `RewardChoiceItemQuantity3` smallint unsigned NOT NULL DEFAULT '0',
  `RewardChoiceItemID4` int unsigned NOT NULL DEFAULT '0',
  `RewardChoiceItemQuantity4` smallint unsigned NOT NULL DEFAULT '0',
  `RewardChoiceItemID5` int unsigned NOT NULL DEFAULT '0',
  `RewardChoiceItemQuantity5` smallint unsigned NOT NULL DEFAULT '0',
  `RewardChoiceItemID6` int unsigned NOT NULL DEFAULT '0',
  `RewardChoiceItemQuantity6` smallint unsigned NOT NULL DEFAULT '0',
  `RewardTitle` tinyint unsigned NOT NULL DEFAULT '0',
  `RewardTalents` tinyint unsigned NOT NULL DEFAULT '0',
  `RewardArenaPoints` smallint unsigned NOT NULL DEFAULT '0',
  `RewardFactionID1` smallint unsigned NOT NULL DEFAULT '0' COMMENT 'faction id from Faction.dbc in this case',
  `RewardFactionValue1` int NOT NULL DEFAULT '0',
  `RewardFactionOverride1` int NOT NULL DEFAULT '0',
  `RewardFactionID2` smallint unsigned NOT NULL DEFAULT '0' COMMENT 'faction id from Faction.dbc in this case',
  `RewardFactionValue2` int NOT NULL DEFAULT '0',
  `RewardFactionOverride2` int NOT NULL DEFAULT '0',
  `RewardFactionID3` smallint unsigned NOT NULL DEFAULT '0' COMMENT 'faction id from Faction.dbc in this case',
  `RewardFactionValue3` int NOT NULL DEFAULT '0',
  `RewardFactionOverride3` int NOT NULL DEFAULT '0',
  `RewardFactionID4` smallint unsigned NOT NULL DEFAULT '0' COMMENT 'faction id from Faction.dbc in this case',
  `RewardFactionValue4` int NOT NULL DEFAULT '0',
  `RewardFactionOverride4` int NOT NULL DEFAULT '0',
  `RewardFactionID5` smallint unsigned NOT NULL DEFAULT '0' COMMENT 'faction id from Faction.dbc in this case',
  `RewardFactionValue5` int NOT NULL DEFAULT '0',
  `RewardFactionOverride5` int NOT NULL DEFAULT '0',
  `TimeAllowed` int unsigned NOT NULL DEFAULT '0',
  `AllowableRaces` int unsigned NOT NULL DEFAULT '0',
  `LogTitle` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  `LogDescription` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  `QuestDescription` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  `AreaDescription` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  `QuestCompletionLog` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  `RequiredNpcOrGo1` int NOT NULL DEFAULT '0',
  `RequiredNpcOrGo2` int NOT NULL DEFAULT '0',
  `RequiredNpcOrGo3` int NOT NULL DEFAULT '0',
  `RequiredNpcOrGo4` int NOT NULL DEFAULT '0',
  `RequiredNpcOrGoCount1` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredNpcOrGoCount2` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredNpcOrGoCount3` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredNpcOrGoCount4` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredItemId1` int unsigned NOT NULL DEFAULT '0',
  `RequiredItemId2` int unsigned NOT NULL DEFAULT '0',
  `RequiredItemId3` int unsigned NOT NULL DEFAULT '0',
  `RequiredItemId4` int unsigned NOT NULL DEFAULT '0',
  `RequiredItemId5` int unsigned NOT NULL DEFAULT '0',
  `RequiredItemId6` int unsigned NOT NULL DEFAULT '0',
  `RequiredItemCount1` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredItemCount2` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredItemCount3` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredItemCount4` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredItemCount5` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredItemCount6` smallint unsigned NOT NULL DEFAULT '0',
  `ObjectiveText1` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  `ObjectiveText2` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  `ObjectiveText3` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  `ObjectiveText4` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `quest_template` WRITE;
INSERT INTO `quest_template` VALUES (1001,10,8,12,0,0,0,0,1500,0,0,4001,0,0,0,0,4002,2,0,0,0,0,0,0,4007,1,0,0,0,0,0,0,4004,1,0,0,0,0,0,0,0,0,0,0,0,0,0,72,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'The Fiery Path','Slay Baron Geddon.','','','',12056,-2843,0,0,1,1,0,0,4002,0,0,0,0,0,3,0,0,0,0,0,'','','',''),
(1002,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'The Fiery Path Continues','','','','',0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'','','',''),
(1003,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'Another Fiery Path','','','','',0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'','','',''),
(1004,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'Active Fiery Path','','','','',0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'','','','');
UNLOCK TABLES;

-- Table structure for table `quest_template_addon`
DROP TABLE IF EXISTS `quest_template_addon`;
CREATE TABLE `quest_template_addon` (
  `ID` int unsigned NOT NULL DEFAULT '0',
  `MaxLevel` tinyint unsigned NOT NULL DEFAULT '0',
  `AllowableClasses` int unsigned NOT NULL DEFAULT '0',
  `SourceSpellID` int unsigned NOT NULL DEFAULT '0',
  `PrevQuestID` int NOT NULL DEFAULT '0',
  `NextQuestID` int unsigned NOT NULL DEFAULT '0',
  `ExclusiveGroup` int NOT NULL DEFAULT '0',
  `BreadcrumbForQuestId` int NOT NULL DEFAULT '0',
  `RequiredSkillID` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredSkillPoints` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredMinRepFaction` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredMaxRepFaction` smallint unsigned NOT NULL DEFAULT '0',
  `RequiredMinRepValue` int NOT NULL DEFAULT '0',
  `RequiredMaxRepValue` int NOT NULL DEFAULT '0',
  `SpecialFlags` tinyint unsigned NOT NULL DEFAULT '0'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `quest_template_addon` WRITE;
INSERT INTO `quest_template_addon` VALUES (1001,0,0,0,0,1002,-1001,0,0,0,0,0,0,0,0),
(1002,0,0,0,1001,0,1002,0,0,0,0,0,0,0,0),
(1003,0,0,0,0,0,1002,0,0,0,0,0,0,0,0),
(1004,0,0,0,-1001,0,-1001,0,0,0,0,0,0,0,0);
UNLOCK TABLES;

-- Table structure for table `creature_queststarter`
DROP TABLE IF EXISTS `creature_queststarter`;
CREATE TABLE `creature_queststarter` (
  `id` int unsigned NOT NULL DEFAULT '0' COMMENT 'Identifier',
  `quest` int unsigned NOT NULL DEFAULT '0' COMMENT 'Quest Identifier'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `creature_queststarter` WRITE;
INSERT INTO `creature_queststarter` VALUES (5002,1001);
UNLOCK TABLES;

-- Table structure for table `creature_questender`
DROP TABLE IF EXISTS `creature_questender`;
CREATE TABLE `creature_questender` (
  `id` int unsigned NOT NULL DEFAULT '0' COMMENT 'Identifier',
  `quest` int unsigned NOT NULL DEFAULT '0' COMMENT 'Quest Identifier'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `creature_questender` WRITE;
INSERT INTO `creature_questender` VALUES (5002,1001);
UNLOCK TABLES;

-- Table structure for table `gameobject_queststarter`
DROP TABLE IF EXISTS `gameobject_queststarter`;
CREATE TABLE `gameobject_queststarter` (
  `id` int unsigned NOT NULL DEFAULT '0',
  `quest` int unsigned NOT NULL DEFAULT '0' COMMENT 'Quest Identifier'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `gameobject_queststarter` WRITE;
INSERT INTO `gameobject_queststarter` VALUES (2843,1002);
UNLOCK TABLES;

-- Table structure for table `gameobject_questender`
DROP TABLE IF EXISTS `gameobject_questender`;
CREATE TABLE `gameobject_questender` (
  `id` int unsigned NOT NULL DEFAULT '0',
  `quest` int unsigned NOT NULL DEFAULT '0' COMMENT 'Quest Identifier'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `gameobject_questender` WRITE;
INSERT INTO `gameobject_questender` VALUES (2843,1002);
UNLOCK TABLES;

-- Table structure for table `creature_loot_template`
DROP TABLE IF EXISTS `creature_loot_template`;
CREATE TABLE `creature_loot_template` (
  `Entry` int unsigned NOT NULL DEFAULT '0',
  `Item` int unsigned NOT NULL DEFAULT '0',
  `Reference` int unsigned NOT NULL DEFAULT '0',
  `Chance` float NOT NULL DEFAULT '100',
  `QuestRequired` tinyint(1) NOT NULL DEFAULT '0',
  `LootMode` smallint unsigned NOT NULL DEFAULT '1',
  `GroupId` tinyint unsigned NOT NULL DEFAULT '0',
  `MinCount` tinyint unsigned NOT NULL DEFAULT '1',
  `MaxCount` tinyint unsigned NOT NULL DEFAULT '1'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `creature_loot_template` WRITE;
INSERT INTO `creature_loot_template` VALUES (12056,19019,0,4.5,0,1,0,1,1),
(12056,0,10000,100,0,1,0,1,1);
UNLOCK TABLES;

-- Table structure for table `gameobject_loot_template`
DROP TABLE IF EXISTS `gameobject_loot_template`;
CREATE TABLE `gameobject_loot_template` (
  `Entry` int unsigned NOT NULL DEFAULT '0',
  `Item` int unsigned NOT NULL DEFAULT '0',
  `Reference` int unsigned NOT NULL DEFAULT '0',
  `Chance` float NOT NULL DEFAULT '100',
  `QuestRequired` tinyint(1) NOT NULL DEFAULT '0',
  `LootMode` smallint unsigned NOT NULL DEFAULT '1',
  `GroupId` tinyint unsigned NOT NULL DEFAULT '0',
  `MinCount` tinyint unsigned NOT NULL DEFAULT '1',
  `MaxCount` tinyint unsigned NOT NULL DEFAULT '1'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `gameobject_loot_template` WRITE;
INSERT INTO `gameobject_loot_template` VALUES (2843,4002,0,50,0,1,0,1,3);
UNLOCK TABLES;

-- Table structure for table `item_loot_template`
DROP TABLE IF EXISTS `item_loot_template`;
CREATE TABLE `item_loot_template` (
  `Entry` int unsigned NOT NULL DEFAULT '0',
  `Item` int unsigned NOT NULL DEFAULT '0',
  `Reference` int unsigned NOT NULL DEFAULT '0',
  `Chance` float NOT NULL DEFAULT '100',
  `QuestRequired` tinyint(1) NOT NULL DEFAULT '0',
  `LootMode` smallint unsigned NOT NULL DEFAULT '1',
  `GroupId` tinyint unsigned NOT NULL DEFAULT '0',
  `MinCount` tinyint unsigned NOT NULL DEFAULT '1',
  `MaxCount` tinyint unsigned NOT NULL DEFAULT '1'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `item_loot_template` WRITE;
INSERT INTO `item_loot_template` VALUES (4008,4002,0,100,0,1,0,1,1);
UNLOCK TABLES;

-- Table structure for table `reference_loot_template`
DROP TABLE IF EXISTS `reference_loot_template`;
CREATE TABLE `reference_loot_template` (
  `Entry` int unsigned NOT NULL DEFAULT '0',
  `Item` int unsigned NOT NULL DEFAULT '0',
  `Reference` int unsigned NOT NULL DEFAULT '0',
  `Chance` float NOT NULL DEFAULT '100',
  `QuestRequired` tinyint(1) NOT NULL DEFAULT '0',
  `LootMode` smallint unsigned NOT NULL DEFAULT '1',
  `GroupId` tinyint unsigned NOT NULL DEFAULT '0',
  `MinCount` tinyint unsigned NOT NULL DEFAULT '1',
  `MaxCount` tinyint unsigned NOT NULL DEFAULT '1'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `reference_loot_template` WRITE;
INSERT INTO `reference_loot_template` VALUES (10000,19020,0,10,0,1,0,1,1);
UNLOCK TABLES;

-- Table structure for table `disenchant_loot_template`
DROP TABLE IF EXISTS `disenchant_loot_template`;
CREATE TABLE `disenchant_loot_template` (
  `Entry` int unsigned NOT NULL DEFAULT '0',
  `Item` int unsigned NOT NULL DEFAULT '0',
  `Reference` int unsigned NOT NULL DEFAULT '0',
  `Chance` float NOT NULL DEFAULT '100',
  `QuestRequired` tinyint(1) NOT NULL DEFAULT '0',
  `LootMode` smallint unsigned NOT NULL DEFAULT '1',
  `GroupId` tinyint unsigned NOT NULL DEFAULT '0',
  `MinCount` tinyint unsigned NOT NULL DEFAULT '1',
  `MaxCount` tinyint unsigned NOT NULL DEFAULT '1'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `disenchant_loot_template` WRITE;
INSERT INTO `disenchant_loot_template` VALUES (1,20725,0,100,0,1,0,1,1);
UNLOCK TABLES;

-- Table structure for table `npc_vendor`
DROP TABLE IF EXISTS `npc_vendor`;
CREATE TABLE `npc_vendor` (
  `entry` int unsigned NOT NULL DEFAULT '0',
  `slot` smallint NOT NULL DEFAULT '0',
  `item` int NOT NULL DEFAULT '0',
  `maxcount` tinyint unsigned NOT NULL DEFAULT '0',
  `incrtime` int unsigned NOT NULL DEFAULT '0',
  `ExtendedCost` int unsigned NOT NULL DEFAULT '0'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `npc_vendor` WRITE;
INSERT INTO `npc_vendor` VALUES (5002,0,-5001,0,0,0),
(5001,0,-5000,0,0,0),
(5000,0,4002,0,0,0),
(5000,1,19020,1,3600,1);
UNLOCK TABLES;

-- Table structure for table `trainer`
DROP TABLE IF EXISTS `trainer`;
CREATE TABLE `trainer` (
  `Id` int unsigned NOT NULL DEFAULT '0',
  `Type` tinyint unsigned NOT NULL DEFAULT '2',
  `Requirement` int unsigned NOT NULL DEFAULT '0',
  `Greeting` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `trainer` WRITE;
INSERT INTO `trainer` VALUES (1,0,8,'Greetings');
UNLOCK TABLES;

-- Table structure for table `trainer_spell`
DROP TABLE IF EXISTS `trainer_spell`;
CREATE TABLE `trainer_spell` (
  `TrainerId` int unsigned NOT NULL DEFAULT '0',
  `SpellId` int unsigned NOT NULL DEFAULT '0',
  `MoneyCost` int unsigned NOT NULL DEFAULT '0',
  `ReqSkillLine` int unsigned NOT NULL DEFAULT '0',
  `ReqSkillRank` int unsigned NOT NULL DEFAULT '0',
  `ReqAbility1` int unsigned NOT NULL DEFAULT '0',
  `ReqAbility2` int unsigned NOT NULL DEFAULT '0',
  `ReqAbility3` int unsigned NOT NULL DEFAULT '0',
  `ReqLevel` tinyint unsigned NOT NULL DEFAULT '0'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `trainer_spell` WRITE;
INSERT INTO `trainer_spell` VALUES (1,133,100,0,0,0,0,0,1),
(1,4001,500,0,0,0,0,0,10);
UNLOCK TABLES;

-- Table structure for table `creature_default_trainer`
DROP TABLE IF EXISTS `creature_default_trainer`;
CREATE TABLE `creature_default_trainer` (
  `CreatureId` int unsigned NOT NULL,
  `TrainerId` int unsigned NOT NULL DEFAULT '0'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `creature_default_trainer` WRITE;
INSERT INTO `creature_default_trainer` VALUES (5001,1);
UNLOCK TABLES;

-- Table structure for table `spell_ranks`
DROP TABLE IF EXISTS `spell_ranks`;
CREATE TABLE `spell_ranks` (
  `first_spell_id` int unsigned NOT NULL DEFAULT '0',
  `spell_id` int unsigned NOT NULL DEFAULT '0',
  `rank` tinyint unsigned NOT NULL DEFAULT '0'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
LOCK TABLES `spell_ranks` WRITE;
INSERT INTO `spell_ranks` VALUES (133,133,1),
(133,143,2);
UNLOCK TABLES;

-- Server-only spells are deliberately not imported.
DROP TABLE IF EXISTS `spell_dbc`;
CREATE TABLE `spell_dbc` (
  `Id` int NOT NULL,
  `SpellName` text
);
LOCK TABLES `spell_dbc` WRITE;
INSERT INTO `spell_dbc` VALUES (90000,'Server Only Spell');
UNLOCK TABLES;

-- Additional search-subtext fixture: leveled creatures with no spawn (priority 1).
INSERT INTO `creature_template` VALUES (5003,0,0,0,0,'Obsidia','Broodmother',60,63,0,11,0,1,1.14286,1,3,0,0,0,0,0,0,0,0,0,0,'',1,1,1,1,0,''),
(5004,0,0,0,0,'Emberclaw','',63,63,0,11,0,1,1.14286,1,3,0,0,0,0,0,0,0,0,0,0,'',1,1,1,1,0,'');

-- Reference loot chain: item 19022 sits in reference 10001, which reference 10002
-- includes; creature 12056 uses both reference tables.
INSERT INTO `reference_loot_template` VALUES (10001,19022,0,25,0,1,0,1,1),(10002,0,10001,50,0,1,0,1,1);
INSERT INTO `creature_loot_template` VALUES (12056,0,10002,100,0,1,0,1,1);

-- A trainer spawn, so taught_by rows can report a zone.
INSERT INTO `creature` VALUES (3,5001,0,12,87,0,0,0,0,0);
-- A vendor spawn without a zone id, as in most TDB 335 spawns, so rows fall back to the map name.
INSERT INTO `creature` VALUES (4,5000,0,0,0,0,0,0,0,0);

-- Third quest of the Fiery Path chain: 1001 -> 1002 -> 1005.
INSERT INTO `quest_template` VALUES (1005,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'Into the Fire','','','','',0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'','','','');
INSERT INTO `quest_template_addon` VALUES (1005,0,0,0,1002,0,0,0,0,0,0,0,0,0,0);
