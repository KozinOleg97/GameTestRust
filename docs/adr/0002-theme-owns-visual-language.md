# The visual language lives in the theme; settings store only preferences

Colors, sizes and the interface font are set by the `UiTheme` resource. Settings keep only what the player chooses: visibility, position and refresh rate of the performance overlay, and its toggle key.

`font_size`, `text_color` and `background_color` were removed from `PerformanceOverlaySettings`. The reason is that before unification these same values were also set by constants in `style.rs` and by literals right in screen code — three sources of truth for a single color. Fields that are already in the user's settings file are now ignored.

## Consequences

The overlay looks the same as the rest of the interface: color and size come from the theme, not from its own set of fields. If later there is a need to customize the overlay's appearance, it will be a deliberate override mechanism, not accidental legacy.