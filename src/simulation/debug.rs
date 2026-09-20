use bevy::prelude::*;

use super::clock::{SimClock, TimeJumpRequest};

/// Отладочное управление временем.
pub fn debug_time_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut clock: ResMut<SimClock>,
    mut jump_request: ResMut<TimeJumpRequest>,
) {
    let tick_duration = clock.tick_duration_safe();

    if keyboard.just_pressed(KeyCode::F5) {
        let hour_ticks = (3600.0 / tick_duration) as u64;
        jump_request.0 += hour_ticks.max(1);
        info!("Прыжок на 1 час вперёд: {} тиков", hour_ticks);
    }

    if keyboard.just_pressed(KeyCode::F6) {
        clock.set_time_scale(10.0);
        info!("Скорость симуляции: 10x");
    }

    if keyboard.just_pressed(KeyCode::F7) {
        clock.set_time_scale(1.0);
        info!("Скорость симуляции: 1x");
    }

    if keyboard.just_pressed(KeyCode::F8) {
        clock.paused = !clock.paused;
        info!("Пауза: {}", clock.paused);
    }

    if keyboard.just_pressed(KeyCode::F9) {
        let day_ticks = (24.0 * 3600.0 / tick_duration) as u64;
        jump_request.0 += day_ticks.max(1);
        info!("Прыжок на 1 сутки вперёд: {} тиков", day_ticks);
    }
}
