use bevy::prelude::*;
use bevy::text::{TextColor, TextFont};
use bevy::ui::{BackgroundColor, Node, PositionType, Val};
use std::collections::VecDeque;

/// Configuration for the performance overlay.
#[derive(Resource, Clone)]
pub struct PerformanceOverlayConfig {
    /// Whether the overlay is visible.
    pub visible: bool,
    /// Position of the overlay (top-left corner) in pixels.
    pub position: (f32, f32),
    /// Font size in pixels.
    pub font_size: f32,
    /// Text color.
    pub text_color: Color,
    /// Background color (optional).
    pub background_color: Option<Color>,
    /// Number of frames to average for FPS calculation.
    pub fps_average_frames: usize,
    /// Key to toggle overlay visibility.
    pub toggle_key: KeyCode,
}

impl Default for PerformanceOverlayConfig {
    fn default() -> Self {
        Self {
            visible: true,
            position: (10.0, 10.0),
            font_size: 24.0,
            text_color: Color::srgb(1.0, 1.0, 1.0),
            background_color: Some(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            fps_average_frames: 60,
            toggle_key: KeyCode::F12,
        }
    }
}

/// Tracks performance metrics over time.
#[derive(Resource)]
pub struct PerformanceMetrics {
    /// Frame times (in seconds) for the last N frames.
    frame_times: VecDeque<f32>,
    /// Current FPS (smoothed).
    fps: f32,
    /// Total time accumulated for FPS calculation.
    accumulated_time: f32,
    /// Number of frames accumulated.
    accumulated_frames: usize,
    /// Last frame's delta time.
    last_frame_time: f32,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(120),
            fps: 0.0,
            accumulated_time: 0.0,
            accumulated_frames: 0,
            last_frame_time: 0.0,
        }
    }
}

impl PerformanceMetrics {
    /// Update metrics with the latest frame's delta time.
    pub fn update(&mut self, delta_seconds: f32) {
        self.last_frame_time = delta_seconds;
        self.frame_times.push_back(delta_seconds);
        if self.frame_times.len() > 120 {
            self.frame_times.pop_front();
        }

        // Update FPS using exponential moving average for smoothness.
        const SMOOTHING: f32 = 0.05;
        let current_fps = 1.0 / delta_seconds.max(0.0001);
        self.fps = self.fps * (1.0 - SMOOTHING) + current_fps * SMOOTHING;
    }

    /// Get the current FPS (smoothed).
    pub fn fps(&self) -> f32 {
        self.fps
    }

    /// Get the last frame time in milliseconds.
    pub fn frame_time_ms(&self) -> f32 {
        self.last_frame_time * 1000.0
    }

    /// Get the average frame time over the last N frames (in milliseconds).
    pub fn average_frame_time_ms(&self, n: usize) -> f32 {
        let count = self.frame_times.len().min(n);
        if count == 0 {
            return 0.0;
        }
        let sum: f32 = self.frame_times.iter().rev().take(count).sum();
        (sum / count as f32) * 1000.0
    }
}

/// System that updates performance metrics each frame.
fn update_performance_metrics(
    time: Res<Time>,
    mut metrics: ResMut<PerformanceMetrics>,
) {
    metrics.update(time.delta_secs());
}

// UI components
#[derive(Component)]
struct PerformanceOverlayRoot;

#[derive(Component)]
struct PerformanceText;

/// System that toggles overlay visibility when the toggle key is pressed.
fn toggle_overlay_visibility(
    keys: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<PerformanceOverlayConfig>,
) {
    if keys.just_pressed(config.toggle_key) {
        config.visible = !config.visible;
        info!("Performance overlay visibility toggled: {}", config.visible);
    }
}

/// System that spawns or despawns the overlay UI based on visibility.
fn manage_overlay_ui(
    mut commands: Commands,
    config: Res<PerformanceOverlayConfig>,
    metrics: Res<PerformanceMetrics>,
    root_query: Query<Entity, With<PerformanceOverlayRoot>>,
    asset_server: Res<AssetServer>,
) {
    let is_spawned = !root_query.is_empty();
    if config.visible && !is_spawned {
        // Spawn overlay
        let (x, y) = config.position;
        let font = asset_server.load("fonts/FiraSans-Bold.ttf"); // default font, ensure it exists

        let root_id = commands.spawn((
            PerformanceOverlayRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(x),
                top: Val::Px(y),
                ..default()
            },
        )).id();

        // Optional background
        if let Some(bg_color) = config.background_color {
            commands.entity(root_id).insert(BackgroundColor(bg_color));
        }

        // Spawn text as a child
        commands.entity(root_id).with_children(|parent| {
            parent.spawn((
                PerformanceText,
                Text::new(format!("FPS: {:.1}\nFrame: {:.2} ms",
                                  metrics.fps(),
                                  metrics.frame_time_ms()
                )),
                TextFont {
                    font,
                    font_size: config.font_size,
                    ..default()
                },
                TextColor(config.text_color),
            ));
        });
        info!("Performance overlay spawned");
    } else if !config.visible && is_spawned {
        // Despawn overlay
        for entity in root_query.iter() {
            commands.entity(entity).despawn();
        }
        info!("Performance overlay despawned");
    }
}

/// System that updates the overlay text each frame.
fn update_overlay_text(
    config: Res<PerformanceOverlayConfig>,
    metrics: Res<PerformanceMetrics>,
    mut text_query: Query<&mut Text, With<PerformanceText>>,
    mut timer: Local<f32>,
    time: Res<Time>,
) {
    if !config.visible {
        return;
    }
    *timer += time.delta_secs();
    let mut updated = false;
    for mut text in text_query.iter_mut() {
        text.0 = format!("FPS: {:.1}\nFrame: {:.2} ms",
                         metrics.fps(),
                         metrics.frame_time_ms()
        );
        updated = true;
    }
    if updated && *timer >= 1.0 {
        info!("Performance overlay updated: FPS {:.1}, Frame time {:.2} ms",
            metrics.fps(), metrics.frame_time_ms());
        *timer = 0.0;
    }
}

/// Plugin for performance overlay.
pub struct PerformanceOverlayPlugin;

impl Plugin for PerformanceOverlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PerformanceMetrics>()
            .init_resource::<PerformanceOverlayConfig>()
            .add_systems(Update, (
                update_performance_metrics,
                toggle_overlay_visibility,
                manage_overlay_ui,
                update_overlay_text,
            ).chain());
    }
}