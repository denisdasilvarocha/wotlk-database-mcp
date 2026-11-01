# Warden module lifecycle

Read for Warden state, module loading/unloading, exports, cached-module serialization, and packet dispatch. Related cache entries remain in [record caches](caches.md); `SMSG_WARDEN_DATA` is in [network packet handlers](network.md#packet-handlers).

Catalog provenance: [Windows x86, build 12340](windows-x86.md). Use the shared [address conventions and verification guidance](address-conventions.md) when applying these constants.

## Warden functions

```cpp
namespace WoWOffsets {
    namespace Warden {
        namespace Warden {
            constexpr uint32_t Tick = 0x007DA3D0; // warden tick/update
            constexpr uint32_t GenerateKeys = 0x007DA360; // generate encryption keys — **label unverified**: body activates/instantiates the pending module (`D31A44` → export #1 → `D31A4C`)
            constexpr uint32_t UnloadMainModule = 0x007DA300; // unload main module
            constexpr uint32_t ShutdownAndUnload = 0x007DA420; // shutdown and unload
            constexpr uint32_t StateLoad = 0x007DA550; // load state
            constexpr uint32_t StateSave = 0x007DA4F0; // save state
            constexpr uint32_t MemAlloc = 0x007DA4B0; // warden memory alloc
            constexpr uint32_t MemFree = 0x007DA4D0; // warden memory free
            constexpr uint32_t ZlibDecompress = 0x00866770; // decompress data
            constexpr uint32_t SendPacket = 0x007DAAB0; // send Warden packet
            constexpr uint32_t SetWardenPacketHandler = 0x007DA8C0; // set packet handler
            constexpr uint32_t SaveModule = 0x007DA9F0; // save module to disk
        }

        namespace WardenModule {
            constexpr uint32_t LoadWardenModule = 0x00872350; // load Warden module
            constexpr uint32_t UnloadWardenModule = 0x008722C0; // unload module
        }

        namespace CWardenClient {
            constexpr uint32_t LoadModule = 0x007DA610; // client load module
        }

        namespace BLLLoader {
            constexpr uint32_t GetExport = 0x00872320; // get export from module
        }

        namespace WardenCached {
            constexpr uint32_t LoadKey = 0x007DA1C0; // load cached key
        }

        namespace WardenCachedModule {
            constexpr uint32_t StoreToCDataStore = 0x007DA220; // store to data store
            constexpr uint32_t LoadFromCDataStore = 0x007DA260; // load from data store
            constexpr uint32_t Copy = 0x007DA5C0; // copy cached module
        }
    }
} // namespace WoWOffsets
```
