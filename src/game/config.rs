use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct GameConfig {
    pub window_title: String,
    pub window_size: (f32, f32),
    pub map_width: i32,
    pub map_height: i32,
    pub generation_seed: u64,
    pub chunk_size: i32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window_title: "Hex Game".to_string(),
            window_size: (1280.0, 720.0),
            map_width: 200,
            map_height: 200,
            generation_seed: 12345,
            chunk_size: 20,
        }
    }
}
