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
use bevy_hanabi::{ParticleEffect, ParticleEffectBundle};
use image::DynamicImage;

use crate::{grid::GridPosition, particles::GradientEffect, GameStates};

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
    particle_handle: Res<GradientEffect>,
) {
    let tileset = images.get(&handles.tileset).unwrap();
    let mut tileset = tileset.clone().try_into_dynamic().unwrap();

    let missing_material = materials.add(StandardMaterial::from(Color::srgb(1., 0., 0.)));
    let cube_mesh = meshes.add(UprightCube);

    if let Some(ldtk) = ldtk_assets.get(&handles.level) {
        if let Some(level) = ldtk.as_standalone().iter_loaded_levels().next() {
            for layer in level.layer_instances().iter() {
                match layer.layer_instance_type {
                    bevy_ecs_ldtk::ldtk::Type::AutoLayer => spawn_layer_walls(
                        &mut commands,
                        &mut materials,
                        &mut images,
                        &mut tileset,
                        &missing_material,
                        &cube_mesh,
                        layer,
                    ),
                    bevy_ecs_ldtk::ldtk::Type::Entities => {
                        spawn_layer_entities(commands.reborrow(), &particle_handle, layer)
                    }
                    bevy_ecs_ldtk::ldtk::Type::IntGrid => spawn_layer_walls(
                        &mut commands,
                        &mut materials,
                        &mut images,
                        &mut tileset,
                        &missing_material,
                        &cube_mesh,
                        layer,
                    ),
                    bevy_ecs_ldtk::ldtk::Type::Tiles => spawn_layer_floor(
                        &mut commands,
                        &mut materials,
                        &mut images,
                        &mut tileset,
                        &missing_material,
                        &cube_mesh,
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
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    tileset: &mut DynamicImage,
    missing_material: &Handle<StandardMaterial>,
    cube_mesh: &Handle<Mesh>,
    layer: &LayerInstance,
) {
    let material_map = generate_material_map(materials, images, tileset, &layer.auto_layer_tiles);

    for tile in layer.auto_layer_tiles.iter() {
        commands.spawn((
            GridPosition(IVec3::new(
                tile.px.x / layer.c_hei,
                -tile.px.y / layer.c_hei,
                0,
            )),
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(
                material_map
                    .get(&(tile.src.x, tile.src.y))
                    .unwrap_or(missing_material)
                    .clone(),
            ),
            RenderLayers::layer(1),
            Collider,
        ));
    }
}

pub fn spawn_layer_floor(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    tileset: &mut DynamicImage,
    missing_material: &Handle<StandardMaterial>,
    cube_mesh: &Handle<Mesh>,
    layer: &LayerInstance,
) {
    let material_map = generate_material_map(materials, images, tileset, &layer.grid_tiles);

    for tile in layer.grid_tiles.iter() {
        commands.spawn((
            GridPosition(IVec3::new(
                tile.px.x / layer.c_hei,
                -tile.px.y / layer.c_hei,
                -1,
            )),
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(
                material_map
                    .get(&(tile.src.x, tile.src.y))
                    .unwrap_or(missing_material)
                    .clone(),
            ),
            RenderLayers::layer(1),
        ));
    }
}

fn spawn_layer_entities(
    mut commands: Commands,
    particle_handle: &Res<GradientEffect>,
    layer: &LayerInstance,
) {
    for entity in layer.entity_instances.iter() {
        commands.spawn((
            GridPosition(IVec3::new(
                entity.px.x / layer.c_hei,
                -entity.px.y / layer.c_hei,
                0,
            )),
            ParticleEffectBundle {
                effect: ParticleEffect::new(particle_handle.0.clone()),
                ..Default::default()
            },
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
