use crate::ui::actions::UiAction;
use crate::ui::theme::UiTheme;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{ActivateOnPress, Button};

/// Корневой узел экрана: на весь экран, содержимое по центру, фон меню.
///
/// Экран рождается при входе в состояние и умирает при выходе из него — это
/// единственное правило жизненного цикла для экранов. Долгоживущий HUD живёт
/// по другому правилу: создаётся один раз и переключает видимость.
pub fn spawn_screen_root<S: States>(
    commands: &mut Commands,
    theme: &UiTheme,
    state: S,
    build_children: impl FnOnce(&mut ChildSpawnerCommands),
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(theme.menu_background),
            DespawnOnExit(state),
        ))
        .with_children(build_children);
}

/// Заголовок экрана.
pub fn spawn_title(parent: &mut ChildSpawnerCommands, theme: &UiTheme, text: &str) {
    parent.spawn((
        Text::new(text),
        theme.text_font(theme.font_title),
        TextColor(theme.text),
        Node {
            margin: UiRect::bottom(Val::Px(40.0)),
            ..default()
        },
    ));
}

/// Кнопка экрана: несёт действие и активируется по нажатию.
pub fn spawn_button(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    label: &str,
    action: UiAction,
) {
    parent
        .spawn((
            Button,
            Hovered(false),
            ActivateOnPress,
            action,
            Node {
                width: Val::Px(theme.button_width),
                height: Val::Px(theme.button_height),
                margin: UiRect::top(Val::Px(theme.button_margin_top)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.button_normal),
        ))
        .with_child((
            Text::new(label),
            theme.text_font(theme.font_button),
            TextColor(theme.text),
        ));
}

/// Компактная кнопка HUD: та же логика активации, другие размеры.
pub fn spawn_hud_button(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    label: &str,
    action: UiAction,
) {
    parent
        .spawn((
            Button,
            Hovered(false),
            ActivateOnPress,
            action,
            Node {
                width: Val::Px(theme.hud_button_width),
                height: Val::Px(theme.hud_button_height),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.button_normal),
        ))
        .with_child((
            Text::new(label),
            theme.text_font(theme.font_tiny),
            TextColor(theme.text),
        ));
}

/// Текстовая строка с маркером, по которому её потом обновляет экран.
pub fn spawn_label<M: Component>(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    text: &str,
    font_size: f32,
    marker: M,
) {
    parent.spawn((
        Text::new(text),
        theme.text_font(font_size),
        TextColor(theme.text),
        marker,
    ));
}

/// Полоса прогресса: дорожка и заполнение с маркером, ширину которого
/// впоследствии меняет вызывающий экран.
pub fn spawn_progress_bar<M: Component>(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    fill_marker: M,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(theme.progress_bar_width),
                height: Val::Px(theme.progress_bar_height),
                ..default()
            },
            BackgroundColor(theme.progress_track),
        ))
        .with_child((
            Node {
                width: Val::Percent(0.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(theme.progress_fill),
            fill_marker,
        ));
}
