pub mod chunk;
pub mod chunk_map;
pub mod coordinates;
pub mod hex_type;
pub mod selection;

pub use chunk::{
    ChunkData, ChunkLayout, HexChunk, MapBounds, DEFAULT_CHUNK_SIZE, MAX_CHUNK_SIZE,
    MIN_CHUNK_SIZE, is_valid_chunk_size, normalize_chunk_size,
};
pub use chunk_map::{ChunkMap, HexInfo, get_hex_info, modify_hex_data, set_hex_data};
pub use coordinates::{HEX_DIRECTIONS, HexCoordinates, SQRT_3};
pub use hex_type::HexType;
pub use selection::{HoveredHex, SelectedHex};
