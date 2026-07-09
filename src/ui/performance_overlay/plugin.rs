use bevy::prelude::IntoScheduleConfigs;
use bevy::app::{App, Plugin, Update};
use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use crate::ui::performance_overlay::performance_overlay::{manage_overlay_ui, toggle_overlay_visibility, update_overlay_text};

pub struct PerformanceOverlayPlugin;

impl Plugin for PerformanceOverlayPlugin {
    fn build(&self, app: &mut App) {
        // Добавляем встроенные диагностические плагины Bevy
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_plugins(EntityCountDiagnosticsPlugin::default());

        app.add_systems(
            Update,
            (
                toggle_overlay_visibility,
                manage_overlay_ui,
                update_overlay_text,
            )
                .chain(),
        );

    }
}