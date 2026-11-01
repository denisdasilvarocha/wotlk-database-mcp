# Game UI, cameras, and input

Read for permission checks (`CheckPermissions`), targeting (`Target`), terrain clicks, action bars, world-frame updates, camera operations, or keyboard/mouse input. Frame, texture, font, and button methods are in [UI widgets](ui-widgets.md).

Catalog provenance: [Windows x86, build 12340](windows-x86.md). Use the shared [address conventions and verification guidance](address-conventions.md) when applying these constants.

## Contents

- [Game UI and actions](#game-ui-and-actions)
- [World frame and cameras](#world-frame-and-cameras)
- [Input](#input)

## Game UI and actions

```cpp
namespace WoWOffsets {
    namespace UI {
        namespace CGGameUI {
            constexpr uint32_t EnterWorld = 0x00528010; // enter world
            constexpr uint32_t LeaveWorld = 0x00528C30; // leave world
            constexpr uint32_t ShutdownGame = 0x00529160; // shutdown game
            constexpr uint32_t Shutdown = 0x00528F00; // shutdown UI
            constexpr uint32_t InitializeGame = 0x0052B550; // initialize game
            constexpr uint32_t Reload = 0x0051D960; // reload UI; sets the reload flag BD0791=1; the Lua ReloadUI wrapper is 5222E0
            constexpr uint32_t CheckBindings = 0x008165E0; // check key bindings
            constexpr uint32_t CheckPermissions = 0x005191C0; // lua protection check (bypass target)
            constexpr uint32_t Target = 0x00524BF0; // target unit
            constexpr uint32_t ClearInteractTarget = 0x00518D50; // clear interact target
            constexpr uint32_t SetInteractTarget = 0x0051FCE0; // set interact target
            constexpr uint32_t CloseInteraction = 0x00512E60; // close interaction
            constexpr uint32_t OpenGuildInvite = 0x00512570; // open guild invite
            constexpr uint32_t OpenGossipWindow = 0x006D1E20; // open gossip window
            constexpr uint32_t ClearCursor = 0x00519280; // clear cursor
            constexpr uint32_t SetCursorItem = 0x00520770; // set cursor item
            constexpr uint32_t GetCursorItem = 0x00513660; // get cursor item
            constexpr uint32_t UnlockItem = 0x00513770; // unlock cursor item
            constexpr uint32_t DisplayError = 0x005216F0; // display error message
            constexpr uint32_t BroadcastUIError = 0x005219E0; // broadcast UI error
            constexpr uint32_t ShowCombatFeedback_0 = 0x00513380; // show combat feedback
            constexpr uint32_t OnTerrainClick = 0x00527360; // handle terrain click
            constexpr uint32_t HandleTerrainClick = 0x00527830; // terrain click handler
            constexpr uint32_t ShowSpellFailed = 0x00808200; // show spell failed UI
        }

        namespace CGameUI {
            constexpr uint32_t Signal_EVENT_ACTIONBAR_UPDATE_COOLDOWN = 0x005A7CC0; // cooldown event
            constexpr uint32_t Signal_EVENT_ACTIONBAR_UPDATE_STATE = 0x005A7CB0; // state event
        }

        namespace CGCursor_C {
            constexpr uint32_t Update = 0x006162C0; // update cursor
            constexpr uint32_t SetCursorFromName = 0x00616830; // set cursor by name
            constexpr uint32_t CursorSetCursorMode = 0x00616800; // set cursor mode
            constexpr uint32_t CursorSetHeldItem = 0x00616630; // set held item cursor
        }

        namespace CGActionBar {
            constexpr uint32_t UseAction = 0x005ABBC0; // use action
            constexpr uint32_t PutActionInSlot = 0x005AB120; // put action in slot
            constexpr uint32_t PickupAction = 0x005ABE70; // pickup action
            constexpr uint32_t GetCooldown = 0x005A8E40; // get cooldown
            constexpr uint32_t GetTexture = 0x005A97F0; // get action texture
            constexpr uint32_t IsCurrentAction = 0x005AA240; // is current action
            constexpr uint32_t HideGrid = 0x005A7A80; // hide action bar grid
        }
    }
} // namespace WoWOffsets
```

## World frame and cameras

```cpp
namespace WoWOffsets {
    namespace UI {
        namespace CGWorldFrame {
            constexpr uint32_t OnWorldUpdate = 0x004FA5F0; // world update
            constexpr uint32_t OnLayerUpdate = 0x004FA040; // layer update
            constexpr uint32_t Render = 0x004F8EA0; // render frame
            constexpr uint32_t RenderWorld = 0x004FAF90; // render world
            constexpr uint32_t Intersect = 0x0077F310; // intersect test
            constexpr uint32_t TranslateToMapCoords = 0x00544140; // map coord translate
            constexpr uint32_t CreateFrames = 0x00521040; // create frames
            constexpr uint32_t GetActiveCamera = 0x004F5960; // get active camera
        }

        namespace CGCamera {
            constexpr uint32_t SetView = 0x00603330; // set camera view
            constexpr uint32_t ResetView = 0x006048A0; // reset view
            constexpr uint32_t UpdateFreeLookFacing = 0x006020B0; // update free look
            constexpr uint32_t SyncFreeLookFacing = 0x006023D0; // sync free look
            constexpr uint32_t ZoomIn = 0x005FF950; // zoom in
            constexpr uint32_t ZoomOut = 0x005FFA60; // zoom out
            constexpr uint32_t ParentToWorld = 0x00600B60; // parent to world
            constexpr uint32_t virt00 = 0x005FF500; // virt00
            constexpr uint32_t virt04 = 0x00600C20; // virt04
            constexpr uint32_t virt08 = 0x00600CC0; // virt08
            constexpr uint32_t virt0C = 0x00600D60; // virt0C
            constexpr uint32_t CreateViewFromCamera = 0x005FE4E0; // create from camera
            constexpr uint32_t Unknown1 = 0x00604940; // unknown function
            constexpr uint32_t Unknown5 = 0x00604A70; // unknown function
        }

        namespace Camera {
            constexpr uint32_t CameraCreate = 0x004BFCA0; // create camera
            constexpr uint32_t CameraDuplicate = 0x004BFCE0; // duplicate camera
        }
    }
} // namespace WoWOffsets
```

## Input

```cpp
namespace WoWOffsets {
    namespace UI {
        namespace CGInputControl {
            constexpr uint32_t CGInputControl = 0x005FCD70; // constructor
            constexpr uint32_t InputControlDestroy_0 = 0x005FD3D0; // destroy
            constexpr uint32_t InputControlDestroy = 0x005FD430; // destroy (alt)
            constexpr uint32_t SetControlBit = 0x005FA170; // set control bit
            constexpr uint32_t UnsetControlBit = 0x005FA450; // unset control bit
            constexpr uint32_t ToggleControlBit = 0x005FBE10; // toggle control bit
            constexpr uint32_t UpdatePlayer = 0x005FBBC0; // update player
            constexpr uint32_t OnMouseMoveRel = 0x005FAB70; // mouse move relative
            constexpr uint32_t EnterWorld = 0x005FAC60; // enter world
            constexpr uint32_t CheckToCancelCurrentChannelSpell = 0x005FAA40; // cancel channeling check
        }

        namespace CInputControl {
            constexpr uint32_t Ctor = 0x005FD1D0; // constructor (alt)
            constexpr uint32_t VehicleAimChange = 0x005FB4B0; // vehicle aim
            constexpr uint32_t RemoveFlags_0xF0000 = 0x005F95E0; // remove flags
            constexpr uint32_t RemoveFlags_0x11000 = 0x005F95F0; // remove flags (alt)
        }

        namespace Input {
            constexpr uint32_t OnKeyDown = 0x00765AF0; // key down event
            constexpr uint32_t OnKeyDownRepeat = 0x00764B90; // key repeat event
            constexpr uint32_t OnKeyUp = 0x00763BE0; // key up event
            constexpr uint32_t OnChar = 0x00764990; // character input
            constexpr uint32_t OnMouseDown = 0x00764A10; // mouse down event
            constexpr uint32_t closeconsole = 0x00763C10; // close console
        }
    }
} // namespace WoWOffsets
```
