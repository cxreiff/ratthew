use bevy::{
    asset::RenderAssetUsages, gltf::Gltf, prelude::*, render::view::RenderLayers, utils::HashMap,
};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use bevy_ecs_ldtk::{
    assets::{LdtkAssetPlugin, LdtkProject},
    ldtk::{LayerInstance, TileInstance},
};
use image::DynamicImage;

use crate::GameStates;

use super::cube::UprightCube;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkAssetPlugin)
        .init_state::<GameStates>()
        .add_loading_state(
            LoadingState::new(GameStates::Loading)
                .continue_to_state(GameStates::Playing)
                .load_collection::<GameAssets>(),
        )
        .add_systems(OnEnter(GameStates::Playing), level_setup_system);
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "level.ldtk")]
    level: Handle<LdtkProject>,
    #[asset(path = "1bit.png")]
    tileset: Handle<Image>,
    #[asset(path = "sword.glb")]
    pub sword: Handle<Gltf>,
}

fn level_setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    handles: Res<GameAssets>,
    ldtk_assets: Res<Assets<LdtkProject>>,
    mut images: ResMut<Assets<Image>>,
) {
    let tileset = images.get(&handles.tileset).unwrap();
    let mut tileset = tileset.clone().try_into_dynamic().unwrap();

    if let Some(ldtk) = ldtk_assets.get(&handles.level) {
        if let Some(level) = ldtk.as_standalone().iter_loaded_levels().next() {
            for layer in level.layer_instances().iter() {
                match layer.layer_instance_type {
                    bevy_ecs_ldtk::ldtk::Type::AutoLayer => spawn_layer_walls(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &mut images,
                        &mut tileset,
                        layer,
                    ),
                    bevy_ecs_ldtk::ldtk::Type::Entities => {}
                    bevy_ecs_ldtk::ldtk::Type::IntGrid => spawn_layer_walls(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &mut images,
                        &mut tileset,
                        layer,
                    ),
                    bevy_ecs_ldtk::ldtk::Type::Tiles => spawn_layer_floor(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &mut images,
                        &mut tileset,
                        layer,
                    ),
                }
            }
        }
    }
}

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
        base_color: Color::srgb(1., 0., 0.),
        ..Default::default()
    });

    let material_map = generate_material_map(materials, images, tileset, &layer.auto_layer_tiles);

    for tile in layer.auto_layer_tiles.iter() {
        commands.spawn((
            Transform::from_xyz(0.0625 * tile.px.x as f32, -0.0625 * tile.px.y as f32, 0.0),
            Mesh3d(cube.clone()),
            MeshMaterial3d(
                material_map
                    .get(&(tile.src.x, tile.src.y))
                    .unwrap_or(&missing_material)
                    .clone(),
            ),
            RenderLayers::layer(1),
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
    let floor = meshes.add(Cuboid::new(1., 1., 0.1));
    let missing_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1., 0., 0.),
        ..Default::default()
    });

    let material_map = generate_material_map(materials, images, tileset, &layer.grid_tiles);

    for tile in layer.grid_tiles.iter() {
        commands.spawn((
            Transform::from_xyz(0.0625 * tile.px.x as f32, -0.0625 * tile.px.y as f32, -0.5),
            Mesh3d(floor.clone()),
            MeshMaterial3d(
                material_map
                    .get(&(tile.src.x, tile.src.y))
                    .unwrap_or(&missing_material)
                    .clone(),
            ),
            RenderLayers::layer(1),
        ));
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
