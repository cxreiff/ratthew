use bevy::prelude::*;
use bevy_tween::{
    component_tween_system, prelude::Interpolator, tween::AnimationTarget, BevyTweenRegisterSystems,
};

use crate::grid::GridPosition;

pub fn plugin(app: &mut App) {
    app.add_observer(grid_animated_setup_observer)
        .add_tween_systems(component_tween_system::<InterpolateGridAnimated>());
}

#[derive(Component, Debug, Default)]
#[require(AnimationTarget(|| AnimationTarget))]
pub struct GridAnimated {
    pub buffer_transform: Vec3,
    pub previous_position: GridPosition,
}

impl GridAnimated {
    pub fn update_previous(&mut self, position: GridPosition) -> GridPosition {
        let previous = self.previous_position;
        self.previous_position = position;

        previous
    }
}

fn grid_animated_setup_observer(
    trigger: Trigger<OnInsert, GridAnimated>,
    mut grid_animated: Query<(&GridPosition, &mut GridAnimated)>,
) {
    let (grid_position, mut grid_animated) = grid_animated.get_mut(trigger.entity()).unwrap();
    grid_animated.buffer_transform = grid_position.into();
    grid_animated.previous_position = *grid_position;
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
