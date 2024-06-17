use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use bevy_ecs_ldtk::assets::{LdtkAssetPlugin, LdtkProject};

use crate::spawning::{spawn_layer_floor, spawn_layer_walls};

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
