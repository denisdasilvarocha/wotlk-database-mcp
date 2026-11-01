# Client lifecycle, memory, and globals

## Globals

```cpp
namespace WoWOffsets {
    namespace Globals {
        constexpr uint32_t Timestamp = 0x00B1D618; // UNVERIFIED: only zero-initialized by a static constructor (object at 0xB1D5E8), never read or written afterwards
    }
} // namespace WoWOffsets
```
