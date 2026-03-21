use crate::hex::{HexCoordinates, HexType, HEX_SIZE};
use crate::hex::utils::{HEX_WIDTH, Y_PITCH};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, MeshVertexAttribute},
    prelude::*,
    render::render_resource::{PrimitiveTopology, VertexFormat},
};

// Custom vertex attribute for hex type index (float 0-1)
pub const ATTRIBUTE_HEX_TYPE: MeshVertexAttribute =
    MeshVertexAttribute::new("HexType", 123456, VertexFormat::Float32);

const CHUNK_SIZE: i32 = 20; // 20x20 hexes per chunk

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

#[derive(Resource, Default)]
struct StaticMeshGenerated(bool);

pub struct BatchedHexRendererPlugin;

impl Plugin for BatchedHexRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_batched_hex_assets)
            .add_systems(Update, update_batched_hex_mesh)
            .init_resource::<StaticMeshGenerated>();
    }
}

fn setup_batched_hex_assets(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a 1D texture with 8 pixels (including Empty)
    let width = 8;
    let height = 1;
    let mut data = Vec::with_capacity(width * height * 4);
    for i in 0..width {
        let hex_type = match i {
            0 => HexType::Empty,
            1 => HexType::Plains,
            2 => HexType::Forest,
            3 => HexType::Mountains,
            4 => HexType::Desert,
            5 => HexType::Ocean,
            6 => HexType::Coast,
            7 => HexType::Swamp,
            _ => HexType::Empty,
        };
        let color = hex_type.color();
        data.push((color.0 * 255.0) as u8);
        data.push((color.1 * 255.0) as u8);
        data.push((color.2 * 255.0) as u8);
        data.push((color.3 * 255.0) as u8);
    }
    let texture = Image::new(
        bevy::render::render_resource::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    let texture_handle = images.add(texture);

    // Create a material that uses the texture
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        cull_mode: None,
        ..default()
    });

    // Insert HexChunks resource with empty chunks (to be filled later)
    commands.insert_resource(HexChunks {
        texture: texture_handle,
        material,
        chunks: Vec::new(),
    });
}

fn update_batched_hex_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut hex_chunks: ResMut<HexChunks>,
    hex_map: Res<crate::hex::map::HexMap>,
    time: Res<Time>,
    mut timer: Local<f32>,
    mut last_hex_count: Local<usize>,
    mut static_generated: ResMut<StaticMeshGenerated>,
) {
    // If static mesh hasn't been generated yet, generate chunk meshes and spawn entities
    if !static_generated.0 {
        let width = hex_map.width();
        let height = hex_map.height();
        let chunk_size = CHUNK_SIZE;
        let mut total_hexes = 0;
        let mut total_chunks = 0;

        // Iterate over chunks
        for chunk_r in (0..height).step_by(chunk_size as usize) {
            for chunk_q in (0..width).step_by(chunk_size as usize) {
                let q_min = chunk_q;
                let r_min = chunk_r;
                let q_max = (chunk_q + chunk_size - 1).min(width - 1);
                let r_max = (chunk_r + chunk_size - 1).min(height - 1);

                // Generate mesh for this chunk
                let (positions, normals, uvs, indices, hex_types) =
                    generate_chunk_mesh(&hex_map, q_min, q_max, r_min, r_max);
                if positions.is_empty() {
                    continue; // chunk empty (no hexes)
                }

                // Create mesh asset
                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                mesh.insert_attribute(ATTRIBUTE_HEX_TYPE, hex_types);
                mesh.insert_indices(Indices::U32(indices));
                let mesh_handle = meshes.add(mesh);

                // Spawn entity for this chunk
                let entity = commands.spawn((
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(hex_chunks.material.clone()),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    Visibility::Visible,
                )).id();

                // Store chunk info
                hex_chunks.chunks.push(HexChunk {
                    mesh: mesh_handle,
                    entity,
                    q_min,
                    r_min,
                    q_max,
                    r_max,
                });

                total_hexes += (q_max - q_min + 1) * (r_max - r_min + 1);
                total_chunks += 1;
            }
        }

        println!("Static chunk meshes generated: {} chunks, {} hexes", total_chunks, total_hexes);
        static_generated.0 = true;
        *last_hex_count = total_hexes as usize;
        return;
    }

    // After static mesh generation, we could skip further updates, but we keep the logging
    // Log hex count every second (just for monitoring)
    *timer += time.delta_secs();
    if *timer >= 1.0 {
        let hex_count = hex_map.width() * hex_map.height();
        if *last_hex_count != hex_count as usize {
            println!("All hexes: {}", hex_count);
            *last_hex_count = hex_count as usize;
        }
        *timer = 0.0;
    }
}

fn generate_unit_hex_vertices(size: f32) -> Vec<(f32, f32)> {
    let rotation = std::f32::consts::PI / 6.0;
    let mut vertices = Vec::with_capacity(7);
    for i in 0..6 {
        let angle = (i as f32) * std::f32::consts::PI / 3.0 + rotation;
        let x = size * angle.cos();
        let z = size * angle.sin();
        vertices.push((x, z));
    }
    vertices.push((0.0, 0.0)); // center
    vertices
}

