use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_control_system);
    }
}

#[derive(Component)]
pub struct CameraController {
    pub pan_speed: f32, // чувствительность панорамирования
    pub zoom_speed: f32, // скорость зума
    pub min_fov: f32,   // минимальный угол обзора в градусах
    pub max_fov: f32,   // максимальный угол обзора в градусах
    pub move_speed: f32, // скорость движения WASD
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            pan_speed: 1.0,
            zoom_speed: 0.05,
            min_fov: 1.0,
            max_fov: 900.0,
            move_speed: 500.0,
        }
    }
}

fn camera_control_system(
    mut mouse_btn: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut scroll: MessageReader<MouseWheel>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut Transform,
            &mut Projection,
            &CameraController,
            &Camera,
            &GlobalTransform,
        ),
        With<Camera>,
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok((mut transform, mut projection, controller, camera, global_transform)) =
        query.single_mut()
    else {
        return;
    };
    let Ok(window) = window_query.single() else {
        return;
    };

    // --- Панорамирование (drag) ---
    if mouse_btn.pressed(MouseButton::Left) {
        let mut total_delta = Vec2::ZERO;
        for ev in mouse_motion.read() {
            total_delta += ev.delta;
        }
        if total_delta != Vec2::ZERO {
            let screen_size = Vec2::new(window.width(), window.height());
            let center = screen_size / 2.0;
            if let Ok(ray_center) = camera.viewport_to_world(global_transform, center) {
                let target_screen_pos = center + total_delta;
                if let Ok(ray_target) =
                    camera.viewport_to_world(global_transform, target_screen_pos)
                {
                    let plane_y = 0.0;
                    let t_center = (plane_y - ray_center.origin.y) / ray_center.direction.y;
                    let t_target = (plane_y - ray_target.origin.y) / ray_target.direction.y;
                    if t_center > 0.0 && t_target > 0.0 {
                        let world_center = ray_center.origin + ray_center.direction * t_center;
                        let world_target = ray_target.origin + ray_target.direction * t_target;
                        let world_delta = world_target - world_center;
                        // Двигаем камеру в противоположную сторону, чтобы точка под курсором осталась на месте
                        transform.translation.x -= world_delta.x;
                        transform.translation.z -= world_delta.z;
                    }
                }
            }
        }
    }

    // --- Зумирование (FOV) ---
    let mut zoom_delta = 0.0;
    for ev in scroll.read() {
        zoom_delta += ev.y;
    }
    if zoom_delta != 0.0 {
        if let Projection::Perspective(ref mut persp) = *projection {
            let new_fov = (persp.fov - zoom_delta * controller.zoom_speed).clamp(
                controller.min_fov.to_radians(),
                controller.max_fov.to_radians(),
            );
            persp.fov = new_fov;
        }
    }

    // --- Перемещение WASD ---
    let mut move_delta = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        move_delta.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        move_delta.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        move_delta.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        move_delta.x += 1.0;
    }
    if move_delta != Vec3::ZERO {
        move_delta = move_delta.normalize_or_zero();
        let speed = controller.move_speed * time.delta_secs();
        transform.translation.x += move_delta.x * speed;
        transform.translation.z += move_delta.z * speed;
    }
}
