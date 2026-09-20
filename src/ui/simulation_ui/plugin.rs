use bevy::prelude::*;
use bevy::ecs::system::ParamSet;

use crate::game::GameSessionState;
use crate::simulation::{SimClock, TimeJumpProgress, TimeJumpRequest, TimeJumpState};
use crate::ui::style::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum TimeControlAction {
    PauseToggle,
    SetSpeed1x,
    SetSpeed2x,
    SetSpeed5x,
    SetSpeed10x,
    JumpHour,
    JumpDay,
}

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
        app.add_systems(OnEnter(GameSessionState::Active), spawn_simulation_ui)
            .add_systems(OnEnter(TimeJumpState::InProgress), spawn_time_jump_ui)
            .add_systems(
                Update,
                (
                    handle_time_control_buttons,
                    update_time_display,
                    toggle_ui_visibility,
                )
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

fn spawn_simulation_ui(mut commands: Commands) {
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
            BackgroundColor(Color::NONE),
            DespawnOnExit(GameSessionState::Active),
            Visibility::Visible,
            SimulationUIRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Time: 00:00:00"),
                ui_font(18.0),
                TextColor(TEXT_COLOR),
                TimeDisplayText,
            ));

            parent.spawn((
                Text::new("Speed: 1x"),
                ui_font(18.0),
                TextColor(TEXT_COLOR),
                SpeedDisplayText,
            ));

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(5.0),
                    ..default()
                })
                .with_children(|buttons| {
                    spawn_time_button(buttons, "⏸", TimeControlAction::PauseToggle);
                    spawn_time_button(buttons, "1x", TimeControlAction::SetSpeed1x);
                    spawn_time_button(buttons, "2x", TimeControlAction::SetSpeed2x);
                    spawn_time_button(buttons, "5x", TimeControlAction::SetSpeed5x);
                    spawn_time_button(buttons, "10x", TimeControlAction::SetSpeed10x);
                    spawn_time_button(buttons, "+1h", TimeControlAction::JumpHour);
                    spawn_time_button(buttons, "+1d", TimeControlAction::JumpDay);
                });
        });
}

fn spawn_time_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: TimeControlAction,
) {
    parent
        .spawn((
            Button,
            action,
            Node {
                width: Val::Px(42.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_child((
            Text::new(label),
            ui_font(16.0),
            TextColor(TEXT_COLOR),
        ));
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

fn handle_time_control_buttons(
    mut clock: ResMut<SimClock>,
    mut jump_request: ResMut<TimeJumpRequest>,
    interaction_query: Query<
        (&Interaction, &TimeControlAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    let tick_duration = clock.tick_duration_safe();

    for (interaction, action) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            TimeControlAction::PauseToggle => {
                clock.paused = !clock.paused;
                info!("Simulation paused: {}", clock.paused);
            }

            TimeControlAction::SetSpeed1x => {
                clock.set_time_scale(1.0);
            }

            TimeControlAction::SetSpeed2x => {
                clock.set_time_scale(2.0);
            }

            TimeControlAction::SetSpeed5x => {
                clock.set_time_scale(5.0);
            }

            TimeControlAction::SetSpeed10x => {
                clock.set_time_scale(10.0);
            }

            TimeControlAction::JumpHour => {
                let ticks = (3600.0 / tick_duration) as u64;
                jump_request.0 += ticks.max(1);
            }

            TimeControlAction::JumpDay => {
                let ticks = (24.0 * 3600.0 / tick_duration) as u64;
                jump_request.0 += ticks.max(1);
            }
        }

        break;
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

fn spawn_time_jump_ui(mut commands: Commands) {
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            DespawnOnExit(TimeJumpState::InProgress),
            Visibility::Visible,
            TimeJumpRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Пропускаем время..."),
                ui_font(28.0),
                TextColor(TEXT_COLOR),
                TimeJumpStatusText,
            ));

            parent.spawn((
                Text::new(""),
                ui_font(22.0),
                TextColor(TEXT_COLOR),
                TimeJumpDateText,
            ));

            parent
                .spawn((
                    Node {
                        width: Val::Px(420.0),
                        height: Val::Px(18.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.25, 0.25, 0.25, 1.0)),
                ))
                .with_child((
                    Node {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.85, 0.3)),
                    TimeJumpProgressFill,
                ));
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

fn ui_font(size: f32) -> TextFont {
    TextFont {
        font_size: FontSize::Px(size),
        ..default()
    }
}