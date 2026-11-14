# ChetaRS Spatial GUI

A client-side native Rust mod for the Horizon Nightly build shown in the user's launcher (2026-11-13).

## What it does

- Opens and closes with F6.
- Draws a floating panel as actual camera-facing geometry in the Minecraft world.
- Uses Horizon's native `world_submit` / world-canvas rendering path, not the 2D HUD renderer.
- The panel uses world depth testing and is drawn before the first-person hand, so nearby world geometry can occlude it.
- Keeps three selectable buttons: HOME, PROFILE and SETTINGS.
- Escape closes the menu.
- The current buttons are visual/navigation placeholders; they do not change gameplay yet.

## Horizon compatibility

Pinned to Horizon commit:

`9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0`

The menu's world-canvas hook is available in this source revision. The launcher compiles native mods against the Horizon source revision used by the instance.

## Build

Use Horizon's Native Mods build screen and select the Dev build profile, or run the native build command supplied by your Horizon installation.

Native mods execute as native code with the permissions of the game process. Review the source before installing or trusting it.
