use animation::sword_bob_system;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::input::common_conditions::input_just_pressed;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::WindowMode;
// use bevy::window::WindowResolution;
use bevy::winit::WinitPlugin;
use bevy_ratatui_camera::RatatuiCameraPlugin;
use bevy_tween::DefaultTweenPlugins;

mod animation;
mod camera;
mod grid;
mod levels;
mod particles;
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
                // resolution: WindowResolution::default().with_scale_factor_override(1.0),
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
        camera::plugin,
        particles::plugin,
        levels::plugin,
        grid::plugin,
        #[cfg(not(feature = "egui"))]
        terminal::plugin,
        #[cfg(feature = "egui")]
        egui::plugin,
    ))
    .insert_resource(Flags { debug: false })
    .insert_resource(ClearColor(Color::BLACK))
    .add_systems(
        Update,
        (
            sword_bob_system.run_if(in_state(GameStates::Playing)),
            debug_print_world_system
                .run_if(in_state(GameStates::Playing))
                .run_if(input_just_pressed(KeyCode::KeyP)),
        ),
    );
}

fn debug_print_world_system(world: &World) {
    let num = world.entities().len();
    log::info!("{num}");
}
