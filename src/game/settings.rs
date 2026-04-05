use bevy::prelude::*;
use bevy_settings_lib::ValidatedSetting;
use serde::{Deserialize, Serialize};


// -----------------------------------------------------------------------------
// Главный ресурс настроек
// -----------------------------------------------------------------------------
#[derive(Resource, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GameSettings {
    pub camera: CameraSettings,
    pub window: WindowSettings,
    pub performance_overlay: PerformanceOverlaySettings,
    pub audio: AudioSettings,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            camera: CameraSettings::default(),
            window: WindowSettings::default(),
            performance_overlay: PerformanceOverlaySettings::default(),
            audio: AudioSettings::default(),
        }
    }
}

impl ValidatedSetting for GameSettings {
    fn validate(&mut self) {}
}

// -----------------------------------------------------------------------------
// Настройки камеры (соответствуют полям CameraController)
// -----------------------------------------------------------------------------
#[derive(Resource, Serialize, Deserialize, Clone, Debug, PartialEq)]
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

impl ValidatedSetting for CameraSettings {
    fn validate(&mut self) {}
}

// -----------------------------------------------------------------------------
// Настройки окна
// -----------------------------------------------------------------------------
#[derive(Resource, Serialize, Deserialize, Clone, Debug, PartialEq)]
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

impl ValidatedSetting for WindowSettings {
    fn validate(&mut self) {}
}

// -----------------------------------------------------------------------------
// Настройки FPS-оверлея (аналог PerformanceOverlayConfig)
// -----------------------------------------------------------------------------
#[derive(Resource, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerformanceOverlaySettings {
    pub visible: bool,
    pub position: (f32, f32),
    pub font_size: f32,
    pub text_color: (f32, f32, f32, f32),               // RGBA
    pub background_color: Option<(f32, f32, f32, f32)>, // RGBA
    pub fps_average_frames: usize,
    pub toggle_key: KeyCodeSettings,
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
        }
    }
}

impl ValidatedSetting for PerformanceOverlaySettings {
    fn validate(&mut self) {}
}

// -----------------------------------------------------------------------------
// Настройки звука (задел)
// -----------------------------------------------------------------------------
#[derive(Resource, Serialize, Deserialize, Clone, Debug, PartialEq)]
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

impl ValidatedSetting for AudioSettings {
    fn validate(&mut self) {}
}

// -----------------------------------------------------------------------------
// Сериализуемые коды клавиш
// -----------------------------------------------------------------------------
#[derive(Resource, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KeyCodeSettings {
    KeyW,
    KeyS,
    KeyA,
    KeyD,
    KeyE,
    F12,
    Escape,
    // Добавьте нужные
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
