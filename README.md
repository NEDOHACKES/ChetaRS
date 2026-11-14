# ChetaRS Spatial GUI

A client-side native Rust mod for the Horizon Nightly build shown in the user's launcher (2026-11-13).

## Interface

- Press **F6** to open or close the floating display.
- The interface is a camera-facing 3D screen drawn with Horizon's native world-canvas API.
- The display has a dark frame, side navigation, shadow and a large **blank white content area**.
- HOME, PROFILE and SETTINGS are visual navigation tabs.
- The white area is intentionally empty. It does not show the player inventory, container slots, item stacks or inventory texture.
- Clicks inside the spatial panel are consumed by the custom menu so they are not forwarded to Minecraft inventory slots.
- Press **Escape** to close the menu.

## Visual reference

The floating-screen presentation is inspired by [Spatial-GUI](https://github.com/tastytrash/Spatial-GUI), which presents existing Minecraft screens as immersive displays in the world.

ChetaRS is a native Rust implementation for Horizon. It does not port Spatial-GUI's Java mixins, screen capture pipeline or inventory UI. The central content is a newly drawn white world-space quad instead.

## Horizon compatibility

Pinned to Horizon commit:

`9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0`

The renderer uses Horizon's native `world_submit` / world-canvas hook and the public `horizon/white` atlas sprite.

## Build

Use Horizon's Native Mods build screen and select the Dev build profile, or run the native build command supplied by your Horizon installation.

Native mods execute as native code with the permissions of the game process. Review the source before installing or trusting it.
