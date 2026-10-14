use bevy::prelude::*;

use crate::game::{GameSessionState, GameSettings, PlayState};

use super::clock::{SimClock, TimeJumpRequest, update_sim_clock};
use super::debug::debug_time_controls;

/// Состояние экрана промотки времени.
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeJumpState {
    #[default]
    Inactive,

    /// Экран промотки активен.
    InProgress,
}

/// Прогресс активного прыжка во времени.
#[derive(Resource)]
pub struct TimeJumpProgress {
    pub start_tick: u64,
    pub target_tick: u64,
}

/// Бюджет обновлений за кадр.
#[derive(Resource)]
pub struct SimulationUpdateBudget(pub usize);

#[derive(Component)]
pub struct LODLevel(pub u8);

#[derive(Component)]
pub struct LastUpdate(pub u64);

#[derive(Component)]
pub struct SimulationUpdateTag;

#[derive(Component)]
pub struct Location {
    pub name: String,
    pub population: u32,
}

#[derive(Component)]
pub struct Npc {
    pub name: String,
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TimeJumpState>()
            .init_resource::<TimeJumpRequest>()
            .add_systems(Startup, setup_sim_clock)
            .add_systems(
                Update,
                (
                    handle_time_jump_requests.run_if(in_state(TimeJumpState::Inactive)),
                    update_sim_clock.run_if(in_state(TimeJumpState::Inactive)),
                    debug_time_controls.run_if(in_state(TimeJumpState::Inactive)),
                    advance_time_jump.run_if(in_state(TimeJumpState::InProgress)),
                    schedule_updates,
                    apply_location_updates,
                    apply_npc_updates,
                    cleanup_unknown_simulation_updates,
                    finish_time_jump.run_if(in_state(TimeJumpState::InProgress)),
                )
                    .chain()
                    .run_if(in_state(GameSessionState::Active))
                    .run_if(in_state(PlayState::Playing)),
            );
    }
}

fn setup_sim_clock(mut commands: Commands, game_settings: Res<GameSettings>) {
    let sim_cfg = &game_settings.simulation;

    commands.insert_resource(SimClock::new(sim_cfg.tick_duration, sim_cfg.max_jump_speed));

    commands.insert_resource(SimulationUpdateBudget(256));
}

fn handle_time_jump_requests(
    mut commands: Commands,
    mut request: ResMut<TimeJumpRequest>,
    clock: Res<SimClock>,
) {
    if request.0 == 0 {
        return;
    }

    let ticks = request.0;
    request.0 = 0;

    let start_tick = clock.current_tick;
    let target_tick = start_tick.saturating_add(ticks);

    commands.insert_resource(TimeJumpProgress {
        start_tick,
        target_tick,
    });

    commands.set_state(TimeJumpState::InProgress);

    info!(
        "Запущен прыжок во времени: {} тиков, цель: {}",
        ticks, target_tick
    );
}

fn advance_time_jump(
    mut clock: ResMut<SimClock>,
    progress: Option<ResMut<TimeJumpProgress>>,
    time: Res<Time>,
) {
    let Some(progress) = progress else {
        return;
    };

    if clock.current_tick >= progress.target_tick {
        return;
    }

    let tick_duration = clock.tick_duration_safe();
    let real_dt = time.delta_secs_f64();

    let jump_speed = clock.max_jump_speed.max(1.0);

    let mut ticks_this_frame = ((real_dt * jump_speed) / tick_duration) as u64;

    if ticks_this_frame == 0 {
        ticks_this_frame = 1;
    }

    ticks_this_frame = ticks_this_frame.clamp(1, 100_000);

    let remaining = progress.target_tick.saturating_sub(clock.current_tick);
    let add = ticks_this_frame.min(remaining);

    clock.current_tick += add;
}

fn schedule_updates(
    clock: Res<SimClock>,
    game_settings: Res<GameSettings>,
    budget: Res<SimulationUpdateBudget>,
    mut commands: Commands,
    query: Query<(Entity, &LODLevel, &LastUpdate), Without<SimulationUpdateTag>>,
) {
    let mut remaining_updates = budget.0;

    if remaining_updates == 0 {
        return;
    }

    let intervals = &game_settings.simulation.lod_intervals;

    for (entity, lod, last_update) in query.iter() {
        if remaining_updates == 0 {
            break;
        }

        let interval = intervals
            .get(lod.0 as usize)
            .copied()
            .unwrap_or(3600)
            .max(1);

        if clock.current_tick >= last_update.0.saturating_add(interval) {
            commands.entity(entity).insert(SimulationUpdateTag);
            remaining_updates -= 1;
        }
    }
}

fn apply_location_updates(
    clock: Res<SimClock>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Location, &LODLevel, &mut LastUpdate),
        With<SimulationUpdateTag>,
    >,
) {
    for (entity, location, lod, mut last_update) in query.iter_mut() {
        let dt_ticks = clock.current_tick.saturating_sub(last_update.0);
        let dt_seconds = dt_ticks as f64 * clock.tick_duration_safe();

        info!(
            "Обновление локации '{}': dt = {} тиков ({:.2} сек), LOD = {}",
            location.name, dt_ticks, dt_seconds, lod.0
        );

        // Здесь логика обновления локации

        last_update.0 = clock.current_tick;
        commands.entity(entity).remove::<SimulationUpdateTag>();
    }
}

fn apply_npc_updates(
    clock: Res<SimClock>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Npc, &LODLevel, &mut LastUpdate),
        (With<SimulationUpdateTag>, Without<Location>),
    >,
) {
    for (entity, npc, lod, mut last_update) in query.iter_mut() {
        let dt_ticks = clock.current_tick.saturating_sub(last_update.0);
        let dt_seconds = dt_ticks as f64 * clock.tick_duration_safe();

        info!(
            "Обновление NPC '{}': dt = {} тиков ({:.2} сек), LOD = {}",
            npc.name, dt_ticks, dt_seconds, lod.0
        );

        // Здесь логика обновления NPC

        last_update.0 = clock.current_tick;
        commands.entity(entity).remove::<SimulationUpdateTag>();
    }
}

fn cleanup_unknown_simulation_updates(
    mut commands: Commands,
    query: Query<Entity, (With<SimulationUpdateTag>, Without<Location>, Without<Npc>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<SimulationUpdateTag>();
    }
}

fn finish_time_jump(
    mut commands: Commands,
    clock: Res<SimClock>,
    progress: Option<Res<TimeJumpProgress>>,
    game_settings: Res<GameSettings>,
    due_query: Query<(&LODLevel, &LastUpdate), Without<SimulationUpdateTag>>,
    tag_query: Query<Entity, With<SimulationUpdateTag>>,
) {
    let Some(progress) = progress else {
        return;
    };

    if clock.current_tick < progress.target_tick {
        return;
    }

    if !tag_query.is_empty() {
        return;
    }

    let intervals = &game_settings.simulation.lod_intervals;

    for (lod, last_update) in due_query.iter() {
        let interval = intervals
            .get(lod.0 as usize)
            .copied()
            .unwrap_or(3600)
            .max(1);

        if clock.current_tick >= last_update.0.saturating_add(interval) {
            return;
        }
    }

    commands.remove_resource::<TimeJumpProgress>();
    commands.set_state(TimeJumpState::Inactive);

    info!(
        "Прыжок во времени завершён. Текущий тик: {}",
        clock.current_tick
    );
}
