#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum HexType {
    #[default]
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
    #[inline]
    pub fn id(self) -> u8 {
        self as u8
    }

    pub fn from_id(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::Empty,
            1 => Self::Plains,
            2 => Self::Forest,
            3 => Self::Mountains,
            4 => Self::Desert,
            5 => Self::Ocean,
            6 => Self::Coast,
            7 => Self::Swamp,
            _ => return None,
        })
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }

    #[inline]
    pub fn is_renderable(self) -> bool {
        !self.is_empty()
    }

    #[inline]
    pub fn is_land(self) -> bool {
        matches!(
            self,
            Self::Plains
                | Self::Forest
                | Self::Mountains
                | Self::Desert
                | Self::Coast
                | Self::Swamp
        )
    }

    #[inline]
    pub fn is_water(self) -> bool {
        matches!(self, Self::Ocean)
    }

    pub fn base_movement_cost(self) -> Option<u8> {
        match self {
            Self::Empty => None,
            Self::Ocean => None,
            Self::Mountains => None,

            Self::Plains => Some(1),
            Self::Coast => Some(1),

            Self::Forest => Some(2),
            Self::Desert => Some(2),

            Self::Swamp => Some(3),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            HexType::Empty => "Пусто",
            HexType::Plains => "Равнины",
            HexType::Forest => "Лес",
            HexType::Mountains => "Горы",
            HexType::Desert => "Пустыня",
            HexType::Ocean => "Океан",
            HexType::Coast => "Побережье",
            HexType::Swamp => "Болото",
        }
    }

    pub fn color(&self) -> [f32; 4] {
        match self {
            HexType::Plains => [0.4, 0.8, 0.2, 1.0],
            HexType::Forest => [0.2, 0.6, 0.1, 1.0],
            HexType::Mountains => [0.5, 0.5, 0.5, 1.0],
            HexType::Ocean => [0.2, 0.4, 0.8, 1.0],
            HexType::Coast => [0.8, 0.8, 0.6, 1.0],
            HexType::Desert => [0.95, 0.9, 0.11, 1.0],
            HexType::Swamp => [0.1, 0.2, 0.1, 1.0],
            HexType::Empty => [0.0, 0.0, 0.0, 1.0],
        }
    }
}
