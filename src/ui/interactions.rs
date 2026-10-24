use crate::ui::theme::UiTheme;
use bevy::app::{App, Plugin, Update};
use bevy::picking::hover::Hovered;
use bevy::prelude::{BackgroundColor, Has, Query, Res, With};
use bevy::ui::Pressed;
use bevy::ui_widgets::Button;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        // Работает всегда, независимо от текущего GameState
        app.add_systems(Update, button_system);
    }
}

/// Перекрашивает кнопки по состоянию наведения и нажатия.
///
/// В Bevy 0.20 `Interaction` заменён на компоненты: `Hovered` (приходит из picking)
/// и `Pressed` (его ставит виджет `ui_widgets::Button`). Оба меняются добавлением и
/// снятием компонента, поэтому фильтр по `Changed` здесь не годится: снятие `Pressed`
/// на отпускании мыши не изменило бы `Hovered`, и кнопка залипла бы в нажатом цвете.
/// Вместо этого состояние читается каждый кадр, а запись идёт только при отличии.
fn button_system(
    theme: Res<UiTheme>,
    mut interaction_query: Query<(&mut BackgroundColor, &Hovered, Has<Pressed>), With<Button>>,
) {
    for (mut color, hovered, pressed) in &mut interaction_query {
        let target = if pressed {
            theme.button_pressed
        } else if hovered.0 {
            theme.button_hovered
        } else {
            theme.button_normal
        };

        if color.0 != target {
            *color = target.into();
        }
    }
}
