use std::io;

use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*, utils::error};
use bevy_ratatui_render::RatatuiRenderContext;
use crossterm::event::KeyCode;

pub struct ViewCameraPlugin;

impl Plugin for ViewCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeysDown>()
            .add_systems(Startup, setup_camera_system)
            .add_systems(Update, move_camera_system.map(error));
    }
}

#[derive(Component)]
pub struct ViewCamera;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct KeysDown(pub Vec<KeyCode>);

fn setup_camera_system(mut commands: Commands, ratatui_render: Res<RatatuiRenderContext>) {
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(-5., 0., -5.)
                    .looking_at(Vec3::new(0., 0., -5.), Vec3::Y),
                tonemapping: Tonemapping::None,
                camera: Camera {
                    target: ratatui_render.target("main").unwrap_or_default(),
                    ..default()
                },
                ..default()
            },
            ViewCamera,
        ))
        .with_children(|parent| {
            parent.spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 100000.,
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            });
        });
}

pub fn move_camera_system(
    mut q_camera: Query<&mut Transform, With<ViewCamera>>,
    keys_down: Res<KeysDown>,
    time: Res<Time>,
) -> io::Result<()> {
    let mut camera_transform = q_camera.single_mut();
    for key in keys_down.iter() {
        match key {
            KeyCode::Up => {
                let forward = camera_transform.forward().normalize();
                camera_transform.translation += forward / 12.;
            }
            KeyCode::Down => {
                let back = camera_transform.back().normalize();
                camera_transform.translation += back / 12.;
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
