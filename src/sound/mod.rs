use bevy::prelude::*;

mod player;

pub use player::SfxAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(player::plugin);
}
