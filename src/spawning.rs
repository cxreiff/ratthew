use bevy::{prelude::*, render::render_asset::RenderAssetUsages, utils::HashMap};
use bevy_ecs_ldtk::ldtk::{LayerInstance, TileInstance};
use image::DynamicImage;

use crate::cube::UprightCube;

#[derive(Component)]
pub struct Collider;

pub fn spawn_layer_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    tileset: &mut DynamicImage,
    layer: &LayerInstance,
) {
    let cube = meshes.add(UprightCube);
    let missing_material = materials.add(StandardMaterial {
        base_color: Color::RED,
        ..Default::default()
    });

    let material_map = generate_material_map(materials, images, tileset, &layer.auto_layer_tiles);

    for tile in layer.auto_layer_tiles.iter() {
        commands.spawn((
            PbrBundle {
                transform: Transform::from_xyz(
                    -0.063 * tile.px.x as f32,
                    0.,
                    -0.063 * tile.px.y as f32,
                ),
                mesh: cube.clone(),
                material: material_map
                    .get(&(tile.src.x, tile.src.y))
                    .unwrap_or(&missing_material)
                    .clone(),
                ..Default::default()
            },
            Collider,
        ));
    }
}

pub fn spawn_layer_floor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    tileset: &mut DynamicImage,
    layer: &LayerInstance,
) {
    let floor = meshes.add(Cuboid::new(1., 0.1, 1.));
    let missing_material = materials.add(StandardMaterial {
        base_color: Color::RED,
        ..Default::default()
    });

    let material_map = generate_material_map(materials, images, tileset, &layer.grid_tiles);

    for tile in layer.grid_tiles.iter() {
        commands.spawn(PbrBundle {
            transform: Transform::from_xyz(
                -0.063 * tile.px.x as f32,
                -0.5,
                -0.063 * tile.px.y as f32,
            ),
            mesh: floor.clone(),
            material: material_map
                .get(&(tile.src.x, tile.src.y))
                .unwrap_or(&missing_material)
                .clone(),
            ..Default::default()
        });
    }
}

fn generate_material_map(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    tileset: &mut DynamicImage,
    tiles: &Vec<TileInstance>,
) -> HashMap<(i32, i32), Handle<StandardMaterial>> {
    let mut material_map = HashMap::new();
    for tile in tiles {
        if material_map.get(&(tile.src.x, tile.src.y)).is_none() {
            let material_handle = generate_spritesheet_material(materials, images, tileset, tile);
            material_map.insert((tile.src.x, tile.src.y), material_handle.clone());
        }
    }

    material_map
}

fn generate_spritesheet_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    tileset: &mut DynamicImage,
    tile: &TileInstance,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        reflectance: 0.,
        base_color_texture: Some(images.add(Image::from_dynamic(
            tileset.crop(tile.src.x as u32, tile.src.y as u32, 16, 16),
            true,
            RenderAssetUsages::RENDER_WORLD,
        ))),
        ..Default::default()
    })
}
