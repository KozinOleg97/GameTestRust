//! Компоненты симулируемых сущностей и уровни детализации.
//!
//! Это данные, а не обвязка плагина: отсюда их читают системы обновления.

use bevy::prelude::*;

/// Уровень детализации сущности: чем меньше число, тем чаще она обновляется.
#[derive(Component)]
pub struct LODLevel(pub u8);

/// Тик, на котором сущность обновлялась в последний раз.
#[derive(Component)]
pub struct LastUpdate(pub u64);

/// Метка «сущность попала в бюджет обновлений этого кадра».
///
/// Ставится в `schedule_updates`, снимается системами обновления. Пока метка
/// висит, сущность не может быть выбрана повторно.
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
