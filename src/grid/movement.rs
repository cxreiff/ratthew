use bevy::prelude::*;

use crate::{animation::GridAnimated, blocks::RampBlockMarker};

use super::{
    animation::GridMoveBlocked,
    utilities::{find_collider_position, find_ramp_position_direction},
    GridDirection, GridPosition, GridSystemSet,
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<GridPositionMoveAttempt>()
        .add_observer(grid_position_setup_observer)
        .add_observer(grid_direction_setup_observer)
        .add_systems(
            Update,
            (
                grid_static_position_movement_system,
                grid_static_direction_movement_system,
            )
                .in_set(GridSystemSet::Movement),
        );
}

#[derive(Event, Debug, Clone)]
pub struct GridPositionMoveAttempt(pub GridDirection);

#[derive(Event, Default, Debug, Clone)]
pub struct GridPositionMove(pub GridPosition);

#[derive(Event, Default, Debug, Clone)]
pub struct GridDirectionMove(pub GridDirection);

#[derive(Component, Debug, Clone)]
pub struct GridCollides;

fn grid_position_setup_observer(
    trigger: Trigger<OnAdd, GridPosition>,
    mut commands: Commands,
    mut positioned: Query<(&GridPosition, &mut Transform)>,
) {
    let (position, mut transform) = positioned.get_mut(trigger.entity()).unwrap();
    transform.translation = position.into();

    commands
        .entity(trigger.entity())
        .observe(grid_movement_attempt_observer)
        .observe(grid_movement_observer);
}

fn grid_movement_observer(
    trigger: Trigger<GridPositionMove>,
    mut grid_position: Query<&mut GridPosition>,
) {
    if let Ok(mut grid_position) = grid_position.get_mut(trigger.entity()) {
        *grid_position = trigger.0;
    }
}

fn grid_movement_attempt_observer(
    trigger: Trigger<GridPositionMoveAttempt>,
    mut commands: Commands,
    grid_positions: Query<&GridPosition>,
    colliders: Query<&GridPosition, With<GridCollides>>,
    ramps: Query<(&GridPosition, &GridDirection), With<RampBlockMarker>>,
) {
    if let Ok(mover_position) = grid_positions.get(trigger.entity()) {
        let mut entity = commands.entity(trigger.entity());

        let ramp_direction = find_ramp_position_direction(mover_position, &ramps)
            .map(|(_, ramp_direction)| *ramp_direction);

        let source_edge_heights = mover_position.edge_heights(trigger.0, ramp_direction);

        for destination in [
            mover_position.forward(&trigger.0).up(),
            mover_position.forward(&trigger.0),
            mover_position.forward(&trigger.0).down(),
        ] {
            let ramp_direction = find_ramp_position_direction(&destination, &ramps)
                .map(|(_, ramp_direction)| *ramp_direction);

            let has_ramp = ramp_direction.is_some();
            let has_collider = find_collider_position(&destination, &colliders).is_some();
            let has_collider_above =
                find_collider_position(&destination.up(), &colliders).is_some();
            let has_collider_below =
                find_collider_position(&destination.down(), &colliders).is_some();

            let destination_edge_heights =
                destination.edge_heights(trigger.0.reverse(), ramp_direction);

            let edge_matches =
                source_edge_heights == (destination_edge_heights.1, destination_edge_heights.0);

            if edge_matches
                && has_collider_below
                && !has_collider
                && !(has_ramp && has_collider_above)
            {
                entity.trigger(GridPositionMove(destination));
                return;
            }
        }

        if ramp_direction.is_some_and(|ramp_direction| ramp_direction.eq(&trigger.0)) {
            entity.trigger(GridMoveBlocked(mover_position.forward(&trigger.0).up()));
        } else {
            entity.trigger(GridMoveBlocked(mover_position.forward(&trigger.0)));
        }
    }
}

fn grid_static_position_movement_system(
    mut grid_position_changed: Query<
        (&GridPosition, &mut Transform),
        (Changed<GridPosition>, Without<GridAnimated>),
    >,
) {
    for (position, mut transform) in &mut grid_position_changed {
        transform.translation = position.into();
    }
}

fn grid_direction_setup_observer(
    trigger: Trigger<OnAdd, GridDirection>,
    mut commands: Commands,
    mut directed: Query<(&GridDirection, &mut Transform)>,
) {
    let (direction, mut transform) = directed.get_mut(trigger.entity()).unwrap();
    transform.rotation = direction.into();

    commands
        .entity(trigger.entity())
        .observe(grid_direction_observer);
}

fn grid_direction_observer(
    trigger: Trigger<GridDirectionMove>,
    mut grid_position: Query<&mut GridDirection>,
) {
    if let Ok(mut grid_position) = grid_position.get_mut(trigger.entity()) {
        *grid_position = trigger.0;
    }
}

fn grid_static_direction_movement_system(
    mut grid_position_changed: Query<
        (&GridDirection, &mut Transform),
        (Changed<GridDirection>, Without<GridAnimated>),
    >,
) {
    for (direction, mut transform) in &mut grid_position_changed {
        transform.rotation = direction.into();
    }
}
