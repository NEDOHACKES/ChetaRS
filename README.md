# ChetaRS Spatial GUI

A client-side native Rust mod for the Horizon Nightly build shown in the user's launcher (2026-11-13), package version 0.4.1.

## Spatial display

- Press **F6** to show or hide the floating display.
- The panel is a floating 3D display drawn through Horizon's native world-canvas API.
- The content area is a white canvas with the mod's function catalogue, placement profile and settings rendered over it. No player inventory, container slots, item stacks or inventory texture are used.
- The display has a dark frame, side navigation, shadow and animated settings switches.

## Position and movement

- **Keep at coordinates** is enabled by default. The panel is placed in the world when first shown and then stays at that world position and orientation while you turn your head or walk.
- Aim at the panel's top title bar, then hold **Right Mouse Button** and turn the camera to orbit the panel around you horizontally and vertically. Release RMB to leave it at the new world position. The panel turns its front face toward the player while orbiting.
- Scroll the mouse wheel while aiming at the top bar to move the panel closer or farther away. In Settings, the **- / +** controls and keyboard keys also change distance.
- The panel is rendered on both sides with readable text from the front and back.
- **Follow me** follows the player's position while preserving the panel's chosen offset/orbit from when the mode was enabled. The front face turns toward the player so the controls remain accessible. **Keep at coordinates** leaves the panel's position fixed in the world after dragging, while still turning its face toward the player.
- The two position modes are mutually exclusive.

## Sections and controls

- Aim the crosshair at **HOME**, **PROFILE**, or **SETTINGS** in the left navigation and left-click to open that page.
- **HOME** shows the function catalogue; **PROFILE** shows the active placement mode, distance and movement controls.
- **SETTINGS** contains the mutually exclusive **Keep at coordinates** / **Follow me** switches and distance controls.
- When following the camera, use **Left / Right Arrow** to switch sections and **Up / Down + Enter** to change placement mode; **- / +** changes distance.

## Settings

Select **SETTINGS** by pressing **Right Arrow** twice from HOME (Left / Right cycle through the three navigation tabs).
- **Up / Down** selects one of the two setting rows.
- **Enter** toggles the selected row.
- You can also aim the crosshair at a setting row and left-click it.
- Switch knobs animate smoothly between off and on.

**F6** hides the panel; **Escape** also hides it (the vanilla pause menu may open as well).

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
