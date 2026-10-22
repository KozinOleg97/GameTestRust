# GameTestRust

A game on a hexagonal grid: procedural world generation, time simulation, and an interface over a 3D scene.

## Language

**Screen**:
A full-screen interface layer that exists while the game is in the corresponding mode: main menu, loading, pause.
_Avoid_: page, window, scene

**HUD**:
A persistent layer over the game world: counters and panels not tied to a specific game mode.
_Avoid_: overlay (in the general sense), widget, panel

**Simulation panel**:
A HUD with game time, speed and time-control buttons.
_Avoid_: time panel, time UI, time bar

**Performance overlay**:
A HUD with frame, memory and entity-count metrics.
_Avoid_: FPS counter, overlay (without qualification)

**Hex properties panel**:
A HUD with hex data: coordinates, biome, elevation, movement cost.
_Avoid_: inspector, info panel, tooltip

**Hover**:
The hex under the cursor. A preview: recomputed every frame and disappears as soon as the mouse moves away. A click on the hover produces the selection.
_Avoid_: focus, active hex, highlight (as a state name)

**Selection**:
A hex pinned by a click. Lives until the next click on another hex; actions on objects will be attached to it.
_Avoid_: focus, active hex, current target