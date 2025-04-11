use std::time::Duration;

use bevy::prelude::*;
use bevy_tween::{
    combinator::{sequence, tween},
    interpolate::rotation,
    prelude::EaseKind,
};

use crate::grid::GridPosition;

use super::{animation_trait::AutoTween, grid_animated_transform};

#[derive(Component, Clone, Debug, Default)]
pub struct GridMoveBlockedTween {
    pub start_position: GridPosition,
    pub blocked_position: GridPosition,
    pub start_rotation: Quat,
    pub end_rotation: Quat,
}

#[derive(Component, Clone, Debug, Default)]
pub struct GridMoveBlockedTweenHolder;

impl AutoTween for GridMoveBlockedTween {
    type Holder = GridMoveBlockedTweenHolder;

    fn insert_tween(
        &self,
        animation: bevy_tween::combinator::AnimationBuilder,
        target: bevy_tween::tween::TargetComponent,
    ) {
        let start_translation = Vec3::from(self.start_position);
        let blocked_translation = Vec3::from(self.blocked_position);
        let distance = start_translation.distance(blocked_translation);
        let bump_position = start_translation.move_towards(blocked_translation, distance / 3.);

        animation.insert(sequence((
            tween(
                Duration::from_millis(222),
                EaseKind::ExponentialOut,
                (
                    target.with(grid_animated_transform(
                        self.start_position.into(),
                        bump_position,
                    )),
                    target.with(rotation(self.start_rotation, self.end_rotation)),
                ),
            ),
            tween(
                Duration::from_millis(222),
                EaseKind::ExponentialOut,
                target.with(grid_animated_transform(bump_position, start_translation)),
            ),
        )));
    }
}
