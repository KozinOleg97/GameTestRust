//! Бюджет обновлений за кадр и выбор сущностей, которым пора обновиться.

use bevy::prelude::*;

use crate::game::GameSettings;

use super::clock::SimClock;
use super::entity::{LODLevel, LastUpdate, SimulationUpdateTag};

/// Сколько сущностей разрешено обновить за один кадр.
#[derive(Resource)]
pub struct SimulationUpdateBudget(pub usize);

/// Помечает готовые к обновлению сущности, не превышая бюджет кадра.
pub fn schedule_updates(
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
