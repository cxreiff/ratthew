use bevy::{
    core_pipeline::Skybox, gltf::Gltf, prelude::*, render::view::RenderLayers, utils::HashMap,
};
use bevy_ratatui_camera::{
    LuminanceConfig, RatatuiCamera, RatatuiCameraEdgeDetection, RatatuiCameraStrategy,
};

use crate::{
    animation::{GridAnimated, ItemBobTween},
    grid::{Direction, GridDirection, GridPosition},
    levels::GameAssets,
    GameStates,
};

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<KeysDown>()
        .add_systems(OnEnter(GameStates::Playing), setup_camera_system);
}

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
    handles: Res<GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    asset_server: Res<AssetServer>,
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
            RatatuiCamera::default(),
            RatatuiCameraStrategy::Luminance(LuminanceConfig {
                luminance_characters: LuminanceConfig::LUMINANCE_CHARACTERS_MISC.into(),
                bg_color_scale: 0.3,
                ..default()
            }),
            GridPosition(IVec3::new(3, 1, 7)),
            GridDirection(Direction::South),
            GridAnimated::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                RenderLayers::layer(0),
                PointLight {
                    intensity: 15000.,
                    shadows_enabled: true,
                    ..default()
                },
            ));

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
                RenderLayers::layer(1),
                PointLight {
                    intensity: 70000.,
                    shadows_enabled: true,
                    ..default()
                },
            ));

            parent.spawn((
                BackgroundCamera,
                RenderLayers::layer(2),
                Camera3d::default(),
                Camera {
                    order: 0,
                    ..default()
                },
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
                    brightness: 200.,
                    ..default()
                },
            ));
        });
}
