# Client lifecycle, memory, and globals

Read for startup/shutdown, the idle loop, `SMem` allocation, build/account/player state, target GUIDs, hardware-input timing, or `ObjectManager_Ptr`. DBC table globals are in [client databases](client-db.md#table-pointers).

Catalog provenance: [Windows x86, build 12340](windows-x86.md). Use the shared [address conventions and verification guidance](address-conventions.md) when applying these constants.

## Contents

- [System functions](#system-functions)
- [Globals](#globals)

## System functions

```cpp
namespace WoWOffsets {
    namespace System {
        namespace Client {
            constexpr uint32_t InitializeGame = 0x00405540; // initialize game client
            constexpr uint32_t DestroyGame = 0x00406510; // destroy game client
            constexpr uint32_t Idle = 0x00403340; // main idle loop
            constexpr uint32_t WinMain = 0x00406C70; // entry point
            constexpr uint32_t start = 0x00401000; // program start
        }

        namespace SMem {
            constexpr uint32_t Alloc = 0x0076E540; // memory allocation
            constexpr uint32_t Free = 0x0076E5A0; // memory free
            constexpr uint32_t ReAlloc = 0x0076E5E0; // memory reallocation
        }
    }
} // namespace WoWOffsets
```

## Globals

```cpp
namespace WoWOffsets {
    namespace Globals {
        constexpr uint32_t PlayerName = 0x00C79D18; // current player name
        constexpr uint32_t CurrentAccount = 0x00B6AA40; // current account name
        constexpr uint32_t CurrentRealm = 0x00C79D08; // cached "realmName" CVar handle (string value at [handle+0x28])
        constexpr uint32_t CurrentTargetGUID = 0x00BD07B0; // target GUID
        constexpr uint32_t LastTargetGUID = 0x00BD07B8; // last target GUID
        constexpr uint32_t MouseOverGUID = 0x00BD07A0; // mouseover GUID
        constexpr uint32_t FollowGUID = 0x00CA11F8; // follow GUID
        constexpr uint32_t IsLoadingOrConnecting = 0x00B6AA38; // connection/login operation enum (0..13), not a loading boolean
        constexpr uint32_t Timestamp = 0x00B1D618; // UNVERIFIED: only zero-initialized by a static constructor (object at 0xB1D5E8), never read or written afterwards
        constexpr uint32_t LastHardwareAction = 0x00B499A4; // last input time
        constexpr uint32_t BuildNumber = 0x00B3203C; // client build number
        constexpr uint32_t LocalGUID = 0x00CA1238; // local player GUID
        constexpr uint32_t ObjectManager_Ptr = 0x00C79CE0; // pointer to ObjectManager
    }
} // namespace WoWOffsets
```
