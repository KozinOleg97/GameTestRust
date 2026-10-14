use crate::game::GameSettings;
use crate::pointer::PointerOcclusion;
use crate::ui::theme::UiTheme;
use bevy::diagnostic::{
    DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
    SystemInformationDiagnosticsPlugin,
};
use bevy::prelude::*;

/// Маркер корня оверлея.
#[derive(Component)]
struct PerformanceOverlayRoot;

/// Маркер текста оверлея.
#[derive(Component)]
struct PerformanceText;

pub struct PerformanceOverlayPlugin;

impl Plugin for PerformanceOverlayPlugin {
    fn build(&self, app: &mut App) {
        // Добавляем встроенные диагностические плагины Bevy
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_plugins(EntityCountDiagnosticsPlugin::default());
        // Плагин для отслеживания CPU и RAM
        app.add_plugins(SystemInformationDiagnosticsPlugin);

        // HUD создаётся один раз при старте, поэтому в Update остаётся только
        // переключение флага, синхронизация видимости и обновление текста.
        app.add_systems(Startup, spawn_overlay_hud);
        app.add_systems(
            Update,
            (
                toggle_overlay_visibility,
                sync_overlay_visibility,
                update_overlay_text,
            )
                .chain(),
        );
    }
}

// -----------------------------------------------------------------------------
// Вспомогательная функция: получение метрик из DiagnosticsStore
// -----------------------------------------------------------------------------
fn get_performance_metrics(diagnostics: &DiagnosticsStore) -> (f32, f32, u32, f64) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.average())
        .unwrap_or(0.0);
    let frame_time_ms = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|d| d.average())
        .unwrap_or(0.0);
    let entities = diagnostics
        .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|d| d.value())
        .unwrap_or(0f64);

    // Получение использования памяти в ГИБИБАЙТАХ (так отдаёт Bevy)
    let process_mem_gib = diagnostics
        .get(&SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE)
        .and_then(|d| d.value())
        .unwrap_or(0.0);

    // Переводим в мегабайты (1 GiB = 1024 MiB)
    let process_mem_mb = process_mem_gib * 1024.0;

    (
        fps as f32,
        frame_time_ms as f32,
        entities as u32,
        process_mem_mb,
    )
}

// -----------------------------------------------------------------------------
// Переключение видимости оверлея с сохранением настроек
// -----------------------------------------------------------------------------
fn toggle_overlay_visibility(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_settings: ResMut<GameSettings>,
) {
    let toggle_key: KeyCode = game_settings.performance_overlay.toggle_key.into();
    if keys.just_pressed(toggle_key) {
        game_settings.performance_overlay.visible = !game_settings.performance_overlay.visible;
    }
}

// -----------------------------------------------------------------------------
// Создание HUD оверлея — один раз при старте приложения
// -----------------------------------------------------------------------------
/// Долгоживущий HUD: создаётся один раз и дальше только переключает видимость.
///
/// Время жизни панели привязано к её контексту: у оверлея контекст — приложение,
/// у панелей симуляции и свойств гекса — игровая сессия. Поэтому оверлей живёт
/// вне состояний, а панели приходят и уходят вместе с состоянием `Active`.
fn spawn_overlay_hud(
    mut commands: Commands,
    theme: Res<UiTheme>,
    game_settings: Res<GameSettings>,
) {
    let config = &game_settings.performance_overlay;
    let (x, y) = config.position;

    commands.spawn((
        PerformanceOverlayRoot,
        PerformanceText,
        Text::new(""),
        theme.text_font(theme.font_button),
        TextColor(theme.text),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(x),
            top: Val::Px(y),
            ..default()
        },
        BackgroundColor(theme.overlay_background),
        // Отладочная панель прозрачна для указателя: у неё нет интерактивных
        // элементов, и она не должна мешать смотреть свойства гексов под собой.
        PointerOcclusion::Transparent,
        if config.visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        },
    ));

    info!("Performance overlay hud spawned");
}

// -----------------------------------------------------------------------------
// Видимость HUD следует за настройкой
// -----------------------------------------------------------------------------
fn sync_overlay_visibility(
    game_settings: Res<GameSettings>,
    mut query: Query<&mut Visibility, With<PerformanceOverlayRoot>>,
) {
    let target = if game_settings.performance_overlay.visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    for mut visibility in &mut query {
        if *visibility != target {
            *visibility = target;
        }
    }
}

// -----------------------------------------------------------------------------
// Обновление текста оверлея (если видим)
// -----------------------------------------------------------------------------
fn update_overlay_text(
    time: Res<Time>,
    mut elapsed: Local<f32>,
    game_settings: Res<GameSettings>,
    diagnostics: Res<DiagnosticsStore>,
    mut text_query: Query<&mut Text, With<PerformanceText>>,
) {
    if !game_settings.performance_overlay.visible {
        return;
    }

    // Получаем интервал из настроек (защита от значений <= 0)
    let interval = game_settings
        .performance_overlay
        .update_interval_secs
        .max(0.01);

    // Накапливаем время, прошедшее с прошлого кадра
    *elapsed += time.delta_secs();

    // Если прошло достаточно времени, обновляем текст
    if *elapsed >= interval {
        *elapsed = 0.0; // Сбрасываем таймер

        let (fps, frame_time_ms, entities, mem_mb) = get_performance_metrics(&diagnostics);
        for mut text in &mut text_query {
            text.0 = format!(
                "FPS: {:.1}\nFrame time: {:.2} ms\nEntities: {}\nRAM: {:.2} MB",
                fps, frame_time_ms, entities, mem_mb
            );
        }
    }
}
