use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

use crate::game::settings::RenderingSettings;
use crate::hex::{ChunkData, ChunkLayout, HexChunk, MapBounds, HEX_DIRECTIONS};

use super::hex_math::{hex_corners, SHARED_CORNERS};

enum NeighborHeight {
    Loaded(i8),
    Unloaded,
    OutOfBounds,
}

pub fn build_chunk_mesh(
    chunk: &HexChunk,
    data: &ChunkData,
    neighbors: [Option<Vec<i8>>; 6],
    layout: &ChunkLayout,
    bounds: MapBounds,
    render: &RenderingSettings,
) -> Option<Mesh> {
    if data.size != layout.size {
        warn!(
            "ChunkData size {} does not match ChunkLayout size {}",
            data.size, layout.size
        );
        return None;
    }

    let size = data.size;
    let corners = hex_corners(render.hex_size);

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(layout.area * 6);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(layout.area * 6);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(layout.area * 6);
    let mut indices: Vec<u32> = Vec::with_capacity(layout.area * 12);

    let origin_q = layout.origin_q(chunk.chunk_x);
    let origin_r = layout.origin_r(chunk.chunk_y);

    let skirt_i8 = (render.skirt_height / render.elevation_step) as i8;

    // ------------------------------------------------------------
    // Top faces
    // ------------------------------------------------------------

    for r in 0..size {
        for q in 0..size {
            let global_q = origin_q + q as i32;
            let global_r = origin_r + r as i32;

            if !bounds.contains_hex(global_q, global_r) {
                continue;
            }

            let idx = data.index(q, r);
            let biome = data.biomes[idx];

            if !biome.is_renderable() {
                continue;
            }

            let elev = data.elevations[idx] as f32 * render.elevation_step;

            let center = hex_center_local(layout, render.hex_size, q, r);
            let center_raised = Vec3::new(center.x, elev, center.z);

            let base_idx = positions.len() as u32;
            let color = biome.color();

            for c in &corners {
                positions.push((center_raised + *c).into());
                colors.push(color);
                normals.push([0.0, 1.0, 0.0]);
            }

            indices.extend_from_slice(&[
                base_idx,
                base_idx + 2,
                base_idx + 1,
                base_idx,
                base_idx + 3,
                base_idx + 2,
                base_idx,
                base_idx + 4,
                base_idx + 3,
                base_idx,
                base_idx + 5,
                base_idx + 4,
            ]);
        }
    }

    // ------------------------------------------------------------
    // Walls
    // ------------------------------------------------------------

    if render.enable_walls {
        for r in 0..size {
            for q in 0..size {
                let global_q = origin_q + q as i32;
                let global_r = origin_r + r as i32;

                if !bounds.contains_hex(global_q, global_r) {
                    continue;
                }

                let idx = data.index(q, r);
                let biome = data.biomes[idx];

                if !biome.is_renderable() {
                    continue;
                }

                let elev_curr_i8 = data.elevations[idx];
                let elev_curr_f32 = elev_curr_i8 as f32 * render.elevation_step;
                let color = biome.color();

                let center = hex_center_local(layout, render.hex_size, q, r);

                for dir_idx in 0..6 {
                    let dir = &HEX_DIRECTIONS[dir_idx];

                    let local_nq = q as i32 + dir.q();
                    let local_nr = r as i32 + dir.r();

                    let global_nq = origin_q + local_nq;
                    let global_nr = origin_r + local_nr;

                    let neighbor_height = get_neighbor_height(
                        data, &neighbors, layout, bounds, local_nq, local_nr, global_nq, global_nr,
                    );

                    let (should_build_wall, elev_n_f32) = match neighbor_height {
                        NeighborHeight::Loaded(elev_n_i8) => (
                            elev_curr_i8 > elev_n_i8,
                            elev_n_i8 as f32 * render.elevation_step,
                        ),

                        NeighborHeight::Unloaded => (false, 0.0),

                        NeighborHeight::OutOfBounds => {
                            (elev_curr_i8 > skirt_i8, render.skirt_height)
                        }
                    };

                    if should_build_wall {
                        add_wall(
                            &mut positions,
                            &mut colors,
                            &mut normals,
                            &mut indices,
                            center,
                            elev_curr_f32,
                            elev_n_f32,
                            color,
                            &corners,
                            dir_idx,
                        );
                    }
                }
            }
        }
    }

    if positions.is_empty() {
        return None;
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));

    Some(mesh)
}

