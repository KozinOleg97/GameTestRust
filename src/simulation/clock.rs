use bevy::prelude::*;

/// Запрос на прыжок во времени.
#[derive(Resource, Default, Debug)]
pub struct TimeJumpRequest(pub u64);

/// Ресурс игрового времени.
#[derive(Resource)]
pub struct SimClock {
    pub current_tick: u64,
    pub tick_duration: f64,
    pub time_scale: f64,
    pub paused: bool,
    pub max_jump_speed: f64,
    accumulator: f64,
}

impl SimClock {
    pub fn new(tick_duration: f64, max_jump_speed: f64) -> Self {
        Self {
            current_tick: 0,
            tick_duration,
            time_scale: 1.0,
            paused: false,
            max_jump_speed,
            accumulator: 0.0,
        }
    }

    pub fn current_seconds(&self) -> f64 {
        self.current_tick as f64 * self.tick_duration
    }

    pub fn tick_duration_safe(&self) -> f64 {
        self.tick_duration.max(f64::EPSILON)
    }

    pub fn set_time_scale(&mut self, scale: f64) {
        self.time_scale = scale.max(0.0);
    }
}

/// Обычное течение времени.
pub fn update_sim_clock(
    mut clock: ResMut<SimClock>,
    time: Res<Time>,
    mut last_log_time: Local<f64>,
) {
    if clock.paused {
        return;
    }

    let tick_duration = clock.tick_duration_safe();
    let real_dt = time.delta_secs_f64();

    let scaled_seconds = real_dt * clock.time_scale.max(0.0);
    let ticks_f = scaled_seconds / tick_duration;
    let whole_ticks = ticks_f.floor();

    clock.accumulator += ticks_f - whole_ticks;
    clock.current_tick += whole_ticks as u64;

    if clock.accumulator >= 1.0 {
        let extra = clock.accumulator.floor() as u64;
        clock.current_tick += extra;
        clock.accumulator -= extra as f64;
    }

    let now = time.elapsed_secs_f64();

    if now - *last_log_time >= 1.0 {
        *last_log_time = now;

        info!(
            "Симуляция: tick = {}, игровое время = {:.2} сек, time_scale = {:.2}, paused = {}",
            clock.current_tick,
            clock.current_seconds(),
            clock.time_scale,
            clock.paused,
        );
    }
}
