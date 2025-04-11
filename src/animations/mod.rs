use animation_trait::AutoTween;
use bevy::prelude::*;

mod animation_trait;
mod grid_animated;
mod grid_blocked;
mod grid_move;
mod item_bob;
mod tween_cleanup;

pub use grid_animated::{grid_animated_transform, GridAnimated};
pub use grid_blocked::GridMoveBlockedAnimation;
pub use grid_move::GridMoveAnimation;
pub use item_bob::ItemBobAnimation;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        grid_animated::plugin,
        item_bob::ItemBobAnimation::autotween_plugin,
        grid_move::GridMoveAnimation::autotween_plugin,
        grid_blocked::GridMoveBlockedAnimation::autotween_plugin,
        tween_cleanup::plugin,
    ));
}
