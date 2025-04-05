use bevy::prelude::*;

mod cube;
mod layer;
pub mod loading;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(loading::plugin);
}
