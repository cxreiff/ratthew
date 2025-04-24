use bevy::prelude::*;

use crate::blocks::RampBlockMarker;

use super::{movement::GridCollides, GridDirection, GridPosition};

pub fn find_collider_position<'a>(
    position: &GridPosition,
    colliders: &'a Query<&GridPosition, With<GridCollides>>,
) -> Option<&'a GridPosition> {
    colliders
        .iter()
        .find(|wall_position| wall_position.eq(position))
}

pub fn find_ramp_position_direction<'a>(
    position: &GridPosition,
    ramps: &'a Query<(&GridPosition, &GridDirection), With<RampBlockMarker>>,
) -> Option<(&'a GridPosition, &'a GridDirection)> {
    ramps
        .iter()
        .find(|(ramp_position, _)| position.eq(ramp_position))
}
