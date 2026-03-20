use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, VertexAttributeValues},
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use crate::hex::{HexType, HexCoordinates, axial_to_pixel, HEX_SIZE};

// Components for hex entities
#[derive(Component)]
pub struct HexComponent;

#[derive(Component)]
pub struct HexTypeComponent {
    pub hex_type: HexType,
}

#[derive(Component)]
pub struct HexPositionComponent {
    pub coordinates: HexCoordinates,
}

// Resources for hex rendering
#[derive(Resource, Default)]
pub struct HexMaterials {
    pub materials: std::collections::HashMap<HexType, Handle<StandardMaterial>>,
}

#[derive(Resource, Default)]
pub struct HexMesh {
    pub mesh: Handle<Mesh>,
}

// Plugin for hex rendering
pub struct HexRendererPlugin;

impl Plugin for HexRendererPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<HexMaterials>()
            .init_resource::<HexMesh>()
            .add_systems(Startup, setup_hex_assets);
    }
}

// System to set up hex assets
fn setup_hex_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Setting up hex assets");
    
    // Create hex mesh
    let hex_mesh = meshes.add(generate_hex_mesh(HEX_SIZE));
    println!("Created hex mesh");
    
    // Create materials for each hex type
    let mut hex_materials = std::collections::HashMap::new();
    
    for hex_type in &[
        HexType::Plains,
        HexType::Forest,
        HexType::Mountains,
        HexType::Desert,
        HexType::Ocean,
        HexType::Coast,
        HexType::Swamp,
    ] {
        let color = hex_type.color();
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(color.0, color.1, color.2, color.3),
            cull_mode: None, // disable backface culling
            ..default()
        });
        println!("Created material for {:?} with color {:?}", hex_type, color);
        hex_materials.insert(*hex_type, material);
    }
    
    commands.insert_resource(HexMesh { mesh: hex_mesh });
    commands.insert_resource(HexMaterials {
        materials: hex_materials,
    });
    
    println!("Finished setting up hex assets");
}

// Generate a hex mesh
fn generate_hex_mesh(size: f32) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    
    // Generate hex vertices with flat sides horizontal (flat-top orientation)
    // Rotate by π/6 so that a flat side is aligned with the X axis
    let rotation = std::f32::consts::PI / 6.0;
    for i in 0..6 {
        let angle = (i as f32) * std::f32::consts::PI / 3.0 + rotation;
        let x = size * angle.cos();
        let z = size * angle.sin();
        positions.push([x, 0.0, z]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([(x / size + 1.0) / 2.0, (z / size + 1.0) / 2.0]);
    }
    
    // Add center vertex
    positions.push([0.0, 0.0, 0.0]);
    normals.push([0.0, 1.0, 0.0]);
    uvs.push([0.5, 0.5]);
    
    // Generate indices for triangles
    for i in 0..6 {
        let next = (i + 1) % 6;
        indices.extend_from_slice(&[i, next, 6]); // 6 is the center vertex
    }
    
    println!("Generated hex mesh with {} vertices and {} indices", positions.len(), indices.len());
    
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices))
}