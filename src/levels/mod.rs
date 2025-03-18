use bevy::prelude::*;

mod collisions;
mod cube;
pub mod loading;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((loading::plugin, collisions::plugin));
}
