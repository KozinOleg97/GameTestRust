use crate::generation::ProceduralWorldGenerator;
use crate::hex::coordinates::HexCoordinates;
use crate::hex::map::HexMap;
use crate::hex::utils::{axial_to_pixel, HEX_SIZE};
use crate::rendering::HexRendererPlugin;
use bevy::prelude::*;

mod generation;
mod hex;
mod performance_overlay;
mod rendering;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            HexRendererPlugin,
            performance_overlay::PerformanceOverlayPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (render_world, debug_frame))
        .run();
}

fn setup(mut commands: Commands) {
    // Set clear color to blue to verify window rendering
    commands.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));

    // camera - position it to view the center of the hex map
    let center_q = 50;
    let center_r = 50;
    let (center_x, center_z) = axial_to_pixel(&HexCoordinates::new(center_q, center_r), HEX_SIZE);
    println!("Camera target: ({}, 0, {})", center_x, center_z);
    // Position camera higher and with wider FOV to see more hexes
    let camera_pos = Vec3::new(center_x, 2000.0, center_z);
    let camera_transform = Transform::from_xyz(camera_pos.x, camera_pos.y, camera_pos.z)
        .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)); // look straight down
    println!("Camera position: {:?}", camera_pos);
    println!("Camera transform: {:?}", camera_transform);
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            far: 10000.0,
            near: 1.0,
            fov: 90.0f32.to_radians(), // wider field of view
            ..default()
        }),
        camera_transform,
    ));

    // Generate a simple world
    let generator = ProceduralWorldGenerator::new(1000, 1000, 12345);
    let hex_map = generator.generate_world();

    // Store the hex map as a resource
    commands.insert_resource(hex_map);

    // Add directional light pointing straight down
    let light_transform =
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));
    let light_forward = light_transform.forward();
    println!("Directional light transform: {:?}", light_transform);
    println!("Light forward direction: {:?}", light_forward);
    commands.spawn((
        DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: false, // disable shadows for simplicity
            ..default()
        },
        light_transform,
    ));
}

fn render_world(
    mut commands: Commands,
    hex_map: Res<HexMap>,
    hex_mesh: Res<rendering::HexMesh>,
    hex_materials: Res<rendering::HexMaterials>,
    mut has_rendered: Local<bool>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // Only render once
    if *has_rendered {
        return;
    }

    println!("Rendering {} hexes", hex_map.size());

    // Get the camera
    let Ok((camera, camera_transform)) = camera_query.single() else {
        println!("No camera found, skipping rendering");
        return;
    };

    let mut rendered_count = 0;

    // Render all hexes that are visible to the camera
    for (i, hex) in hex_map.iter().enumerate() {
        let coordinates = hex.coordinates();
        let (x, y) = crate::hex::utils::axial_to_pixel(coordinates, crate::hex::HEX_SIZE);

        // World position of the hex (ground plane)
        let world_pos = Vec3::new(x, 0.0, y);

        // Если гекс не виден, пропускаем
        if camera
            .world_to_viewport(camera_transform, world_pos)
            .is_err()
        {
            continue;
        }

        // println!("Rendering hex at ({}, {}) with type {:?} at position ({}, {})",
        //     coordinates.q(), coordinates.r(), hex.hex_type(), x, y);

        commands.spawn((
            rendering::HexComponent,
            rendering::HexTypeComponent {
                hex_type: *hex.hex_type(),
            },
            rendering::HexPositionComponent {
                coordinates: *coordinates,
            },
            Mesh3d(hex_mesh.mesh.clone()),
            MeshMaterial3d(hex_materials.materials[hex.hex_type()].clone()),
            Transform::from_xyz(x, 0.0, y),
        ));

        rendered_count += 1;

        // For debugging, only render first 10 hexes
        // if i >= 9 {
        //     break;
        // }
    }

    *has_rendered = true;
    println!(
        "Finished rendering {} hexes (culled to {})",
        hex_map.size(),
        rendered_count
    );
}

fn debug_frame(time: Res<Time>, mut timer: Local<f32>) {
    *timer += time.delta_secs();
    if *timer >= 1.0 {
        println!("Frame update - app is still running");
        *timer = 0.0;
    }
}
