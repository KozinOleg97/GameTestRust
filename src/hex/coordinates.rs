use std::fmt::Display;
use std::ops::Add;

/// Корень из трёх: отношение ширины гекса к его размеру для Pointy-Top.
pub const SQRT_3: f32 = 1.732_050_8;

/// Размер гекса по умолчанию — тот же, что в `rendering::hex_math`.
const DEFAULT_HEX_SIZE: f32 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCoordinates {
    q: i32,
    r: i32,
}

impl HexCoordinates {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    #[inline]
    pub fn q(&self) -> i32 {
        self.q
    }

    #[inline]
    pub fn r(&self) -> i32 {
        self.r
    }

    /// Все 6 соседей гекса.
    pub fn neighbors(&self) -> [HexCoordinates; 6] {
        HEX_DIRECTIONS.map(|dir| *self + dir)
    }

    /// Hex distance в axial coordinates.
    pub fn distance(self, other: Self) -> i32 {
        let dq = self.q - other.q;
        let dr = self.r - other.r;
        let ds = dq + dr;

        (dq.abs() + dr.abs() + ds.abs()) / 2
    }

    /// Обратное к [`crate::rendering::hex_math::hex_world_position_with`]:
    /// точка в плоскости XZ -> гекс, в который она попадает.
    ///
    /// Гексы Pointy-Top, поэтому сначала получаем дробные осевые координаты,
    /// а затем округляем их в кубических — простое округление `q` и `r` по
    /// отдельности у границ гекса уводит к соседу не по ближайшему центру.
    pub fn from_world_xz(x: f32, z: f32, hex_size: f32) -> Self {
        let size = if hex_size.abs() < f32::EPSILON {
            DEFAULT_HEX_SIZE
        } else {
            hex_size
        };

        // Прямое преобразование (см. rendering::hex_math):
        //   x = size * SQRT_3 * (q + r / 2)
        //   z = size * 1.5 * r
        let r = (2.0 / 3.0) * z / size;
        let q = x / (SQRT_3 * size) - z / (3.0 * size);

        Self::round_cube(q, r)
    }

    /// Кубическое округление осевых координат: округляем все три оси и правим ту,
    /// где дробная часть отклонилась сильнее всего.
    fn round_cube(q: f32, r: f32) -> Self {
        let s = -q - r;

        let mut round_q = q.round();
        let mut round_r = r.round();
        let round_s = s.round();

        let diff_q = (round_q - q).abs();
        let diff_r = (round_r - r).abs();
        let diff_s = (round_s - s).abs();

        if diff_q > diff_r && diff_q > diff_s {
            round_q = -round_r - round_s;
        } else if diff_r > diff_s {
            round_r = -round_q - round_s;
        }

        Self::new(round_q as i32, round_r as i32)
    }
}

impl Add for HexCoordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.q + rhs.q, self.r + rhs.r)
    }
}

impl Display for HexCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.q, self.r)
    }
}

/// Стандартные направления для Pointy-Top гексов.
///
/// Порядок:
/// 0: East
/// 1: Southeast
/// 2: Southwest
/// 3: West
/// 4: Northwest
/// 5: Northeast
pub const HEX_DIRECTIONS: [HexCoordinates; 6] = [
    HexCoordinates { q: 1, r: 0 },  // East
    HexCoordinates { q: 0, r: 1 },  // Southeast
    HexCoordinates { q: -1, r: 1 }, // Southwest
    HexCoordinates { q: -1, r: 0 }, // West
    HexCoordinates { q: 0, r: -1 }, // Northwest
    HexCoordinates { q: 1, r: -1 }, // Northeast
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::hex_math::hex_world_position_with;

    /// Прямое и обратное преобразования обязаны сходиться на центрах гексов:
    /// на этой паре формул держится выбор гекса курсором, и если они разойдутся,
    /// панель свойств начнёт показывать соседний гекс.
    #[test]
    fn world_round_trip_returns_same_hex() {
        let size = 1.0;

        for q in -4..=4 {
            for r in -4..=4 {
                let world = hex_world_position_with(size, q, r);
                let back = HexCoordinates::from_world_xz(world.x, world.z, size);

                assert_eq!(back, HexCoordinates::new(q, r), "гекс ({q}, {r})");
            }
        }
    }

    /// Точки у границы гекса не должны уезжать к соседу «по диагонали» —
    /// именно от этого спасает кубическое округление.
    #[test]
    fn points_near_center_stay_in_same_hex() {
        let size = 1.0;
        let center = hex_world_position_with(size, 2, 3);

        for offset in [-0.2, 0.0, 0.2] {
            let back = HexCoordinates::from_world_xz(center.x + offset, center.z + offset, size);
            assert_eq!(back, HexCoordinates::new(2, 3));
        }
    }
}
