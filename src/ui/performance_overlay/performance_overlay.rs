use crate::game::GameSettings;
use bevy::prelude::*;
use bevy::text::{TextColor, TextFont};
use bevy::ui::{BackgroundColor, Node, PositionType, Val};
use bevy_settings_lib::{PersistSetting, ReloadSetting};
use std::collections::VecDeque;


// -----------------------------------------------------------------------------
// Компоненты для UI оверлея
// -----------------------------------------------------------------------------
#[derive(Component)]
struct PerformanceOverlayRoot;

#[derive(Component)]
struct PerformanceText;

// -----------------------------------------------------------------------------
// Метрики производительности (без изменений)
// -----------------------------------------------------------------------------
#[derive(Resource)]
pub struct PerformanceMetrics {
    frame_times: VecDeque<f32>,
    fps: f32,
    accumulated_time: f32,
    accumulated_frames: usize,
    last_frame_time: f32,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(120),
            fps: 0.0,
            accumulated_time: 0.0,
            accumulated_frames: 0,
            last_frame_time: 0.0,
        }
    }
}

impl PerformanceMetrics {
    pub fn update(&mut self, delta_seconds: f32) {
        self.last_frame_time = delta_seconds;
        self.frame_times.push_back(delta_seconds);
        if self.frame_times.len() > 120 {
            self.frame_times.pop_front();
        }
        const SMOOTHING: f32 = 0.05;
        let current_fps = 1.0 / delta_seconds.max(0.0001);
        self.fps = self.fps * (1.0 - SMOOTHING) + current_fps * SMOOTHING;
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn frame_time_ms(&self) -> f32 {
        self.last_frame_time * 1000.0
    }

    pub fn average_frame_time_ms(&self, n: usize) -> f32 {
        let count = self.frame_times.len().min(n);
        if count == 0 {
            return 0.0;
        }
        let sum: f32 = self.frame_times.iter().rev().take(count).sum();
        (sum / count as f32) * 1000.0
    }
}

// -----------------------------------------------------------------------------
// Система обновления метрик
// -----------------------------------------------------------------------------
pub(crate) fn update_performance_metrics(time: Res<Time>, mut metrics: ResMut<PerformanceMetrics>) {
    metrics.update(time.delta_secs());
}

// -----------------------------------------------------------------------------
// Переключение видимости оверлея с сохранением настроек
// -----------------------------------------------------------------------------
pub(crate) fn toggle_overlay_visibility(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_settings: ResMut<GameSettings>,
    mut commands: Commands,
) {
    let toggle_key: KeyCode = game_settings.performance_overlay.toggle_key.clone().into();
    if keys.just_pressed(toggle_key) {
        game_settings.performance_overlay.visible = !game_settings.performance_overlay.visible;
        commands.trigger(PersistSetting::<GameSettings> { value: None });
        info!(
            "Performance overlay visibility toggled: {}",
            game_settings.performance_overlay.visible
        );
    }
}

// -----------------------------------------------------------------------------
// Создание / удаление UI оверлея на основе текущих настроек
// -----------------------------------------------------------------------------
fn manage_overlay_ui(
    mut commands: Commands,
    game_settings: Res<GameSettings>,
    metrics: Res<PerformanceMetrics>,
    root_query: Query<Entity, With<PerformanceOverlayRoot>>,
    asset_server: Res<AssetServer>,
) {
    let config = &game_settings.performance_overlay;
    let is_spawned = !root_query.is_empty();

    if config.visible && !is_spawned {
        let (x, y) = config.position;
        // Загружаем шрифт (убедитесь, что путь правильный, или используйте системный)
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");

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
                    "FPS: {:.1}\nFrame: {:.2} ms",
                    metrics.fps(),
                    metrics.frame_time_ms()
                )),
                TextFont {
                    font,
                    font_size: config.font_size,
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
    metrics: Res<PerformanceMetrics>,
    mut text_query: Query<&mut Text, With<PerformanceText>>,
) {
    if !game_settings.performance_overlay.visible {
        return;
    }
    for mut text in text_query.iter_mut() {
        text.0 = format!(
            "FPS: {:.1}\nFrame: {:.2} ms",
            metrics.fps(),
            metrics.frame_time_ms()
        );
    }
}

// -----------------------------------------------------------------------------
// Вспомогательная функция: кортеж (r,g,b,a) -> Color
// -----------------------------------------------------------------------------
fn tuple_to_color(rgba: (f32, f32, f32, f32)) -> Color {
    Color::srgba(rgba.0, rgba.1, rgba.2, rgba.3)
}

// -----------------------------------------------------------------------------
// Наблюдатель: пересоздаём оверлей после перезагрузки настроек из файла
// -----------------------------------------------------------------------------

fn reload_overlay_on_settings_reload(
    _event: On<ReloadSetting<GameSettings>>,
    mut commands: Commands,
    root_query: Query<Entity, With<PerformanceOverlayRoot>>,
) {
    for entity in root_query.iter() {
        commands.entity(entity).despawn();
    }
}

// -----------------------------------------------------------------------------
// Плагин
// -----------------------------------------------------------------------------
pub struct PerformanceOverlayPlugin;

impl Plugin for PerformanceOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PerformanceMetrics>()
            .add_systems(
                Update,
                (
                    update_performance_metrics,
                    toggle_overlay_visibility,
                    manage_overlay_ui,
                    update_overlay_text,
                )
                    .chain(),
            )
            .add_observer(reload_overlay_on_settings_reload);
    }
}
