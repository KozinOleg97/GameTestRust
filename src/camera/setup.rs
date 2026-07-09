use crate::camera::controller::CameraController;
use crate::game::GameSettings;
use crate::game::WorldGeneratedEvent;
use crate::hex::{utils::axial_to_pixel, HexCoordinates, HEX_SIZE};
use bevy::prelude::*;

pub fn setup_camera_on_world_generated(
    mut commands: Commands,
    mut events: MessageReader<WorldGeneratedEvent>,
    settings: Res<GameSettings>, // добавить
) {
    for _ in events.read() {
        let center_q = settings.generation.map_width / 2;
        let center_r = settings.generation.map_height / 2;
        let (center_x, center_z) = axial_to_pixel(&HexCoordinates::new(center_q, center_r), HEX_SIZE);

        // Создаём контроллер с параметрами из настроек
        let camera_controller = CameraController {
            pan_speed: settings.camera.pan_speed,
            zoom_speed: settings.camera.zoom_speed,
            min_fov: settings.camera.min_fov,
            max_fov: settings.camera.max_fov,
            move_speed: settings.camera.move_speed,
            rotate_speed: settings.camera.rotate_speed,
            min_pitch: settings.camera.min_pitch,
            max_pitch: settings.camera.max_pitch,
        };

        commands.spawn((
            Camera3d::default(),
            Projection::Perspective(PerspectiveProjection {
                far: 5000.0, // увеличить
                near: 1.0,
                fov: 90.0f32.to_radians(),
                ..default()
            }),
            Transform::from_xyz(center_x, 2000.0, center_z)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            camera_controller,
        ));

        // Add directional light pointing straight down
        let light_transform =
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));
        let light_forward = light_transform.forward();
        println!("Directional light transform: {:?}", light_transform);
        println!("Light forward direction: {:?}", light_forward);
        commands.spawn((
            DirectionalLight {
                illuminance: 1000.0,
                shadow_maps_enabled: false, // disable shadows for simplicity
                ..default()
            },
            light_transform,
        ));
    }
}
