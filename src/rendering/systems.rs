use std::collections::{HashSet, VecDeque};

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

use crate::game::GameSettings;
use crate::hex::{ChunkData, ChunkLayout, ChunkMap, HexChunk, MapBounds, HEX_DIRECTIONS};

use super::mesh_builder::build_chunk_mesh;

// -----------------------------------------------------------------------------
// Components / resources
// -----------------------------------------------------------------------------

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeshGeneration(pub u32);

#[derive(Component)]
pub struct MeshBuildTask {
    task: Task<Option<Mesh>>,
    chunk_entity: Entity,
    generation: u32,
}

#[derive(Resource, Default)]
pub struct TerrainMaterial(pub Option<Handle<StandardMaterial>>);

#[derive(Resource, Default)]
pub struct MeshBuildQueue {
    entities: VecDeque<Entity>,
    set: HashSet<Entity>,
}

impl MeshBuildQueue {
    pub fn request(&mut self, entity: Entity) {
        if self.set.insert(entity) {
            self.entities.push_back(entity);
        }
    }

    pub fn drain_batch(&mut self, batch_size: usize) -> Vec<Entity> {
        let mut result = Vec::with_capacity(batch_size.min(self.entities.len()));

        while result.len() < batch_size {
            let Some(entity) = self.entities.pop_front() else {
                break;
            };

            self.set.remove(&entity);
            result.push(entity);
        }

        result
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.set.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }
}

// -----------------------------------------------------------------------------
// Setup / utility systems
// -----------------------------------------------------------------------------

pub fn setup_terrain_material(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut terrain_material: ResMut<TerrainMaterial>,
) {
    if terrain_material.0.is_none() {
        let handle = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            ..default()
        });

        terrain_material.0 = Some(handle);
    }
}

pub fn ensure_mesh_generation(
    mut commands: Commands,
    query: Query<Entity, (With<HexChunk>, Without<MeshGeneration>)>,
) {
    for entity in &query {
        commands.entity(entity).insert(MeshGeneration::default());
    }
}

pub fn clear_mesh_queue(mut queue: ResMut<MeshBuildQueue>) {
    queue.clear();
}

// -----------------------------------------------------------------------------
// Dirty tracking
// -----------------------------------------------------------------------------

