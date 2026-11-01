# Networking and serialization

Read for packet buffers, client connections, login, chat, or named packet handlers. `Packets` entries are handler addresses; their names do not make them opcode values. Movement serialization helpers also appear in [movement and world](movement-world.md).

Catalog provenance: [Windows x86, build 12340](windows-x86.md). Use the shared [address conventions and verification guidance](address-conventions.md) when applying these constants.

## Contents

- [Serialization](#serialization)
- [Connections and login](#connections-and-login)
- [Packet handlers](#packet-handlers)

## Serialization

```cpp
namespace WoWOffsets {
    namespace Network {
        namespace CDataStore {
            constexpr uint32_t New = 0x008CD0B0; // create new data store
            constexpr uint32_t Free = 0x00401030; // free data store
            constexpr uint32_t Free_0 = 0x008CD160; // free (alternate)
            constexpr uint32_t Destroy = 0x00403880; // frees the buffer and restores the base CDataStore vtable; matches Packets::Destroy
            constexpr uint32_t Reset = 0x004010E0; // reset data store
            constexpr uint32_t InternalDestroy = 0x0047AE50; // internal destroy
            constexpr uint32_t dtor = 0x008CCF50; // destructor
            constexpr uint32_t GetByte = 0x0047B340; // read byte (uint8)
            constexpr uint32_t GetInt8_2 = 0x00401140; // read int8
            constexpr uint32_t GetInt16 = 0x0047B380; // read int16
            constexpr uint32_t GetInt = 0x0047B3C0; // read int32
            constexpr uint32_t ReadInt32 = 0x0050F420; // read int32 (alt)
            constexpr uint32_t GetUInt64 = 0x0047B400; // read uint64
            constexpr uint32_t GetFloat = 0x0047B440; // read float
            constexpr uint32_t GetString = 0x0047B480; // read string
            constexpr uint32_t GetCString = 0x0047B560; // read C string
            constexpr uint32_t GetCString_2 = 0x0047B6A0; // read C string (alt)
            constexpr uint32_t GetIntArray = 0x0047B5F0; // read int array
            constexpr uint32_t ReadBytes = 0x0047B6B0; // read raw bytes
            constexpr uint32_t GetWowGUID = 0x0076DC20; // read WoW GUID
            constexpr uint32_t ReadPackedVector3 = 0x007152B0; // read packed vector
            constexpr uint32_t PutByte = 0x0047AFE0; // write byte
            constexpr uint32_t PutInt8AtPos = 0x0047AEF0; // write int8 at position
            constexpr uint32_t PutInt16 = 0x0047B040; // write int16
            constexpr uint32_t PutInt16AtPos = 0x0047AF40; // write int16 at position
            constexpr uint32_t PutInt32AtPos = 0x0047AF90; // write int32 at position
            constexpr uint32_t Put = 0x0047B0A0; // write data
            constexpr uint32_t PutGuid = 0x0047B100; // write GUID
            constexpr uint32_t PutFloat = 0x0047B160; // write float
            constexpr uint32_t PutString = 0x0047B300; // write string
            constexpr uint32_t PutCString_2 = 0x0047B1C0; // write C string
            constexpr uint32_t PutCString_3 = 0x0047B280; // write C string (alt)
            constexpr uint32_t PutVector3 = 0x00406DA0; // write vector3
            constexpr uint32_t GetBufferParams = 0x0047ADE0; // get buffer params
            constexpr uint32_t DetachBuffer = 0x0047AE10; // detach buffer
            constexpr uint32_t DetachBufferProxy = 0x008CCFC0; // detach buffer proxy
            constexpr uint32_t CanRead = 0x0047B290; // check readable bytes
            constexpr uint32_t CanReadNumBytes = 0x008CBBF0; // check specific count
            constexpr uint32_t InternalFetchWrite = 0x0047AEA0; // internal write fetch
            constexpr uint32_t InternalFetchRead = 0x00936900; // internal read fetch
            constexpr uint32_t Alloc = 0x00401010; // allocate packet
            constexpr uint32_t GenPacket = 0x00401050; // generate packet
            constexpr uint32_t InitPacket2 = 0x00403EC0; // initialize packet
            constexpr uint32_t GetPosition = 0x00401190; // get read position
            constexpr uint32_t Finalize = 0x00401130; // finalize packet
            constexpr uint32_t ReleasePacket2 = 0x00403F10; // release packet
            constexpr uint32_t ReleaseSomePacket = 0x008CD040; // release packet (alt)
            constexpr uint32_t InitSomePacket = 0x008CCEE0; // init packet
        }
    }
} // namespace WoWOffsets
```

## Connections and login

```cpp
namespace WoWOffsets {
    namespace Network {
        namespace NetClient {
            constexpr uint32_t NetClient = 0x00632A40; // constructor
            constexpr uint32_t _NetClient = 0x006331B0; // destructor
            constexpr uint32_t Initialize = 0x00631D30; // initialize client
            constexpr uint32_t Destroy = 0x006322A0; // destroy client
            constexpr uint32_t SetMessageHandler = 0x00631FA0; // set message handler
            constexpr uint32_t HandleDisconnect = 0x00632540; // handle disconnect
            constexpr uint32_t HandleData = 0x00632460; // handle incoming data
            constexpr uint32_t WCMessageReady = 0x00633330; // message ready callback
            constexpr uint32_t WCDisconnected = 0x00632060; // disconnected callback
            constexpr uint32_t DelayedDelete = 0x00632390; // delayed delete
            constexpr uint32_t Disconnect = 0x00631EA0; // disconnect
            constexpr uint32_t GetNetStats = 0x006320D0; // get network stats
        }

        namespace CNetClient {
            constexpr uint32_t ResetHandler = 0x00631FC0; // reset handler
            constexpr uint32_t Process = 0x00631FE0; // process network
            constexpr uint32_t SetUserNameAndSessionKey = 0x00631F70; // set credentials
        }

        namespace ClientServices {
            constexpr uint32_t GetSessionKey = 0x00631F90; // get session key
            constexpr uint32_t Initialize = 0x006B2200; // initialize
            constexpr uint32_t PollStatus = 0x006B1100; // poll status
            constexpr uint32_t Connect = 0x006B1390; // connect
            constexpr uint32_t Disconnect = 0x006B0940; // disconnect
            constexpr uint32_t CharacterCreate = 0x006B1620; // create character
            constexpr uint32_t DeleteCharacter = 0x006B1A70; // delete character
            constexpr uint32_t CharacterForceLogout = 0x006B1930; // force logout
            constexpr uint32_t ClearMessageHandler = 0x006B0BC0; // clear handler
            constexpr uint32_t GetErrorToken = 0x006B0F40; // get error token
            constexpr uint32_t SendPacket = 0x00406F40; // send packet
        }

        namespace CClientConnection {
            constexpr uint32_t Send = 0x00926A70; // send packet — **label unverified**: FMOD static-lib socket-send loop (`send()`+`WSAEWOULDBLOCK` retry); both callers are FMOD translation units
        }

        namespace ClientConnection {
            constexpr uint32_t ClientConnection = 0x004650E0; // constructor
            constexpr uint32_t _ClientConnection = 0x00465320; // destructor
            constexpr uint32_t HandleAuthResponse = 0x00464640; // auth response
            constexpr uint32_t HandleCharEnum = 0x00464C10; // character enum
            constexpr uint32_t SendCharEnumOpcode = 0x00464A40; // send char enum
            constexpr uint32_t SendPlayerLoginOpcode = 0x00464AB0; // send player login
            constexpr uint32_t HandleCharacterLoginFailed = 0x00464460; // login failed
            constexpr uint32_t HandleCharacterCreate = 0x00464780; // character create
            constexpr uint32_t HandleClientCacheVersion = 0x00464730; // cache version
            constexpr uint32_t HandleAddonInfo = 0x00464410; // addon info
        }

        namespace ClientLink {
            constexpr uint32_t CMD_AUTH_LOGON_CHALLENGE = 0x008CC3E0; // logon challenge
            constexpr uint32_t CMD_AUTH_LOGON_PROOF = 0x008CC7A0; // logon proof
            constexpr uint32_t CMD_AUTH_RECONNECT_CHALLENGE = 0x008CC980; // reconnect challenge
            constexpr uint32_t CMD_AUTH_RECONNECT_PROOF = 0x008CCA50; // reconnect proof
            constexpr uint32_t CMD_REALM_LIST = 0x008CC250; // realm list
            constexpr uint32_t CMD_XFER_INITIATE = 0x008CCB40; // transfer initiate
            constexpr uint32_t CMD_XFER_DATA = 0x008CCC20; // transfer data
        }

        namespace GruntLogin {
            constexpr uint32_t GruntLogin = 0x008CACA0; // constructor
            constexpr uint32_t vFunc0 = 0x008CA0B0; // vFunc0
            constexpr uint32_t vFunc1 = 0x008CA1D0; // vFunc1
            constexpr uint32_t vFunc2 = 0x008CA140; // vFunc2
            constexpr uint32_t vFunc14 = 0x008CB160; // vFunc14
            constexpr uint32_t vFunc15 = 0x008CB020; // vFunc15
            constexpr uint32_t vFunc18 = 0x008CA4F0; // vFunc18
            constexpr uint32_t vFunc8 = 0x008CAD40; // vFunc8
        }

        namespace BattleNetLogin {
            constexpr uint32_t BattlenetLogin = 0x008C8C00; // constructor
            constexpr uint32_t vFunc0 = 0x008C52E0; // vFunc0
            constexpr uint32_t vFunc18 = 0x008C52D0; // vFunc18
            constexpr uint32_t vFunc10 = 0x008C62B0; // vFunc10
            constexpr uint32_t PatchInstruction = 0x008C6550; // patch instruction
            constexpr uint32_t GameAccount = 0x008C64B0; // game account
        }

        namespace CGChat_C {
            constexpr uint32_t SendMessage = 0x00509DD0; // send chat message
        }
    }
} // namespace WoWOffsets
```

## Packet handlers

```cpp
namespace WoWOffsets {
    namespace Network {
        namespace Packets {
            constexpr uint32_t SMSG_UPDATE_OBJECT = 0x004D73A0; // object update
            constexpr uint32_t SMSG_COMPRESSED_UPDATE_OBJECT = 0x004D74D0; // compressed update
            constexpr uint32_t SMSG_DESTROY_OBJECT = 0x004D7610; // destroy object
            constexpr uint32_t SMSG_COMPRESSED_MOVES = 0x007169A0; // compressed moves
            constexpr uint32_t SMSG_SPELLLOGEXECUTE = 0x006D3220; // spell log execute
            constexpr uint32_t SMSG_SPELLHEALLOG = 0x006D3C30; // spell heal log
            constexpr uint32_t SMSG_SPELLENERGIZELOG = 0x006D3DF0; // spell energize log
            constexpr uint32_t SMSG_SPELLLOGMISS = 0x006D3090; // spell miss log
            constexpr uint32_t SMSG_SPELLNONMELEEDAMAGELOG = 0x006D3C10; // spell damage log
            constexpr uint32_t SMSG_SPELLBREAKLOG = 0x006CE2B0; // spell break log
            constexpr uint32_t SMSG_SPELLINSTAKILLLOG = 0x006CE260; // instant kill log
            constexpr uint32_t SMSG_AURA_UPDATE = 0x007300A0; // aura update
            constexpr uint32_t SMSG_POWER_UPDATE = 0x007236C0; // power update
            constexpr uint32_t SMSG_CHANNEL_NOTIFY = 0x0050E120; // channel notify
            constexpr uint32_t SMSG_CHANNEL_LIST = 0x00505DC0; // channel list
            constexpr uint32_t SMSG_GM_MESSAGECHAT = 0x0050EBC0; // GM message
            constexpr uint32_t SMSG_LOGIN_VERIFY_WORLD = 0x00403DE0; // verify world
            constexpr uint32_t SMSG_NEW_WORLD = 0x00403D10; // new world
            constexpr uint32_t SMSG_TRANSFER_ABORTED = 0x00403910; // transfer aborted
            constexpr uint32_t SMSG_TRANSFER_PENDING = 0x00401480; // transfer pending
            constexpr uint32_t SMSG_LOGIN_SETTIMESPEED = 0x007E28E0; // set time speed
            constexpr uint32_t SMSG_SPELL_GO = 0x0080FEE0; // spell go packet (original row name Packet_PACKET_SMSG_SPELL_GO)
            constexpr uint32_t SMSG_SPELL_FAILURE = 0x00809C70; // spell failure (original row name Packet_PACKET_SMSG_SPELL_FAILURE)
            constexpr uint32_t SMSG_CAST_FAILED = 0x00809AF0; // cast failed (original row name Packet_PACKET_SMSG_CAST_FAILED)
            constexpr uint32_t SMSG_SPELL_COOLDOWN = 0x00806DD0; // cooldown packet (original row name Packet_PACKET_SMSG_SPELL_COOLDOWN)
            constexpr uint32_t SMSG_SPELL_DELAYED = 0x00801B80; // spell delayed (original row name Packet_PACKET_SMSG_SPELL_DELAYED)
            constexpr uint32_t MSG_CHANNEL_START = 0x00801C90; // channel start (original row name Packet_PACKET_MSG_CHANNEL_START)
            constexpr uint32_t MSG_CHANNEL_UPDATE = 0x00801DB0; // channel update (original row name Packet_PACKET_MSG_CHANNEL_UPDATE)
            constexpr uint32_t SMSG_MOVE_KNOCK_BACK = 0x00732450; // knockback packet (Packet_PACKET_SMSG_MOVE_KNOCK_BACK)
            constexpr uint32_t SMSG_WARDEN_DATA = 0x007DA850; // warden data packet
        }
    }
} // namespace WoWOffsets
```
