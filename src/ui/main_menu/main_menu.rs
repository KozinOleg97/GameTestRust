use crate::game::GameState;
use bevy::prelude::*;
use bevy_settings::SaveSettingsSync;
use crate::ui::main_menu::buttons::MainMenuAction;

// Компонент для маркировки корневого узла главного меню (по желанию)
#[derive(Component)]
struct MainMenuUI;

// Компонент для маркировки камеры меню (по желанию)
#[derive(Component)]
struct MainMenuCamera;

// Цвета кнопок
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn spawn_main_menu(mut commands: Commands) {
    info!("Spawned main menu");

    // Камера и UI будут автоматически удалены при выходе из GameState::MainMenu
    commands.spawn((
        Camera2d,
        MainMenuCamera,
        DespawnOnExit(GameState::MainMenu),
    ));

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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            MainMenuUI,
            DespawnOnExit(GameState::MainMenu), // автоматическое удаление UI
        ))
        .with_children(|parent| {
            // Заголовок
            parent.spawn((
                Text::new("Hex Game"),
                TextFont {
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Кнопка "Start Game"
            parent
                .spawn((
                    Button,
                    MainMenuAction::StartGame,       // <- вместо StartGameButton
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect::top(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Start Game"),
                    TextFont {
                        font_size: FontSize::Px(24.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

            // Кнопка "Save settings"
            parent
                .spawn((
                    Button,
                    MainMenuAction::SaveSettings,    // <- вместо SaveSettingsButton
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect::top(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Save settings"),
                    TextFont {
                        font_size: FontSize::Px(24.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
        });
}

/// Общая система подсветки кнопок (меняет цвет при наведении/нажатии)
fn button_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON,
            Interaction::Hovered => HOVERED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        }
            .into();
    }
}

/// Централизованная обработка всех нажатий кнопок главного меню
fn menu_action(
    mut interaction_query: Query<(&Interaction, &MainMenuAction), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands
) {
    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                MainMenuAction::StartGame => {
                    next_state.set(GameState::Loading);
                }
                MainMenuAction::SaveSettings => {
                    commands.queue(SaveSettingsSync::IfChanged);
                    info!("Save settings button pressed");
                }
            }
        }
    }
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Спавн UI при входе в состояние меню
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            // Системы, работающие только когда мы в главном меню
            .add_systems(Update, (button_system, menu_action).run_if(in_state(GameState::MainMenu)));
    }
}