use std::time::Duration;

use bevy::prelude::*;
use bevy_tween::{interpolate::rotation, prelude::EaseKind};

use crate::{animation::grid_animated::grid_animated_transform, grid::GridPosition};

use super::animation_trait::AutoTween;

#[derive(Component, Clone, Debug, Default)]
pub struct GridMoveTween {
    pub start_position: GridPosition,
    pub end_position: GridPosition,
    pub start_rotation: Quat,
    pub end_rotation: Quat,
}

#[derive(Component, Clone, Debug, Default)]
pub struct GridMoveTweenHolder;

impl AutoTween for GridMoveTween {
    type Holder = GridMoveTweenHolder;

    fn insert_tween(
        &self,
        animation: bevy_tween::combinator::AnimationBuilder,
        target: bevy_tween::tween::TargetComponent,
    ) {
        animation.insert_tween_here(
            Duration::from_millis(666),
            EaseKind::ExponentialOut,
            (
                target.with(grid_animated_transform(
                    self.start_position.into(),
                    self.end_position.into(),
                )),
                target.with(rotation(self.start_rotation, self.end_rotation)),
            ),
        );
    }
}
