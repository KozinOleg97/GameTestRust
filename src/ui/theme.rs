use bevy::prelude::*;

/// Единый источник визуального языка интерфейса: цвета, размеры и шрифт.
///
/// До унификации эти значения жили в трёх местах: константы в `style.rs`,
/// литералы прямо в коде экранов и поля `PerformanceOverlaySettings` в
/// настройках. Теперь дизайн принадлежит теме, а настройки хранят только
/// пользовательские предпочтения — видимость, позицию и интервал обновления.
#[derive(Resource)]
pub struct UiTheme {
    // --- Цвета ---
    pub button_normal: Color,
    pub button_hovered: Color,
    pub button_pressed: Color,
    pub text: Color,
    pub menu_background: Color,
    pub panel_background: Color,
    pub overlay_background: Color,
    pub progress_track: Color,
    pub progress_fill: Color,

    // --- Размеры кнопок экранов ---
    pub button_width: f32,
    pub button_height: f32,
    pub button_margin_top: f32,

    // --- Размеры элементов HUD ---
    pub hud_button_width: f32,
    pub hud_button_height: f32,
    pub progress_bar_width: f32,
    pub progress_bar_height: f32,
    pub hex_panel_width: f32,

    // --- Шкала шрифтов ---
    pub font_title: f32,
    pub font_heading: f32,
    pub font_button: f32,
    pub font_body: f32,
    pub font_small: f32,
    pub font_tiny: f32,

    /// Шрифт проекта. Грузится один раз здесь, а не по месту использования.
    pub font: Handle<Font>,
}

impl FromWorld for UiTheme {
    fn from_world(world: &mut World) -> Self {
        let font: Handle<Font> = world
            .resource::<AssetServer>()
            .load("fonts/FiraSans-Bold.ttf");

        Self {
            button_normal: Color::srgb(0.15, 0.15, 0.15),
            button_hovered: Color::srgb(0.25, 0.25, 0.25),
            button_pressed: Color::srgb(0.35, 0.75, 0.35),
            text: Color::WHITE,
            menu_background: Color::srgba(0.0, 0.0, 0.0, 0.7),
            panel_background: Color::srgba(0.0, 0.0, 0.0, 0.85),
            overlay_background: Color::srgba(0.0, 0.0, 0.0, 0.5),
            progress_track: Color::srgba(0.25, 0.25, 0.25, 1.0),
            progress_fill: Color::srgb(0.2, 0.85, 0.3),

            button_width: 200.0,
            button_height: 50.0,
            button_margin_top: 20.0,

            hud_button_width: 42.0,
            hud_button_height: 30.0,
            progress_bar_width: 420.0,
            progress_bar_height: 18.0,
            hex_panel_width: 220.0,

            font_title: 48.0,
            font_heading: 28.0,
            font_button: 24.0,
            font_body: 22.0,
            font_small: 18.0,
            font_tiny: 16.0,

            font,
        }
    }
}

impl UiTheme {
    /// `TextFont` в шрифте проекта заданного размера.
    pub fn text_font(&self, size: f32) -> TextFont {
        TextFont {
            font: self.font.clone().into(),
            font_size: FontSize::Px(size),
            ..default()
        }
    }
}
