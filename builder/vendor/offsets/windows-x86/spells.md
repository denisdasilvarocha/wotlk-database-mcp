# Spells and spellbook

Read for casting, cooldowns, range, pending casts, spell-record helpers, and spellbook operations. Search `Spell_C`, `Spell`, `SpellRec`, or `CGSpellBook`. DBC access and table pointers are in [client databases](client-db.md); packet handlers are in [networking](network.md#packet-handlers).

Catalog provenance: [Windows x86, build 12340](windows-x86.md). Use the shared [address conventions and verification guidance](address-conventions.md) when applying these constants.

## Spell functions

```cpp
namespace WoWOffsets {
    namespace Spell {
        namespace Spell_C {
            constexpr uint32_t CastSpell = 0x0080DA40; // cast spell by ID
            constexpr uint32_t SystemInitialize = 0x008100E0; // initialize spell system
            constexpr uint32_t GetSpellCooldown = 0x00807980; // get spell cooldown
            constexpr uint32_t GetSpellCooldown_Proxy = 0x00809000; // cooldown proxy; indexes the cooldown pointer table by book type (24-byte stride)
            constexpr uint32_t GetItemCooldown = 0x00809030; // get item cooldown
            constexpr uint32_t GetSpellRange = 0x00802C30; // get spell range
            constexpr uint32_t TargetTradeItem = 0x005198A0; // target trade item
            constexpr uint32_t HandleTerrainClick = 0x0080C340; // handle terrain targeting
            constexpr uint32_t HandleSpriteRay = 0x00803A50; // handle sprite ray
        }

        namespace Spell {
            constexpr uint32_t CreatePendingSpellCast = 0x00805010; // create pending cast
            constexpr uint32_t GetPendingSpellCast = 0x00805180; // get pending cast
            constexpr uint32_t Alloc_PeriodicClientTrigger = 0x0080DF10; // allocate periodic trigger
            constexpr uint32_t SendCancelChannelingOpcode = 0x008008D0; // cancel channeling
            constexpr uint32_t GetSpellIdForDifficulty = 0x00802850; // get difficulty spell ID
            constexpr uint32_t HasApplyAuraEffect = 0x0076BD50; // has apply aura
            constexpr uint32_t IsValidSpell = 0x00540650; // validate spell ID
            constexpr uint32_t HasSpell = 0x0053C5B0; // player knows spell
            constexpr uint32_t GetSpellVisualRow = 0x007FA290; // get spell visual
            constexpr uint32_t GetSpellFailedResult = 0x007FCCA0; // get fail reason
            constexpr uint32_t GetSpellFailedEventString = 0x007FEF10; // get fail string
            constexpr uint32_t BroadcastSpellEvent = 0x0060BF60; // broadcast spell event
            constexpr uint32_t SpellTableInitialize = 0x00812200; // initialize spell table
            constexpr uint32_t GetSpellIdByName = 0x00540200; // get spell ID by name
            constexpr uint32_t GetSpellManaCostByID = 0x008012F0; // get mana cost
        }

        namespace SpellRec {
            constexpr uint32_t GetCastTime = 0x007FF180; // get base cast time; reads the spell record's cast-time index, resolved through SpellCastTimes.dbc
            constexpr uint32_t GetDuration = 0x00800A70; // get spell duration; reads the spell record's duration index, resolved through SpellDuration.dbc (a different index from GetCastTime)
            constexpr uint32_t GetLevel = 0x007FF070; // get required level
            constexpr uint32_t GetClassMask = 0x007FDE80; // get class mask
            constexpr uint32_t GetModifiedStats = 0x007FD970; // get modified stats
            constexpr uint32_t GetModifiedStatValue = 0x007FDB50; // get modified stat value
            constexpr uint32_t IsModifiedStat = 0x00800770; // check if stat modified
            constexpr uint32_t HasEffect = 0x007FDE20; // check spell effect type
            constexpr uint32_t HasAura = 0x007FDE50; // check aura type
            constexpr uint32_t HasTrackingAura = 0x007FDF60; // has tracking aura
            constexpr uint32_t WillSummonCritter = 0x008009B0; // will summon critter
            constexpr uint32_t IsHelpfulOrHarmful = 0x007FE1B0; // check helpful/harmful
            constexpr uint32_t UsableInShapeshift = 0x007FE850; // usable in shapeshift
            constexpr uint32_t NotUsableInShapeshiftForm = 0x007FE890; // not usable in form
            constexpr uint32_t HasShapeshiftFlag_0x1 = 0x00800950; // has shapeshift flag
            constexpr uint32_t RangeHasFlag_0x1 = 0x007FF380; // range flag check
            constexpr uint32_t RangeHasFlag_0x2 = 0x007FF3C0; // range flag check
        }

        namespace CGSpellBook {
            constexpr uint32_t CastSpell = 0x0053BC40; // cast from spellbook
            constexpr uint32_t UpdateSpells = 0x0053CA70; // update spellbook
            constexpr uint32_t PickupSpell = 0x0053BAF0; // pickup spell to cursor
        }
    }
} // namespace WoWOffsets
```
