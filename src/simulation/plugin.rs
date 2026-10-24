//! Сборка подсистемы симуляции: только регистрация, без логики.
//!
//! Сама логика разложена по файлам рядом: `clock` (игровое время), `time_jump`
//! (промотка), `budget` (сколько обновлять за кадр), `updates` (применение
//! обновлений), `entity` (данные сущностей), `messages` (запросы из UI).

use bevy::prelude::*;

use crate::game::{GameSessionState, GameSettings, PlayState};

use super::budget::{SimulationUpdateBudget, schedule_updates};
use super::clock::{SimClock, TimeJumpRequest, update_sim_clock};
use super::debug::debug_time_controls;
use super::messages::{SimulationRequest, handle_simulation_requests};
use super::time_jump::{
    TimeJumpState, advance_time_jump, finish_time_jump, handle_time_jump_requests,
};
use super::updates::{
    apply_location_updates, apply_npc_updates, cleanup_unknown_simulation_updates,
};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TimeJumpState>()
            .init_resource::<TimeJumpRequest>()
            .add_message::<SimulationRequest>()
            .add_systems(Startup, setup_sim_clock)
            .add_systems(
                Update,
                (
                    // Запросы обрабатываются первыми: если игрок попросил прыжок во
                    // времени, `handle_time_jump_requests` должен увидеть его в этом
                    // же кадре, а не в следующем.
                    handle_simulation_requests,
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
