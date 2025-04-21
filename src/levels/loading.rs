use std::ops::Deref;

use bevy::{asset::RenderAssetUsages, prelude::*, render::view::RenderLayers, utils::HashMap};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_ecs_ldtk::{
    assets::{LdtkAssetPlugin, LdtkProject},
    ldtk::{TileInstance, TilesetRectangle},
    prelude::LdtkFields,
    EntityInstance,
};
use bevy_hanabi::{ParticleEffect, ParticleEffectBundle};
use image::DynamicImage;

use crate::{
    camera::PlayerCamera,
    grid::{Direction, GridDirection, GridPosition},
    levels::layer::LayerData,
    GameStates,
};

use super::{
    flipped_ramp::FlippedRamp, layer::LayerVariant, torch_effect::TorchEffect,
    upright_billboard::UprightBillboard, upright_cube::UprightCube, upright_ramp::UprightRamp,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkAssetPlugin)
        .add_systems(Startup, level_load_setup_system)
        .add_systems(OnEnter(GameStates::Playing), initial_load_system)
        .add_systems(
            Update,
            (level_load_system, billboard_movement_system).run_if(in_state(GameStates::Playing)),
        )
        .add_observer(level_load_observer)
        .add_observer(billboard_setup_observer);
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

#[derive(Component, Debug, Clone)]
pub struct SpawnedFromLdtk;

#[derive(Component, Default, Debug, Clone, Deref, DerefMut)]
pub struct LdtkTilesetMaterials(HashMap<(i32, i32), Handle<StandardMaterial>>);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct UprightCubeMesh(Handle<Mesh>);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct RampMesh(Handle<Mesh>);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct FlippedRampMesh(Handle<Mesh>);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct BillboardMesh(Handle<Mesh>);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct TorchMesh(Handle<Mesh>);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct MissingMaterial(Handle<StandardMaterial>);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct TorchMaterial(Handle<StandardMaterial>);

#[derive(Component, Debug, Clone)]
pub struct Collides;

#[derive(Component, Debug, Clone)]
pub struct WallBlock;

#[derive(Component, Debug, Clone)]
#[require(GridDirection)]
pub struct RampBlock;

#[derive(Component, Debug, Clone)]
pub struct Billboard;

#[derive(Component, Debug, Clone)]
pub struct Standee;

fn level_load_setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(UprightCubeMesh(meshes.add(UprightCube)));
    commands.insert_resource(RampMesh(meshes.add(UprightRamp)));
    commands.insert_resource(FlippedRampMesh(meshes.add(FlippedRamp)));
    commands.insert_resource(BillboardMesh(meshes.add(UprightBillboard)));
    commands.insert_resource(TorchMesh(meshes.add(Cylinder::new(0.03, 0.4))));
    commands.insert_resource(MissingMaterial(
        materials.add(StandardMaterial::from(Color::srgb(1., 0., 0.))),
    ));
    commands.insert_resource(TorchMaterial(
        materials.add(StandardMaterial::from(Color::srgb(0.6, 0.4, 0.3))),
    ));
}

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
    mut images: ResMut<Assets<Image>>,
    handles: Res<LevelAssets>,
    spawned_from_ldtk: Query<Entity, With<SpawnedFromLdtk>>,
    ldtk_material_maps: Query<(Entity, &LdtkTilesetMaterials)>,
    ldtk_assets: Res<Assets<LdtkProject>>,
    particle_handle: Res<TorchEffect>,
    upright_cube_mesh: Res<UprightCubeMesh>,
    ramp_mesh: Res<RampMesh>,
    flipped_ramp_mesh: Res<FlippedRampMesh>,
    billboard_mesh: Res<BillboardMesh>,
    torch_mesh: Res<TorchMesh>,
    missing_material: Res<MissingMaterial>,
    torch_material: Res<TorchMaterial>,
) {
    let tileset = images.get(&handles.spritesheet).unwrap();
    let mut tileset = tileset.clone().try_into_dynamic().unwrap();

    for entity in &spawned_from_ldtk {
        commands.entity(entity).despawn_recursive();
    }

    for (entity, material_map) in &ldtk_material_maps {
        for material in material_map.values() {
            materials.remove(material);
        }
        commands.entity(entity).despawn();
    }

    if let Some(ldtk) = ldtk_assets.get(&handles.level) {
        for level in ldtk.as_standalone().iter_loaded_levels() {
            let offset = IVec3::new(*level.world_x(), *level.world_depth(), *level.world_y());

            for layer in level.layer_instances().iter() {
                let Ok(layer_data) = LayerData::try_from(layer) else {
                    log::info!("FAILED TO PARSE: {}", layer.identifier);
                    continue;
                };

                match layer_data.variant {
                    LayerVariant::Torches(instances) => {
                        spawn_layer_torches(
                            commands.reborrow(),
                            &particle_handle,
                            &torch_mesh,
                            &torch_material,
                            offset,
                            layer_data.sprite_size,
                            &instances,
                        );
                    }
                    LayerVariant::Walls(instances) => {
                        spawn_layer_walls(
                            &mut commands,
                            &mut materials,
                            &mut images,
                            &mut tileset,
                            offset,
                            &missing_material,
                            &upright_cube_mesh,
                            layer_data.sprite_size,
                            &instances,
                            (WallBlock, Collides),
                        );
                    }
                    LayerVariant::Ramps(instances) => {
                        spawn_layer_ramps(
                            commands.reborrow(),
                            &mut materials,
                            &mut images,
                            &mut tileset,
                            offset,
                            &missing_material,
                            layer_data.sprite_size,
                            &instances,
                            ramp_mesh.0.clone(),
                            RampBlock,
                        );
                    }
                    LayerVariant::FlippedRamps(instances) => {
                        spawn_layer_ramps(
                            commands.reborrow(),
                            &mut materials,
                            &mut images,
                            &mut tileset,
                            offset,
                            &missing_material,
                            layer_data.sprite_size,
                            &instances,
                            flipped_ramp_mesh.0.clone(),
                            Collides,
                        );
                    }
                    LayerVariant::Billboards(instances) => {
                        spawn_billboard(
                            commands.reborrow(),
                            &mut materials,
                            &mut images,
                            &mut tileset,
                            offset,
                            &missing_material,
                            layer_data.sprite_size,
                            &instances,
                            &billboard_mesh,
                            (Billboard, Collides),
                        );
                    }
                    LayerVariant::Standees(instances) => {
                        spawn_billboard(
                            commands.reborrow(),
                            &mut materials,
                            &mut images,
                            &mut tileset,
                            offset,
                            &missing_material,
                            layer_data.sprite_size,
                            &instances,
                            &billboard_mesh,
                            (Standee, Collides),
                        );
                    }
                };
            }
        }
    }
}

