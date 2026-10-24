use crate::game::GameSessionState;
use crate::hex::{ChunkData, ChunkLayout, ChunkMap, HexInfo, SelectedHex, get_hex_info};
use crate::pointer::PointerOcclusion;
use crate::ui::kit::{spawn_label, spawn_panel};
use crate::ui::theme::UiTheme;
use bevy::prelude::*;

/// Маркер панели свойств гекса.
#[derive(Component)]
struct HexInfoPanel;

/// Маркер тела панели — текста, который обновляется при смене выделения.
#[derive(Component)]
struct HexInfoBody;

pub struct HexInfoPlugin;

impl Plugin for HexInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameSessionState::Active), spawn_hex_info_panel)
            .add_systems(
                Update,
                update_hex_info_panel.run_if(in_state(GameSessionState::Active)),
            );
    }
}

fn spawn_hex_info_panel(mut commands: Commands, theme: Res<UiTheme>) {
    let theme = &*theme;

    let panel = spawn_panel(
        &mut commands,
        theme,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(140.0),
            width: Val::Px(theme.hex_panel_width),
            padding: UiRect::all(Val::Px(8.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        },
        HexInfoPanel,
        GameSessionState::Active,
        PointerOcclusion::Opaque,
    );

    commands.entity(panel).with_children(|parent| {
        spawn_label(parent, theme, "Гекс", theme.font_body, HexInfoTitle);
        spawn_label(
            parent,
            theme,
            NOTHING_SELECTED,
            theme.font_small,
            HexInfoBody,
        );
    });
}

#[derive(Component)]
struct HexInfoTitle;

const NOTHING_SELECTED: &str = "Ничего не выбрано";

/// Панель читает домен напрямую: это правило по умолчанию для панелей.
fn update_hex_info_panel(
    selected: Res<SelectedHex>,
    chunk_map: Res<ChunkMap>,
    layout: Res<ChunkLayout>,
    chunks: Query<&ChunkData>,
    mut bodies: Query<&mut Text, With<HexInfoBody>>,
) {
    if !selected.is_changed() {
        return;
    }

    let text = match selected.0 {
        None => NOTHING_SELECTED.to_string(),
        Some(coords) => match get_hex_info(&chunk_map, &layout, &chunks, coords.q(), coords.r()) {
            Some(info) => format_hex_info(&info, &layout),
            None => "Гекс вне загруженных чанков".to_string(),
        },
    };

    for mut body in &mut bodies {
        body.0 = text.clone();
    }
}

fn format_hex_info(info: &HexInfo, layout: &ChunkLayout) -> String {
    let (chunk_x, chunk_y) = layout.chunk_coords(info.coords.q(), info.coords.r());

    let movement = match info.biome.base_movement_cost() {
        Some(cost) => cost.to_string(),
        None => "непроходимо".to_string(),
    };

    let terrain = if info.biome.is_water() {
        "вода"
    } else if info.biome.is_land() {
        "земля"
    } else {
        "пусто"
    };

    format!(
        "Координаты: {}\nЧанк: ({}, {})\nБиом: {}\nВысота: {}\nПеремещение: {}\nМестность: {}\nОбъекты: —",
        info.coords,
        chunk_x,
        chunk_y,
        info.biome.name(),
        info.elevation,
        movement,
        terrain,
    )
}
