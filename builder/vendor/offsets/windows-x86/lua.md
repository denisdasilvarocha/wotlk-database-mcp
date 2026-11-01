# Lua and FrameScript

Read for native Lua execution, state access, stack operations, C-function registration, and event dispatch. Search `FrameScript`, `Execute`, `GetLuaState`, or the Lua C API member name. Native widget method addresses are in [UI widgets](ui-widgets.md).

Catalog provenance: [Windows x86, build 12340](windows-x86.md). Use the shared [address conventions and verification guidance](address-conventions.md) when applying these constants.

## FrameScript

```cpp
namespace WoWOffsets {
    namespace Lua {
        namespace FrameScript {
            constexpr uint32_t Execute = 0x00819210; // execute Lua code string
            constexpr uint32_t GetLuaState = 0x00817DB0; // get active lua_State*
            constexpr uint32_t RegisterFunction = 0x00817F90; // register Lua C function
            constexpr uint32_t UnregisterFunction = 0x00817FD0; // unregister Lua function
            constexpr uint32_t GetVariable = 0x00818010; // get Lua global variable
            constexpr uint32_t GetTop = 0x0084DBD0; // get stack top index
            constexpr uint32_t SetTop = 0x0084DBF0; // set stack top index
            constexpr uint32_t pushnil = 0x0084E280; // push nil to stack
            constexpr uint32_t PushNumber = 0x0084E2A0; // push number to stack
            constexpr uint32_t pushinteger = 0x0084E2D0; // push integer to stack
            constexpr uint32_t PushString = 0x0084E350; // push string to stack
            constexpr uint32_t pushboolean = 0x0084E4D0; // push boolean to stack
            constexpr uint32_t pushcclosure = 0x0084E400; // push C closure
            constexpr uint32_t ToNumber = 0x0084E030; // convert to number
            constexpr uint32_t tointeger = 0x0084E070; // convert to integer
            constexpr uint32_t ToLString = 0x0084E0E0; // convert to string with length
            constexpr uint32_t toboolean = 0x0084E0B0; // convert to boolean
            constexpr uint32_t touserdata = 0x0084E1C0; // get userdata pointer
            constexpr uint32_t tocfunction = 0x0084E1C0; // **Not present in build 12340** — 0x84E1C0 is lua_touserdata; nearest predicate is lua_iscfunction at 0x0084DEF0
            constexpr uint32_t IsNumber = 0x0084DF20; // check if number
            constexpr uint32_t IsString = 0x0084DF60; // check if string
            constexpr uint32_t equal = 0x0084DFE0; // compare values
            constexpr uint32_t CreateTable = 0x0084E6E0; // create new table
            constexpr uint32_t getfield = 0x0084F3B0; // get table field
            constexpr uint32_t setfield = 0x0084E900; // set table field
            constexpr uint32_t GetTable = 0x0084E560; // get table by reference
            constexpr uint32_t FindTable = 0x0084E590; // find table by name
            constexpr uint32_t SignalEvent = 0x0081B530; // fire UI event
            constexpr uint32_t SignalCombatLogEvent = 0x00750400; // fire combat log event
            constexpr uint32_t DisplayError = 0x0084F280; // display error message
            constexpr uint32_t GetParamValue = 0x00815500; // get event parameter
        }
    }
} // namespace WoWOffsets
```
