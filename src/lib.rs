use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::winit::WinitPlugin;
use bevy_rand::plugin::EntropyPlugin;
use bevy_rand::prelude::WyRand;
use bevy_ratatui_camera::RatatuiCameraPlugin;
use bevy_tween::DefaultTweenPlugins;

mod animation;
mod blocks;
mod camera;
mod config;
mod grid;
mod loading;
mod sound;
mod widgets;

#[cfg(not(feature = "egui"))]
mod terminal;

#[cfg(feature = "egui")]
mod egui;

#[derive(Component)]
pub struct Cube;

#[derive(Resource, Default)]
pub struct Flags {
    debug: bool,
    sound: bool,
}

#[derive(Default, States, Clone, Debug, Hash, Eq, PartialEq)]
pub enum GameStates {
    #[default]
    Loading,
    Playing,
}

pub fn plugin(app: &mut App) {
    let mut default_plugins = DefaultPlugins.set(ImagePlugin::default_nearest());

    if cfg!(not(feature = "egui")) {
        default_plugins = default_plugins
            .disable::<LogPlugin>()
            .disable::<WinitPlugin>();
    }

    if cfg!(feature = "egui") {
        default_plugins = default_plugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        });
    }

    app.add_plugins((
        default_plugins,
        DefaultTweenPlugins,
        RatatuiCameraPlugin,
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        EntropyPlugin::<WyRand>::default(),
        animation::plugin,
        blocks::plugin,
        camera::plugin,
        grid::plugin,
        loading::plugin,
        sound::plugin,
        #[cfg(not(feature = "egui"))]
        terminal::plugin,
        #[cfg(feature = "egui")]
        egui::plugin,
    ))
    .init_state::<GameStates>()
    .insert_resource(Flags {
        debug: false,
        sound: false,
    })
    .insert_resource(ClearColor(Color::BLACK))
    .add_systems(Update, global_input_system);
}

pub fn global_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    mut flags: ResMut<Flags>,
) {
    for press in input.get_just_pressed() {
        match press {
            KeyCode::Escape => {
                exit.send_default();
            }
            KeyCode::Tab => {
                flags.debug = !flags.debug;
            }
            KeyCode::KeyM => {
                flags.sound = !flags.sound;
            }
            _ => {}
        }
    }
}
