use bevy::prelude::*;
use bevy_tween::prelude::*;
use interpolate::{rotation, translation};

use crate::GameStates;

use super::{direction::GridDirection, GridPosition};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        grid_movement_system.run_if(in_state(GameStates::Playing)),
    )
    .add_observer(grid_position_setup_observer)
    .add_observer(grid_direction_setup_observer);
}

pub fn grid_position_setup_observer(
    trigger: Trigger<OnAdd, GridPosition>,
    mut positioned: Query<(&GridPosition, &mut Transform)>,
) {
    let (position, mut transform) = positioned.get_mut(trigger.entity()).unwrap();
    transform.translation = position.into();
}

pub fn grid_direction_setup_observer(
    trigger: Trigger<OnAdd, GridDirection>,
    mut directed: Query<(&GridDirection, &mut Transform)>,
) {
    let (direction, mut transform) = directed.get_mut(trigger.entity()).unwrap();
    transform.rotation = direction.into();
}

pub fn grid_movement_system(
    mut commands: Commands,
    mut grid_position_changed: Query<
        (Entity, &GridPosition, &GridDirection, &mut Transform),
        Or<(Changed<GridPosition>, Changed<GridDirection>)>,
    >,
) {
    for (entity, position, direction, transform) in &mut grid_position_changed {
        let target = entity.into_target();
        commands.entity(entity).animation().insert_tween_here(
            Duration::from_millis(500),
            EaseKind::ExponentialOut,
            (
                target.with(translation(transform.translation, position.into())),
                target.with(rotation(transform.rotation, direction.into())),
            ),
        );
    }
}
