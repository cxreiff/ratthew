use bevy::prelude::*;

mod animation;
mod direction;
mod input;
mod movement;
mod position;
mod utilities;

pub use animation::GridAnimated;
use bevy_tween::TweenSystemSet;
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
