use crate::game::GameSettings;
use crate::pointer::PointerBlocked;
use crate::rendering::hex_math::ray_ground_point;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Маркер управляемой камеры.
///
/// Настроек здесь нет намеренно: их единственный источник — `GameSettings`, и
/// система читает их каждый кадр. Копия в компоненте означала бы, что правка
/// настроек не действует до следующей генерации мира.
#[derive(Component)]
pub struct CameraController;

/// Точка, за которую держится перетаскивание.
///
/// Запоминается в момент нажатия: поза камеры, точка мира под курсором и позиция
/// камеры. Дальше камера считает себя от этого якоря, а не накапливает сдвиги по
/// кадрам, — поэтому точка под курсором не уезжает и ошибка не накапливается.
struct DragAnchor {
    pose: Transform,
    ground_point: Vec3,
    camera_position: Vec3,
}

impl DragAnchor {
    /// Ставит якорь по текущему положению курсора.
    fn new(transform: &Transform, camera: &Camera, cursor: Vec2) -> Option<Self> {
        Some(Self {
            pose: *transform,
            ground_point: ground_point_at(transform, camera, cursor)?,
            camera_position: transform.translation,
        })
    }

    /// Мировая точка под курсором, посчитанная в позе якоря.
    fn ground_point_now(&self, camera: &Camera, cursor: Vec2) -> Option<Vec3> {
        ground_point_at(&self.pose, camera, cursor)
    }
}

/// Мировая точка под экранной позицией.
///
/// Глобальная матрица считается здесь из локальной, а не читается из
/// `GlobalTransform`: тот пересчитывается в `PostUpdate`, поэтому в `Update`
/// доступно значение прошлого кадра — и луч отставал бы от камеры на кадр.
/// Камера — корневая сущность, так что её локальное преобразование и есть мировое.
fn ground_point_at(transform: &Transform, camera: &Camera, cursor: Vec2) -> Option<Vec3> {
    let global = GlobalTransform::from(*transform);
    let ray = camera.viewport_to_world(&global, cursor).ok()?;

    ray_ground_point(ray.origin, *ray.direction)
}

/// Состояние жестов камеры.
///
/// Захват решается один раз, в момент нажатия: панель забирает жест себе, но
/// начатый в мире жест она не обрывает. Якоря — те же данные, которые не дают
/// панорамированию и вращению уезжать.
///
/// `pub(crate)` — потому что тип виден в подписи системы, которую регистрирует
/// плагин из другого модуля.
#[derive(Default)]
pub(crate) struct CameraDrag {
    pan: bool,
    rotate: bool,
    pan_anchor: Option<DragAnchor>,
    rotate_anchor: Option<DragAnchor>,
}

