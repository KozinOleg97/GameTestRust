use crate::hex::{HexType, HEX_SIZE};
use crate::hex::utils::{HEX_WIDTH, Y_PITCH};
use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::{
        render_resource::{Extent3d, PrimitiveTopology, TextureDimension, TextureFormat},
    },
};

// Вершины гекса в порядке, используемом в Java-рендерере:
// 0 – нижняя, 1 – нижний правый, 2 – верхний правый,
// 3 – верхняя, 4 – верхний левый, 5 – нижний левый
const HEX_CORNERS: [(f32, f32); 6] = [
    (0.0, -HEX_SIZE),               // V0
    (HEX_WIDTH / 2.0, -HEX_SIZE / 2.0), // V1
    (HEX_WIDTH / 2.0, HEX_SIZE / 2.0),  // V2
    (0.0, HEX_SIZE),                // V3
    (-HEX_WIDTH / 2.0, HEX_SIZE / 2.0), // V4
    (-HEX_WIDTH / 2.0, -HEX_SIZE / 2.0), // V5
];

const CHUNK_SIZE: i32 = 20;

#[derive(Resource)]
pub struct TriangleStripHexChunks {
    pub texture: Handle<Image>,
    pub material: Handle<StandardMaterial>,
    pub chunks: Vec<TriangleStripHexChunk>,
}

pub struct TriangleStripHexChunk {
    pub mesh: Handle<Mesh>,
    pub entity: Entity,
    pub q_min: i32,
    pub r_min: i32,
    pub q_max: i32,
    pub r_max: i32,
}

#[derive(Resource, Default)]
struct StaticMeshGenerated(bool);

pub struct TriangleStripHexRendererPlugin;

impl Plugin for TriangleStripHexRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_assets)
            .add_systems(Update, generate_meshes)
            .init_resource::<StaticMeshGenerated>();
    }
}

fn setup_assets(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Создаём текстуру-палитру 8×1 с фильтрацией Nearest
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

    let mut texture = Image::new(
        Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    texture.sampler = ImageSampler::nearest(); 

    let texture_handle = images.add(texture);

    // Материал без освещения (unlit) – цвет берётся прямо из текстуры
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        unlit: true,   // отключаем освещение – плоская заливка
        cull_mode: None,
        ..default()
    });

    commands.insert_resource(TriangleStripHexChunks {
        texture: texture_handle,
        material,
        chunks: Vec::new(),
    });
}

fn generate_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut hex_chunks: ResMut<TriangleStripHexChunks>,
    hex_map: Res<crate::hex::map::HexMap>,
    mut static_generated: ResMut<StaticMeshGenerated>,
) {
    if static_generated.0 {
        return;
    }

    let width = hex_map.width();
    let height = hex_map.height();
    let chunk_size = CHUNK_SIZE;

    for r_min in (0..height).step_by(chunk_size as usize) {
        for q_min in (0..width).step_by(chunk_size as usize) {
            let q_max = (q_min + chunk_size - 1).min(width - 1);
            let r_max = (r_min + chunk_size - 1).min(height - 1);

            let (positions, uvs, hex_types) =
                generate_chunk_mesh(&hex_map, q_min, q_max, r_min, r_max);
            if positions.is_empty() {
                continue;
            }

            let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip, RenderAssetUsages::default());
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            // Сохраняем тип гекса для возможных будущих расширений
            mesh.insert_attribute(crate::rendering::batched_hex_renderer::ATTRIBUTE_HEX_TYPE, hex_types);

            let mesh_handle = meshes.add(mesh);
            let entity = commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(hex_chunks.material.clone()),
                Transform::IDENTITY,
                Visibility::Visible,
            )).id();

            hex_chunks.chunks.push(TriangleStripHexChunk {
                mesh: mesh_handle,
                entity,
                q_min,
                r_min,
                q_max,
                r_max,
            });
        }
    }

    println!("Triangle strip chunks generated: {} chunks", hex_chunks.chunks.len());
    static_generated.0 = true;
}

fn generate_chunk_mesh(
    hex_map: &crate::hex::map::HexMap,
    q_min: i32,
    q_max: i32,
    r_min: i32,
    r_max: i32,
) -> (Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<f32>) {
    let mut positions = Vec::new();
    let mut uvs = Vec::new();
    let mut hex_types = Vec::new();

    for r in r_min..=r_max {
        let odd_offset = if r % 2 == 1 { HEX_WIDTH / 2.0 } else { 0.0 };
        let y = Y_PITCH * r as f32;

        // Начало строки: V5, V5, V4
        let (v5x, v5z) = HEX_CORNERS[5];
        let (v4x, v4z) = HEX_CORNERS[4];
        for _ in 0..2 {
            positions.push([v5x, 0.0, y + v5z]);
            uvs.push([0.0, 0.5]);
            hex_types.push(0.0);
        }
        positions.push([v4x, 0.0, y + v4z]);
        uvs.push([0.0, 0.5]);
        hex_types.push(0.0);

        // Основные гексы строки
        for q in q_min..=q_max {
            let hex = hex_map.get_hex(q, r).unwrap();
            let hex_type = *hex.hex_type();
            let type_index = hex_type_to_index(hex_type) as f32;
            let uv_u = type_index / 8.0;
            let x = HEX_WIDTH * q as f32 + odd_offset;

            // Порядок вершин: V0, V3, V2, V4
            let (v0x, v0z) = HEX_CORNERS[0];
            let (v3x, v3z) = HEX_CORNERS[3];
            let (v2x, v2z) = HEX_CORNERS[2];
            let (v4x, v4z) = HEX_CORNERS[4];

            positions.push([x + v0x, 0.0, y + v0z]);
            uvs.push([uv_u, 0.5]);
            hex_types.push(type_index);

            positions.push([x + v3x, 0.0, y + v3z]);
            uvs.push([uv_u, 0.5]);
            hex_types.push(type_index);

            positions.push([x + v2x, 0.0, y + v2z]);
            uvs.push([uv_u, 0.5]);
            hex_types.push(type_index);

            positions.push([x + v4x, 0.0, y + v4z]);
            uvs.push([uv_u, 0.5]);
            hex_types.push(type_index);
        }

        // Завершение строки: V3, V3
        let (v3x, v3z) = HEX_CORNERS[3];
        for _ in 0..2 {
            positions.push([v3x, 0.0, y + v3z]);
            uvs.push([0.0, 0.5]);
            hex_types.push(0.0);
        }
    }

    (positions, uvs, hex_types)
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