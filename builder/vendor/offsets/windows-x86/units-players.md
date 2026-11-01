# Units and players

Read for unit position/facing, auras, reactions, player inventory and quests, interactions, or click-to-move (`ClickToMove`, `CTMFace`, `CTMInteractGO`). Movement mechanics are in [movement and world](movement-world.md).

Catalog provenance: [Windows x86, build 12340](windows-x86.md). Use the shared [address conventions and verification guidance](address-conventions.md) when applying these constants.

## Contents

- [Units](#units)
- [Players](#players)

## Units

```cpp
namespace WoWOffsets {
    namespace Unit {
        namespace CGUnit_C {
            constexpr uint32_t virt00 = 0x00737BA0; // virt00
            constexpr uint32_t CGUnit_C = 0x0073F660; // constructor
            constexpr uint32_t Initialize = 0x00742220; // initialize unit
            constexpr uint32_t Shutdown = 0x00742BB0; // shutdown unit
            constexpr uint32_t PostInit = 0x0073FCC0; // post initialization
            constexpr uint32_t PostShutdown = 0x00724EA0; // post shutdown
            constexpr uint32_t Disable = 0x00734FD0; // disable unit
            constexpr uint32_t Reenable = 0x007237F0; // re-enable unit
            constexpr uint32_t GetPosition = 0x006E6F10; // get unit position
            constexpr uint32_t GetFacing = 0x006E6F40; // get unit facing angle; world, transport-adjusted; vtable slot 13; the client's own GetPlayerFacing uses this
            constexpr uint32_t UpdateFacing = 0x006E6F60; // update facing; reads the raw movement facing, vtable slot 14
            constexpr uint32_t GetWorldMatrix = 0x0071FD80; // get world transform matrix
            constexpr uint32_t GetUnitName = 0x0072A000; // get unit name string
            constexpr uint32_t DisplayInfoNeedsUpdate = 0x0072A480; // check display update
            constexpr uint32_t UnitReaction = 0x007251C0; // get reaction to target
            constexpr uint32_t CanAttack = 0x00729A70; // check if can attack target
            constexpr uint32_t CanAssist = 0x007293D0; // check if can assist target
            constexpr uint32_t IsFriendlyTowards = 0x0051A230; // **Uncertain**: 30-byte no-arg predicate [unit+0x9C0]>0 && !([unit+0xA30]&0x10000000); a friendly-towards test needs a target + faction logic the body lacks
            constexpr uint32_t GetThreatEntry = 0x00734790; // get threat table entry
            constexpr uint32_t AffectedByAura = 0x007283A0; // check aura effect
            constexpr uint32_t GetParryingItem = 0x00754EF0; // get parry item
            constexpr uint32_t OnRightClick = 0x00731260; // handle right click
            constexpr uint32_t OnMovementPacket = 0x00740D30; // handle movement packet
            constexpr uint32_t SendMovementPacket = 0x007413F0; // send movement packet
            constexpr uint32_t WriteMovementPacketWithTransport = 0x0071F0C0; // write transport movement
            constexpr uint32_t SendMovementPackets_954_958 = 0x00721B90; // send movement updates; note the name says "954_958" while the code uses 950h/954h
            constexpr uint32_t OnKnockBackPacket = 0x0072D1B0; // handle knockback
            constexpr uint32_t OnMoveTeleportAck = 0x0072D2D0; // teleport acknowledgement
            constexpr uint32_t OnSplineSpeedChange = 0x00740A60; // spline speed change
            constexpr uint32_t SendMoveTimeSkipped = 0x00717D90; // send time skipped
            constexpr uint32_t IsActiveMover = 0x004D43C0; // check if active mover
            constexpr uint32_t ToggleMovementFlag2_0x40 = 0x0074B9B0; // toggle movement flag
            constexpr uint32_t GetAuraCount = 0x004F8850; // get aura count
            constexpr uint32_t GetAuraSpellId = 0x004F8870; // get aura spell ID
            constexpr uint32_t HasAuraBySpellId = 0x007282A0; // check aura by spell ID
            constexpr uint32_t GetAura = 0x00556E10; // get aura info
            constexpr uint32_t GetAuraCaster = 0x00723490; // get aura caster GUID
            constexpr uint32_t GetAuraFlags = 0x00565510; // get aura flags
            constexpr uint32_t GetCreatureType = 0x0071F300; // get creature type
            constexpr uint32_t GetCreatureRank = 0x00718A00; // get creature rank
            constexpr uint32_t GetCreatureModelDataRec = 0x00717A20; // get model data
            constexpr uint32_t GetShapeshiftFormId = 0x0071AF70; // get shapeshift form
            constexpr uint32_t GetBasePowerForType = 0x0071B770; // get base power
            constexpr uint32_t GetPowerModifier = 0x0071B960; // get power modifier
            constexpr uint32_t GetHealthRegenRateFromSpirit = 0x0071BA60; // get health regen rate
            constexpr uint32_t GetStatsField_34 = 0x0071B8D0; // get stats field
            constexpr uint32_t GetTitleString = 0x00719950; // get title string
            constexpr uint32_t GetControllingPlayer = 0x00718B70; // get controlling player
            constexpr uint32_t GetModel = 0x00717B20; // get unit model
            constexpr uint32_t IsInFrontOfTargetWithGuid = 0x0071BCE0; // check if in front
            constexpr uint32_t IsPartyMember = 0x0052C8C0; // check party membership
            constexpr uint32_t IsRaidMember = 0x00573200; // check raid membership
            constexpr uint32_t IsInMyParty = 0x00718D70; // in my party
            constexpr uint32_t IsInMyPartyOrRaid = 0x00718CA0; // in my party or raid
            constexpr uint32_t SetCurrentSpellDestLocation = 0x00715400; // set destination
            constexpr uint32_t SetCurrentSpellAndCastId = 0x00715A80; // set spell and cast ID
            constexpr uint32_t OnMoveStop = 0x006F0EB0; // stop moving
            constexpr uint32_t HasMovementFlags2_0x1 = 0x0074B9A0; // check flag; flag bit is named in the identifier
            constexpr uint32_t TrySendMSG_MOVE_FALL_LAND = 0x0073D4A0; // send fall land
            constexpr uint32_t OnUnitMoveEventNoActive = 0x007235C0; // unit move event
        }
    }
} // namespace WoWOffsets
```

## Players

```cpp
namespace WoWOffsets {
    namespace Unit {
        namespace CGPlayer_C {
            constexpr uint32_t CanInteract = 0x00729530; // check interaction
            constexpr uint32_t IsInPartyOrRaid = 0x0071C570; // player in party/raid
            constexpr uint32_t CGPlayer_C_0 = 0x006E6B40; // constructor
            constexpr uint32_t Initialize = 0x006E45D0; // initialize player
            constexpr uint32_t Shutdown = 0x006E4780; // shutdown player
            constexpr uint32_t PostInit = 0x006E8280; // post initialization
            constexpr uint32_t PostReenable = 0x006E1D70; // post re-enable
            constexpr uint32_t Disable = 0x006E6020; // disable player
            constexpr uint32_t Reenable = 0x006E6150; // re-enable player
            constexpr uint32_t destructor = 0x006E78D0; // destructor
            constexpr uint32_t GetCoinage = 0x0051A2B0; // get gold amount
            constexpr uint32_t ClickToMove = 0x00727400; // click-to-move
            constexpr uint32_t IsClickMoving = 0x00721F90; // is click moving
            constexpr uint32_t SwapItems = 0x006DF890; // swap inventory items
            constexpr uint32_t AutoEquipCursorItem = 0x006DFC40; // auto-equip cursor item
            constexpr uint32_t AutoEquipItem = 0x006DFEE0; // auto-equip item
            constexpr uint32_t ClearPendingEquip = 0x006E01A0; // clear pending equip
            constexpr uint32_t FindSlotIndex = 0x006D29F0; // find inventory slot
            constexpr uint32_t ReadItemResult = 0x006D5D90; // read item result
            constexpr uint32_t GetVisibleItemEntryId = 0x006DE330; // get visible item entry
            constexpr uint32_t GetSkillValue = 0x0051A250; // get skill value
            constexpr uint32_t GetSkillMaxRank = 0x00594710; // get skill max rank
            constexpr uint32_t GetSkillIndexById = 0x006DC1C0; // get skill by ID
            constexpr uint32_t AcceptQuest = 0x006D4CA0; // accept quest
            constexpr uint32_t GiveQuestItems = 0x006D4DD0; // give quest items
            constexpr uint32_t CompleteQuest = 0x006D4D40; // complete quest
            constexpr uint32_t QuestLogRemoveQuest = 0x006D4F80; // remove quest from log
            constexpr uint32_t OnQuestGiverSendQuest = 0x006D0460; // quest giver send
            constexpr uint32_t OnQuestGiverChooseReward = 0x006D0AB0; // choose reward
            constexpr uint32_t OnQuestGiverListQuests = 0x006D0240; // list quests
            constexpr uint32_t OnQuestGiverRequestItems = 0x006D08F0; // request items
            constexpr uint32_t OnQuestGiverStatus = 0x006D11C0; // quest giver status
            constexpr uint32_t SendTextEmote = 0x006DD9E0; // send text emote
            constexpr uint32_t PlayEmote = 0x0073AD70; // play emote animation
            constexpr uint32_t AcceptGuild = 0x006D4760; // accept guild invite
            constexpr uint32_t DeclineGuild = 0x006D47D0; // decline guild invite
            constexpr uint32_t AcceptGroup = 0x006D4450; // accept group invite
            constexpr uint32_t DeclineGroup = 0x006D44D0; // decline group invite
            constexpr uint32_t LeaveGroup = 0x006D4540; // leave group
            constexpr uint32_t GetAFKText = 0x006DC8D0; // get AFK message
            constexpr uint32_t GetDNDText = 0x006DC940; // get DND message
            constexpr uint32_t TalkToBinder = 0x006D2120; // talk to innkeeper
            constexpr uint32_t OnTrainerList = 0x006D12C0; // trainer list
            constexpr uint32_t TrainerBuySpell = 0x00594DA0; // buy spell from trainer
            constexpr uint32_t TalkToBanker = 0x006D2340; // talk to banker
            constexpr uint32_t TalkToAuctioneer = 0x006D2640; // talk to auctioneer
            constexpr uint32_t TalkToBattlemaster = 0x006D25C0; // talk to battlemaster
            constexpr uint32_t TalkToGuildBanker = 0x006D23C0; // talk to guild banker
            constexpr uint32_t TalkToStableMaster = 0x006D26C0; // talk to stable master
            constexpr uint32_t TalkToSpiritHealer = 0x006DB010; // talk to spirit healer
            constexpr uint32_t ShopFromMerchant = 0x006DAF60; // shop from merchant
            constexpr uint32_t CTMAttackUnit = 0x0072EE20; // click-to-move attack
            constexpr uint32_t CTMInteractGO = 0x0072B730; // CTM interact gameobject
            constexpr uint32_t CTMFace = 0x0072B660; // CTM face direction
            constexpr uint32_t CTMClickTerrain = 0x0072B6D0; // CTM click terrain
            constexpr uint32_t OnAttackIconPressed = 0x0072C2B0; // attack icon pressed
            constexpr uint32_t SetCombatMode = 0x006E1660; // set combat mode
            constexpr uint32_t AcceptLevelGrant = 0x00521B00; // accept level grant
            constexpr uint32_t OnTerrainClick = 0x00519640; // **Uncertain**: body is a Map.dbc query (record type 1/2 & flag 0x100); no terrain-click semantics
        }
    }
} // namespace WoWOffsets
```
