use crate::levels::{RampBlock, WallBlock};
use bevy::prelude::*;

use super::{GridDirection, GridPosition};

pub fn find_wall_position<'a>(
    position: &GridPosition,
    walls: &'a Query<&GridPosition, With<WallBlock>>,
) -> Option<&'a GridPosition> {
    walls
        .iter()
        .find(|wall_position| wall_position.eq(position))
}

pub fn find_ramp_position_direction<'a>(
    position: &GridPosition,
    ramps: &'a Query<(&GridPosition, &GridDirection), With<RampBlock>>,
) -> Option<(&'a GridPosition, &'a GridDirection)> {
    ramps
        .iter()
        .find(|(ramp_position, _)| position.eq(ramp_position))
}
