use animation_trait::AutoTween;
use bevy::prelude::*;

mod animation_trait;
mod grid_animated;
mod grid_blocked_tween;
mod grid_move_tween;
mod item_bob_tween;

pub use grid_animated::{grid_animated_transform, GridAnimated};
pub use grid_blocked_tween::GridMoveBlockedTween;
pub use grid_move_tween::GridMoveTween;
pub use item_bob_tween::ItemBobTween;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        grid_animated::plugin,
        item_bob_tween::ItemBobTween::autotween_plugin,
        grid_move_tween::GridMoveTween::autotween_plugin,
        grid_blocked_tween::GridMoveBlockedTween::autotween_plugin,
    ));
}
