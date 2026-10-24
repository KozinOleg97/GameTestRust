# UI is built on `ui_widgets` and our own kit

The trigger for revisiting the interface layer was the move to Bevy 0.20: `Interaction` and `ui::widgets::Button` became unusable for external crates there, and the UI had to be rewritten anyway. We decided to assemble it from headless `ui_widgets` widgets (they are part of Bevy's standard features) plus a thin project kit: `UiTheme` for appearance, `ui/kit.rs` for spawning nodes, `ui/actions.rs` for actions.

Feathers was rejected because its API changes from release to release — in 0.20 the theme system was rewritten around semantic tokens — and it imposes an "editor" look that would have to be adapted to the game. BSN was rejected as unstable: its syntax changed within the 0.19 → 0.20 transition, and there are few examples for game UI. Neither option is forbidden: Feathers can be adopted pointwise for individual widgets such as a slider, BSN — later, and only on the structure axis.

## Consequences

A panel's lifetime is tied to its context: the performance overlay lives for the whole application and toggles visibility, while the simulation panel and the hex properties panel live exactly as long as the `Active` state and die via `DespawnOnExit`. The folder structure mirrors this: `ui/screens/` for things that live by enter-exit states, `ui/hud/` for panels over the game.

Before unification there were three ways for UI to appear and disappear, and one of them was recreating nodes on key press.