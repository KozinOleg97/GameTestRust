use crate::game::WorldGeneratedEvent;
use crate::hex::map::HexMap;
use crate::rendering::materials::setup_hex_materials;
use crate::rendering::mesh_builder::generate_full_mesh;
use bevy::prelude::*;

pub struct FullMeshRenderingPlugin;

impl Plugin for FullMeshRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_rendering_resources);
        app.add_systems(
            Update,
            create_full_mesh_system.run_if(on_message::<WorldGeneratedEvent>),
        );
    }
}

/// Ресурс, хранящий handle материала и текстуры
#[derive(Resource)]
pub struct HexRenderResources {
    pub texture: Handle<Image>,
    pub material: Handle<StandardMaterial>,
}

/// Ресурс для хранения entity всей карты (чтобы при повторной генерации удалять старую)
#[derive(Resource)]
struct MapEntity(Entity);

fn init_rendering_resources(
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let (texture, material) = setup_hex_materials(images, materials);
    commands.insert_resource(HexRenderResources { texture, material });
}

fn create_full_mesh_system(
    hex_map: Res<HexMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    render_res: Res<HexRenderResources>,
    mut commands: Commands,
    mut events: MessageReader<WorldGeneratedEvent>,
    map_entity: Option<Res<MapEntity>>,
) {
    for _ in events.read() {
        // Despawn old entity if it exists
        if let Some(ref map_entity) = map_entity {
            commands.entity(map_entity.0).despawn();
        }

        let mesh = generate_full_mesh(&hex_map);
        let mesh_handle = meshes.add(mesh);

        let entity = commands
            .spawn((
                Mesh3d(mesh_handle),
                MeshMaterial3d(render_res.material.clone()),
                Transform::IDENTITY,
                Visibility::Visible,
            ))
            .id();

        commands.insert_resource(MapEntity(entity));
    }
}
