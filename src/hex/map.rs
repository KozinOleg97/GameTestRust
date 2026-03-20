use crate::hex::coordinates::HexCoordinates;
use crate::hex::{Hex, HexType};
use bevy::prelude::*;

#[derive(Resource)]
pub struct HexMap {
    hex_grid: Vec<Hex>,
    width: i32,
    height: i32,
}

impl HexMap {
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        let mut hex_grid = Vec::with_capacity(size);
        for r in 0..height {
            for q in 0..width {
                hex_grid.push(Hex::new(HexCoordinates::new(q, r), HexType::Empty));
            }
        }
        Self {
            hex_grid,
            width,
            height,
        }
    }

    ///Преобразование координат гексов в индекс с проверкой границ
    fn idx(&self, q: i32, r: i32) -> Option<usize> {
        if (q >= 0) && (q < self.width) && (r >= 0) && (r < self.height) {
            Some((r * self.width + q) as usize)
        } else {
            None
        }
    }

    /// Получить неизменяемую ссылку на гекс по координатам.
    pub fn get_hex(&self, q: i32, r: i32) -> Option<&Hex> {
        self.idx(q, r).map(|i| &self.hex_grid[i])
    }

    pub fn get_hex_by_coords(&self, coords: &HexCoordinates) -> Option<&Hex> {
        self.get_hex(coords.q(), coords.r())
    }

    /// Получить изменяемую ссылку на гекс по координатам.
    pub fn get_hex_mut(&mut self, q: i32, r: i32) -> Option<&mut Hex> {
        self.idx(q, r).map(|i| &mut self.hex_grid[i])
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = HexCoordinates> + '_ {
        (0..self.height).flat_map(move |r| (0..self.width).map(move |q| HexCoordinates::new(q, r)))
    }

    /// Заменить гекс по координатам (если он существует).
    pub fn set_hex(&mut self, q: i32, r: i32, hex: Hex) {
        if let Some(idx) = self.idx(q, r) {
            self.hex_grid[idx] = hex;
        } else {
            panic!("Hex coordinates out of bounds: ({}, {})", q, r);
        }
    }

    /// Удалить гекс нельзя - заменить на дефолтный.
    pub fn reset_hex(&mut self, q: i32, r: i32) {
        if let Some(idx) = self.idx(q, r) {
            self.hex_grid[idx] = Hex::new(HexCoordinates::new(q, r), HexType::Empty);
        } else {
            panic!("Hex coordinates out of bounds: ({}, {})", q, r);
        }
    }

    /// Получить всех гексов (итератор по ссылкам).
    pub fn iter(&self) -> std::slice::Iter<'_, Hex> {
        self.hex_grid.iter()
    }

    /// Получить всех гексов (итератор с возможностью изменения).
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Hex> {
        self.hex_grid.iter_mut()
    }

    pub fn size(&self) -> usize {
        self.hex_grid.len()
    }

    pub fn clear(&mut self) {
        for r in 0..self.height {
            for q in 0..self.width {
                self.hex_grid[(r * self.width + q) as usize] =
                    Hex::new(HexCoordinates::new(q, r), HexType::Empty);
            }
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    fn is_within_bounds(&self, q: i32, r: i32) -> bool {
        q >= 0 && q < self.width && r >= 0 && r < self.height
    }
}
