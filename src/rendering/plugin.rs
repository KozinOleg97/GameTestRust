use crate::game::WorldGeneratedEvent;
use crate::rendering::chunk_manager::{create_chunks, HexChunks};
use crate::rendering::materials::setup_hex_materials;
use bevy::prelude::*;

pub struct HexRenderingPlugin;

impl Plugin for HexRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_rendering_resources);
        app.add_systems(
            Update,
            create_chunks.run_if(on_message::<WorldGeneratedEvent>),
        );
    }
}

fn init_rendering_resources(
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let (texture, material) = setup_hex_materials(images, materials);
    commands.insert_resource(HexChunks {
        texture,
        material,
        chunks: Vec::new(),
    });
}
