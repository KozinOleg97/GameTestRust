use crate::ui::style::*;
use bevy::prelude::*;

/// Создает корневой узел меню на весь экран и автоматически удаляет его
/// при выходе из указанного состояния (работает как с обычными States, так и с SubStates)
pub fn spawn_menu_root<S: States>(
    commands: &mut Commands,
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
            BackgroundColor(MENU_BACKGROUND_COLOR),
            DespawnOnExit(state),
        ))
        .with_children(build_children); // Передаем замыкание внутрь
}

/// Создает стилизованный заголовок меню
pub fn spawn_title(parent: &mut ChildSpawnerCommands, text: &str) {
    parent.spawn((
        Text::new(text),
        TextFont {
            font_size: FontSize::Px(TITLE_FONT_SIZE),
            ..default()
        },
        TextColor(TEXT_COLOR),
        // Node для того, чтобы задать отступ снизу
        Node {
            margin: UiRect::bottom(Val::Px(40.0)), // Пространство между заголовком и кнопками
            ..default()
        },
    ));
}

/// Универсальная функция для создания текстовой кнопки с экшеном
pub fn spawn_text_button<T: Component>(
    parent: &mut ChildSpawnerCommands,
    text: &str,
    action: T, // Принимает компонент MainMenuAction, PauseMenuAction ...
) {
    parent
        .spawn((
            Button,
            action, // Привязываем конкретный экшен
            Node {
                width: Val::Px(BUTTON_WIDTH),
                height: Val::Px(BUTTON_HEIGHT),
                margin: UiRect::top(Val::Px(BUTTON_MARGIN_TOP)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_child((
            Text::new(text),
            TextFont {
                font_size: FontSize::Px(BUTTON_FONT_SIZE),
                ..default()
            },
            TextColor(TEXT_COLOR),
        ));
}