pub fn queue_dirty_chunks_system(
    mut queue: ResMut<MeshBuildQueue>,
    chunk_map: Res<ChunkMap>,
    changed_chunks: Query<(Entity, &HexChunk), Changed<ChunkData>>,
) {
    for (entity, chunk) in &changed_chunks {
        queue.request(entity);

        for dir in &HEX_DIRECTIONS {
            let nx = chunk.chunk_x + dir.q();
            let ny = chunk.chunk_y + dir.r();

            if let Some(neighbor_entity) = chunk_map.get_chunk_entity(nx, ny) {
                queue.request(neighbor_entity);
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Process queue
// -----------------------------------------------------------------------------

pub fn process_mesh_build_queue_system(
    mut commands: Commands,
    mut queue: ResMut<MeshBuildQueue>,
    mut generation_query: Query<&mut MeshGeneration>,
    chunk_query: Query<(&HexChunk, &ChunkData)>,
    all_chunks: Query<&ChunkData>,
    chunk_map: Res<ChunkMap>,
    layout: Res<ChunkLayout>,
    bounds: Res<MapBounds>,
    config: Res<GameSettings>,
) {
    if queue.is_empty() {
        return;
    }

    let batch_size = config.rendering.mesh_build_batch_size.max(1);
    let batch = queue.drain_batch(batch_size);

    if batch.is_empty() {
        return;
    }

    let pool = AsyncComputeTaskPool::get();

    for entity in batch {
        let Ok((chunk, data)) = chunk_query.get(entity) else {
            continue;
        };

        if data.size != layout.size {
            warn!(
                "Skipping chunk {:?}: data size {} != layout size {}",
                chunk, data.size, layout.size
            );
            continue;
        }

        let Ok(mut generation) = generation_query.get_mut(entity) else {
            commands.entity(entity).insert(MeshGeneration::default());
            queue.request(entity);
            continue;
        };

        generation.0 = generation.0.wrapping_add(1);
        let generation_value = generation.0;

        let neighbors = collect_neighbor_elevations(chunk, &chunk_map, &all_chunks, &layout);

        let chunk_copy = *chunk;
        let data_clone = data.clone();
        let layout_copy = *layout;
        let bounds_copy = *bounds;
        let render_clone = config.rendering.clone();

        let task = pool.spawn(async move {
            build_chunk_mesh(
                &chunk_copy,
                &data_clone,
                neighbors,
                &layout_copy,
                bounds_copy,
                &render_clone,
            )
        });

        commands.spawn(MeshBuildTask {
            task,
            chunk_entity: entity,
            generation: generation_value,
        });
    }
}

fn collect_neighbor_elevations(
    chunk: &HexChunk,
    chunk_map: &ChunkMap,
    all_chunks: &Query<&ChunkData>,
    layout: &ChunkLayout,
) -> [Option<Vec<i8>>; 6] {
    std::array::from_fn(|i| {
        let dir = HEX_DIRECTIONS[i];

        let nx = chunk.chunk_x + dir.q();
        let ny = chunk.chunk_y + dir.r();

        chunk_map
            .get_chunk_entity(nx, ny)
            .and_then(|entity| all_chunks.get(entity).ok())
            .filter(|neighbor_data| neighbor_data.size == layout.size)
            .map(|neighbor_data| neighbor_data.elevations.clone())
    })
}

// -----------------------------------------------------------------------------
// Apply ready tasks
// -----------------------------------------------------------------------------

pub fn apply_mesh_tasks_system(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut MeshBuildTask)>,
    chunks: Query<
        (
            &MeshGeneration,
            Option<&Mesh3d>,
            Option<&MeshMaterial3d<StandardMaterial>>,
        ),
        With<HexChunk>,
    >,
    mut terrain_material: ResMut<TerrainMaterial>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (task_entity, mut task) in &mut tasks {
        let Some(result) = future::block_on(future::poll_once(&mut task.task)) else {
            continue;
        };

        let chunk_info = chunks
            .get(task.chunk_entity)
            .ok()
            .map(|(generation, mesh, material)| {
                (
                    generation.0,
                    mesh.map(|m| m.0.clone()),
                    material.map(|m| m.0.clone()),
                )
            });

        let Some((current_generation, existing_mesh, existing_material)) = chunk_info else {
            commands.entity(task_entity).despawn();
            continue;
        };

        if current_generation != task.generation {
            commands.entity(task_entity).despawn();
            continue;
        }

        let material_handle = match terrain_material.0.clone() {
            Some(handle) => handle,
            None => {
                let handle = materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    unlit: true,
                    ..default()
                });

                terrain_material.0 = Some(handle.clone());
                handle
            }
        };

        match result {
            None => {
                let mut entity_commands = commands.entity(task.chunk_entity);

                if existing_mesh.is_some() {
                    entity_commands.remove::<Mesh3d>();
                }

                if existing_material.is_some() {
                    entity_commands.remove::<MeshMaterial3d<StandardMaterial>>();
                }
            }

            Some(mesh) => {
                let mesh_handle = match existing_mesh.clone() {
                    Some(handle) => {
                        if let Some(mut existing_mesh_asset) = meshes.get_mut(&handle) {
                            *existing_mesh_asset = mesh;
                            handle
                        } else {
                            meshes.add(mesh)
                        }
                    }
                    None => meshes.add(mesh),
                };

                let mut entity_commands = commands.entity(task.chunk_entity);

                if existing_mesh != Some(mesh_handle.clone()) {
                    entity_commands.insert(Mesh3d(mesh_handle));
                }

                if existing_material != Some(material_handle.clone()) {
                    entity_commands.insert(MeshMaterial3d(material_handle));
                }
            }
        }

        commands.entity(task_entity).despawn();
    }
}
