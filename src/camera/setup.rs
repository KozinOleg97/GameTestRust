use crate::camera::controller::CameraController;
use crate::game::{GameSettings, GameSessionState};
use crate::game::WorldGeneratedEvent;
use crate::hex::{utils::axial_to_pixel, HexCoordinates, HEX_SIZE};
use bevy::prelude::*;

pub fn setup_camera_on_world_generated(
    mut commands: Commands,
    mut events: MessageReader<WorldGeneratedEvent>,
    settings: Res<GameSettings>,
) {
    for _ in events.read() {
        let center_q = settings.generation.map_width / 2;
        let center_r = settings.generation.map_height / 2;

        let center_pixel = axial_to_pixel(&HexCoordinates::new(center_q, center_r), HEX_SIZE);

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

        // Метод extend(2000.0) берёт 2D-вектор (X, Z) и добавляет третью координату (Y = 2000.0)
        let camera_transform = Transform::from_translation(center_pixel.extend(2000.0))
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));

        commands.spawn((
            Camera3d::default(),
            Camera {
                order: 0,
                clear_color: ClearColorConfig::Default,
                ..default()
            },
            Projection::Perspective(PerspectiveProjection {
                far: 5000.0, // увеличить
                near: 1.0,
                fov: 90.0f32.to_radians(),
                ..default()
            }),
            camera_transform,
            camera_controller,
            DespawnOnExit(GameSessionState::Active), // Удаляется при выходе из состояния игры
        ));

        // Add directional light pointing straight down
        let light_transform = Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));

        println!("Directional light transform: {:?}", light_transform);
        println!("Light forward direction: {:?}", light_transform.forward());

        commands.spawn((
            DirectionalLight {
                illuminance: 1000.0,
                shadow_maps_enabled: false, // disable shadows for simplicity
                ..default()
            },
            light_transform,
            DespawnOnExit(GameSessionState::Active), // Свет удаляется
        ));
    }
}
