use animation::sword_bob_system;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
// use bevy::window::WindowResolution;
use bevy::winit::WinitPlugin;
use bevy_ratatui_camera::RatatuiCameraPlugin;
use bevy_tween::DefaultTweenPlugins;
use camera::ViewCameraPlugin;

mod animation;
mod camera;
mod grid;
mod levels;
mod particles;
mod widgets;

#[cfg(not(feature = "windowed"))]
mod terminal;

#[cfg(feature = "windowed")]
mod windowed;

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

    if cfg!(not(feature = "windowed")) {
        default_plugins = default_plugins
            .disable::<LogPlugin>()
            .disable::<WinitPlugin>();
    }

    if cfg!(feature = "windowed") {
        default_plugins = default_plugins.set(WindowPlugin {
            primary_window: Some(Window {
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
        ViewCameraPlugin,
        particles::plugin,
        levels::plugin,
        grid::plugin,
        #[cfg(not(feature = "windowed"))]
        terminal::plugin,
        #[cfg(feature = "windowed")]
        windowed::plugin,
    ))
    .insert_resource(Flags { debug: false })
    .insert_resource(ClearColor(Color::BLACK))
    .add_systems(
        Update,
        sword_bob_system.run_if(in_state(GameStates::Playing)),
    );
}
