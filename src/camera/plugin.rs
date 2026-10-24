use crate::camera::{camera_control_system, setup_camera_on_world_generated};
use crate::game::{GameSessionState, WorldGeneratedEvent};
use crate::pointer::update_pointer_blocked;
use bevy::ecs::schedule::common_conditions::on_message;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // Управление камерой только в игровом состоянии, и только после того,
        // как посчитан перехват указателя панелями.
        app.add_systems(
            Update,
            camera_control_system
                .run_if(in_state(GameSessionState::Active))
                .after(update_pointer_blocked),
        )
        // Настройка камеры при получении события WorldGeneratedEvent
        .add_systems(
            Update,
            setup_camera_on_world_generated.run_if(on_message::<WorldGeneratedEvent>),
        );
    }
}
