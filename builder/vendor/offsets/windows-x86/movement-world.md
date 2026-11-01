# Movement and world

Read for movement controls, flags, speeds, splines, transports, movement status, or flyable-area queries. Player click-to-move actions are in [units and players](units-players.md#players); keyboard and mouse controls are in [game UI and input](ui-game.md#input).

Catalog provenance: [Windows x86, build 12340](windows-x86.md). Use the shared [address conventions and verification guidance](address-conventions.md) when applying these constants.

## Contents

- [Movement](#movement)
- [World](#world)

## Movement

```cpp
namespace WoWOffsets {
    namespace Movement {
        namespace CMovement {
            constexpr uint32_t MoveUnit = 0x006F09F0; // move unit
            constexpr uint32_t OnMoveStart = 0x006F0CF0; // start moving
            constexpr uint32_t OnMoveStop = 0x0098C8A0; // movement stop handler
            constexpr uint32_t OnJump = 0x006F0DD0; // jump handler
            constexpr uint32_t StopMove = 0x0098BD10; // force stop movement
            constexpr uint32_t OnStrafeStart = 0x006F0D60; // start strafing
            constexpr uint32_t OnStrafeStop = 0x00988DC0; // stop strafing
            constexpr uint32_t StartTurn = 0x00988DF0; // start turning
            constexpr uint32_t OnTurnStart = 0x006F1010; // turn start handler
            constexpr uint32_t OnTurnStop = 0x00989010; // turn stop handler
            constexpr uint32_t OnPitchStop = 0x00989450; // pitch stop handler
            constexpr uint32_t IsFalling = 0x005FEDE0; // check if falling
            constexpr uint32_t CreateMovementStatus = 0x00987140; // create movement status
            constexpr uint32_t CreateMovementStatus_NoMovementFlags = 0x006E90E0; // status without flags
            constexpr uint32_t FillFromStatus = 0x009872C0; // fill from status
            constexpr uint32_t FillFromStatus_Falling = 0x00988990; // fill falling status
            constexpr uint32_t UpdateStatus = 0x006ED990; // update status
            constexpr uint32_t GetMoveStartTime = 0x006E9F50; // get move start time
            constexpr uint32_t GetMoveEventMsgId = 0x006E9870; // get event message ID
            constexpr uint32_t CalcCurrentSpeed = 0x00987570; // calculate current speed
            constexpr uint32_t ReCalcCurrentSpeed = 0x009880C0; // recalculate speed
            constexpr uint32_t OnMoveSetRunSpeed = 0x006EDB80; // set run speed
            constexpr uint32_t OnMoveSetRunBackSpeed = 0x006EDC70; // set run back speed
            constexpr uint32_t OnMoveSetWalkSpeed = 0x006EDD60; // set walk speed
            constexpr uint32_t OnMoveSetSwimSpeed = 0x006EDE50; // set swim speed
            constexpr uint32_t OnMoveSetSwimBackSpeed = 0x006EDF40; // set swim back speed
            constexpr uint32_t OnFlightSpeedChange_ActiveMover = 0x006EE030; // flight speed change
            constexpr uint32_t OnFlightBackSpeedChange = 0x006EE180; // flight back speed
            constexpr uint32_t OnMoveSetTurnRate = 0x006EE210; // set turn rate
            constexpr uint32_t OnPitchSpeedChange_ActiveMover = 0x006EE300; // pitch speed change
            constexpr uint32_t SplineFlags = 0x006E9AD0; // get spline flags
            constexpr uint32_t SplineEnableFlying = 0x00989890; // enable spline flying
            constexpr uint32_t SplineToggleEnableFlying = 0x006EBE50; // toggle flying
            constexpr uint32_t OnSplineMoveUnRoot = 0x006EBA20; // spline unroot
            constexpr uint32_t OnSplineStopSwim = 0x006EB020; // spline stop swim
            constexpr uint32_t OnSplineMoveToggleHover = 0x006EB060; // toggle hover
            constexpr uint32_t OnSplineMoveToggleRunMode = 0x006E90D0; // toggle run mode
            constexpr uint32_t OnSplineMoveToggleWaterWalk = 0x006E9240; // toggle water walk
            constexpr uint32_t OnSplineMoveToggleFeatherFall = 0x006E9250; // toggle feather fall
            constexpr uint32_t ToggleMovementFlag_0x20000000_FeatherFall = 0x00987440; // toggle feather fall; flag bit is named in the identifier
            constexpr uint32_t ToggleMovementFlag_0x10000000_WaterWalk = 0x009873F0; // toggle water walk; flag bit is named in the identifier
            constexpr uint32_t ToggleMovementFlag_0x40000000_Hover = 0x00987460; // toggle hover; flag bit is named in the identifier
            constexpr uint32_t SetMovementFlag_0x8000000 = 0x009872A0; // set flag; flag bit is named in the identifier
            constexpr uint32_t ClearMovementFlag_0x8000000 = 0x009872B0; // clear flag; flag bit is named in the identifier
            constexpr uint32_t ToggleMovementFlag2_0x80 = 0x0098B570; // toggle flag2; flag bit is named in the identifier
            constexpr uint32_t ToggleMovementFlag2_0x100 = 0x0098B590; // toggle flag2; flag bit is named in the identifier
            constexpr uint32_t ProcessMoveEvent = 0x006ED0F0; // process move event
            constexpr uint32_t AddPlayerMoveEvent = 0x006EC8B0; // add player move event
            constexpr uint32_t HandleSplinePackets_1235_1236 = 0x006EBC20; // handle spline packets
            constexpr uint32_t WriteMovementStatusToPacket = 0x007164B0; // write status to packet
            constexpr uint32_t CalcFallStartElevation = 0x00986F00; // calculate fall start
            constexpr uint32_t Swimming_FallCalc = 0x0098BFF0; // swimming fall calc
            constexpr uint32_t IsFallingSwimmingFlying_6636D0 = 0x006EABA0; // check state
            constexpr uint32_t OnStartAscendOrDescendPacket = 0x006EF5C0; // ascend/descend
            constexpr uint32_t OnMoveStopAscendPacket = 0x006EF630; // stop ascend
            constexpr uint32_t OnFallChangePacket = 0x006EEE50; // fall change
            constexpr uint32_t OnFallLand_OnHeartbeat = 0x006EF680; // fall land heartbeat
            constexpr uint32_t DisableFlying = 0x0098C240; // disable flying
            constexpr uint32_t EnableHover = 0x009883F0; // enable hover
            constexpr uint32_t DisableHover = 0x0098B710; // disable hover
            constexpr uint32_t OnMoveHover = 0x006EEF60; // move hover handler
            constexpr uint32_t OnMoveUpdateCanFlyPacket = 0x006EF0D0; // can fly update
            constexpr uint32_t OnCanFlyChangePacket = 0x006EF060; // can fly change
            constexpr uint32_t ForceSetTransportInt = 0x006EC400; // force set transport
            constexpr uint32_t SetTransportAndSend_909 = 0x006F0C70; // set transport and send
            constexpr uint32_t OnKnockback = 0x006F0E30; // knockback handler
            constexpr uint32_t OnKnockbackPacket = 0x007187F0; // knockback packet
        }

        namespace CMovementStatus {
            constexpr uint32_t ctor = 0x004F4C50; // status constructor
            constexpr uint32_t Read = 0x004F4D40; // read status
            constexpr uint32_t Skip = 0x004F4CA0; // skip status
            constexpr uint32_t Write = 0x004F4ED0; // write status
        }

        namespace CMoveSpline {
            constexpr uint32_t Read = 0x004F4B50; // read spline
            constexpr uint32_t Skip = 0x004F4A50; // skip spline
            constexpr uint32_t CopyFrom = 0x006F1240; // copy spline
            constexpr uint32_t Dtor = 0x0074B7E0; // spline destructor
            constexpr uint32_t Alloc = 0x0074B810; // allocate spline
        }

        namespace Movement {
            constexpr uint32_t LoadMovePacket = 0x0071EF80; // load move packet
            constexpr uint32_t WriteMovementPacket = 0x0071F060; // write move packet
            constexpr uint32_t AllocPlayerMoveEvent = 0x006EBC70; // allocate move event
            constexpr uint32_t AllocCMoveSpline = 0x0074B7B0; // allocate move spline
            constexpr uint32_t MovementAddTransport = 0x0074B730; // add transport
            constexpr uint32_t MovementMoveTransports = 0x0074B6E0; // move transports
            constexpr uint32_t MovementRemoveTransport = 0x0074B750; // remove transport
            constexpr uint32_t MovementGetTransportMatrix = 0x0074B4C0; // get transport matrix
            constexpr uint32_t MovementGetTransportMatrixInternal = 0x0074B430; // get matrix internal
        }

        namespace CPlayerMoveEvent {
            constexpr uint32_t FromMoveStatus = 0x006E9050; // create from status
        }
    }
} // namespace WoWOffsets
```

## World

```cpp
namespace WoWOffsets {
    namespace World {
        constexpr uint32_t IsInFlyableArea = 0x0051A1D0; // check flyable area
    }
} // namespace WoWOffsets
```
