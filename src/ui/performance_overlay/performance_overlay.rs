use crate::game::GameSettings;
use bevy::prelude::*;
use bevy::text::{TextColor, TextFont};
use bevy::ui::{BackgroundColor, Node, PositionType, Val};

// --- Импорты для диагностики Bevy ---
use bevy::diagnostic::{
    DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};

// -----------------------------------------------------------------------------
// Компоненты для UI оверлея
// -----------------------------------------------------------------------------
#[derive(Component)]
pub(crate) struct PerformanceOverlayRoot;

#[derive(Component)]
pub(crate) struct PerformanceText;

// -----------------------------------------------------------------------------
// Вспомогательная функция: получение метрик из DiagnosticsStore
// -----------------------------------------------------------------------------
fn get_performance_metrics(diagnostics: &DiagnosticsStore) -> (f32, f32, u32) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);
    let frame_time_ms = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|d| d.average())
        .unwrap_or(0.0)
        * 1000.0;
    let entities = diagnostics
        .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|d| d.value())
        .unwrap_or(0f64);
    (fps as f32, frame_time_ms as f32, entities as u32)
}

// -----------------------------------------------------------------------------
// Переключение видимости оверлея с сохранением настроек
// -----------------------------------------------------------------------------
pub(crate) fn toggle_overlay_visibility(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_settings: ResMut<GameSettings>,
) {
    let toggle_key: KeyCode = game_settings.performance_overlay.toggle_key.into();
    if keys.just_pressed(toggle_key) {
        game_settings.performance_overlay.visible = !game_settings.performance_overlay.visible;
    }
}

// -----------------------------------------------------------------------------
// Создание / удаление UI оверлея на основе текущих настроек
// -----------------------------------------------------------------------------
pub(crate) fn manage_overlay_ui(
    mut commands: Commands,
    game_settings: Res<GameSettings>,
    diagnostics: Res<DiagnosticsStore>,
    root_query: Query<Entity, With<PerformanceOverlayRoot>>,
    asset_server: Res<AssetServer>,
) {
    let config = &game_settings.performance_overlay;
    let is_spawned = !root_query.is_empty();

    if config.visible && !is_spawned {
        let (x, y) = config.position;
        let font = asset_server.load("fonts/FiraSans-Bold.ttf").into();

        let (fps, frame_time_ms, entities) = get_performance_metrics(&diagnostics);

        let mut root_cmd = commands.spawn((
            PerformanceOverlayRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(x),
                top: Val::Px(y),
                ..default()
            },
        ));

        if let Some(bg_color) = config.background_color {
            root_cmd.insert(BackgroundColor(tuple_to_color(bg_color)));
        }

        root_cmd.with_children(|parent| {
            parent.spawn((
                PerformanceText,
                Text::new(format!(
                    "FPS: {:.1}\nFrame: {:.2} ms\nEntities: {}",
                    fps, frame_time_ms, entities
                )),
                TextFont {
                    font,
                    font_size: FontSize::Px(config.font_size),
                    ..default()
                },
                TextColor(tuple_to_color(config.text_color)),
            ));
        });
        info!("Performance overlay spawned");
    } else if !config.visible && is_spawned {
        for entity in root_query.iter() {
            commands.entity(entity).despawn();
        }
        info!("Performance overlay despawned");
    }
}

// -----------------------------------------------------------------------------
// Обновление текста оверлея (если видим)
// -----------------------------------------------------------------------------
pub(crate) fn update_overlay_text(
    game_settings: Res<GameSettings>,
    diagnostics: Res<DiagnosticsStore>,
    mut text_query: Query<&mut Text, With<PerformanceText>>,
) {
    if !game_settings.performance_overlay.visible {
        return;
    }

    let (fps, frame_time_ms, entities) = get_performance_metrics(&diagnostics);

    for mut text in text_query.iter_mut() {
        text.0 = format!(
            "FPS: {:.1}\nFrame: {:.2} ms\nEntities: {}",
            fps, frame_time_ms, entities
        );
    }
}

// -----------------------------------------------------------------------------
// Вспомогательная функция: кортеж (r,g,b,a) -> Color
// -----------------------------------------------------------------------------
fn tuple_to_color(rgba: (f32, f32, f32, f32)) -> Color {
    Color::srgba(rgba.0, rgba.1, rgba.2, rgba.3)
}
