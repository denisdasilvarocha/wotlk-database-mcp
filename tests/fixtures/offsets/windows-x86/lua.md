# Lua and FrameScript

## FrameScript

```cpp
namespace WoWOffsets {
    namespace Lua {
        namespace FrameScript {
            constexpr uint32_t Execute = 0x00819210; // execute Lua code string
            constexpr uint32_t tocfunction = 0x0084E1C0; // **Not present in build 12340** — 0x84E1C0 is lua_touserdata; nearest predicate is lua_iscfunction at 0x0084DEF0
        }
    }
} // namespace WoWOffsets
```
