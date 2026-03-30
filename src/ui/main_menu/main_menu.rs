use crate::game::GameState;
use bevy::prelude::*;

// Компонент для маркировки корневого узла главного меню
#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct MainMenuCamera;

// Система, создающая интерфейс главного меню при входе в состояние MainMenu
fn spawn_main_menu(mut commands: Commands) {
    info!("Spawned main menu");

    commands.spawn((Camera2d, MainMenuCamera));

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
        ))
        .with_children(|parent| {
            // Заголовок
            parent.spawn((
                Text::new("Hex Game"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Кнопка "Start Game"
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect::top(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_child((
                    Text::new("Start Game"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
        });
}

// Система, удаляющая главное меню при выходе из состояния MainMenu
fn despawn_main_menu(
    mut commands: Commands,
    camera_query: Query<Entity, With<MainMenuCamera>>,
    ui_query: Query<Entity, With<MainMenuUI>>,
) {
    info!("Despawn main menu");
    for entity in camera_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in ui_query.iter() {
        commands.entity(entity).despawn();
    }
}

// Обработка нажатия на кнопки меню
fn handle_main_menu_buttons(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                // Переход в состояние загрузки
                next_state.set(GameState::Loading);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.3, 0.3, 0.3).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.2, 0.2, 0.2).into();
            }
        }
    }
}

// Плагин для главного меню
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
            );
    }
}
