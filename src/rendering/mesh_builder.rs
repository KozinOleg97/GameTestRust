use crate::hex::map::HexMap;
use crate::hex::utils::axial_to_pixel;
use crate::hex::{HexCoordinates, HexType, HEX_SIZE};
use bevy::math::Vec2;
use bevy::mesh::{Indices, Mesh};
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

const UP_NORMAL: [f32; 3] = [0.0, 1.0, 0.0];
pub fn generate_unit_hex_vertices(size: f32) -> Vec<Vec2> {
    let rotation = std::f32::consts::PI / 6.0;
    let mut vertices = Vec::with_capacity(7);
    for i in 0..6 {
        let angle = (i as f32) * std::f32::consts::PI / 3.0 + rotation;
        let x = size * angle.cos();
        let z = size * angle.sin();
        vertices.push(Vec2::new(x, z));
    }
    vertices.push(Vec2::ZERO); // Центр гекса
    vertices
}

/// Создаёт меш для чанка (старый метод)
#[allow(dead_code)]
pub fn generate_chunk_mesh(
    hex_map: &HexMap,
    q_min: i32,
    q_max: i32,
    r_min: i32,
    r_max: i32,
) -> (
    Vec<[f32; 3]>,
    Vec<[f32; 3]>,
    Vec<[f32; 2]>,
    Vec<u32>,
    Vec<f32>,
) {
    info!("Generating chunk mesh");

    let unit_vertices = generate_unit_hex_vertices(HEX_SIZE);
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    let mut hex_types = Vec::new();
    let mut vertex_offset = 0;

    for r in r_min..=r_max {
        for q in q_min..=q_max {
            let Some(hex) = hex_map.get_hex(q, r) else {
                continue;
            };

            let center = axial_to_pixel(&HexCoordinates::new(q, r), HEX_SIZE);
            let type_index = *hex.hex_type() as u8 as f32;

            // Используем деструктуризацию ссылки `&corner` для удобства
            for &corner in &unit_vertices {
                let pos = center + corner; // Векторное сложение!

                positions.push([pos.x, 0.0, pos.y]);
                normals.push(UP_NORMAL);
                uvs.push([(type_index + 0.5) / 8.0, 0.5]); // центр пикселя
                hex_types.push(type_index);
            }

            for i in 0..6 {
                let next = (i + 1) % 6;
                indices.extend_from_slice(&[
                    vertex_offset + i,
                    vertex_offset + next,
                    vertex_offset + 6,
                ]);
            }
            vertex_offset += 7;
        }
    }
    return (positions, normals, uvs, indices, hex_types);
}

