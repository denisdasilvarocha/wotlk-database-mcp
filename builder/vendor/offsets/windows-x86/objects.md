# Object manager and objects

Read for GUID lookup, visible-object enumeration, and object, item, container, corpse, or game-object methods. Unit and player methods are in [units and players](units-players.md).

Catalog provenance: [Windows x86, build 12340](windows-x86.md). Use the shared [address conventions and verification guidance](address-conventions.md) when applying these constants.

## Contents

- [Object manager](#object-manager)
- [Object types](#object-types)

## Object manager

```cpp
namespace WoWOffsets {
    namespace ObjectManager {
        namespace ClntObjMgr {
            constexpr uint32_t GetObjectPtr = 0x004D4BB0; // get object pointer by GUID
            constexpr uint32_t EnumVisibleObjects = 0x004D4B30; // enumerate visible objects (callback)
            constexpr uint32_t ObjectPtr = 0x004D4DB0; // get object from ObjectManager
            constexpr uint32_t GetActivePlayer = 0x004D3790; // get local player GUID
            constexpr uint32_t GetActivePlayerObj = 0x004038F0; // get local player object
            constexpr uint32_t GetMapID = 0x006CD770; // get current map ID — **label unverified**: body is an event/opcode handler (registered as callback id 0x4C0)
            constexpr uint32_t Create = 0x004D7750; // create ObjectManager
            constexpr uint32_t Destroy = 0x004D6750; // destroy ObjectManager
            constexpr uint32_t Initialize = 0x004D76E0; // initialize ObjectManager
            constexpr uint32_t InitializeShared = 0x004D4AC0; // initialize shared state
        }
    }
} // namespace WoWOffsets
```

## Object types

```cpp
namespace WoWOffsets {
    namespace Object {
        namespace CGObject_C {
            constexpr uint32_t CGObject_C = 0x00745E60; // constructor
            constexpr uint32_t Initialize = 0x007460C0; // initialize object
            constexpr uint32_t Shutdown = 0x00744E50; // shutdown object
            constexpr uint32_t Enable = 0x00744DB0; // enable object
            constexpr uint32_t Disable = 0x00744D20; // disable object
            constexpr uint32_t PostInit = 0x00744A50; // post initialization
            constexpr uint32_t Destructor = 0x004D6010; // destructor
            constexpr uint32_t GetPosition = 0x004D5EA0; // get object position
            constexpr uint32_t GetFacing = 0x004D5EE0; // get facing angle
            constexpr uint32_t GetFacing_Override = 0x004D5EF0; // get facing (override)
            constexpr uint32_t virt3C = 0x004D5F10; // virtual position func
            constexpr uint32_t TranslateRotation = 0x004D5F20; // translate rotation
            constexpr uint32_t CopyMatrix = 0x00744380; // copy transform matrix
            constexpr uint32_t SetTypeByTypeId = 0x00743130; // set object type
            constexpr uint32_t GetQuestStatus = 0x00745DA0; // get quest status — **label unverified**: body rebuilds the selection-circle model (no quest code)
            constexpr uint32_t SetupDescriptorBounds = 0x00743640; // setup descriptor bounds
            constexpr uint32_t IsLocalPlayer = 0x004CEE50; // check if local player
            constexpr uint32_t virt14 = 0x007438E0; // virt14
            constexpr uint32_t virt20 = 0x007451B0; // virt20
            constexpr uint32_t virt9C = 0x007432E0; // virt9C
            constexpr uint32_t virt98 = 0x00743250; // virt98
            constexpr uint32_t virtE8 = 0x00743300; // virtE8
            constexpr uint32_t virtE4 = 0x00743330; // virtE4
            constexpr uint32_t virtB8 = 0x00743530; // virtB8
            constexpr uint32_t PreAnimate = 0x00743EC0; // pre-animation update
            constexpr uint32_t virt50 = 0x007444E0; // animation virt
            constexpr uint32_t PostReenable = 0x00743FF0; // post re-enable
            constexpr uint32_t AddWorldObject = 0x00743760; // add to world
        }

        namespace CGxDevice {
            constexpr uint32_t BuildSelectionMatrix = 0x00744150; // build selection matrix
        }

        namespace CGItem_C {
            constexpr uint32_t CGItem_C = 0x00709840; // constructor
            constexpr uint32_t Initialize = 0x00709440; // initialize item
            constexpr uint32_t Shutdown = 0x00709A10; // shutdown item
            constexpr uint32_t PostInit = 0x0070AF80; // post initialization
            constexpr uint32_t GetItemEntry = 0x00707360; // get item entry ID
            constexpr uint32_t BuildItemName = 0x00706D70; // build item name string
            constexpr uint32_t GetClassID = 0x00707220; // get item class ID
            constexpr uint32_t GetInventoryType = 0x00707280; // get inventory type
            constexpr uint32_t virt5C = 0x00707150; // virtual function
            constexpr uint32_t GetInventoryArt = 0x0070A910; // get inventory icon
            constexpr uint32_t OnRightClick = 0x00708200; // handle right click
            constexpr uint32_t RequestRefundInfo = 0x007089E0; // request refund info
            constexpr uint32_t SetItemRefundToken = 0x00706CF0; // set refund token
            constexpr uint32_t SignalEvent = 0x0060BFE0; // signal item event
            constexpr uint32_t CreateItemLink = 0x0061E290; // create item link
            constexpr uint32_t CanEquipInSlot = 0x005176F0; // check equip slot
            constexpr uint32_t GetBag = 0x00706AD0; // get containing bag — address is a **CGContainer_C** member (its vtable 0xA332D0 slots 9/10, returns `this+0x760`)
            constexpr uint32_t GetStatName = 0x0061BB20; // get stat name
            constexpr uint32_t GetBagFamily = 0x007072B0; // get bag family
        }

        namespace Item {
            constexpr uint32_t GetItemName = 0x007071D0; // get item name
            constexpr uint32_t GetItemIDByName = 0x00709DE0; // get item ID by name
            constexpr uint32_t GetPageTextId = 0x00707180; // get page text ID
        }

        namespace CGItemStats_C {
            constexpr uint32_t GetSocketFlags = 0x0061B930; // get socket flags
            constexpr uint32_t Create = 0x0061B5B0; // create item stats
            constexpr uint32_t CleanStatNames = 0x0061BA90; // clean stat names
            constexpr uint32_t MakeStatNamesPretty = 0x0061B9E0; // format stat names
        }

        namespace CGGameObject_C {
            constexpr uint32_t CGGameObject_C = 0x00714250; // constructor
            constexpr uint32_t Initialize = 0x007140A0; // initialize
            constexpr uint32_t Shutdown = 0x00714150; // shutdown
            constexpr uint32_t PostInit = 0x00712F30; // post initialization
            constexpr uint32_t Disable = 0x007130A0; // disable
            constexpr uint32_t PostReenable = 0x00713130; // post re-enable
            constexpr uint32_t Destructor = 0x00713F20; // destructor
            constexpr uint32_t OnRightClick = 0x00711140; // handle right click
            constexpr uint32_t CloseInteraction = 0x0070BAC0; // close interaction
            constexpr uint32_t LoadBaseObject = 0x00712400; // load base object
            constexpr uint32_t GetLockRecord = 0x0070EF30; // get lock record
            constexpr uint32_t virt64 = 0x00713F50; // virt64
            constexpr uint32_t virt3C = 0x007124A0; // virt3C
            constexpr uint32_t virt38 = 0x007111A0; // virt38
            constexpr uint32_t virt84 = 0x0070F580; // virt84
            constexpr uint32_t virt40 = 0x0070EE80; // virt40
            constexpr uint32_t virt18 = 0x0070F940; // virt18
        }

        namespace Level2Object {
            constexpr uint32_t SetupDescriptorBounds = 0x0070CBA0; // setup descriptors
        }

        namespace CGContainer_C {
            constexpr uint32_t CGContainer_C = 0x00706A20; // constructor
            constexpr uint32_t virt00 = 0x00706B60; // virt00
        }

        namespace CGCorpse_C {
            constexpr uint32_t CGCorpse_C = 0x00706430; // constructor
            constexpr uint32_t PostInit = 0x00705B20; // post initialization
            constexpr uint32_t GetCorpsePosition = 0x0051F430; // get corpse position
        }

        namespace CGDynamicObject_C {
            constexpr uint32_t CGDynamicObject_C = 0x007053A0; // constructor
            constexpr uint32_t PostInit = 0x00705230; // post initialization
        }

        namespace CGBag_C {
            constexpr uint32_t FindItem_0 = 0x007546F0; // find item in bag
            constexpr uint32_t FindItem = 0x00754CD0; // find item (alt)
            constexpr uint32_t GetItemTypeCount = 0x00754D00; // get item type count
            constexpr uint32_t GetItemIdByName = 0x00754B90; // get item by name
            constexpr uint32_t GetBagItem = 0x00754390; // get bag item
            constexpr uint32_t GetBagAtIndex = 0x005D6F20; // get bag at index
        }
    }
} // namespace WoWOffsets
```
