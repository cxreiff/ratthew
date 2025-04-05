use bevy::prelude::*;

mod layer;
mod loading;
mod ramp;
mod upright_cube;

pub use loading::{GameAssets, Ramp, Wall};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(loading::plugin);
}
