use bevy::{
    asset::RenderAssetUsages, gltf::Gltf, prelude::*, render::view::RenderLayers, utils::HashMap,
};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use bevy_ecs_ldtk::{
    assets::{LdtkAssetPlugin, LdtkProject},
    ldtk::TileInstance,
    EntityInstance,
};
use bevy_hanabi::{ParticleEffect, ParticleEffectBundle};
use image::DynamicImage;

use crate::{
    grid::{GridDirection, GridPosition},
    levels::layer::LayerData,
    particles::GradientEffect,
    GameStates,
};

use super::{layer::LayerVariant, ramp::UprightRamp, upright_cube::UprightCube};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkAssetPlugin)
        .init_state::<GameStates>()
        .add_loading_state(
            LoadingState::new(GameStates::Loading)
                .continue_to_state(GameStates::Playing)
                .load_collection::<GameAssets>(),
        )
        .add_systems(Startup, level_load_setup_system)
        .add_systems(OnEnter(GameStates::Playing), initial_load_system)
        .add_systems(
            Update,
            level_load_system.run_if(in_state(GameStates::Playing)),
        )
        .add_observer(level_load_observer);
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
pub struct MissingMaterial(Handle<StandardMaterial>);

#[derive(Component, Debug, Clone)]
pub struct Wall;

#[derive(Component, Debug, Clone)]
#[require(GridDirection)]
pub struct Ramp;

fn level_load_setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(UprightCubeMesh(meshes.add(UprightCube)));
    commands.insert_resource(RampMesh(meshes.add(UprightRamp)));
    commands.insert_resource(MissingMaterial(
        materials.add(StandardMaterial::from(Color::srgb(1., 0., 0.))),
    ));
}

fn initial_load_system(mut commands: Commands) {
    commands.trigger(LdtkLevelLoad);
}

fn level_load_system(
    mut commands: Commands,
    handles: Res<GameAssets>,
    mut ldtk_asset_events: EventReader<AssetEvent<LdtkProject>>,
    mut image_asset_events: EventReader<AssetEvent<Image>>,
) {
    for event in image_asset_events.read() {
        if let AssetEvent::Modified { id } = event {
            if handles.tileset.id() == *id {
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
    handles: Res<GameAssets>,
    spawned_from_ldtk: Query<Entity, With<SpawnedFromLdtk>>,
    ldtk_material_maps: Query<(Entity, &LdtkTilesetMaterials)>,
    ldtk_assets: Res<Assets<LdtkProject>>,
    particle_handle: Res<GradientEffect>,
    upright_cube_mesh: Res<UprightCubeMesh>,
    ramp_mesh: Res<RampMesh>,
    missing_material: Res<MissingMaterial>,
) {
    let tileset = images.get(&handles.tileset).unwrap();
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
            let altitude = level.world_depth();

            for layer in level.layer_instances().iter() {
                let Ok(layer_data) = LayerData::try_from(layer) else {
                    log::info!("FAILED TO PARSE: {}", layer.identifier);
                    continue;
                };

                match layer_data.variant {
                    LayerVariant::Particles(instances) => {
                        spawn_layer_entities(
                            commands.reborrow(),
                            &particle_handle,
                            *altitude,
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
                            &missing_material,
                            &upright_cube_mesh,
                            *altitude,
                            layer_data.sprite_size,
                            &instances,
                            Wall,
                        );
                    }
                    LayerVariant::Ramps(instances) => {
                        spawn_layer_walls(
                            &mut commands,
                            &mut materials,
                            &mut images,
                            &mut tileset,
                            &missing_material,
                            &ramp_mesh,
                            *altitude,
                            layer_data.sprite_size,
                            &instances,
                            Ramp,
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
    missing_material: &Handle<StandardMaterial>,
    cube_mesh: &Handle<Mesh>,
    altitude: i32,
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
                    tile.px.x / sprite_size.y,
                    altitude,
                    tile.px.y / sprite_size.y,
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

fn spawn_layer_entities(
    mut commands: Commands,
    particle_handle: &Res<GradientEffect>,
    altitude: i32,
    sprite_size: IVec2,
    instances: &[EntityInstance],
) {
    for entity in instances.iter() {
        commands.spawn((
            SpawnedFromLdtk,
            GridPosition(IVec3::new(
                entity.px.x / sprite_size.y,
                altitude,
                entity.px.y / sprite_size.y,
            )),
            ParticleEffectBundle {
                effect: ParticleEffect::new(particle_handle.0.clone()),
                ..Default::default()
            },
            RenderLayers::layer(1),
        ));
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
