//! Прыжок во времени: состояние экрана промотки, прогресс и его продвижение.
//!
//! Пока идёт прыжок, обычные системы симуляции приостановлены состоянием
//! [`TimeJumpState::InProgress`] — тикает только `advance_time_jump`.

use bevy::prelude::*;

use crate::game::GameSettings;

use super::clock::{SimClock, TimeJumpRequest};
use super::entity::{LODLevel, LastUpdate, SimulationUpdateTag};

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

pub fn handle_time_jump_requests(
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

pub fn advance_time_jump(
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

/// Завершает прыжок, когда цель достигнута и все просроченные обновления сделаны.
pub fn finish_time_jump(
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
