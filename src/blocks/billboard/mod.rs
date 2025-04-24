use bevy::prelude::*;

mod block;
mod mesh;

pub use block::BillboardBlock;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(block::plugin);
}