pub fn spawn_layer_walls<T>(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    tileset: &mut DynamicImage,
    offset: IVec3,
    missing_material: &Handle<StandardMaterial>,
    cube_mesh: &Handle<Mesh>,
    sprite_size: IVec2,
    instances: &Vec<TileInstance>,
    markers: T,
) where
    T: Clone + Bundle,
{
    let material_map = generate_ldtk_material_map(materials, images, tileset, instances);

    for tile in instances.iter() {
        commands
            .spawn((
                SpawnedFromLdtk,
                GridPosition(IVec3::new(
                    (tile.px.x + offset.x) / sprite_size.y,
                    offset.y,
                    (tile.px.y + offset.z) / sprite_size.y,
                )),
                Mesh3d(cube_mesh.clone()),
                MeshMaterial3d(
                    material_map
                        .get(&(tile.src.x, tile.src.y))
                        .unwrap_or(missing_material)
                        .clone(),
                ),
                RenderLayers::layer(1),
            ))
            .insert(markers.clone());
    }

    commands.spawn(material_map);
}

fn spawn_layer_torches(
    mut commands: Commands,
    particle_handle: &Res<TorchEffect>,
    torch_mesh: &Handle<Mesh>,
    torch_material: &Handle<StandardMaterial>,
    offset: IVec3,
    sprite_size: IVec2,
    instances: &[EntityInstance],
) {
    for entity in instances.iter() {
        commands
            .spawn((
                SpawnedFromLdtk,
                GridPosition(IVec3::new(
                    (entity.px.x + offset.x) / sprite_size.y,
                    offset.y,
                    (entity.px.y + offset.z) / sprite_size.y,
                )),
                ParticleEffectBundle {
                    effect: ParticleEffect::new(particle_handle.0.clone()),
                    ..Default::default()
                },
                RenderLayers::layer(1),
            ))
            .with_child((
                RenderLayers::layer(1),
                Mesh3d(torch_mesh.clone()),
                MeshMaterial3d(torch_material.clone()),
                Transform::from_xyz(0., -0.25, 0.),
            ));
    }
}

fn spawn_layer_ramps<T>(
    mut commands: Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    tileset: &mut DynamicImage,
    offset: IVec3,
    missing_material: &Handle<StandardMaterial>,
    sprite_size: IVec2,
    instances: &[EntityInstance],
    ramp_mesh: Handle<Mesh>,
    markers: T,
) where
    T: Clone + Bundle,
{
    let material_map = generate_ldtk_entity_material_map(materials, images, tileset, instances);

    for entity in instances.iter() {
        let Some(tile) = entity.tile else {
            continue;
        };

        let direction = match entity.get_enum_field("Direction").unwrap().deref() {
            "north" => Direction::North,
            "east" => Direction::East,
            "south" => Direction::South,
            "west" => Direction::West,
            _ => unreachable!(),
        };

        commands
            .spawn((
                SpawnedFromLdtk,
                GridPosition(IVec3::new(
                    (entity.px.x + offset.x) / sprite_size.y,
                    offset.y,
                    (entity.px.y + offset.z) / sprite_size.y,
                )),
                GridDirection(direction),
                Mesh3d(ramp_mesh.clone()),
                MeshMaterial3d(
                    material_map
                        .get(&(tile.x, tile.y))
                        .unwrap_or(missing_material)
                        .clone(),
                ),
                RenderLayers::layer(1),
                RampBlock,
            ))
            .insert(markers.clone());
    }
}