// Список параметров — свойство ECS-системы, группировать тут нечего. Как и в
// `process_mesh_build_queue_system`, разрешаем явно.
#[allow(clippy::too_many_arguments)]
pub fn camera_control_system(
    mouse_btn: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut scroll: MessageReader<MouseWheel>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<GameSettings>,
    pointer: Res<PointerBlocked>,
    mut query: Query<(&mut Transform, &Camera), With<CameraController>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut drag: Local<CameraDrag>,
) {
    let Ok((mut transform, camera)) = query.single_mut() else {
        return;
    };
    let Ok(window) = window_query.single() else {
        return;
    };

    let cursor = window.cursor_position();

    // Собираем общую дельту движения мыши за кадр
    let mut mouse_delta = Vec2::ZERO;
    for ev in mouse_motion.read() {
        mouse_delta += ev.delta;
    }

    // --- Захват перетаскивания ---
    if mouse_btn.just_pressed(MouseButton::Left) {
        drag.pan = !pointer.get();
        drag.pan_anchor = cursor.and_then(|cursor| DragAnchor::new(&transform, camera, cursor));
        // Якорь не встал — курсор вне окна или луч не пересекает землю: жеста нет.
        drag.pan &= drag.pan_anchor.is_some();
    }
    if mouse_btn.just_released(MouseButton::Left) {
        drag.pan = false;
        drag.pan_anchor = None;
    }
    if mouse_btn.just_pressed(MouseButton::Right) {
        drag.rotate = !pointer.get();
        drag.rotate_anchor = cursor.and_then(|cursor| DragAnchor::new(&transform, camera, cursor));
        drag.rotate &= drag.rotate_anchor.is_some();
    }
    if mouse_btn.just_released(MouseButton::Right) {
        drag.rotate = false;
        drag.rotate_anchor = None;
    }

    // --- Панорамирование ---
    // Позиция считается заново от якоря, а не сдвигается накопленной дельтой:
    // так точка, за которую взялись курсором, остаётся под ним.
    if drag.pan
        && mouse_btn.pressed(MouseButton::Left)
        && mouse_delta != Vec2::ZERO
        && let Some(anchor) = &drag.pan_anchor
        && let Some(cursor) = cursor
        && let Some(current) = anchor.ground_point_now(camera, cursor)
    {
        let shift = (current - anchor.ground_point) * settings.camera.pan_speed;

        transform.translation.x = anchor.camera_position.x - shift.x;
        transform.translation.z = anchor.camera_position.z - shift.z;
    }

    // --- Вращение вокруг точки, где началось перетаскивание ---
    if drag.rotate
        && mouse_btn.pressed(MouseButton::Right)
        && mouse_delta != Vec2::ZERO
        && let Some(anchor) = &drag.rotate_anchor
    {
        let target = anchor.ground_point;
        let rotate_speed = settings.camera.rotate_speed;

        let yaw_rot = Quat::from_axis_angle(Vec3::Y, -mouse_delta.x * rotate_speed);
        let pitch_rot = Quat::from_axis_angle(*transform.local_x(), -mouse_delta.y * rotate_speed);

        // Сначала тангаж вокруг локальной оси, затем рыскание вокруг мировой.
        transform.rotate_around(target, yaw_rot * pitch_rot);

        // Ограничение тангажа остаётся приближением: корректирующий доворот
        // тянет угол к границе, но не выставляет его точно. Точное решение —
        // хранить yaw и pitch отдельно и собирать поворот заново.
        let (_, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        let pitch_deg = pitch.to_degrees();
        let clamped = pitch_deg.clamp(settings.camera.min_pitch, settings.camera.max_pitch);

        if pitch_deg != clamped {
            let correction =
                Quat::from_axis_angle(*transform.local_x(), (clamped - pitch_deg).to_radians());
            transform.rotate_around(target, correction);
        }
    }

    // --- Зум: отъезд камерой вдоль взгляда ---
    let mut zoom_delta = 0.0;
    for ev in scroll.read() {
        zoom_delta += ev.y;
    }

    if zoom_delta != 0.0 {
        let forward = *transform.forward();
        let altitude = transform.translation.y;

        if forward.y.abs() > f32::EPSILON {
            // Шаг пропорционален высоте: на любой высоте один щелчок меняет
            // масштаб на одну и ту же долю, а не на одинаковое число единиц.
            let step = zoom_delta * settings.camera.zoom_speed * altitude.max(1.0);
            let target_altitude = (altitude + step * forward.y).clamp(
                settings.camera.zoom_min_altitude,
                settings.camera.zoom_max_altitude,
            );

            // Проходим ровно столько, чтобы упереться в границу, если она рядом.
            let allowed = (target_altitude - altitude) / forward.y;
            transform.translation += forward * allowed;
        }
    }

    // --- Перемещение WASD ---
    // Пока по мировым осям: после рыскания «вперёд» может не совпадать с экраном.
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
        let speed = settings.camera.move_speed * time.delta_secs();
        transform.translation.x += move_delta.x * speed;
        transform.translation.z += move_delta.z * speed;
    }
}
