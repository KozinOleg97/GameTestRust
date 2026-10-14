use crate::camera::controller::CameraController;
use crate::game::{GameSessionState, GameSettings, WorldGeneratedEvent};
use crate::rendering::hex_world_position;
use bevy::prelude::*;

pub fn setup_camera_on_world_generated(
    mut commands: Commands,
    mut events: MessageReader<WorldGeneratedEvent>,
    settings: Res<GameSettings>,
) {
    for _ in events.read() {
        // 1. Вычисляем центр карты в гексовых координатах
        let center_q = settings.generation.map_width / 2;
        let center_r = settings.generation.map_height / 2;

        // 2. Получаем мировые координаты центра (Vec3 с x, y=0, z)
        let center_world = hex_world_position(center_q, center_r);

        // Параметры камеры
        let camera_height = 400.0; // Высота над картой
        let camera_pitch = -1.0; // Наклон ~57° вниз (чтобы видеть рельеф)

        // X = позиция центра карты по X
        // Y = camera_height (ВВЕРХ!)
        // Z = позиция центра карты по Z
        let camera_position = Vec3::new(
            center_world.x, // X из центра карты
            camera_height,  // Y = высота над картой
            center_world.z, // Z из центра карты
        );

        // Это позволяет видеть высоту гексов (рельеф)
        let camera_rotation = Quat::from_euler(
            EulerRot::YXZ,
            camera_pitch, // Наклон вниз
            0.0,          // Нет поворота по Y
            0.0,          // Нет крена
        )
        .normalize();

        let camera_transform =
            Transform::from_translation(camera_position).with_rotation(camera_rotation);

        commands.spawn((
            Camera3d::default(),
            Camera {
                order: 0,
                clear_color: ClearColorConfig::Default,
                ..default()
            },
            Projection::Perspective(PerspectiveProjection {
                fov: 60.0_f32.to_radians(),
                near: 1.0,
                far: 5000.0,
                ..default()
            }),
            camera_transform,
            CameraController,
            DespawnOnExit(GameSessionState::Active),
        ));

        // Свет под углом для создания объёма (а не строго сверху)
        let light_rotation = Quat::from_euler(
            EulerRot::XYZ,
            -0.8, // ~45° наклон
            0.5,  // Лёгкий поворот
            0.0,
        )
        .normalize();

        commands.spawn((
            DirectionalLight {
                illuminance: 15000.0,
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_rotation(light_rotation),
            DespawnOnExit(GameSessionState::Active),
        ));

        info!(
            "Camera spawned: center=({}, {}), position={:?}",
            center_q, center_r, camera_position
        );
    }
}
