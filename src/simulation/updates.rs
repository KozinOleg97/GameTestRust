//! Применение обновлений к помеченным сущностям.
//!
//! Три системы держатся в одном файле намеренно: `apply_location_updates` и
//! `apply_npc_updates` различаются только `Without<Location>` в фильтре, а
//! `cleanup_unknown_simulation_updates` снимает метку с тех, кому она досталась
//! по ошибке. Разнести их — значит разнести один инвариант по трём файлам.

use bevy::prelude::*;

use super::clock::SimClock;
use super::entity::{LODLevel, LastUpdate, Location, Npc, SimulationUpdateTag};

pub fn apply_location_updates(
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

pub fn apply_npc_updates(
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

pub fn cleanup_unknown_simulation_updates(
    mut commands: Commands,
    query: Query<Entity, (With<SimulationUpdateTag>, Without<Location>, Without<Npc>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<SimulationUpdateTag>();
    }
}
