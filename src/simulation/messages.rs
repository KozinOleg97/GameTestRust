use bevy::prelude::*;

use super::clock::{SimClock, TimeJumpRequest};

/// Запросы к симуляции времени.
///
/// Тип живёт в домене симуляции: панель управления временем только сообщает, чего
/// хочет игрок, а как это отразить в часах — решает симуляция.
#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub enum SimulationRequest {
    PauseToggle,
    SetSpeed(f64),
    JumpHours(u64),
    JumpDays(u64),
}

/// Обработка запросов к симуляции.
pub fn handle_simulation_requests(
    mut requests: MessageReader<SimulationRequest>,
    mut clock: ResMut<SimClock>,
    mut jump_request: ResMut<TimeJumpRequest>,
) {
    for request in requests.read() {
        match *request {
            SimulationRequest::PauseToggle => {
                clock.paused = !clock.paused;
                info!("Simulation paused: {}", clock.paused);
            }
            SimulationRequest::SetSpeed(scale) => clock.set_time_scale(scale),
            SimulationRequest::JumpHours(hours) => {
                let ticks = (hours as f64 * 3600.0 / clock.tick_duration_safe()) as u64;
                jump_request.0 += ticks.max(1);
            }
            SimulationRequest::JumpDays(days) => {
                let ticks = (days as f64 * 24.0 * 3600.0 / clock.tick_duration_safe()) as u64;
                jump_request.0 += ticks.max(1);
            }
        }
    }
}
