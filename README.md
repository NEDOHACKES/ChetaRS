# ChetaRS Spatial GUI

A client-side native Rust mod for the Horizon Nightly build shown in the user's launcher (2026-11-13), package version 0.5.0.

## Spatial display

- Press **F6** to show or hide the floating display.
- The panel is a floating 3D display drawn through Horizon's native world-canvas API.
- The content area is a white canvas with the mod's function catalogue, placement profile and settings rendered over it. No player inventory, container slots, item stacks or inventory texture are used.
- The display has a dark frame, side navigation, shadow and animated settings switches.

## Position and movement

- **Keep** preserves the panel's world position and the exact orientation it had when placed. It does not rotate to follow the player.
- **Follow** tracks the player's position at the saved world-relative offset and turns the face toward the player.
- Hold **Right Mouse Button** on a panel's top title bar and turn the camera to move that panel around the player. Each panel moves independently.
- Scroll the wheel while aiming at a panel's title bar to change its distance. In Settings, the **− / +** controls and keyboard keys change the selected panel's distance.

## Multiple panels

- Press the **+** button in a panel's upper-right corner to create another panel.
- Up to **10 panels** can exist at once. Each panel has its own position, orientation, Follow/Keep choice and distance.
- New panels are placed at separate angles around the player. Aim at a panel before using its controls to select and configure that one.

## Sections and settings

- Aim at **HOME**, **PROFILE**, or **SETTINGS** in the left navigation and left-click.
- **SETTINGS → Follow / Keep** opens a small list with the two choices. They are mutually exclusive for the selected panel.
- **2D Panel** is a separate setting. It hides all world-space 3D panels and shows a screen-space 2D window. The mouse cursor is released while 2D mode is active; switching back to 3D restores mouse-look.
- **F7** also switches between 2D and 3D while the panel is open.
- **F6** shows or hides the interface; **Escape** closes it.

## Visual reference

The floating-screen presentation is inspired by [Spatial-GUI](https://github.com/tastytrash/Spatial-GUI).

ChetaRS is a native Rust implementation for Horizon. It does not port Spatial-GUI's Java mixins, screen capture pipeline or inventory UI. The central content is a newly drawn white world-space canvas with the mod's catalogue and controls layered on top.

## Horizon compatibility

Pinned to Horizon commit:

`9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0`

The renderer uses Horizon's native `world_submit` / world-canvas hook and the public `horizon/white` atlas sprite.

## Build

Use Horizon's Native Mods build screen and select the Dev build profile, or run the native build command supplied by your Horizon installation.

Native mods execute as native code with the permissions of the game process. Review the source before installing or trusting it.
