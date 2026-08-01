use bevy::prelude::*;
use bevy_settings::{
    ReflectSettingsGroup, SaveSettingsDeferred, SaveSettingsSync, SettingsGroup, SettingsPlugin,
};

use crate::hex::{DEFAULT_CHUNK_SIZE, MAX_CHUNK_SIZE, MIN_CHUNK_SIZE};

// -----------------------------------------------------------------------------
// Главный ресурс настроек
// -----------------------------------------------------------------------------

#[derive(Resource, SettingsGroup, Reflect)]
#[reflect(Resource, SettingsGroup, Default)]
pub struct GameSettings {
    pub camera: CameraSettings,
    pub window: WindowSettings,
    pub performance_overlay: PerformanceOverlaySettings,
    pub audio: AudioSettings,

    pub generation: GenerationSettings,
    pub rendering: RenderingSettings,
    pub debug: DebugSettings,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            camera: CameraSettings::default(),
            window: WindowSettings::default(),
            performance_overlay: PerformanceOverlaySettings::default(),
            audio: AudioSettings::default(),

            generation: GenerationSettings::default(),
            rendering: RenderingSettings::default(),
            debug: DebugSettings::default(),
        }
    }
}

impl GameSettings {
    pub fn validate_all(&mut self) {
        self.generation.validate();
        self.rendering.validate();
    }
}

pub fn validate_game_settings(mut config: ResMut<GameSettings>) {
    config.validate_all();
}

// -----------------------------------------------------------------------------
// Настройки генерации мира
// -----------------------------------------------------------------------------

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource, Default)]
pub struct GenerationSettings {
    /// Ширина карты в гексах.
    pub map_width: i32,

    /// Высота карты в гексах.
    pub map_height: i32,

    /// Seed генерации.
    pub generation_seed: u64,

    /// Размер чанка в гексах.
    /// Должен быть степенью двойки: 8, 16, 32, 64, 128.
    pub chunk_size: i32,
}

impl Default for GenerationSettings {
    fn default() -> Self {
        Self {
            map_width: 1000,
            map_height: 1000,
            generation_seed: 12345,
            chunk_size: DEFAULT_CHUNK_SIZE as i32,
        }
    }
}

impl GenerationSettings {
    pub fn validate(&mut self) {
        if self.map_width <= 0 {
            warn!(
                "GenerationSettings.map_width was {}, forcing to 1",
                self.map_width
            );
            self.map_width = 1;
        }

        if self.map_height <= 0 {
            warn!(
                "GenerationSettings.map_height was {}, forcing to 1",
                self.map_height
            );
            self.map_height = 1;
        }

        if self.chunk_size <= 0 || !is_valid_chunk_size_i32(self.chunk_size) {
            warn!(
                "GenerationSettings.chunk_size was {} (allowed: power of two, {}..{}), forcing to {}",
                self.chunk_size, MIN_CHUNK_SIZE, MAX_CHUNK_SIZE, DEFAULT_CHUNK_SIZE
            );

            self.chunk_size = DEFAULT_CHUNK_SIZE as i32;
        }
    }

    pub fn chunk_size_usize(&self) -> usize {
        self.chunk_size as usize
    }
}

fn is_valid_chunk_size_i32(size: i32) -> bool {
    size >= MIN_CHUNK_SIZE as i32
        && size <= MAX_CHUNK_SIZE as i32
        && size > 0
        && (size & (size - 1)) == 0
}

// -----------------------------------------------------------------------------
// Настройки рендера
// -----------------------------------------------------------------------------

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource, Default)]
pub struct RenderingSettings {
    pub hex_size: f32,
    pub elevation_step: f32,
    pub skirt_height: f32,
    pub enable_walls: bool,
    pub mesh_build_batch_size: usize,
}

impl Default for RenderingSettings {
    fn default() -> Self {
        Self {
            hex_size: 1.0,
            elevation_step: 0.9,
            skirt_height: -10.0,
            enable_walls: true,
            mesh_build_batch_size: 32,
        }
    }
}

