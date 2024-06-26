use std::io;

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    gltf::Gltf,
    prelude::*,
    render::view::RenderLayers,
    utils::{error, HashMap},
};
use bevy_ratatui_render::RatatuiRenderContext;
use bevy_scene_hook::{HookedSceneBundle, SceneHook};
use crossterm::event::KeyCode;

use crate::loading::{GameAssets, GameStates};

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
pub struct Player;

#[derive(Component)]
pub struct Sword;

#[derive(Resource, Default, Deref, DerefMut, Debug)]
pub struct KeysDown(pub HashMap<KeyCode, f32>);

fn setup_camera_system(
    mut commands: Commands,
    ratatui_render: Res<RatatuiRenderContext>,
    handles: Res<GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(-5., 0., -5.)
                    .looking_at(Vec3::new(0., 0., -5.), Vec3::Y),
                tonemapping: Tonemapping::None,
                camera: Camera {
                    order: 1,
                    target: ratatui_render.target("main").unwrap_or_default(),
                    ..default()
                },
                ..default()
            },
            Player,
        ))
        .with_children(|parent| {
            parent.spawn((PointLightBundle {
                point_light: PointLight {
                    intensity: 100000.,
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            },));
            parent.spawn((
                Camera3dBundle {
                    tonemapping: Tonemapping::None,
                    camera: Camera {
                        target: ratatui_render.target("main").unwrap_or_default(),
                        ..default()
                    },
                    ..default()
                },
                RenderLayers::layer(1),
            ));

            if let Some(gltf) = assets_gltf.get(&handles.sword) {
                let mut sword_transform =
                    Transform::from_xyz(0.3, -0.15, -0.7).with_scale(Vec3::new(0.4, 0.4, 0.4));
                sword_transform.rotate_local_x(-1.4);
                sword_transform.rotate_local_y(0.3);
                sword_transform.rotate_local_z(-0.15);

                parent.spawn((
                    HookedSceneBundle {
                        scene: SceneBundle {
                            transform: sword_transform,
                            scene: gltf.scenes[0].clone(),
                            ..Default::default()
                        },
                        hook: SceneHook::new(|_, cmds| {
                            cmds.insert(RenderLayers::layer(1));
                        }),
                    },
                    Sword,
                    RenderLayers::layer(1),
                ));
            }
            parent.spawn((
                PointLightBundle {
                    point_light: PointLight {
                        intensity: 100000.,
                        shadows_enabled: true,
                        ..default()
                    },
                    ..default()
                },
                RenderLayers::layer(1),
            ));
        });
}

pub fn move_camera_system(
    mut q_camera: Query<&mut Transform, With<Player>>,
    keys_down: Res<KeysDown>,
    time: Res<Time>,
) -> io::Result<()> {
    let mut camera_transform = q_camera.single_mut();
    for (key, _) in keys_down.iter() {
        match key {
            KeyCode::Up => {
                let forward = camera_transform.forward().normalize();
                camera_transform.translation += forward / 16.;
            }
            KeyCode::Down => {
                let back = camera_transform.back().normalize();
                camera_transform.translation += back / 16.;
            }
            KeyCode::Left => {
                camera_transform.rotate_local_y(time.delta_seconds() * 1.8);
            }
            KeyCode::Right => {
                camera_transform.rotate_local_y(-time.delta_seconds() * 1.8);
            }
            KeyCode::Char(' ') => {
                *camera_transform = Transform::from_xyz(0., 10., 0.);
                camera_transform.look_at(-Vec3::Y, Vec3::Z)
            }
            _ => {}
        }
    }

    Ok(())
}
