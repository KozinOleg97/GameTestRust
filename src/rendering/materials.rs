use crate::hex::HexType;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub fn setup_hex_materials(
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> (Handle<Image>, Handle<StandardMaterial>) {
    let width = 8;
    let height = 1;
    let mut data = Vec::with_capacity(width * height * 4);
    for i in 0..width {
        let hex_type = match i {
            0 => HexType::Empty,
            1 => HexType::Plains,
            2 => HexType::Forest,
            3 => HexType::Mountains,
            4 => HexType::Desert,
            5 => HexType::Ocean,
            6 => HexType::Coast,
            7 => HexType::Swamp,
            _ => HexType::Empty,
        };
        let color = hex_type.color();
        data.push((color.0 * 255.0) as u8);
        data.push((color.1 * 255.0) as u8);
        data.push((color.2 * 255.0) as u8);
        data.push((color.3 * 255.0) as u8);
    }
    let texture = Image::new(
        Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    let texture_handle = images.add(texture);
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        cull_mode: None,
        ..default()
    });
    (texture_handle, material)
}
