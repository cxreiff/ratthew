use std::io;

use bevy::{
    gltf::Gltf,
    prelude::*,
    render::view::RenderLayers,
    utils::{error, HashMap},
};
use bevy_hanabi::{ParticleEffect, ParticleEffectBundle};
use bevy_ratatui_camera::{LuminanceConfig, RatatuiCamera, RatatuiCameraStrategy};

use crate::{levels::loading::GameAssets, particles::GradientEffect, Flags, GameStates};

pub struct ViewCameraPlugin;

impl Plugin for ViewCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeysDown>()
            .add_systems(OnEnter(GameStates::Playing), setup_camera_system)
            .add_systems(
                Update,
                move_camera_system
                    .map(error)
                    .run_if(in_state(GameStates::Playing)),
            );
    }
}

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct WorldCamera;

#[derive(Component)]
pub struct ParticleCamera;

#[derive(Component)]
pub struct Sword;

#[derive(Resource, Default, Deref, DerefMut, Debug)]
pub struct KeysDown(pub HashMap<KeyCode, f32>);

fn setup_camera_system(
    mut commands: Commands,
    handles: Res<GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    commands
        .spawn((
            RenderLayers::layer(0),
            Camera {
                order: 1,
                ..default()
            },
            Camera3d::default(),
            Transform::from_translation(Vec3::new(3., -13., 0.))
                .looking_at(Vec3::new(2., -13., 0.), Vec3::Z),
            RatatuiCamera::default(),
            RatatuiCameraStrategy::Luminance(LuminanceConfig {
                mask_color: Some(ratatui::style::Color::Rgb(0, 0, 0)),
                luminance_characters: LuminanceConfig::LUMINANCE_CHARACTERS_SHADING.into(),
                ..default()
            }),
            PlayerCamera,
        ))
        .with_children(|parent| {
            parent.spawn((
                RenderLayers::layer(0),
                PointLight {
                    intensity: 100000.,
                    shadows_enabled: true,
                    ..default()
                },
            ));

            parent.spawn((
                RenderLayers::layer(1),
                Camera3d::default(),
                RatatuiCamera::default(),
                RatatuiCameraStrategy::luminance_misc(),
                WorldCamera,
            ));
            if let Some(gltf) = assets_gltf.get(&handles.sword) {
                let mut sword_transform =
                    Transform::from_xyz(0.3, -0.15, -0.7).with_scale(Vec3::new(0.4, 0.4, 0.4));
                sword_transform.rotate_local_x(-1.4);
                sword_transform.rotate_local_y(0.3);
                sword_transform.rotate_local_z(-0.15);

                parent.spawn((
                    RenderLayers::layer(1),
                    SceneRoot(gltf.scenes[0].clone()),
                    sword_transform,
                    Sword,
                ));
            }
            parent.spawn((
                RenderLayers::layer(1),
                PointLight {
                    intensity: 100000.,
                    shadows_enabled: true,
                    ..default()
                },
            ));

            parent.spawn((
                RenderLayers::layer(2),
                Camera {
                    order: 2,
                    ..default()
                },
                Camera3d::default(),
                RatatuiCamera::default(),
                RatatuiCameraStrategy::Luminance(LuminanceConfig {
                    mask_color: Some(ratatui::style::Color::Rgb(0, 0, 0)),
                    luminance_characters: LuminanceConfig::LUMINANCE_CHARACTERS_MISC.into(),
                    ..default()
                }),
                ParticleCamera,
            ));
        });
}

pub fn move_camera_system(
    mut commands: Commands,
    mut q_camera: Query<&mut Transform, With<PlayerCamera>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut exit: EventWriter<AppExit>,
    mut flags: ResMut<Flags>,
    effect_handle: Res<GradientEffect>,
) -> io::Result<()> {
    let mut camera_transform = q_camera.single_mut();

    for press in input.get_pressed() {
        match press {
            KeyCode::ArrowUp => {
                let forward = camera_transform.forward().normalize();
                camera_transform.translation += forward / 16.;
            }
            KeyCode::ArrowDown => {
                let back = camera_transform.back().normalize();
                camera_transform.translation += back / 16.;
            }
            KeyCode::ArrowLeft => {
                camera_transform.rotate_local_y(time.delta_secs() * 1.8);
            }
            KeyCode::ArrowRight => {
                camera_transform.rotate_local_y(-time.delta_secs() * 1.8);
            }
            _ => {}
        }
    }

    for press in input.get_just_pressed() {
        match press {
            KeyCode::KeyQ => {
                exit.send_default();
            }
            KeyCode::KeyD => {
                flags.debug = !flags.debug;
            }
            KeyCode::KeyG => {
                commands.spawn((
                    ParticleEffectBundle {
                        effect: ParticleEffect::new(effect_handle.0.clone()),
                        transform: Transform::from_xyz(3., -8., 0.),
                        ..Default::default()
                    },
                    RenderLayers::layer(2),
                ));
            }
            _ => {}
        }
    }

    Ok(())
}