/// Создаёт один меш для всей карты (новый метод)
#[allow(dead_code)]
pub fn generate_full_mesh(hex_map: &HexMap) -> Mesh {
    info!("Generating full mesh");
    let unit_vertices = generate_unit_hex_vertices(HEX_SIZE);
    let total_hexes = hex_map.size();

    let mut positions = Vec::with_capacity(total_hexes * 7);
    let mut normals = Vec::with_capacity(total_hexes * 7);
    let mut uvs = Vec::with_capacity(total_hexes * 7);
    let mut indices = Vec::with_capacity(total_hexes * 6 * 3); // 6 треугольников на гекс

    let mut vertex_offset = 0;

    for r in 0..hex_map.height() {
        for q in 0..hex_map.width() {
            let hex = hex_map.get_hex(q, r).expect("hex exists");

            let center = axial_to_pixel(&HexCoordinates::new(q, r), HEX_SIZE);
            let type_index = *hex.hex_type() as u8 as f32;

            // Добавляем 7 вершин гекса
            for &corner in &unit_vertices {
                let pos = center + corner;

                positions.push([pos.x, 0.0, pos.y]);
                normals.push(UP_NORMAL);
                uvs.push([(type_index + 0.5) / 8.0, 0.5]); // центр пикселя
            }

            // Индексы для 6 треугольников (центр + две соседние вершины)
            for i in 0..6 {
                let next = (i + 1) % 6;
                indices.extend_from_slice(&[
                    vertex_offset + i,
                    vertex_offset + next,
                    vertex_offset + 6,
                ]);
            }

            vertex_offset += 7;
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

// /// Генерирует меш с общими вершинами для всей карты.
// /// Каждый гекс представляется 6 углами, вершины на стыках используются повторно.
// /// Для хранения типа гекса используются вершинные цвета (R,G,B,A).
// pub fn generate_shared_vertices_mesh(hex_map: &HexMap) -> Mesh {
//     info!("Generating mesh with shared vertices");
//
//     let unit_vertices = generate_unit_hex_vertices(HEX_SIZE);
//     let corners = &unit_vertices[0..6]; // берём только углы
//
//     let mut vertex_map: HashMap<[i32; 6], u32> = HashMap::new();
//     let mut positions: Vec<[f32; 3]> = Vec::new();
//     let mut colors: Vec<[f32; 4]> = Vec::new();
//     let mut indices: Vec<u32> = Vec::new();
//
//     // Вспомогательная функция для получения трёх гексов, образующих угол
//     // Для гекса (q,r) и направления i (0..5) возвращает три координаты (q,r) гексов,
//     // которые встречаются в этом углу.
//     fn get_three_hexes(q: i32, r: i32, i: usize) -> [(i32, i32); 3] {
//         // Направления для pointy-top гексов:
//         // 0: право, 1: вниз-вправо, 2: вниз-влево, 3: влево, 4: вверх-влево, 5: вверх-вправо
//         let dirs = [
//             (1, 0), (0, 1), (-1, 1),
//             (-1, 0), (0, -1), (1, -1),
//         ];
//         let d1 = dirs[i];
//         let d2 = dirs[(i + 5) % 6];
//         let hex1 = (q, r);
//         let hex2 = (q + d1.0, r + d1.1);
//         let hex3 = (q + d2.0, r + d2.1);
//
//         let mut triple = [hex1, hex2, hex3];
//         triple.sort_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
//         triple
//     }
//
//     for r in 0..hex_map.height() {
//         for q in 0..hex_map.width() {
//             let hex = hex_map.get_hex(q, r).expect("hex exists");
//             let hex_type = *hex.hex_type();
//
//             // center теперь Vec2
//             let center = axial_to_pixel(&HexCoordinates::new(q, r), HEX_SIZE);
//
//             for i in 0..6 {
//                 let corner = corners[i]; // corner теперь Vec2
//                 let world_pos = center + corner; // Векторное сложение!
//
//                 // Получаем три гекса, которые делят этот угол
//                 let triple = get_three_hexes(q, r, i);
//                 // Ключ – плоский массив из шести целых чисел
//                 let key = [
//                     triple[0].0, triple[0].1,
//                     triple[1].0, triple[1].1,
//                     triple[2].0, triple[2].1,
//                 ];
//
//                 // Если вершина уже существует, берём её индекс
//                 if let Some(&idx) = vertex_map.get(&key) {
//                     indices.push(idx);
//                     continue;
//                 }
//
//                 // Иначе создаём новую вершину
//                 let idx = positions.len() as u32;
//                 positions.push([world_pos.x, 0.0, world_pos.y]);
//
//                 // Задаём цвет в зависимости от типа гекса (можно настроить позже)
//                 let hex_color = match hex_type {
//                     HexType::Empty    => [0.0, 0.0, 0.0, 1.0],
//                     HexType::Plains   => [0.0, 1.0, 0.0, 1.0],
//                     HexType::Forest   => [0.0, 0.5, 0.0, 1.0],
//                     HexType::Mountains=> [0.5, 0.5, 0.5, 1.0],
//                     HexType::Desert   => [1.0, 1.0, 0.0, 1.0],
//                     HexType::Ocean    => [0.0, 0.0, 1.0, 1.0],
//                     HexType::Coast    => [0.0, 1.0, 1.0, 1.0],
//                     HexType::Swamp    => [0.5, 0.0, 0.5, 1.0],
//                 };
//                 colors.push(hex_color);
//
//                 vertex_map.insert(key, idx);
//                 indices.push(idx);
//             }
//         }
//     }
//
//     // Теперь нужно построить индексы треугольников для каждого гекса.
//     // Мы имеем список индексов вершин в порядке обхода гексов, но вершины
//     // были добавлены в произвольном порядке. Чтобы восстановить, какие шесть
//     // вершин принадлежат конкретному гексу, можно либо сохранять их при генерации,
//     // либо перебрать все гексы и собрать индексы заново, используя vertex_map.
//     // Для простоты пересоберём индексы треугольников, опираясь на те же тройки.
//
//     let mut triangle_indices = Vec::new();
//     for r in 0..hex_map.height() {
//         for q in 0..hex_map.width() {
//             // Для каждого гекса нужно найти индексы его 6 углов
//             let mut hex_vertex_indices = [0u32; 6];
//             for i in 0..6 {
//                 let triple = get_three_hexes(q, r, i);
//                 let key = [
//                     triple[0].0, triple[0].1,
//                     triple[1].0, triple[1].1,
//                     triple[2].0, triple[2].1,
//                 ];
//                 hex_vertex_indices[i] = *vertex_map.get(&key).expect("vertex should exist");
//             }
//             // Разбиваем на 4 треугольника (0-1-2, 0-2-3, 0-3-4, 0-4-5)
//             for i in 1..5 {
//                 triangle_indices.extend_from_slice(&[
//                     hex_vertex_indices[0],
//                     hex_vertex_indices[i],
//                     hex_vertex_indices[i+1],
//                 ]);
//             }
//         }
//     }
//
//     let mut mesh = Mesh::new(
//         PrimitiveTopology::TriangleList,
//         bevy::asset::RenderAssetUsages::default(),
//     );
//     mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
//     mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
//     mesh.insert_indices(Indices::U32(triangle_indices));
//
//     mesh
// }
