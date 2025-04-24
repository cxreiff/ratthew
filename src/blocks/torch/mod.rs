use bevy::prelude::*;

mod block;
mod particles;

pub use block::TorchBlock;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((block::plugin, particles::plugin));
}
