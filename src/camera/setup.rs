use crate::camera::controller::{CameraController, CameraOrbit};
use crate::game::{GameSessionState, GameSettings, WorldGeneratedEvent};
use crate::rendering::hex_world_position;
use bevy::prelude::*;

/// Ставит камеру и свет, когда мир сгенерирован.
///
/// Событие — повод поставить камеру, но не повод поставить вторую: повторная
/// генерация в том же сеансе не должна плодить камеры и источники света. Камера
/// живёт ровно столько, сколько состояние `Active`.
pub fn setup_camera_on_world_generated(
    mut commands: Commands,
    mut events: MessageReader<WorldGeneratedEvent>,
    settings: Res<GameSettings>,
    cameras: Query<(), With<CameraController>>,
) {
    if events.read().count() == 0 || !cameras.is_empty() {
        return;
    }

    // 1. Центр карты в гексовых координатах
    let center_q = settings.generation.map_width / 2;
    let center_r = settings.generation.map_height / 2;

    // 2. Мировые координаты центра (Vec3 с x, y=0, z)
    let center_world = hex_world_position(center_q, center_r);

    // Параметры камеры
    let camera_height = 400.0; // Высота над картой
    let camera_pitch = -1.0; // Наклон ~57° вниз (чтобы видеть рельеф)

    let camera_position = Vec3::new(center_world.x, camera_height, center_world.z);

    // Ориентация задаётся углами — тем же состоянием, которым потом управляет
    // контроллер. Поэтому начальный наклон и ограничение тангажа согласованы,
    // а не заданы дважды в разных представлениях.
    let orbit = CameraOrbit {
        yaw: 0.0,
        pitch: camera_pitch,
    };

    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Default,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: 60.0_f32.to_radians(),
            near: 1.0,
            far: 5000.0,
            ..default()
        }),
        Transform::from_translation(camera_position).with_rotation(orbit.rotation()),
        CameraController,
        orbit,
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