fn hex_center_local(layout: &ChunkLayout, hex_size: f32, local_q: usize, local_r: usize) -> Vec3 {
    let q = local_q as f32;
    let r = local_r as f32;

    let x = hex_size * super::hex_math::SQRT_3 * (q + r / 2.0);
    let z = hex_size * 1.5 * r;

    Vec3::new(x, 0.0, z)
}

#[allow(clippy::too_many_arguments)]
fn get_neighbor_height(
    current: &ChunkData,
    neighbors: &[Option<Vec<i8>>; 6],
    layout: &ChunkLayout,
    bounds: MapBounds,
    local_nq: i32,
    local_nr: i32,
    global_nq: i32,
    global_nr: i32,
) -> NeighborHeight {
    if !bounds.contains_hex(global_nq, global_nr) {
        return NeighborHeight::OutOfBounds;
    }

    if layout.contains_local(local_nq, local_nr) {
        let idx = layout.index(local_nq as usize, local_nr as usize);
        return NeighborHeight::Loaded(current.elevations[idx]);
    }

    let chunk_offset_q = local_nq >> layout.shift;
    let chunk_offset_r = local_nr >> layout.shift;

    let neighbor_idx = HEX_DIRECTIONS
        .iter()
        .position(|d| d.q() == chunk_offset_q && d.r() == chunk_offset_r);

    if let Some(idx) = neighbor_idx {
        if let Some(neighbor_data) = &neighbors[idx] {
            let local_q = (local_nq & layout.mask) as usize;
            let local_r = (local_nr & layout.mask) as usize;

            let neighbor_index = layout.index(local_q, local_r);

            if neighbor_index < neighbor_data.len() {
                return NeighborHeight::Loaded(neighbor_data[neighbor_index]);
            }
        }
    }

    NeighborHeight::Unloaded
}

#[allow(clippy::too_many_arguments)]
fn add_wall(
    pos: &mut Vec<[f32; 3]>,
    col: &mut Vec<[f32; 4]>,
    norm: &mut Vec<[f32; 3]>,
    idx: &mut Vec<u32>,
    center: Vec3,
    elev_curr: f32,
    elev_n: f32,
    color: [f32; 4],
    corners: &[Vec3; 6],
    dir: usize,
) {
    let [c1_idx, c2_idx] = SHARED_CORNERS[dir];

    let c1 = corners[c1_idx];
    let c2 = corners[c2_idx];

    let base = pos.len() as u32;

    let dark_color = [color[0] * 0.6, color[1] * 0.6, color[2] * 0.6, 1.0];

    let outward = ((c1 + c2) / 2.0).normalize();

    let v0 = center + c1 + Vec3::Y * elev_curr;
    let v1 = center + c2 + Vec3::Y * elev_curr;
    let v2 = center + c2 + Vec3::Y * elev_n;
    let v3 = center + c1 + Vec3::Y * elev_n;

    pos.push(v0.into());
    pos.push(v1.into());
    pos.push(v2.into());
    pos.push(v3.into());

    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let computed_normal = edge1.cross(edge2).normalize();

    let (final_normal, indices) = if computed_normal.dot(outward) >= 0.0 {
        (
            computed_normal,
            [base, base + 1, base + 2, base, base + 2, base + 3],
        )
    } else {
        (
            -computed_normal,
            [base, base + 2, base + 1, base, base + 3, base + 2],
        )
    };

    for _ in 0..4 {
        col.push(dark_color);
        norm.push(final_normal.into());
    }

    idx.extend_from_slice(&indices);
}
