use crate::game::GameSessionState;
use crate::simulation::{SimClock, TimeJumpProgress, TimeJumpState};
use crate::ui::actions::UiAction;
use crate::ui::kit::{spawn_hud_button, spawn_label, spawn_progress_bar};
use crate::ui::theme::UiTheme;
use bevy::ecs::system::ParamSet;
use bevy::prelude::*;

#[derive(Component)]
struct SimulationUIRoot;

#[derive(Component)]
struct TimeDisplayText;

#[derive(Component)]
struct SpeedDisplayText;

#[derive(Component)]
struct TimeJumpRoot;

#[derive(Component)]
struct TimeJumpStatusText;

#[derive(Component)]
struct TimeJumpDateText;

#[derive(Component)]
struct TimeJumpProgressFill;

pub struct SimulationUIPlugin;

impl Plugin for SimulationUIPlugin {
    fn build(&self, app: &mut App) {
        // Кнопки панели больше не обрабатываются отдельной системой: они несут
        // `UiAction`, а разбирает их единый обработчик в `ui::actions`.
        app.add_systems(OnEnter(GameSessionState::Active), spawn_simulation_ui)
            .add_systems(OnEnter(TimeJumpState::InProgress), spawn_time_jump_ui)
            .add_systems(
                Update,
                (update_time_display, toggle_ui_visibility)
                    .chain()
                    .run_if(in_state(GameSessionState::Active))
                    .run_if(in_state(TimeJumpState::Inactive)),
            )
            .add_systems(
                Update,
                update_time_jump_ui
                    .run_if(in_state(GameSessionState::Active))
                    .run_if(in_state(TimeJumpState::InProgress)),
            );
    }
}

fn spawn_simulation_ui(mut commands: Commands, theme: Res<UiTheme>) {
    let theme = &*theme;

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                bottom: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                row_gap: Val::Px(5.0),
                ..default()
            },
            DespawnOnExit(GameSessionState::Active),
            Visibility::Visible,
            SimulationUIRoot,
        ))
        .with_children(|parent| {
            spawn_label(
                parent,
                theme,
                "Time: 00:00:00",
                theme.font_small,
                TimeDisplayText,
            );

            spawn_label(
                parent,
                theme,
                "Speed: 1x",
                theme.font_small,
                SpeedDisplayText,
            );

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(5.0),
                    ..default()
                })
                .with_children(|buttons| {
                    spawn_hud_button(buttons, theme, "⏸", UiAction::PauseToggle);
                    spawn_hud_button(buttons, theme, "1x", UiAction::SetSpeed(1.0));
                    spawn_hud_button(buttons, theme, "2x", UiAction::SetSpeed(2.0));
                    spawn_hud_button(buttons, theme, "5x", UiAction::SetSpeed(5.0));
                    spawn_hud_button(buttons, theme, "10x", UiAction::SetSpeed(10.0));
                    spawn_hud_button(buttons, theme, "+1h", UiAction::JumpHours(1));
                    spawn_hud_button(buttons, theme, "+1d", UiAction::JumpDays(1));
                });
        });
}

fn update_time_display(
    clock: Res<SimClock>,
    time: Res<Time>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<TimeDisplayText>>,
        Query<&mut Text, With<SpeedDisplayText>>,
    )>,
    mut last_update: Local<f64>,
) {
    const UPDATE_INTERVAL: f64 = 0.25;

    let now = time.elapsed_secs_f64();

    if now - *last_update < UPDATE_INTERVAL {
        return;
    }

    *last_update = now;

    let seconds = clock.current_seconds().max(0.0);

    let total_seconds = seconds as u64;

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;

    let time_str = format!("Time: {:02}:{:02}:{:02}", hours, minutes, secs);

    let speed_str = if clock.paused {
        "Speed: paused".to_string()
    } else {
        format!("Speed: {}x", clock.time_scale)
    };

    for mut text in text_queries.p0().iter_mut() {
        text.0 = time_str.clone();
    }

    for mut text in text_queries.p1().iter_mut() {
        text.0 = speed_str.clone();
    }
}

fn toggle_ui_visibility(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<SimulationUIRoot>>,
) {
    if !keyboard.just_pressed(KeyCode::F10) {
        return;
    }

    for mut visibility in query.iter_mut() {
        *visibility = match *visibility {
            Visibility::Visible => Visibility::Hidden,
            _ => Visibility::Visible,
        };
    }
}

fn spawn_time_jump_ui(mut commands: Commands, theme: Res<UiTheme>) {
    let theme = &*theme;

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(18.0),
                ..default()
            },
            BackgroundColor(theme.panel_background),
            DespawnOnExit(TimeJumpState::InProgress),
            Visibility::Visible,
            TimeJumpRoot,
        ))
        .with_children(|parent| {
            spawn_label(
                parent,
                theme,
                "Пропускаем время...",
                theme.font_heading,
                TimeJumpStatusText,
            );

            spawn_label(parent, theme, "", theme.font_body, TimeJumpDateText);

            spawn_progress_bar(parent, theme, TimeJumpProgressFill);
        });
}

fn update_time_jump_ui(
    clock: Res<SimClock>,
    progress: Option<Res<TimeJumpProgress>>,
    mut queries: ParamSet<(
        Query<&mut Text, With<TimeJumpDateText>>,
        Query<&mut Text, With<TimeJumpStatusText>>,
        Query<&mut Node, With<TimeJumpProgressFill>>,
    )>,
) {
    let Some(progress) = progress else {
        return;
    };

    let total_ticks = progress
        .target_tick
        .saturating_sub(progress.start_tick)
        .max(1);

    let current_tick = clock
        .current_tick
        .clamp(progress.start_tick, progress.target_tick);

    let processed_ticks = current_tick.saturating_sub(progress.start_tick);

    let fraction = (processed_ticks as f64 / total_ticks as f64).clamp(0.0, 1.0) as f32;

    let current_seconds = current_tick as f64 * clock.tick_duration;
    let target_seconds = progress.target_tick as f64 * clock.tick_duration;

    let date_text = format!(
        "{}  ->  {}",
        format_sim_time(current_seconds),
        format_sim_time(target_seconds)
    );

    let status_text = if clock.current_tick < progress.target_tick {
        "Пропускаем время...".to_string()
    } else {
        "Обрабатываем события...".to_string()
    };

    for mut text in queries.p0().iter_mut() {
        text.0 = date_text.clone();
    }

    for mut text in queries.p1().iter_mut() {
        text.0 = status_text.clone();
    }

    for mut node in queries.p2().iter_mut() {
        node.width = Val::Percent(fraction * 100.0);
    }
}

fn format_sim_time(seconds: f64) -> String {
    let total_seconds = seconds.max(0.0) as u64;

    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;

    format!("Day {} {:02}:{:02}:{:02}", days, hours, minutes, secs)
}
