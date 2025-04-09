use bevy::prelude::*;

mod layer;
mod loading;
mod upright_billboard;
mod upright_cube;
mod upright_ramp;

pub use loading::{GameAssets, RampBlock, WallBlock};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(loading::plugin);
}
