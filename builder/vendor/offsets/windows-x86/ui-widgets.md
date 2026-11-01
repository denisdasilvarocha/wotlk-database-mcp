# UI widgets and fonts

Read for native frame, texture, font-string, button, tooltip, and font-rendering addresses. Search `CSimpleFrame`, `Font`, `LuaFrameMethods`, `LuaTextureMethods`, `LuaFontStringMethods`, or `LuaButtonMethods`. Lua state and stack functions are in [Lua and FrameScript](lua.md).

Catalog provenance: [Windows x86, build 12340](windows-x86.md). Use the shared [address conventions and verification guidance](address-conventions.md) when applying these constants.

## Widget functions

```cpp
namespace WoWOffsets {
    namespace UI {
        namespace CSimpleFrame {
            constexpr uint32_t LoadXML = 0x004932C0; // load XML
            constexpr uint32_t OnFrameRender = 0x00490840; // frame render
        }

        namespace CSimpleTop {
            constexpr uint32_t CSimpleTop = 0x00495EB0; // constructor
        }

        namespace CSimpleUI {
            constexpr uint32_t CreateTooltip = 0x00621070; // create tooltip
        }

        namespace CSimpleFontString {
            constexpr uint32_t UpdateString = 0x00486B20; // update font string
        }

        namespace Font {
            constexpr uint32_t GxuFontCreateFont = 0x006BDE80; // create font
            constexpr uint32_t TextBlockGenerateFont = 0x004BE9C0; // generate font block
            constexpr uint32_t TextBlockCreate = 0x004BDAA0; // create text block
        }

        namespace LuaFrameMethods {
            constexpr uint32_t SetPoint = 0x0049DB10; // script method %s:SetPoint - set anchor point
            constexpr uint32_t SetWidth = 0x0049D480; // script method %s:SetWidth - set width
            constexpr uint32_t SetHeight = 0x0049D620; // script method %s:SetHeight - set height
            constexpr uint32_t SetSize = 0x0049D7A0; // script method %s:SetSize - set size
            constexpr uint32_t SetAlpha_0 = 0x0049F9E0; // script method %s:SetAlpha_0 - set alpha
            constexpr uint32_t SetID = 0x0049FB00; // script method %s:SetID - set frame ID
            constexpr uint32_t SetScale = 0x0049F820; // script method %s:SetScale - set scale
            constexpr uint32_t SetFrameStrata = 0x0049E880; // script method %s:SetFrameStrata - set strata
            constexpr uint32_t SetFrameLevel = 0x0049E9D0; // script method %s:SetFrameLevel - set level
            constexpr uint32_t HookScript = 0x0049EDB0; // script method %s:HookScript - hook script
            constexpr uint32_t RegisterEvent = 0x0049EFE0; // script method %s:RegisterEvent - register event
            constexpr uint32_t UnregisterEvent = 0x0049F060; // script method %s:UnregisterEvent - unregister event
            constexpr uint32_t IsEventRegistered = 0x0049F160; // script method %s:IsEventRegistered - check event
            constexpr uint32_t GetAttribute = 0x0049F2D0; // script method %s:GetAttribute - get attribute
            constexpr uint32_t SetAttribute = 0x0049F610; // script method %s:SetAttribute - set attribute
            constexpr uint32_t SetHitRectInsets = 0x004A0090; // script method %s:SetHitRectInsets - set hit rect
            constexpr uint32_t SetClampRectInsets = 0x004A0310; // script method %s:SetClampRectInsets - set clamp rect
            constexpr uint32_t SetMinResize = 0x004A0520; // script method %s:SetMinResize - set min resize
            constexpr uint32_t SetMaxResize = 0x004A06E0; // script method %s:SetMaxResize - set max resize
            constexpr uint32_t SetBackdrop = 0x004A15A0; // script method %s:SetBackdrop - set backdrop
        }

        namespace LuaTextureMethods {
            constexpr uint32_t SetBlendMode = 0x0048C040; // script method %s:SetBlendMode - set blend mode
            constexpr uint32_t SetGradient = 0x0048C240; // script method %s:SetGradient - set gradient
            constexpr uint32_t SetGradientAlpha = 0x0048C310; // script method %s:SetGradientAlpha - set gradient alpha
            constexpr uint32_t SetAlpha_1 = 0x0048C3E0; // script method %s:SetAlpha_1 - set alpha (texture)
            constexpr uint32_t SetRotation = 0x0048C860; // script method %s:SetRotation - set rotation
            constexpr uint32_t SetTexCoord = 0x0048E170; // script method %s:SetTexCoord - set texture coords
        }

        namespace LuaFontStringMethods {
            constexpr uint32_t SetAlpha_0_0 = 0x0048D030; // script method %s:SetAlpha_0_0 - set alpha
            constexpr uint32_t SetAlphaGradient = 0x0048D0F0; // script method %s:SetAlphaGradient - set alpha gradient
            constexpr uint32_t SetFontObject = 0x0048D380; // script method %s:SetFontObject - set font object
            constexpr uint32_t SetFont = 0x0048D5B0; // script method %s:SetFont - set font
            constexpr uint32_t SetShadowOffset = 0x0048DBB0; // script method %s:SetShadowOffset - set shadow offset
            constexpr uint32_t SetSpacing = 0x0048DD00; // script method %s:SetSpacing - set spacing
            constexpr uint32_t SetTextHeight = 0x0048DDB0; // script method %s:SetTextHeight - set text height
            constexpr uint32_t SetJustifyH = 0x0048E360; // script method %s:SetJustifyH - set horizontal justify
            constexpr uint32_t SetJustifyV = 0x0048E430; // script method %s:SetJustifyV - set vertical justify
        }

        namespace LuaButtonMethods {
            constexpr uint32_t SetButtonState = 0x00977280; // script method %s:SetButtonState - set button state
            constexpr uint32_t SetNormalFontObject = 0x00977340; // script method %s:SetNormalFontObject - set normal font
            constexpr uint32_t SetDisabledFontObject = 0x009774C0; // script method %s:SetDisabledFontObject - set disabled font
            constexpr uint32_t SetHighlightFontObject = 0x00977640; // script method %s:SetHighlightFontObject - set highlight font
            constexpr uint32_t SetHighlightTexture = 0x00977D30; // script method %s:SetHighlightTexture - set highlight texture
            constexpr uint32_t SetPushedTextOffset = 0x00977F60; // script method %s:SetPushedTextOffset - set pushed offset
        }
    }
} // namespace WoWOffsets
```
