use std::fmt::Display;
use std::ops::Add;

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
