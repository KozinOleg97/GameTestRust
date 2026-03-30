use crate::rendering::chunk_renderer::ChunkRenderingPlugin;
use crate::rendering::{FullMeshRenderingPlugin, RenderingMode};
use bevy::prelude::*;

pub struct HexRenderingPlugin {
    pub mode: RenderingMode,
}

impl Plugin for HexRenderingPlugin {
    fn build(&self, app: &mut App) {
        match self.mode {
            RenderingMode::Chunked => {
                app.add_plugins(ChunkRenderingPlugin);
            }
            RenderingMode::FullMesh => {
                app.add_plugins(FullMeshRenderingPlugin);
            }
        }
    }
}
