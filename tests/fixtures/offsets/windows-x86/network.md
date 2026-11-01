# Network

```cpp
namespace WoWOffsets {
    namespace Network {
        namespace CClientConnection {
            constexpr uint32_t Send = 0x00926A70; // send packet — **label unverified**: FMOD static-lib socket-send loop (`send()`+`WSAEWOULDBLOCK` retry); both callers are FMOD translation units
        }
    }
} // namespace WoWOffsets
```
