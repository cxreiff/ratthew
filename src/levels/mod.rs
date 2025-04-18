use bevy::prelude::*;

mod flipped_ramp;
mod layer;
mod loading;
mod upright_billboard;
mod upright_cube;
mod upright_ramp;

pub use loading::{Collides, GameAssets, RampBlock};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(loading::plugin);
}