impl RenderingSettings {
    pub fn validate(&mut self) {
        if !self.hex_size.is_finite() || self.hex_size <= 0.0 {
            warn!(
                "RenderingSettings.hex_size was {}, forcing to 1.0",
                self.hex_size
            );
            self.hex_size = 1.0;
        }

        if !self.elevation_step.is_finite() || self.elevation_step <= 0.0 {
            warn!(
                "RenderingSettings.elevation_step was {}, forcing to 0.9",
                self.elevation_step
            );
            self.elevation_step = 0.9;
        }

        if !self.skirt_height.is_finite() {
            warn!(
                "RenderingSettings.skirt_height was {}, forcing to -10.0",
                self.skirt_height
            );
            self.skirt_height = -10.0;
        }

        self.mesh_build_batch_size = self.mesh_build_batch_size.clamp(1, 256);
    }
}

// -----------------------------------------------------------------------------
// Debug-настройки
// -----------------------------------------------------------------------------

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource, Default)]
pub struct DebugSettings {
    pub chunk_bounds: bool,
    pub log_generation: bool,
    pub log_mesh_queue: bool,
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
            chunk_bounds: false,
            log_generation: true,
            log_mesh_queue: false,
        }
    }
}

// -----------------------------------------------------------------------------
// Камера
// -----------------------------------------------------------------------------

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource, Default)]
pub struct CameraSettings {
    pub pan_speed: f32,
    pub zoom_speed: f32,
    pub min_fov: f32,
    pub max_fov: f32,
    pub move_speed: f32,
    pub rotate_speed: f32,
    pub min_pitch: f32,
    pub max_pitch: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            pan_speed: 1.0,
            zoom_speed: 0.05,
            min_fov: 1.0,
            max_fov: 150.0,
            move_speed: 500.0,
            rotate_speed: 0.005,
            min_pitch: -90.0,
            max_pitch: 90.0,
        }
    }
}

// -----------------------------------------------------------------------------
// Окно
// -----------------------------------------------------------------------------

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource, Default)]
pub struct WindowSettings {
    pub width: f32,
    pub height: f32,
    pub fullscreen: bool,
    pub vsync: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 1280.0,
            height: 720.0,
            fullscreen: false,
            vsync: true,
        }
    }
}

// -----------------------------------------------------------------------------
// FPS overlay
// -----------------------------------------------------------------------------

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource, Default)]
pub struct PerformanceOverlaySettings {
    pub visible: bool,
    pub position: (f32, f32),
    pub font_size: f32,
    pub text_color: (f32, f32, f32, f32),
    pub background_color: Option<(f32, f32, f32, f32)>,
    pub fps_average_frames: usize,
    pub toggle_key: KeyCodeSettings,
    pub update_interval_secs: f32,
}

impl Default for PerformanceOverlaySettings {
    fn default() -> Self {
        Self {
            visible: true,
            position: (10.0, 10.0),
            font_size: 24.0,
            text_color: (1.0, 1.0, 1.0, 1.0),
            background_color: Some((0.0, 0.0, 0.0, 0.5)),
            fps_average_frames: 60,
            toggle_key: KeyCodeSettings::F12,
            update_interval_secs: 0.5,
        }
    }
}

// -----------------------------------------------------------------------------
// Audio
// -----------------------------------------------------------------------------

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource, Default)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.8,
            music_volume: 0.7,
            sfx_volume: 1.0,
        }
    }
}

// -----------------------------------------------------------------------------
// Key codes
// -----------------------------------------------------------------------------

#[derive(Resource, Reflect, Copy, Clone)]
#[reflect(Resource)]
pub enum KeyCodeSettings {
    KeyW,
    KeyS,
    KeyA,
    KeyD,
    KeyE,
    F12,
    Escape,
}

impl From<KeyCodeSettings> for KeyCode {
    fn from(kc: KeyCodeSettings) -> Self {
        match kc {
            KeyCodeSettings::KeyW => KeyCode::KeyW,
            KeyCodeSettings::KeyS => KeyCode::KeyS,
            KeyCodeSettings::KeyA => KeyCode::KeyA,
            KeyCodeSettings::KeyD => KeyCode::KeyD,
            KeyCodeSettings::KeyE => KeyCode::KeyE,
            KeyCodeSettings::F12 => KeyCode::F12,
            KeyCodeSettings::Escape => KeyCode::Escape,
        }
    }
}