fn generate_full_mesh(hex_map: &crate::hex::map::HexMap) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>, Vec<f32>) {
    generate_chunk_mesh(hex_map, 0, hex_map.width() - 1, 0, hex_map.height() - 1)
}

fn generate_chunk_mesh(
    hex_map: &crate::hex::map::HexMap,
    q_min: i32,
    q_max: i32,
    r_min: i32,
    r_max: i32,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>, Vec<f32>) {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    let mut hex_types = Vec::new();

    let unit_vertices = generate_unit_hex_vertices(HEX_SIZE);
    let mut vertex_offset = 0;

    for r in r_min..=r_max {
        for q in q_min..=q_max {
            let Some(hex) = hex_map.get_hex(q, r) else {
                continue;
            };
            let (x, z) = crate::hex::utils::axial_to_pixel(&HexCoordinates::new(q, r), HEX_SIZE);
            let hex_type = *hex.hex_type();
            let type_index = hex_type_to_index(hex_type);

            // Add vertices for this hex
            for (vx, vz) in unit_vertices.iter() {
                positions.push([x + vx, 0.0, z + vz]);
                normals.push([0.0, 1.0, 0.0]);
                uvs.push([type_index as f32 / 8.0, 0.5]);
                hex_types.push(type_index as f32);
            }

            // Add indices (18 per hex)
            for i in 0..6 {
                let next = (i + 1) % 6;
                indices.extend_from_slice(&[
                    vertex_offset + i,
                    vertex_offset + next,
                    vertex_offset + 6, // center vertex
                ]);
            }
            vertex_offset += 7; // 6 outer + 1 center
        }
    }

    (positions, normals, uvs, indices, hex_types)
}

fn hex_type_to_index(hex_type: HexType) -> u8 {
    match hex_type {
        HexType::Empty => 0,
        HexType::Plains => 1,
        HexType::Forest => 2,
        HexType::Mountains => 3,
        HexType::Desert => 4,
        HexType::Ocean => 5,
        HexType::Coast => 6,
        HexType::Swamp => 7,
    }
}

// Returns the range of hex coordinates that may be visible to the camera.
fn visible_hex_range(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    hex_map: &crate::hex::map::HexMap,
    window: Option<&Window>,
) -> (i32, i32, i32, i32) {
    // Get logical viewport size (if camera doesn't provide one, use window size)
    let viewport_size = camera
        .logical_viewport_size()
        .or_else(|| window.map(|w| Vec2::new(w.width(), w.height())))
        .unwrap_or(Vec2::new(1920.0, 1080.0));

    // Four corners of the screen in viewport coordinates
    let corners = [
        Vec2::new(0.0, 0.0),
        Vec2::new(viewport_size.x, 0.0),
        Vec2::new(viewport_size.x, viewport_size.y),
        Vec2::new(0.0, viewport_size.y),
    ];

    let mut world_points = Vec::new();

    for corner in corners {
        // viewport_to_world returns Result, handle via ok()
        if let Ok(ray) = camera.viewport_to_world(camera_transform, corner) {
            // Ensure ray is not parallel to ground plane
            if ray.direction.y.abs() > f32::EPSILON {
                let t = -ray.origin.y / ray.direction.y;
                if t > 0.0 {
                    let point = ray.origin + ray.direction * t;
                    world_points.push(point);
                }
            }
        }
    }

    if world_points.is_empty() {
        return (0, hex_map.width() - 1, 0, hex_map.height() - 1);
    }

    let min_x = world_points
        .iter()
        .map(|p| p.x)
        .fold(f32::INFINITY, f32::min);
    let max_x = world_points
        .iter()
        .map(|p| p.x)
        .fold(f32::NEG_INFINITY, f32::max);
    let min_z = world_points
        .iter()
        .map(|p| p.z)
        .fold(f32::INFINITY, f32::min);
    let max_z = world_points
        .iter()
        .map(|p| p.z)
        .fold(f32::NEG_INFINITY, f32::max);

    // Hex dimensions (defined in hex::utils)
    let hex_width = HEX_WIDTH;
    let y_pitch = Y_PITCH;

    // Approximate conversion to hex coordinates (with margin)
    let min_q = ((min_x) / hex_width).floor() as i32 - 1;
    let max_q = ((max_x) / hex_width).ceil() as i32 + 1;
    let min_r = ((min_z) / y_pitch).floor() as i32 - 1;
    let max_r = ((max_z) / y_pitch).ceil() as i32 + 1;

    // Clamp to map boundaries
    let min_q = min_q.max(0);
    let max_q = max_q.min(hex_map.width() - 1);
    let min_r = min_r.max(0);
    let max_r = max_r.min(hex_map.height() - 1);

    (min_q, max_q, min_r, max_r)
}