# Client databases and DBC records

Read for client database loading, row lookup, record helpers, faction data, or `ClientDbPointer` table globals. Spell-record behavior is in [spells](spells.md); cached server records are in [record caches](caches.md).

Catalog provenance: [Windows x86, build 12340](windows-x86.md). Use the shared [address conventions and verification guidance](address-conventions.md) when applying these constants.

## Contents

- [Database access and record helpers](#database-access-and-record-helpers)
- [Table pointers](#table-pointers)

## Database access and record helpers

```cpp
namespace WoWOffsets {
    namespace Cache {
        namespace DbCache {
            constexpr uint32_t LoadAll = 0x00635060; // load all cache entries
            constexpr uint32_t LoadAll_Jmp = 0x00635520; // jump to load all (original: j_DbCache_LoadAll)
            constexpr uint32_t dbcLoadFunction = 0x006348B0; // load function
        }

        namespace ClientDB {
            constexpr uint32_t Free = 0x00636270; // free client DB
        }

        namespace ClientDb {
            constexpr uint32_t RegisterBase = 0x006337D0; // register base DB
            constexpr uint32_t StringLookup = 0x00634910; // string lookup
            constexpr uint32_t GetLocalizedRow = 0x004CFD20; // get localized row
            constexpr uint32_t Unpack = 0x004CFBB0; // unpack DB
            constexpr uint32_t GetRow = 0x0065C290; // get row from client DB (original: GetRow_ClientDB)
        }

        namespace CClientDb {
            constexpr uint32_t CopyRowData = 0x004CFD90; // copy row data
        }

        namespace WowClientDB {
            constexpr uint32_t GetRow = 0x004BB1C0; // get row (alt)
        }

        namespace RecordLookup {
            constexpr uint32_t GetSpellVisualRow = 0x007FA290; // get spell visual
            constexpr uint32_t GetGameTableValue = 0x007F6990; // get game table value
            constexpr uint32_t GetMaxPlayersForMap = 0x00634AB0; // get max players
            constexpr uint32_t GetMapResetTime = 0x00634A80; // get map reset time
            constexpr uint32_t GetMapDifficultyInfoByMapIdAndDifficulty = 0x00634950; // map difficulty
            constexpr uint32_t GetAnimationDataRecField_6 = 0x007176B0; // animation data
            constexpr uint32_t GetPetPersonalityRow = 0x0071F390; // pet personality
            constexpr uint32_t IsAnimationDataId_RifleRelated = 0x0071DAA0; // animation check
            constexpr uint32_t GetWMOAreaRec = 0x00990560; // get WMO area record
            constexpr uint32_t GetLiquidTypeRecForArea = 0x009905C0; // get liquid type
        }

        namespace FactionRec {
            constexpr uint32_t LoadDB = 0x0063F080; // load faction DB
        }

        namespace Faction {
            constexpr uint32_t GetReputationIndexForFaction = 0x00718B30; // get rep index
        }
    }
} // namespace WoWOffsets
```

## Table pointers

```cpp
namespace WoWOffsets {
    namespace Cache {
        namespace ClientDbPointer {
            constexpr uint32_t Achievement = 0x00AD305C; // pointer to the Achievement ClientDB object; record type 0xEB
            constexpr uint32_t Achievement_Criteria = 0x00AD3080; // pointer to the Achievement_Criteria ClientDB object; record type 0xEC
            constexpr uint32_t AnimationData = 0x00AD30C8; // pointer to the AnimationData ClientDB object; record type 0xEE
            constexpr uint32_t AreaTable = 0x00AD3134; // pointer to the AreaTable ClientDB object; record type 0xF1
            constexpr uint32_t AuctionHouse = 0x00AD31C4; // pointer to the AuctionHouse ClientDB object; record type 0xF5
            constexpr uint32_t BattlemasterList = 0x00AD3254; // pointer to the BattlemasterList ClientDB object; record type 0xF9
            constexpr uint32_t ChrClasses = 0x00AD3404; // pointer to the ChrClasses ClientDB object; record type 0x105
            constexpr uint32_t ChrRaces = 0x00AD3428; // pointer to the ChrRaces ClientDB object; record type 0x106
            constexpr uint32_t CreatureDisplayInfo = 0x00AD34B8; // pointer to the CreatureDisplayInfo ClientDB object; record type 0x109
            constexpr uint32_t CreatureModelData = 0x00AD3500; // pointer to the CreatureModelData ClientDB object; record type 0x10C
            constexpr uint32_t CreatureSpellData = 0x00AD356C; // pointer to the CreatureSpellData ClientDB object; record type 0x10F
            constexpr uint32_t Faction = 0x00AD3860; // pointer to the Faction ClientDB object; record type 0x121
            constexpr uint32_t FactionTemplate = 0x00AD3884; // pointer to the FactionTemplate ClientDB object; record type 0x123
            constexpr uint32_t GameObjectDisplayInfo = 0x00AD3938; // pointer to the GameObjectDisplayInfo ClientDB object; record type 0x128
            constexpr uint32_t Item = 0x00AD3D4C; // pointer to the Item ClientDB object; record type 0x145
            constexpr uint32_t ItemDisplayInfo = 0x00AD3DDC; // pointer to the ItemDisplayInfo ClientDB object; record type 0x149
            constexpr uint32_t ItemExtendedCost = 0x00AD3E00; // pointer to the ItemExtendedCost ClientDB object; record type 0x14A
            constexpr uint32_t Map = 0x00AD4160; // pointer to the Map ClientDB object; record type 0x167
            constexpr uint32_t Spell = 0x00AD49D0; // pointer to the Spell ClientDB object; record type 0x194
            constexpr uint32_t SpellCastTimes = 0x00AD4748; // pointer to the SpellCastTimes ClientDB object; record type 0x191
            constexpr uint32_t SpellDuration = 0x00AD4820; // pointer to the SpellDuration ClientDB object; record type 0x198
            constexpr uint32_t SpellItemEnchantment = 0x00AD48B0; // pointer to the SpellItemEnchantment ClientDB object; record type 0x19C
            constexpr uint32_t SpellRange = 0x00AD4988; // pointer to the SpellRange ClientDB object; record type 0x1A2
            constexpr uint32_t Talent = 0x00AD4B5C; // pointer to the Talent ClientDB object; record type 0x1AE
            constexpr uint32_t TalentTab = 0x00AD4B80; // pointer to the TalentTab ClientDB object; record type 0x1AF
        }
    }
} // namespace WoWOffsets
```
