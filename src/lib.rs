use animation::sword_bob_system;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use camera::ViewCameraPlugin;

mod animation;
mod camera;
mod levels;
mod logging;
mod particles;
mod terminal;
mod widgets;
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
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .disable::<LogPlugin>(),
        FrameTimeDiagnosticsPlugin,
        ViewCameraPlugin,
        terminal::plugin,
        particles::plugin,
        logging::plugin,
        levels::plugin,
        // windowed::plugin,
    ))
    .insert_resource(Flags { debug: false })
    .insert_resource(ClearColor(Color::BLACK))
    .add_systems(
        Update,
        sword_bob_system.run_if(in_state(GameStates::Playing)),
    );
}
