use bevy::prelude::*;

mod flipped_ramp;
mod layer;
mod loading;
mod torch_effect;
mod upright_billboard;
mod upright_cube;
mod upright_ramp;

pub use loading::{Collides, LevelAssets, RampBlock};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((loading::plugin, torch_effect::plugin));
}
