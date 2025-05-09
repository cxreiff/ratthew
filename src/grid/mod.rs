use bevy::prelude::*;

mod animation;
mod direction;
mod input;
mod movement;
mod position;
mod utilities;

use bevy_tween::TweenSystemSet;
pub use direction::{Direction, GridDirection};
pub use movement::{GridCollides, GridDirectionMove, GridPositionMoveAttempt};
pub use position::GridPosition;

use crate::GameStates;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GridSystemSet {
    HandleInput,
    Movement,
    Cleanup,
}

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (GridSystemSet::HandleInput, GridSystemSet::Movement)
            .chain()
            .run_if(in_state(GameStates::Playing)),
    )
    .configure_sets(
        PostUpdate,
        GridSystemSet::Cleanup
            .after(TweenSystemSet::ApplyTween)
            .run_if(in_state(GameStates::Playing)),
    )
    .add_plugins((animation::plugin, input::plugin, movement::plugin));
}
