use bevy::prelude::*;
use bevy_tween::{
    component_tween_system, prelude::Interpolator, tween::AnimationTarget, BevyTweenRegisterSystems,
};

use crate::grid::GridPosition;

pub fn plugin(app: &mut App) {
    app.add_tween_systems(component_tween_system::<InterpolateGridAnimated>());
}

#[derive(Component, Debug, Default)]
#[require(AnimationTarget(|| AnimationTarget))]
pub struct GridAnimated {
    pub buffer_transform: Vec3,
    pub previous_position: GridPosition,
}

pub struct InterpolateGridAnimated {
    start: Vec3,
    end: Vec3,
}

impl Interpolator for InterpolateGridAnimated {
    type Item = GridAnimated;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.buffer_transform = self.start.lerp(self.end, value);
    }
}

pub fn grid_animated_transform(start: Vec3, end: Vec3) -> InterpolateGridAnimated {
    InterpolateGridAnimated { start, end }
}
