use crate::hex::map::HexMap;
use crate::hex::utils::axial_to_pixel;
use crate::hex::{HexCoordinates, HexType, HEX_SIZE};

pub fn generate_unit_hex_vertices(size: f32) -> Vec<(f32, f32)> {
    let rotation = std::f32::consts::PI / 6.0;
    let mut vertices = Vec::with_capacity(7);
    for i in 0..6 {
        let angle = (i as f32) * std::f32::consts::PI / 3.0 + rotation;
        let x = size * angle.cos();
        let z = size * angle.sin();
        vertices.push((x, z));
    }
    vertices.push((0.0, 0.0));
    vertices
}

pub fn hex_type_to_index(hex_type: HexType) -> u8 {
    match hex_type {
        HexType::Empty => 0,
        HexType::Plains => 1,
        HexType::Forest => 2,
        HexType::Mountains => 3,
        HexType::Desert => 4,
        HexType::Ocean => 5,
        HexType::Coast => 6,
        HexType::Swamp => 7,
    }
}

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
            let (x, z) = axial_to_pixel(&HexCoordinates::new(q, r), HEX_SIZE);
            let type_index = hex_type_to_index(*hex.hex_type()) as f32;

            for (vx, vz) in &unit_vertices {
                positions.push([x + vx, 0.0, z + vz]);
                normals.push([0.0, 1.0, 0.0]);
                uvs.push([type_index / 8.0, 0.5]);
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
