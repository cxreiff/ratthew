use bevy::{prelude::*, render::render_asset::RenderAssetUsages, utils::HashMap};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use bevy_ecs_ldtk::assets::{LdtkAssetPlugin, LdtkProject};

use crate::cube::UprightCube;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkAssetPlugin)
            .init_state::<GameStates>()
            .add_loading_state(
                LoadingState::new(GameStates::Loading)
                    .continue_to_state(GameStates::Playing)
                    .load_collection::<GameAssets>(),
            )
            .add_systems(OnEnter(GameStates::Playing), level_setup_system);
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "level.ldtk")]
    level: Handle<LdtkProject>,
    #[asset(path = "1bit.png")]
    tileset: Handle<Image>,
}

#[derive(Default, States, Clone, Debug, Hash, Eq, PartialEq)]
pub enum GameStates {
    #[default]
    Loading,
    Playing,
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

    let cube = meshes.add(UprightCube);
    let red_material = materials.add(StandardMaterial {
        base_color: Color::RED,
        ..Default::default()
    });

    let mut material_map = HashMap::new();

    if let Some(ldtk) = ldtk_assets.get(&handles.level) {
        if let Some(level) = ldtk.as_standalone().iter_loaded_levels().next() {
            for layer in level.layer_instances().iter() {
                for tile in layer.auto_layer_tiles.iter() {
                    if material_map.get(&(tile.src.x, tile.src.y)).is_none() {
                        let material_handle = materials.add(StandardMaterial {
                            reflectance: 0.1,
                            base_color_texture: Some(images.add(Image::from_dynamic(
                                tileset.crop(tile.src.x as u32, tile.src.y as u32, 16, 16),
                                true,
                                RenderAssetUsages::RENDER_WORLD,
                            ))),
                            ..Default::default()
                        });
                        material_map.insert((tile.src.x, tile.src.y), material_handle.clone());
                    }

                    commands.spawn(PbrBundle {
                        transform: Transform::from_xyz(
                            -0.065 * tile.px.x as f32,
                            0.,
                            -0.065 * tile.px.y as f32,
                        ),
                        mesh: cube.clone(),
                        material: material_map
                            .get(&(tile.src.x, tile.src.y))
                            .unwrap_or(&red_material)
                            .clone(),
                        ..Default::default()
                    });
                }
            }
        }
    }
}
