use std::ops::Deref;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_ecs_ldtk::assets::{LdtkAssetPlugin, LdtkProject};

use crate::{
    blocks::{
        BillboardBlock, BlockLayer, BlockMeshMap, BlockSpawnedFromLdtk, RampBlock,
        RampFlippedBlock, TorchBlock, WallBlock,
    },
    GameStates,
};

use super::column::ColumnBlock;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkAssetPlugin)
        .add_systems(OnEnter(GameStates::Playing), initial_load_system)
        .add_systems(
            Update,
            level_load_system.run_if(in_state(GameStates::Playing)),
        )
        .add_observer(level_load_observer);
}

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    #[asset(key = "levels.level")]
    level: Handle<LdtkProject>,
    #[asset(key = "levels.spritesheet")]
    spritesheet: Handle<Image>,
}

#[derive(Event, Debug, Clone)]
pub struct LdtkLevelLoad;

fn initial_load_system(mut commands: Commands) {
    commands.trigger(LdtkLevelLoad);
}

fn level_load_system(
    mut commands: Commands,
    handles: Res<LevelAssets>,
    mut ldtk_asset_events: EventReader<AssetEvent<LdtkProject>>,
    mut image_asset_events: EventReader<AssetEvent<Image>>,
) {
    for event in image_asset_events.read() {
        if let AssetEvent::Modified { id } = event {
            if handles.spritesheet.id() == *id {
                commands.trigger(LdtkLevelLoad);
            }
        }
    }

    for event in ldtk_asset_events.read() {
        if let AssetEvent::Modified { .. } | AssetEvent::Added { .. } = event {
            commands.trigger(LdtkLevelLoad);
        }
    }
}

fn level_load_observer(
    _trigger: Trigger<LdtkLevelLoad>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    images: ResMut<Assets<Image>>,
    handles: Res<LevelAssets>,
    mesh_maps: Query<
        (
            Entity,
            Option<&BlockMeshMap<WallBlock>>,
            Option<&BlockMeshMap<RampBlock>>,
            Option<&BlockMeshMap<RampFlippedBlock>>,
            Option<&BlockMeshMap<BillboardBlock>>,
            Option<&BlockMeshMap<ColumnBlock>>,
        ),
        Or<(
            With<BlockMeshMap<WallBlock>>,
            With<BlockMeshMap<RampBlock>>,
            With<BlockMeshMap<RampFlippedBlock>>,
            With<BlockMeshMap<BillboardBlock>>,
            With<BlockMeshMap<ColumnBlock>>,
        )>,
    >,
    spawned_from_ldtk: Query<Entity, With<BlockSpawnedFromLdtk>>,
    ldtk_assets: Res<Assets<LdtkProject>>,
) {
    let tileset = images.get(&handles.spritesheet).unwrap();
    let tileset = tileset.clone().try_into_dynamic().unwrap();

    for entity in &spawned_from_ldtk {
        commands.entity(entity).despawn_recursive();
    }

    for (entity, w, r, rf, b, c) in &mesh_maps {
        if let Some(w) = w {
            for mesh in w.values() {
                meshes.remove(mesh.deref());
            }
            commands.entity(entity).despawn_recursive();
        };

        if let Some(r) = r {
            for mesh in r.values() {
                meshes.remove(mesh.deref());
            }
            commands.entity(entity).despawn_recursive();
        };

        if let Some(rf) = rf {
            for mesh in rf.values() {
                meshes.remove(mesh.deref());
            }
            commands.entity(entity).despawn_recursive();
        };

        if let Some(b) = b {
            for mesh in b.values() {
                meshes.remove(mesh.deref());
            }
            commands.entity(entity).despawn_recursive();
        };

        if let Some(c) = c {
            for mesh in c.values() {
                meshes.remove(mesh.deref());
            }
            commands.entity(entity).despawn_recursive();
        };
    }

    // TODO: Should only be added once, in a setup system.
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(handles.spritesheet.clone()),
        alpha_mode: AlphaMode::AlphaToCoverage,
        perceptual_roughness: 1.0,
        reflectance: 0.,
        double_sided: true,
        ..default()
    });

    if let Some(ldtk) = ldtk_assets.get(&handles.level) {
        for ref level in ldtk.as_standalone().iter_loaded_levels() {
            for layer in level.layer_instances().iter() {
                let Some((variant_str, _)) = layer.identifier.split_once('_') else {
                    log::error!("FAILED TO PARSE: {}", layer.identifier);
                    continue;
                };

                match variant_str {
                    "walls" => BlockLayer::<WallBlock>::build(level, layer).spawn(
                        commands.reborrow(),
                        &mut meshes,
                        &tileset,
                        &material,
                    ),
                    "ramps" => BlockLayer::<RampBlock>::build(level, layer).spawn(
                        commands.reborrow(),
                        &mut meshes,
                        &tileset,
                        &material,
                    ),
                    "flippedramps" => BlockLayer::<RampFlippedBlock>::build(level, layer).spawn(
                        commands.reborrow(),
                        &mut meshes,
                        &tileset,
                        &material,
                    ),
                    "billboards" => BlockLayer::<BillboardBlock>::build(level, layer).spawn(
                        commands.reborrow(),
                        &mut meshes,
                        &tileset,
                        &material,
                    ),
                    "torches" => BlockLayer::<TorchBlock>::build(level, layer).spawn(
                        commands.reborrow(),
                        &mut meshes,
                        &tileset,
                        &material,
                    ),
                    "columns" => BlockLayer::<ColumnBlock>::build(level, layer).spawn(
                        commands.reborrow(),
                        &mut meshes,
                        &tileset,
                        &material,
                    ),
                    _ => {
                        log::error!("FAILED TO PARSE: {}", layer.identifier);
                        continue;
                    }
                };
            }
        }
    }
}