fn spawn_billboard<T>(
    mut commands: Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    tileset: &mut DynamicImage,
    offset: IVec3,
    missing_material: &Handle<StandardMaterial>,
    sprite_size: IVec2,
    instances: &[EntityInstance],
    billboard_mesh: &BillboardMesh,
    markers: T,
) where
    T: Clone + Bundle,
{
    let material_map = generate_ldtk_entity_material_map(materials, images, tileset, instances);

    for entity in instances.iter() {
        let Some(tile) = entity.tile else {
            continue;
        };

        let direction = entity
            .get_enum_field("direction")
            .map(|dir| match dir.as_str() {
                "north" => Direction::North,
                "east" => Direction::East,
                "south" => Direction::South,
                "west" => Direction::West,
                _ => unreachable!(),
            })
            .unwrap_or(Direction::North);

        let tile_material = material_map
            .get(&(tile.x, tile.y))
            .unwrap_or(missing_material)
            .clone();

        commands
            .spawn((
                SpawnedFromLdtk,
                GridPosition(IVec3::new(
                    (entity.px.x + offset.x) / sprite_size.y,
                    offset.y,
                    (entity.px.y + offset.z) / sprite_size.y,
                )),
                GridDirection::default(),
                Mesh3d(billboard_mesh.0.clone()),
                MeshMaterial3d(tile_material),
                RenderLayers::layer(1),
            ))
            .insert((markers.clone(), GridDirection(direction)));
    }
}

fn billboard_setup_observer(
    trigger: Trigger<OnInsert, Billboard>,
    player_camera: Query<&GridDirection, With<PlayerCamera>>,
    mut billboards: Query<&mut GridDirection, (Without<PlayerCamera>, With<Billboard>)>,
) {
    let Ok(camera_direction) = player_camera.get_single() else {
        return;
    };
    let Ok(mut billboard_direction) = billboards.get_mut(trigger.entity()) else {
        return;
    };

    *billboard_direction = camera_direction.reverse();
}

fn billboard_movement_system(
    player_camera: Query<&GridDirection, (With<PlayerCamera>, Changed<GridDirection>)>,
    mut billboards: Query<&mut GridDirection, (Without<PlayerCamera>, With<Billboard>)>,
) {
    if let Ok(camera_direction) = player_camera.get_single() {
        for mut billboard_direction in &mut billboards {
            *billboard_direction = camera_direction.reverse();
        }
    }
}

fn generate_ldtk_material_map(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    tileset: &mut DynamicImage,
    tiles: &Vec<TileInstance>,
) -> LdtkTilesetMaterials {
    let mut ldtk_material_map = LdtkTilesetMaterials::default();
    for tile in tiles {
        if ldtk_material_map.get(&(tile.src.x, tile.src.y)).is_none() {
            let material_handle = generate_spritesheet_material(materials, images, tileset, tile);
            ldtk_material_map.insert((tile.src.x, tile.src.y), material_handle.clone());
        }
    }

    ldtk_material_map
}

fn generate_spritesheet_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    tileset: &mut DynamicImage,
    tile: &TileInstance,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        reflectance: 0.,
        perceptual_roughness: 1.0,

        base_color_texture: Some(images.add(Image::from_dynamic(
            tileset.crop(tile.src.x as u32, tile.src.y as u32, 16, 16),
            true,
            RenderAssetUsages::RENDER_WORLD,
        ))),
        ..Default::default()
    })
}

fn generate_ldtk_entity_material_map(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    tileset: &mut DynamicImage,
    entities: &[EntityInstance],
) -> LdtkTilesetMaterials {
    let mut ldtk_material_map = LdtkTilesetMaterials::default();
    for entity in entities {
        if let Some(tile) = entity.tile {
            if ldtk_material_map.get(&(tile.x, tile.y)).is_none() {
                let material_handle =
                    generate_spritesheet_material_entity(materials, images, tileset, &tile);
                ldtk_material_map.insert((tile.x, tile.y), material_handle.clone());
            }
        }
    }

    ldtk_material_map
}

fn generate_spritesheet_material_entity(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    tileset: &mut DynamicImage,
    tile: &TilesetRectangle,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        reflectance: 0.,
        perceptual_roughness: 1.0,
        alpha_mode: AlphaMode::AlphaToCoverage,
        base_color_texture: Some(images.add(Image::from_dynamic(
            tileset.crop(tile.x as u32, tile.y as u32, 16, 16),
            true,
            RenderAssetUsages::RENDER_WORLD,
        ))),
        ..Default::default()
    })
}
