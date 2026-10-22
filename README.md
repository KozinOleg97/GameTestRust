# GameTestRust

A game on a hexagonal grid: **Bevy 0.20**, Rust edition 2024.

## Requirements

Verified in this environment:

| Component | Version |
|---|---|
| `cargo` | 1.97.1 |
| `rustc` | 1.97.1 |
| Toolchain | `stable-x86_64-pc-windows-msvc` |
| Components | `rustfmt`, `clippy`, `rust-src`, `rust-docs` |

Edition 2024 requires Rust **1.85+**, so 1.97.1 is fine.

## Build and run

```powershell
cargo run                  # debug build
cargo run --release        # release build
```

Profiles in `Cargo.toml` are already tuned for gamedev:
`opt-level = 0` for our own code (fast compilation) and `opt-level = 3` for
all dependencies (fast runtime in dev builds).

### Fast iteration

```powershell
cargo rd      # = cargo run --features dynamic_linking
cargo cd      # = cargo check --features dynamic_linking
```

Bevy is linked as a dynamic library — incremental rebuilds
happen noticeably faster. **Development only**, release must not be built this way.
The first run with this flag will rebuild the dependencies, so it is slow.

`.cargo/config.toml` also enables the `rust-lld` linker for
`x86_64-pc-windows-msvc`: it was already switched on, which cost one full
rebuild of the ~600 Bevy crates.

## Pre-commit checks

```powershell
cargo fmt --all          # formatting
cargo l          # lint  (currently 0 errors, 9 warnings: type_complexity, non_snake_case)
cargo c          # types (currently 0 errors, 1 warning: crate name is not snake_case)
```

Short aliases (see `.cargo/config.toml`): `cargo c`, `cargo l`, `cargo f`, `cargo ft`.

## Controls

| Input | Action |
|---|---|
| `Enter` | start the game from the main menu |
| `Esc` | pause / resume |
| `W` `A` `S` `D` | move the camera, relative to its orientation |
| LMB + drag | pan (the world point under the cursor stays pinned) |
| RMB + drag | rotate the camera around the point under the cursor |
| Mouse wheel | change altitude (dolly, clamped to `zoom_min_altitude` / `zoom_max_altitude`) |
| `F5` `F9` | jump simulation time by 1 hour / 1 day |
| `F6` `F7` `F8` | simulation speed 10x / 1x / toggle pause |
| `F10` | log frustum culling stats, hide the simulation panel |
| `F12` | performance overlay |

The overlay key is configurable: `PerformanceOverlaySettings::toggle_key`.

## Structure

```
src/
├── lib.rs             library crate: declares all modules, so tests/ can reach them
├── main.rs            entry point: window + GamePlugin, nothing else
├── game/              session states, settings, events, main plugin
│   ├── state.rs       GameSessionState (MainMenu/Loading/Active) + PlayState (SubStates)
│   └── settings.rs    GameSettings and settings groups (Reflect + bevy-settings)
├── pointer.rs         pointer occlusion: PointerOcclusion (panel swallows clicks), PointerBlocked
├── generation/        world generation: noise, world_generator
├── hex/               data model: chunks, chunk map, coordinates, hex types
├── rendering/         mesh building, hex math, debug drawing
├── camera/            camera setup and controller (pan/rotate/altitude)
├── simulation/        game clock (SimClock) and simulation ticks
└── ui/                main menu, pause, performance overlay, simulation UI
```

Startup flow: `MainMenu` → `Enter` → `Loading` (world generation on a background thread)
→ `WorldGeneratedEvent` → `Active`. Pause is the substate
`PlayState::Paused`, which lives only inside `GameSessionState::Active`.

- To run the release build, the `assets/` directory must sit next to the `.exe`.