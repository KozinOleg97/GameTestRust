use crate::generation::ProceduralWorldGenerator;
use crate::hex::coordinates::HexCoordinates;
use crate::hex::map::HexMap;
use crate::hex::utils::{axial_to_pixel, HEX_SIZE};
use crate::rendering::HexRendererPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy::math::Vec2;
use std::collections::HashMap;

use camera_controller::{CameraController, CameraControllerPlugin};

mod generation;
mod hex;
mod performance_overlay;
mod rendering;
mod camera_controller;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            HexRendererPlugin,
            performance_overlay::PerformanceOverlayPlugin,
            CameraControllerPlugin,
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
        CameraController::default(),
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
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    hex_query: Query<(Entity, &rendering::HexPositionComponent)>,
) {
    let (camera, camera_transform) = match camera_query.single() {
        Ok(pair) => pair,
        Err(_) => {
            println!("No camera found, skipping rendering");
            return;
        }
    };

    let window = window_query.iter().next();

    // Вычисляем видимый диапазон координат
    let (min_q, max_q, min_r, max_r) =
        visible_hex_range(camera, camera_transform, &hex_map, window);

    // Создаём сущности только для гексов в видимом диапазоне
    let mut visible_coords = std::collections::HashSet::new();
    for r in min_r..=max_r {
        for q in min_q..=max_q {
            if hex_map.get_hex(q, r).is_some() {
                visible_coords.insert(HexCoordinates::new(q, r));
            }
        }
    }

    // Collect existing hex entities into a map
    let mut existing_hexes = std::collections::HashMap::new();
    for (entity, pos) in hex_query.iter() {
        existing_hexes.insert(pos.coordinates, entity);
    }

    // Spawn new hexes for coordinates that are not already spawned
    for coords in &visible_coords {
        if !existing_hexes.contains_key(coords) {
            let hex = hex_map.get_hex(coords.q(), coords.r()).unwrap(); // safe because we checked earlier
            let (x, y) = axial_to_pixel(coords, HEX_SIZE);
            commands.spawn((
                rendering::HexComponent,
                rendering::HexTypeComponent {
                    hex_type: *hex.hex_type(),
                },
                rendering::HexPositionComponent {
                    coordinates: *coords,
                },
                Mesh3d(hex_mesh.mesh.clone()),
                MeshMaterial3d(hex_materials.materials[hex.hex_type()].clone()),
                Transform::from_xyz(x, 0.0, y),
            ));
        }
    }

    // Despawn hexes that are no longer visible
    for (coords, entity) in existing_hexes.iter() {
        if !visible_coords.contains(coords) {
            commands.entity(*entity).despawn();
        }
    }
}

// Возвращает диапазон координат гексов, которые могут быть видны камерой.
fn visible_hex_range(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    hex_map: &HexMap,
    window: Option<&Window>,
) -> (i32, i32, i32, i32) {
    // Получаем логический размер вьюпорта (если камера не задаёт свой – используем размер окна)
    let viewport_size = camera
        .logical_viewport_size()
        .or_else(|| window.map(|w| Vec2::new(w.width(), w.height())))
        .unwrap_or(Vec2::new(1920.0, 1080.0));

    // Четыре угла экрана в координатах вьюпорта
    let corners = [
        Vec2::new(0.0, 0.0),
        Vec2::new(viewport_size.x, 0.0),
        Vec2::new(viewport_size.x, viewport_size.y),
        Vec2::new(0.0, viewport_size.y),
    ];

    let mut world_points = Vec::new();

    for corner in corners {
        // viewport_to_world возвращает Result, обрабатываем через ok()
        if let Ok(ray) = camera.viewport_to_world(camera_transform, corner) {
            // Проверяем, что луч не параллелен плоскости земли
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

    // Размеры гекса (определены в hex::utils)
    let hex_width = crate::hex::HEX_WIDTH;
    let y_pitch = crate::hex::Y_PITCH;

    // Приблизительное преобразование в координаты гексов (с запасом)
    let min_q = ((min_x) / hex_width).floor() as i32 - 1;
    let max_q = ((max_x) / hex_width).ceil() as i32 + 1;
    let min_r = ((min_z) / y_pitch).floor() as i32 - 1;
    let max_r = ((max_z) / y_pitch).ceil() as i32 + 1;

    // Ограничиваем границами карты
    let min_q = min_q.max(0);
    let max_q = max_q.min(hex_map.width() - 1);
    let min_r = min_r.max(0);
    let max_r = max_r.min(hex_map.height() - 1);

    (min_q, max_q, min_r, max_r)
}

fn debug_frame(time: Res<Time>, mut timer: Local<f32>) {
    *timer += time.delta_secs();
    if *timer >= 1.0 {
        println!("Frame update - app is still running");
        *timer = 0.0;
    }
}
