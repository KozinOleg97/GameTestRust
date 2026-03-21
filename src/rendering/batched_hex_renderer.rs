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

#[derive(Resource)]
pub struct BatchedHexMesh {
    pub mesh: Handle<Mesh>,
    pub texture: Handle<Image>,
    pub material: Handle<StandardMaterial>,
    pub entity: Option<Entity>,
}

pub struct BatchedHexRendererPlugin;

impl Plugin for BatchedHexRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_batched_hex_assets)
            .add_systems(Update, update_batched_hex_mesh);
    }
}

fn setup_batched_hex_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
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

    // Create an empty mesh (will be populated each frame)
    let mesh = meshes.add(Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    ));

    // Spawn a single entity for the batched mesh
    let entity = commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::Visible,
    )).id();

    commands.insert_resource(BatchedHexMesh {
        mesh: mesh.clone(),
        texture: texture_handle,
        material,
        entity: Some(entity),
    });
}

fn update_batched_hex_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    batched_mesh: Res<BatchedHexMesh>,
    hex_map: Res<crate::hex::map::HexMap>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let (camera, camera_transform) = match camera_query.single() {
        Ok(pair) => pair,
        Err(_) => return,
    };
    let window = window_query.iter().next();

    // Compute visible hex range using local function
    let (min_q, max_q, min_r, max_r) = visible_hex_range(
        camera,
        camera_transform,
        &hex_map,
        window,
    );

    // Build mesh data
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    let mut hex_types = Vec::new();

    // Precompute unit hex vertices (same as in hex_renderer)
    let unit_vertices = generate_unit_hex_vertices(HEX_SIZE);

    let mut vertex_offset = 0;
    for r in min_r..=max_r {
        for q in min_q..=max_q {
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
                uvs.push([type_index as f32 / 8.0, 0.5]); // u based on type, v center
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

    // Update the mesh
    if let Some(mesh) = meshes.get_mut(&batched_mesh.mesh) {
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(ATTRIBUTE_HEX_TYPE, hex_types);
        mesh.insert_indices(Indices::U32(indices));
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