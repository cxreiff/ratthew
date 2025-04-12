use std::time::Duration;

use bevy::prelude::*;
use bevy_tween::{
    interpolate::translation,
    prelude::{EaseKind, Repeat, RepeatStyle},
};

use super::auto_tween_trait::AutoTween;

#[derive(Component, Clone, Debug, Default)]
pub struct ItemBobTween;

#[derive(Component, Clone, Debug, Default)]
pub struct ItemBobTweenHolder;

impl AutoTween for ItemBobTween {
    type Holder = ItemBobTweenHolder;

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
