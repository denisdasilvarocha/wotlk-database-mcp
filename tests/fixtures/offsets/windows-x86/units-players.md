# Units and players

## Units

```cpp
namespace WoWOffsets {
    namespace Unit {
        namespace CGUnit_C {
            constexpr uint32_t IsFriendlyTowards = 0x0051A230; // **Uncertain**: 30-byte no-arg predicate [unit+0x9C0]>0 && !([unit+0xA30]&0x10000000); a friendly-towards test needs a target + faction logic the body lacks
        }
    }
} // namespace WoWOffsets
```
