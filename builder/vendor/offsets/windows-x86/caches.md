# Record caches and storage

Read for item, creature, NPC, quest, name, guild, game-object, pet, and text caches, cache callbacks, and hash-table allocation. Search `GetInfoBlockById`, `GetInfoBlockByID`, `DbWoWCache`, or `TSHashTable`. DBC records are in [client databases](client-db.md); module lifecycle is in [Warden](warden.md).

Catalog provenance: [Windows x86, build 12340](windows-x86.md). Use the shared [address conventions and verification guidance](address-conventions.md) when applying these constants.

## Contents

- [Record caches](#record-caches)
- [Callbacks and allocation](#callbacks-and-allocation)

## Record caches

```cpp
namespace WoWOffsets {
    namespace Cache {
        namespace DBItemCache {
            constexpr uint32_t GetInfoBlockByID = 0x0067CA30; // get item by ID
            constexpr uint32_t GetItemNameByIndex = 0x004FD200; // get item name by index
        }

        namespace DbItemCache {
            constexpr uint32_t Load = 0x0067CDD0; // load item cache
            constexpr uint32_t Unpack = 0x0098D910; // unpack item cache
        }

        namespace DbCreatureCache {
            constexpr uint32_t GetInfoBlockById = 0x0067B6A0; // get creature by ID
            constexpr uint32_t Load = 0x0067BA40; // load creature cache
            constexpr uint32_t Unpack = 0x0098D4C0; // unpack creature
        }

        namespace CreatureStats {
            constexpr uint32_t GetName = 0x0053B8E0; // get creature name
        }

        namespace DbNpcCache {
            constexpr uint32_t GetInfoBlockById = 0x0067D0D0; // get NPC by ID
            constexpr uint32_t Load = 0x0067D470; // load NPC cache
        }

        namespace DbQuestCache {
            constexpr uint32_t GetInfoBlockById = 0x0067DE90; // get quest by ID
            constexpr uint32_t Load = 0x0067E0E0; // load quest cache
            constexpr uint32_t Unpack = 0x007F70E0; // unpack quest
        }

        namespace QuestCache {
            constexpr uint32_t Constructor = 0x00634E70; // quest cache constructor
        }

        namespace DbNameCache {
            constexpr uint32_t GetInfoBlockById = 0x0067D770; // get name by ID
            constexpr uint32_t Load = 0x00680D70; // load name cache
        }

        namespace DbGuildCache {
            constexpr uint32_t GetInfoBlockById = 0x0067D930; // get guild by ID
            constexpr uint32_t Load_0 = 0x0067DB90; // load guild cache
            constexpr uint32_t Unpack = 0x0075B3E0; // unpack guild
        }

        namespace DbGameObjectCache {
            constexpr uint32_t GetInfoBlockById = 0x0067BD40; // get GO by ID
            constexpr uint32_t Load_0 = 0x0067C0E0; // load GO cache
            constexpr uint32_t Unpack = 0x0098D750; // unpack GO
        }

        namespace GameObjectStats {
            constexpr uint32_t GetName = 0x004FD1E0; // get GO name
        }

        namespace DbPetNameCache {
            constexpr uint32_t GetInfoBlockById = 0x0067EA30; // get pet name by ID
            constexpr uint32_t Load = 0x0067EC80; // load pet name cache
            constexpr uint32_t Invalidate = 0x0067A940; // invalidate pet cache
        }

        namespace DbArenaTeamCache {
            constexpr uint32_t GetInfoBlockById = 0x00680170; // arena team
        }

        namespace DbItemTextCache {
            constexpr uint32_t GetInfoBlockById = 0x0067F4C0; // item text
        }

        namespace DbPetitionCache {
            constexpr uint32_t GetInfoBlockById = 0x0067EF70; // petition
        }

        namespace DbPageTextCache {
            constexpr uint32_t GetInfoBlockById = 0x0067E3E0; // page text
        }

        namespace DbDanceCache {
            constexpr uint32_t GetInfoBlockById = 0x006806D0; // dance
        }

        namespace DbItemNameCache {
            constexpr uint32_t GetInfoBlockById = 0x0067C3E0; // item name
        }

        namespace DbWoWCache {
            constexpr uint32_t Load = 0x0067FE00; // load WoW cache
            constexpr uint32_t GetInfoBlockById = 0x0067FA80; // get info block
            constexpr uint32_t Shutdown_WARDEN_UNLOAD = 0x007DA930; // cache shutdown on unload — **label unverified**: host-interface slot 1, *loads* the cached module (key + `DbWoWCache` lookup → `0x7DA610`)
        }
    }
} // namespace WoWOffsets
```

## Callbacks and allocation

```cpp
namespace WoWOffsets {
    namespace Cache {
        namespace DBCache {
            constexpr uint32_t CancelCallback_0 = 0x0067A3A0; // cancel callback
            constexpr uint32_t CancelCallback = 0x00679F00; // cancel callback (alt)
            constexpr uint32_t Warden = 0x00676AC0; // warden DB cache
            constexpr uint32_t WardenCachedModule_InternalDelete = 0x006791B0; // delete cached module
        }

        namespace TSHashTable {
            constexpr uint32_t DBCache_ItemStats_InternalNew = 0x006707C0; // new item stats (original: TSHashTable_DBCache_ItemStats...InternalNew)
            constexpr uint32_t DBCache_CGPetition_InternalNew = 0x006713E0; // new petition (original: TSHashTable_DBCache_CGPetition...InternalNew)
            constexpr uint32_t DBCache_GameObjectStats_InternalNew = 0x00670420; // new GO stats (original: TSHashTable_DBCache_GameObjectStats...InternalNew)
            constexpr uint32_t DBCache_GameObjectStats_Destroy = 0x00672FF0; // destroy GO (original: TSHashTable_DBCache_GameObjectStats...Destroy)
            constexpr uint32_t DBCache_ItemStats_Destroy = 0x00673550; // destroy item stats (original: TSHashTable_DBCache_ItemStats...Destroy)
            constexpr uint32_t DBCache_CGPetition_Destroy = 0x00674810; // destroy petition (original: TSHashTable_DBCache_CGPetition...Destroy)
            constexpr uint32_t ITEMCOOLDOWNHASHNODE_InternalNew = 0x0080A3A0; // new cooldown entry (original row name TSHashTable_ITEMCOOLDOWNHASHNODE...InternalNew)
            constexpr uint32_t ITEMCOOLDOWNHASHNODE_InternalDelete = 0x0080A400; // delete cooldown entry (original row name TSHashTable_ITEMCOOLDOWNHASHNODE...InternalDelete)
            constexpr uint32_t ITEMCOOLDOWNHASHNODE_Destroy = 0x0080A4D0; // destroy cooldown table (original row name TSHashTable_ITEMCOOLDOWNHASHNODE...Destroy)
        }

        namespace DBCache_CreatureStats {
            constexpr uint32_t InternalNew = 0x00670270; // new creature stats
            constexpr uint32_t Destroy = 0x00672D40; // destroy creature
        }

        namespace DBCache_ItemName {
            constexpr uint32_t InternalNew = 0x00670650; // new item name
            constexpr uint32_t Destroy = 0x006732A0; // destroy item name
        }

        namespace DBCache_NpcText {
            constexpr uint32_t InternalNew = 0x00670970; // new NPC text
            constexpr uint32_t Destroy = 0x00673800; // destroy NPC text
        }
    }
} // namespace WoWOffsets
```
