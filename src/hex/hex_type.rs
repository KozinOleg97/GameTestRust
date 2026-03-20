use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HexType {
    Empty,
    Plains,
    Forest,
    Mountains,
    Desert,
    Ocean,
    Coast,
    Swamp,
}

impl Hash for HexType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            HexType::Empty => 0.hash(state),
            HexType::Forest => 1.hash(state),
            HexType::Mountains => 2.hash(state),
            HexType::Desert => 3.hash(state),
            HexType::Ocean => 4.hash(state),
            HexType::Coast => 5.hash(state),
            HexType::Swamp => 6.hash(state),
            HexType::Plains => 7.hash(state),
        }
    }
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
