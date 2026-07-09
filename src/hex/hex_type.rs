use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HexType {
    Empty = 0,
    Plains = 1,
    Forest = 2,
    Mountains = 3,
    Desert = 4,
    Ocean = 5,
    Coast = 6,
    Swamp = 7,
}

impl HexType {
    pub fn name(&self) -> &'static str {
        match self {
            HexType::Plains => "Равнины",
            HexType::Forest => "Лес",
            HexType::Mountains => "Горы",
            HexType::Desert => "Пустыня",
            HexType::Ocean => "Океан",
            HexType::Coast => "Побережье",
            HexType::Swamp => "Болото",
            &HexType::Empty => "Пусто",
        }
    }

    pub fn color(&self) -> (f32, f32, f32, f32) {
        match self {
            HexType::Plains => (0.4, 0.8, 0.2, 1.0), // Green for plains
            HexType::Forest => (0.2, 0.6, 0.1, 1.0), // Dark green for forest
            HexType::Mountains => (0.5, 0.5, 0.5, 1.0), // Gray for mountains
            HexType::Ocean => (0.2, 0.4, 0.8, 1.0),  // Blue for ocean
            HexType::Coast => (0.8, 0.8, 0.6, 1.0),  // Beige for coast
            HexType::Desert => (0.95, 0.9, 0.11, 1.0), // Yellow for desert
            HexType::Swamp => (0.1, 0.2, 0.1, 1.0),  // Dark green for swamp
            &HexType::Empty => (0.0, 0.0, 0.0, 1.0),
        }
    }
}
