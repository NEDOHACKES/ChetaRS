# ChetaRS Spatial GUI

A client-side native Horizon mod written in Rust.

## What it does

- Opens with F6.
- Shows a floating 3D-style panel over the world.
- Uses Horizon's native screen API and GUI depth buffer.
- Adds perspective/skew based on the mouse position.
- Has three clickable menu buttons.
- Escape closes the menu.
- The panel is intentionally visual-only; it does not change gameplay.

## Horizon

This targets Horizon main at:
3cfcd1e548b395dc903a84e24249ce3d32c5b12b

Native mods are full native code and are not sandboxed. Read the source before installing/trusting it.

## Development

The official Horizon native-mod workflow uses horizon-mixins:

    horizon-launcher native-build ./ --trust-native-code

For the fastest iteration, use Horizon's Dev native build profile.

The mod can also be installed from GitHub using Horizon's Add from GitHub/native-mod workflow once a release is published.
