use crate::game::WorldGeneratedEvent;
use crate::hex::map::HexMap;
use crate::rendering::mesh_builder::generate_chunk_mesh;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, MeshVertexAttribute};
use bevy::prelude::*;
use bevy::render::render_resource::{PrimitiveTopology, VertexFormat};

pub const ATTRIBUTE_HEX_TYPE: MeshVertexAttribute =
    MeshVertexAttribute::new("HexType", 123456, VertexFormat::Float32);
const CHUNK_SIZE: i32 = 20;

#[derive(Resource)]
pub struct HexChunks {
    pub texture: Handle<Image>,
    pub material: Handle<StandardMaterial>,
    pub chunks: Vec<HexChunk>,
}

pub struct HexChunk {
    pub mesh: Handle<Mesh>,
    pub entity: Entity,
    pub q_min: i32,
    pub r_min: i32,
    pub q_max: i32,
    pub r_max: i32,
}

pub fn create_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut hex_chunks: ResMut<HexChunks>,
    hex_map: Res<HexMap>,
    mut events: MessageReader<WorldGeneratedEvent>,
) {
    for _ in events.read() {
        // Удаляем старые чанки
        for chunk in hex_chunks.chunks.drain(..) {
            commands.entity(chunk.entity).despawn();
        }

        let width = hex_map.width();
        let height = hex_map.height();
        let chunk_size = CHUNK_SIZE;

        for r_min in (0..height).step_by(chunk_size as usize) {
            for q_min in (0..width).step_by(chunk_size as usize) {
                let q_max = (q_min + chunk_size - 1).min(width - 1);
                let r_max = (r_min + chunk_size - 1).min(height - 1);

                let (positions, normals, uvs, indices, hex_types) =
                    generate_chunk_mesh(&hex_map, q_min, q_max, r_min, r_max);
                if positions.is_empty() {
                    continue;
                }

                let mut mesh = Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::default(),
                );
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                mesh.insert_attribute(ATTRIBUTE_HEX_TYPE, hex_types);
                mesh.insert_indices(Indices::U32(indices));
                let mesh_handle = meshes.add(mesh);

                let entity = commands
                    .spawn((
                        Mesh3d(mesh_handle.clone()),
                        MeshMaterial3d(hex_chunks.material.clone()),
                        Transform::IDENTITY,
                        Visibility::Visible,
                    ))
                    .id();

                hex_chunks.chunks.push(HexChunk {
                    mesh: mesh_handle,
                    entity,
                    q_min,
                    r_min,
                    q_max,
                    r_max,
                });
            }
        }
        info!("Generated {} chunks", hex_chunks.chunks.len());
    }
}
