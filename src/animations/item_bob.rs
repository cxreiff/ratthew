use std::time::Duration;

use bevy::prelude::*;
use bevy_tween::{
    interpolate::translation,
    prelude::{EaseKind, Repeat, RepeatStyle},
};

use super::animation_trait::AutoTween;

#[derive(Component, Clone, Debug, Default)]
pub struct ItemBobAnimation;

#[derive(Component, Clone, Debug, Default)]
pub struct ItemBobAnimationHolder;

impl AutoTween for ItemBobAnimation {
    type Holder = ItemBobAnimationHolder;

    fn insert_tween(
        &self,
        animation: bevy_tween::combinator::AnimationBuilder,
        target: bevy_tween::tween::TargetComponent,
    ) {
        animation
            .repeat(Repeat::Infinitely)
            .repeat_style(RepeatStyle::PingPong)
            .insert_tween_here(
                Duration::from_millis(2000),
                EaseKind::SineInOut,
                target.with(translation(
                    Vec3::new(0.3, -0.15, -0.7),
                    Vec3::new(0.3, -0.14, -0.7),
                )),
            );
    }
}
