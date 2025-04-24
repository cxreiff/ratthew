use std::f32::consts::PI;

use bevy::{
    core_pipeline::Skybox, gltf::Gltf, prelude::*, render::view::RenderLayers, utils::HashMap,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_persistent::{Persistent, StorageFormat};
use bevy_ratatui_camera::{
    LuminanceConfig, RatatuiCamera, RatatuiCameraEdgeDetection, RatatuiCameraStrategy,
};
use serde::{Deserialize, Serialize};

use crate::{
    animation::{GridAnimated, ItemBobTween},
    config::{PLAYER_STARTING_DIRECTION, PLAYER_STARTING_POSITION},
    grid::{GridDirection, GridPosition},
    GameStates,
};

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<KeysDown>()
        .add_systems(Startup, initialize_player_persist_system)
        .add_systems(OnEnter(GameStates::Playing), setup_camera_system)
        .add_observer(update_player_persist_observer)
        .add_observer(clear_player_persist_observer);
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(key = "player.sword")]
    pub sword: Handle<Gltf>,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct PlayerPersist {
    pub position: GridPosition,
    pub direction: GridDirection,
}

impl Default for PlayerPersist {
    fn default() -> Self {
        Self {
            position: GridPosition(PLAYER_STARTING_POSITION),
            direction: GridDirection(PLAYER_STARTING_DIRECTION),
        }
    }
}

#[derive(Event, Default, Debug)]
pub struct PersistEvent;

#[derive(Event, Default, Debug)]
pub struct PersistClearEvent;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct WorldCamera;

#[derive(Component)]
pub struct BackgroundCamera;

#[derive(Component)]
pub struct Sword;

#[derive(Resource, Default, Deref, DerefMut, Debug)]
pub struct KeysDown(pub HashMap<KeyCode, f32>);

fn setup_camera_system(
    mut commands: Commands,
    handles: Res<PlayerAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    asset_server: Res<AssetServer>,
    persist: Res<Persistent<PlayerPersist>>,
) {
    commands.insert_resource(AmbientLight {
        brightness: 500.,
        ..default()
    });

    commands
        .spawn((
            PlayerCamera,
            RenderLayers::layer(0),
            Camera {
                order: 2,
                clear_color: ClearColorConfig::Custom(Color::srgba(0., 0., 0., 0.)),
                ..default()
            },
            Camera3d::default(),
            Projection::from(PerspectiveProjection {
                fov: 70.0_f32.to_radians(),
                ..default()
            }),
            RatatuiCamera::default(),
            RatatuiCameraStrategy::Luminance(LuminanceConfig {
                luminance_characters: LuminanceConfig::LUMINANCE_CHARACTERS_MISC.into(),
                bg_color_scale: 0.3,
                ..default()
            }),
            persist.position,
            persist.direction,
            GridAnimated::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                RenderLayers::layer(0),
                PointLight {
                    intensity: 13000.,
                    shadows_enabled: true,
                    ..default()
                },
            ));
            if let Some(gltf) = assets_gltf.get(&handles.sword) {
                let mut sword_transform = Transform::default().with_scale(Vec3::new(0.4, 0.4, 0.4));
                sword_transform.rotate_local_x(-1.5);
                sword_transform.rotate_local_y(0.3);
                sword_transform.rotate_local_z(-0.16);

                parent.spawn((
                    RenderLayers::layer(0), // setting this does not set gltf children
                    SceneRoot(gltf.scenes[0].clone()),
                    ItemBobTween,
                    sword_transform,
                    Sword,
                ));
            }

            parent.spawn((
                WorldCamera,
                RenderLayers::layer(1),
                Camera3d::default(),
                Camera {
                    order: 1,
                    clear_color: ClearColorConfig::Custom(Color::srgba(0., 0., 0., 0.)),
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                RatatuiCamera::default(),
                RatatuiCameraStrategy::Luminance(LuminanceConfig {
                    luminance_characters: LuminanceConfig::LUMINANCE_CHARACTERS_MISC.into(),
                    luminance_scale: 9.,
                    bg_color_scale: 0.1,
                    ..default()
                }),
                RatatuiCameraEdgeDetection {
                    color_enabled: false,
                    ..default()
                },
            ));

            parent.spawn((
                RenderLayers::layer(1),
                PointLight {
                    intensity: 40000.,
                    shadows_enabled: true,
                    ..default()
                },
            ));

            parent.spawn((
                BackgroundCamera,
                RenderLayers::layer(2),
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                RatatuiCamera::default(),
                RatatuiCameraStrategy::Luminance(LuminanceConfig {
                    bg_color_scale: 0.4,
                    luminance_characters: LuminanceConfig::LUMINANCE_CHARACTERS_MISC.into(),
                    luminance_scale: 4.,
                    ..default()
                }),
                Skybox {
                    image: asset_server.load("skybox.ktx2"),
                    rotation: Quat::from_rotation_y(PI / 2.),
                    brightness: 200.,
                },
            ));
        });
}

fn initialize_player_persist_system(mut commands: Commands) {
    commands.insert_resource(
        Persistent::<PlayerPersist>::builder()
            .name("player_persist")
            .format(StorageFormat::Toml)
            .path("persist.toml")
            .revertible(true)
            .default(PlayerPersist::default())
            .build()
            .expect("failed to initialize persistent player data"),
    );
}

fn update_player_persist_observer(
    trigger: Trigger<PersistEvent>,
    mut player_persist: ResMut<Persistent<PlayerPersist>>,
    player: Query<(&GridPosition, &GridDirection), With<PlayerCamera>>,
) {
    let Ok((&position, &direction)) = player.get(trigger.entity()) else {
        return;
    };

    player_persist
        .set(PlayerPersist {
            position,
            direction,
        })
        .expect("failed to persist player data to persist.toml");
}

fn clear_player_persist_observer(
    _trigger: Trigger<PersistClearEvent>,
    mut player_persist: ResMut<Persistent<PlayerPersist>>,
) {
    player_persist
        .revert_to_default()
        .expect("failed to clear persistent player data.");
}
