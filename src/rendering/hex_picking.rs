use bevy::prelude::*;

use super::hex_math::{hex_corners, hex_world_position_with, ray_ground_point};
use crate::game::GameSettings;
use crate::hex::{
    ChunkData, ChunkLayout, ChunkMap, HexCoordinates, HoveredHex, MapBounds, SelectedHex,
};
use crate::pointer::PointerBlocked;

/// Цвет контура наведённого гекса.
const HOVERED_OUTLINE: Color = Color::srgb(1.0, 0.85, 0.2);

/// Цвет контура выделенного гекса.
const SELECTED_OUTLINE: Color = Color::srgb(0.3, 0.9, 0.4);

/// Порог в пикселях, до которого движение мыши считается кликом.
///
/// Левая кнопка одновременно панорамирует камеру, поэтому без порога
/// перетаскивание заодно выбирало бы гекс под курсором.
const CLICK_DRAG_THRESHOLD: f32 = 4.0;

/// Гекс под курсором.
///
/// Луч берётся тем же способом, что и в управлении камерой: `viewport_to_world`
/// и пересечение с плоскостью `y = 0`. Это намеренно простое приближение — на
/// возвышенностях оно промахивается, и первое, что здесь стоит заменить, это
/// марш луча по высотам из `ChunkData.elevations`.
// Длина списка — свойство ECS-системы: это разные части мира, и группировать тут
// нечего. Как и в `process_mesh_build_queue_system`, разрешаем явно.
#[allow(clippy::too_many_arguments)]
pub fn update_hovered_hex(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &Transform), With<Camera3d>>,
    settings: Res<GameSettings>,
    layout: Res<ChunkLayout>,
    bounds: Res<MapBounds>,
    chunk_map: Res<ChunkMap>,
    pointer: Res<PointerBlocked>,
    mut hovered: ResMut<HoveredHex>,
) {
    // Над непрозрачной панелью мир не наводится: пропадает и контур, и выбор,
    // потому что он смотрит на этот же `hovered`.
    if pointer.get() {
        hovered.0 = None;
        return;
    }

    let candidate = cursor_hex(&windows, &cameras, settings.rendering.hex_size);

    // За пределами карты и в незагруженных чанках наведения нет: панель не должна
    // показывать гекс, которого ещё не существует.
    hovered.0 = candidate.filter(|coords| {
        bounds.contains_hex(coords.q(), coords.r())
            && chunk_map
                .get_chunk_entity_by_hex(&layout, coords.q(), coords.r())
                .is_some()
    });
}

/// Где началось нажатие и не перехватила ли его панель.
///
/// `pub(crate)` — потому что тип виден в подписи системы, которую регистрирует
/// плагин из другого модуля.
pub(crate) struct PressStart {
    position: Vec2,
    blocked: bool,
}

/// Закрепление выбора кликом.
///
/// Это не запрос интерфейса, а взаимодействие с миром, поэтому живёт в домене:
/// UI здесь ни при чём, он только показывает результат.
///
/// Выбор ставится на отпускании и только если мышь не уехала: левая кнопка
/// одновременно панорамирует камеру, и без порога перетаскивание заодно
/// выбирало бы гекс. Нажатие, начатое над непрозрачной панелью, не выбирает
/// ничего — панель забрала жест себе, и это решается в момент нажатия, а не
/// отпускания.
pub fn select_hex_on_click(
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    hovered: Res<HoveredHex>,
    pointer: Res<PointerBlocked>,
    mut selected: ResMut<SelectedHex>,
    mut press: Local<Option<PressStart>>,
) {
    let cursor = windows
        .single()
        .ok()
        .and_then(|window| window.cursor_position());

    if buttons.just_pressed(MouseButton::Left) {
        *press = cursor.map(|position| PressStart {
            position,
            blocked: pointer.get(),
        });
        return;
    }

    if !buttons.just_released(MouseButton::Left) {
        return;
    }

    let Some(start) = press.take() else {
        return;
    };

    if start.blocked {
        return;
    }

    let Some(end) = cursor else {
        return;
    };

    if start.position.distance(end) > CLICK_DRAG_THRESHOLD {
        return;
    }

    if selected.0 != hovered.0 {
        selected.0 = hovered.0;
    }
}

