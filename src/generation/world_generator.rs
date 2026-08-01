use crate::generation::PerlinNoise;
use crate::hex::{ChunkData, ChunkLayout, HexChunk, HexType, MapBounds, DEFAULT_CHUNK_SIZE};

use bevy::prelude::*;

/// Основной масштаб шума.
const NOISE_SCALE: f64 = 0.003;

/// Процедурный генератор мира.
///
/// Координаты карты начинаются с `(0, 0)`.
#[derive(Resource)]
pub struct ProceduralWorldGenerator {
    seed: u64,

    continental_noise: PerlinNoise,
    terrain_noise: PerlinNoise,
    moisture_noise: PerlinNoise,
    temperature_noise: PerlinNoise,
    mountain_noise: PerlinNoise,
    aridity_noise: PerlinNoise,
}

impl ProceduralWorldGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            continental_noise: PerlinNoise::new(subseed(seed, 0)),
            terrain_noise: PerlinNoise::new(subseed(seed, 1)),
            moisture_noise: PerlinNoise::new(subseed(seed, 2)),
            temperature_noise: PerlinNoise::new(subseed(seed, 3)),
            mountain_noise: PerlinNoise::new(subseed(seed, 4)),
            aridity_noise: PerlinNoise::new(subseed(seed, 5)),
        }
    }

    #[inline]
    pub fn seed(&self) -> u64 {
        self.seed
    }

    // -------------------------------------------------------------------------
    // Стартовая генерация всей карты
    // -------------------------------------------------------------------------

    /// Генерирует все чанки для карты заданного размера в гексах.
    pub fn generate_initial_chunks(
        &self,
        map_width: i32,
        map_height: i32,
        chunk_size: usize,
    ) -> Vec<(HexChunk, ChunkData)> {
        let layout = ChunkLayout::from_size(chunk_size);
        let bounds = MapBounds::new(map_width, map_height);

        self.generate_initial_chunks_with_layout(&layout, &bounds)
    }

    /// То же самое, но с использованием стандартного размера чанка.
    ///
    /// Может быть полезно для совместимости или быстрых тестов.
    pub fn generate_initial_chunks_default_chunk_size(
        &self,
        map_width: i32,
        map_height: i32,
    ) -> Vec<(HexChunk, ChunkData)> {
        self.generate_initial_chunks(map_width, map_height, DEFAULT_CHUNK_SIZE)
    }

    /// Генерирует все чанки, используя уже готовые `ChunkLayout` и `MapBounds`.
    ///
    /// Этот метод удобно использовать, если `ChunkLayout` и `MapBounds`
    /// уже лежат в ресурсах.
    pub fn generate_initial_chunks_with_layout(
        &self,
        layout: &ChunkLayout,
        bounds: &MapBounds,
    ) -> Vec<(HexChunk, ChunkData)> {
        if bounds.width <= 0 || bounds.height <= 0 {
            warn!(
                "ProceduralWorldGenerator: invalid map bounds {}x{}, skipping generation",
                bounds.width, bounds.height
            );

            return Vec::new();
        }

        let chunk_size = layout.size as i32;

        let chunks_x = div_up(bounds.width, chunk_size) as usize;
        let chunks_y = div_up(bounds.height, chunk_size) as usize;

        info!(
            "Generating initial chunks: map={}x{}, chunk_size={}, chunks={}x{}",
            bounds.width, bounds.height, layout.size, chunks_x, chunks_y
        );

        let mut chunks = Vec::with_capacity(chunks_x.saturating_mul(chunks_y));

        for chunk_y in 0..chunks_y {
            for chunk_x in 0..chunks_x {
                let chunk_x = chunk_x as i32;
                let chunk_y = chunk_y as i32;

                let chunk = HexChunk::new(chunk_x, chunk_y);
                let data = self.generate_chunk(layout, bounds, chunk_x, chunk_y);

                chunks.push((chunk, data));
            }
        }

        info!("Generated {} chunks", chunks.len());

        chunks
    }

    // -------------------------------------------------------------------------
    // Генерация одного чанка
    // -------------------------------------------------------------------------

    /// Генерирует данные одного чанка с учётом границ карты.
    ///
    /// Гексы за пределами `MapBounds` остаются `HexType::Empty`.
    pub fn generate_chunk(
        &self,
        layout: &ChunkLayout,
        bounds: &MapBounds,
        chunk_x: i32,
        chunk_y: i32,
    ) -> ChunkData {
        self.fill_chunk(layout, Some(bounds), chunk_x, chunk_y)
    }

    /// Генерирует один чанк без учёта границ карты.
    ///
    /// Может использоваться для бесконечного мира или стриминга,
    /// если позже ты решишь уйти от конечной карты.
    pub fn generate_chunk_unbounded(
        &self,
        layout: &ChunkLayout,
        chunk_x: i32,
        chunk_y: i32,
    ) -> ChunkData {
        self.fill_chunk(layout, None, chunk_x, chunk_y)
    }

    /// Внутренний метод заполнения чанка.
    fn fill_chunk(
        &self,
        layout: &ChunkLayout,
        bounds: Option<&MapBounds>,
        chunk_x: i32,
        chunk_y: i32,
    ) -> ChunkData {
        let mut data = ChunkData::new(layout.size);

        let origin_q = layout.origin_q(chunk_x);
        let origin_r = layout.origin_r(chunk_y);

        for local_r in 0..layout.size {
            for local_q in 0..layout.size {
                let global_q = origin_q + local_q as i32;
                let global_r = origin_r + local_r as i32;

                // Если карта конечная, пропускаем гексы за её пределами.
                if let Some(bounds) = bounds {
                    if !bounds.contains_hex(global_q, global_r) {
                        continue;
                    }
                }

                let (biome, elevation) = self.determine_hex(global_q, global_r);

                data.set_hex(local_q, local_r, biome, elevation);
            }
        }

        data
    }

    // -------------------------------------------------------------------------
    // Биомы и высота
    // -------------------------------------------------------------------------

    /// Определяет биом и высоту для конкретного глобального гекса.
    fn determine_hex(&self, global_q: i32, global_r: i32) -> (HexType, i8) {
        let nx = global_q as f64 * NOISE_SCALE;
        let ny = global_r as f64 * NOISE_SCALE;

        // 1. Базовая высота: континенты и океаны.
        let height = normalized_noise(self.continental_noise.noise(nx * 2.0, ny * 2.0));

        // 2. Горные зоны: крупные хребты.
        let mountain_zone = normalized_noise(self.mountain_noise.noise(nx * 1.2, ny * 1.2));

        // 3. Детальный шум для микро-неровностей равнин.
        let detail = normalized_noise(self.terrain_noise.noise(nx * 8.0, ny * 8.0));

        // 4. Влажность.
        let moisture = normalized_noise(self.moisture_noise.noise(nx * 3.0, ny * 3.0));

        // 5. Температура.
        let temperature = normalized_noise(self.temperature_noise.noise(nx * 2.5, ny * 2.5));

        // 6. Аридность: сухость климата.
        let aridity = normalized_noise(self.aridity_noise.noise(nx * 1.5, ny * 1.5));

        let biome: HexType;
        let mut elevation: i8;

        // ---------------------------------------------------------------------
        // Океан и побережье
        // ---------------------------------------------------------------------

        if height < 0.20 {
            biome = HexType::Ocean;
            elevation = 0;
        } else if height < 0.25 {
            biome = HexType::Coast;
            elevation = 2;
        }
        // ---------------------------------------------------------------------
        // Горы
        // ---------------------------------------------------------------------
        else if mountain_zone > 0.75 && height > 0.40 {
            biome = HexType::Mountains;

            // Нелинейная кривая:
            // края горных зон получаются более пологими,
            // а пики — более высокими.
            let peak_intensity = ((mountain_zone - 0.75) / 0.25).powi(2);

            let base_elev = (height * 10.0) as i16;
            let peak_bonus = (peak_intensity * 90.0) as i16;

            elevation = (base_elev + peak_bonus).clamp(0, 120) as i8;
        }
        // ---------------------------------------------------------------------
        // Равнины и остальные биомы
        // ---------------------------------------------------------------------
        else {
            // Равнины делаем относительно плоскими.
            let plain_elev = (height * 8.0 + detail * 3.0) as i16;

            // Минимальная высота 3, чтобы суша не сливалась с водой.
            elevation = plain_elev.clamp(3, 120) as i8;

            biome = if moisture < 0.30 && aridity > 0.60 && temperature > 0.55 {
                HexType::Desert
            } else if moisture > 0.65 {
                HexType::Forest
            } else if moisture > 0.50 && height < 0.35 && temperature > 0.35 {
                // Болота всегда низкие.
                elevation = 2;
                HexType::Swamp
            } else {
                HexType::Plains
            };
        }

        (biome, elevation)
    }
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

/// Приводит значение шума из диапазона `[-1, 1]` к диапазону `[0, 1]`.
#[inline]
fn normalized_noise(value: f64) -> f64 {
    (value + 1.0) / 2.0
}

/// Создаёт устойчивый subseed для отдельных шумов.
#[inline]
fn subseed(seed: u64, offset: u64) -> u32 {
    seed.wrapping_add(offset) as u32
}

/// Деление вверх для положительных значений.
///
/// Используется вместо `i32::div_ceil`, чтобы не зависеть
/// от версии Rust / toolchain.
#[inline]
fn div_up(value: i32, divisor: i32) -> i32 {
    debug_assert!(divisor > 0);

    if value <= 0 {
        return 0;
    }

    (value + divisor - 1) / divisor
}
