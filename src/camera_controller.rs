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
    pub pan_speed: f32,    // чувствительность панорамирования
    pub zoom_speed: f32,   // скорость зума
    pub min_fov: f32,      // минимальный угол обзора в градусах
    pub max_fov: f32,      // максимальный угол обзора в градусах
    pub move_speed: f32,   // скорость движения WASD
    pub rotate_speed: f32, // чувствительность вращения
    pub min_pitch: f32,    // минимальный угол наклона (pitch) в градусах
    pub max_pitch: f32,    // максимальный угол наклона (pitch) в градусах
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            pan_speed: 1.0,
            zoom_speed: 0.05,
            min_fov: 1.0,
            max_fov: 150.0,
            move_speed: 500.0,
            rotate_speed: 0.005, // чувствительность вращения (радианы на пиксель)
            min_pitch: -90.0,    // минимальный угол наклона (градусы)
            max_pitch: 90.0,     // максимальный угол наклона (градусы)
        }
    }
}

fn camera_control_system(
    mouse_btn: Res<ButtonInput<MouseButton>>,
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

    // Собираем общую дельту движения мыши за кадр
    let mut mouse_delta = Vec2::ZERO;
    for ev in mouse_motion.read() {
        mouse_delta += ev.delta;
    }

    // --- Панорамирование (drag) ---
    if mouse_btn.pressed(MouseButton::Left) && mouse_delta != Vec2::ZERO {
        let screen_size = Vec2::new(window.width(), window.height());
        let center = screen_size / 2.0;
        if let Ok(ray_center) = camera.viewport_to_world(global_transform, center) {
            let target_screen_pos = center + mouse_delta;
            if let Ok(ray_target) = camera.viewport_to_world(global_transform, target_screen_pos) {
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

    // --- Вращение вокруг цели (правой кнопкой мыши) ---
    if mouse_btn.pressed(MouseButton::Right) && mouse_delta != Vec2::ZERO {
        // Вычисляем целевую точку на плоскости y=0 под камерой (в центре экрана)
        let screen_size = Vec2::new(window.width(), window.height());
        let center = screen_size / 2.0;
        let target_point = if let Ok(ray) = camera.viewport_to_world(global_transform, center) {
            if ray.direction.y.abs() > f32::EPSILON {
                let t = -ray.origin.y / ray.direction.y;
                if t > 0.0 {
                    Some(ray.origin + ray.direction * t)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        match target_point {
            Some(target) => {
                // Вычисляем углы вращения
                let yaw_delta = -mouse_delta.x * controller.rotate_speed;
                let pitch_delta = -mouse_delta.y * controller.rotate_speed;

                // Текущие углы (из кватерниона) можно вычислить, но проще вращать напрямую
                // Создаём кватернионы вращения
                let yaw_rot = Quat::from_axis_angle(Vec3::Y, yaw_delta);
                let pitch_rot = Quat::from_axis_angle(*transform.local_x(), pitch_delta);

                // Комбинируем вращения: сначала рыскание (yaw) вокруг мировой оси Y,
                // затем тангаж (pitch) вокруг локальной оси X камеры
                let combined_rot = yaw_rot * pitch_rot;

                // Применяем вращение вокруг целевой точки
                transform.rotate_around(target, combined_rot);

                // Ограничиваем угол наклона (pitch) чтобы камера не переворачивалась
                let (_, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                let pitch_deg = pitch.to_degrees();
                let clamped_pitch = pitch_deg.clamp(controller.min_pitch, controller.max_pitch);
                if pitch_deg != clamped_pitch {
                    // Корректируем вращение до допустимого угла
                    let correction = Quat::from_axis_angle(
                        *transform.local_x(),
                        (clamped_pitch - pitch_deg).to_radians(),
                    );
                    transform.rotate_around(target, correction);
                }
            }
            None => {}
        }
    }

    // --- Зумирование (FOV) ---
    let mut zoom_delta = 0.0;
    for ev in scroll.read() {
        zoom_delta += ev.y;
    }
    if zoom_delta != 0.0 {
        match *projection {
            Projection::Perspective(ref mut persp) => {
                let new_fov = (persp.fov - zoom_delta * controller.zoom_speed).clamp(
                    controller.min_fov.to_radians(),
                    controller.max_fov.to_radians(),
                );
                persp.fov = new_fov;
            }
            _ => {}
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