/// Контуры наведённого и выделенного гекса.
pub fn draw_hex_selection(
    mut gizmos: Gizmos,
    hovered: Res<HoveredHex>,
    selected: Res<SelectedHex>,
    settings: Res<GameSettings>,
    layout: Res<ChunkLayout>,
    chunk_map: Res<ChunkMap>,
    chunks: Query<&ChunkData>,
) {
    let hex_size = settings.rendering.hex_size;
    let elevation_step = settings.rendering.elevation_step;

    if let Some(coords) = hovered.0
        && selected.0 != Some(coords)
    {
        draw_hex_outline(
            &mut gizmos,
            coords,
            hex_size,
            elevation_step,
            &layout,
            &chunk_map,
            &chunks,
            HOVERED_OUTLINE,
        );
    }

    if let Some(coords) = selected.0 {
        draw_hex_outline(
            &mut gizmos,
            coords,
            hex_size,
            elevation_step,
            &layout,
            &chunk_map,
            &chunks,
            SELECTED_OUTLINE,
        );
    }
}

/// Гекс под курсором, посчитанный по лучу камеры.
fn cursor_hex(
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &Transform), With<Camera3d>>,
    hex_size: f32,
) -> Option<HexCoordinates> {
    let window = windows.single().ok()?;
    let cursor = window.cursor_position()?;
    let (camera, transform) = cameras.single().ok()?;

    // Глобальная матрица считается здесь из локальной, а не читается из
    // `GlobalTransform`: тот пересчитывается в `PostUpdate`, поэтому в `Update`
    // доступно значение прошлого кадра — и наведение отставало бы от камеры.
    // Камера — корневая сущность, так что её локальное преобразование и есть мировое.
    let global = GlobalTransform::from(*transform);
    let ray = camera.viewport_to_world(&global, cursor).ok()?;

    ray_to_hex(ray.origin, *ray.direction, hex_size)
}

/// Пересечение луча с землёй и перевод точки в гекс.
///
/// Чистая функция: её можно проверить без окна, камеры и мира.
fn ray_to_hex(origin: Vec3, direction: Vec3, hex_size: f32) -> Option<HexCoordinates> {
    let hit = ray_ground_point(origin, direction)?;

    Some(HexCoordinates::from_world_xz(hit.x, hit.z, hex_size))
}

/// Контур гекса, поднятый на его собственную высоту.
///
/// Без подъёма контур на возвышенностях уходил бы под рельеф.
#[allow(clippy::too_many_arguments)]
fn draw_hex_outline(
    gizmos: &mut Gizmos,
    coords: HexCoordinates,
    hex_size: f32,
    elevation_step: f32,
    layout: &ChunkLayout,
    chunk_map: &ChunkMap,
    chunks: &Query<&ChunkData>,
    color: Color,
) {
    let center = hex_world_position_with(hex_size, coords.q(), coords.r());
    let height = hex_height(coords, elevation_step, layout, chunk_map, chunks);
    let base = Vec3::new(center.x, height + 0.05, center.z);

    let corners = hex_corners(hex_size);
    let mut points = [Vec3::ZERO; 7];

    for (index, corner) in corners.iter().enumerate() {
        points[index] = base + *corner;
    }

    points[6] = points[0];

    gizmos.linestrip(points, color);
}

/// Высота гекса в мире: `elevation * elevation_step`, как и при постройке меша.
fn hex_height(
    coords: HexCoordinates,
    elevation_step: f32,
    layout: &ChunkLayout,
    chunk_map: &ChunkMap,
    chunks: &Query<&ChunkData>,
) -> f32 {
    let Some(chunk_entity) = chunk_map.get_chunk_entity_by_hex(layout, coords.q(), coords.r())
    else {
        return 0.0;
    };

    let Ok(data) = chunks.get(chunk_entity) else {
        return 0.0;
    };

    let (local_q, local_r) = layout.local_coords(coords.q(), coords.r());
    let index = data.index(local_q, local_r);

    data.elevations[index] as f32 * elevation_step
}
