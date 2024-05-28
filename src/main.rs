use std::{io, time::Duration};

use bevy::{
    app::ScheduleRunnerPlugin,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    utils::error,
    window::ExitCondition,
};
use bevy_rat::{
    rat_create, rat_receive, RatCreateOutput, RatEvent, RatPlugin, RatReceiveOutput,
    RatRenderPlugin, RatRenderWidget, RatResource,
};
use crossterm::event;
use ratatui::{text::Text, widgets::Padding};

#[derive(Resource, Default)]
pub struct Flags {
    debug: bool,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: ExitCondition::DontExit,
                    close_when_requested: false,
                }),
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(1. / 60.)),
            FrameTimeDiagnosticsPlugin,
            RatPlugin,
            RatRenderPlugin::new(256, 256),
        ))
        .insert_resource(Flags::default())
        .insert_resource(ClearColor(Color::srgb_u8(0, 0, 0)))
        .add_systems(Startup, rat_create.pipe(setup_camera))
        .add_systems(Update, rat_receive.pipe(rat_print).map(error))
        .add_systems(Update, handle_keys.map(error))
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_camera(In(target): In<RatCreateOutput>, mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3., 3., 3.0).looking_at(Vec3::ZERO, Vec3::Z),
        camera: Camera {
            target,
            ..default()
        },
        ..default()
    });
}

fn rat_print(
    In(image): In<RatReceiveOutput>,
    mut rat: ResMut<RatResource>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
) -> io::Result<()> {
    if let Some(image) = image {
        rat.terminal.draw(|frame| {
            frame.render_widget(RatRenderWidget::new(image), frame.size());

            if flags.debug {
                frame.render_widget(
                    Text::from(format!(
                        "{:.0}",
                        diagnostics
                            .get(&FrameTimeDiagnosticsPlugin::FPS)
                            .and_then(|fps| fps.smoothed())
                            .unwrap_or(0.)
                    ))
                    .right_aligned(),
                    frame.size(),
                );
            }
        })?;
    }

    Ok(())
}

pub fn handle_keys(
    mut rat_events: EventReader<RatEvent>,
    mut exit: EventWriter<AppExit>,
    mut flags: ResMut<Flags>,
) -> io::Result<()> {
    for ev in rat_events.read() {
        if let RatEvent(event::Event::Key(key_event)) = ev {
            if key_event.kind == event::KeyEventKind::Press {
                match key_event.code {
                    event::KeyCode::Char('q') => {
                        exit.send(AppExit::Success);
                    }

                    event::KeyCode::Char('d') => {
                        flags.debug = !flags.debug;
                    }

                    _ => {}
                };
            }
        }
    }

    Ok(())
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(StandardMaterial {
            base_color: bevy::prelude::Color::srgb_u8(100, 140, 180),
            ..Default::default()
        }),
        transform: Transform::default(),
        ..Default::default()
    },));
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::new(0., 0., 1.), Vec2::new(8., 8.))),
        material: materials.add(StandardMaterial::default()),
        transform: Transform::from_xyz(0., 0., -6.),
        ..Default::default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 4.0, 6.0),
        ..default()
    });
}
