// mod batched_hex_renderer;
// mod mesh_builder;
// mod plugin;
// mod materials;
// mod chunk_manager;
//
// pub use plugin::HexRenderPlugin;

mod chunk_renderer;
mod full_mesh_renderer;
mod materials;
mod mesh_builder;
mod plugin;

pub use full_mesh_renderer::FullMeshRenderingPlugin;
pub use plugin::HexRenderingPlugin;

/// Режимы рендеринга шестиугольников
pub enum RenderingMode {
    Chunked,
    FullMesh,
}
