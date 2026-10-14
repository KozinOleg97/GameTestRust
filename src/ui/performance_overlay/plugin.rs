use crate::ui::performance_overlay::performance_overlay::{
    manage_overlay_ui, toggle_overlay_visibility, update_overlay_text,
};
use bevy::app::{App, Plugin, Update};
use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
};
use bevy::prelude::IntoScheduleConfigs;

pub struct PerformanceOverlayPlugin;

impl Plugin for PerformanceOverlayPlugin {
    fn build(&self, app: &mut App) {
        // Добавляем встроенные диагностические плагины Bevy
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_plugins(EntityCountDiagnosticsPlugin::default());
        // Плагин для отслеживания CPU и RAM
        app.add_plugins(SystemInformationDiagnosticsPlugin);

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
