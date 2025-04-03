use bevy::prelude::*;

mod animation;
mod direction;
mod input;
mod position;

pub use animation::GridAnimated;
pub use direction::{Direction, GridDirection};
pub use position::GridPosition;

use crate::GameStates;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum GridSystemSet {
    HandleInput,
    Movement,
    Cleanup,
}

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            GridSystemSet::HandleInput,
            GridSystemSet::Movement,
            GridSystemSet::Cleanup,
        )
            .chain()
            .run_if(in_state(GameStates::Playing)),
    )
    .add_plugins((
        animation::plugin,
        direction::plugin,
        input::plugin,
        position::plugin,
    ));
}
