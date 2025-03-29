use bevy::prelude::*;

mod direction;
mod input;
mod movement;
mod position;

pub use position::GridPosition;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((input::plugin, movement::plugin));
}
