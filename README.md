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
cargo run --features dynamic_linking
```

Bevy is linked as a dynamic library — incremental rebuilds
happen noticeably faster. **Development only**, release must not be built this way.
The first run with this flag will rebuild the dependencies, so it is slow.

## Pre-commit checks

```powershell
cargo fmt --all          # formatting
cargo clippy --all-targets   # lint (currently 0 errors, 52 warnings)
cargo check --all-targets    # types (currently 0 errors, 29 warnings)
```

Short aliases (see `.cargo/config.toml`): `cargo c`, `cargo l`, `cargo f`, `cargo ft`.

## Controls

| Input | Action |
|---|---|
| `Enter` | start the game from the main menu |
| `Esc` | pause / resume |
| `W` `A` `S` `D` | move the camera |
| LMB + drag | pan |
| RMB + drag | rotate the camera around the point under the cursor |
| Mouse wheel | zoom (via FOV) |
| `F12` | performance overlay |

The overlay key is configurable: `PerformanceOverlaySettings::toggle_key`.

## Structure

```
src/
├── main.rs            entry point, window + GamePlugin
├── game/              session states, settings, events, main plugin
│   ├── state.rs       GameSessionState (MainMenu/Loading/Active) + PlayState (SubStates)
│   └── settings.rs    GameSettings and settings groups (Reflect + bevy-settings)
├── generation/        world generation: noise, world_generator
├── hex/               data model: chunks, chunk map, coordinates, hex types
├── rendering/         mesh building, hex math, debug drawing
├── camera/            camera setup and controller (pan/rotate/zoom)
├── simulation/        game clock (SimClock) and simulation ticks
└── ui/                main menu, pause, performance overlay, simulation UI
```

Startup flow: `MainMenu` → `Enter` → `Loading` (world generation on a background thread)
→ `WorldGeneratedEvent` → `Active`. Pause is the substate
`PlayState::Paused`, which lives only inside `GameSessionState::Active`.

## Note about artifacts

- The `target/` directory takes **~51 GB**.
- `target/release/New folder/` is the **packaged 0.1.5 release**
  (`GameTestRust.exe`, `assets/`, `v1_release.7z`), not junk.
  `cargo clean` will delete it — copy it beforehand if needed.
- To run the release build, the `assets/` directory must sit next to the `.exe`.