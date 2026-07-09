use bevy::app::{App, Plugin, Update};
use bevy::prelude::{BackgroundColor, Button, Changed, Interaction, Query, With};
use crate::ui::style::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        // Работает всегда, независимо от текущего GameState
        app.add_systems(Update, button_system);
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON,
            Interaction::Hovered => HOVERED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        }.into();
    }
}