use bevy::prelude::*;

use crate::{
    camera::PlayerCamera,
    levels::{Ramp, Wall},
};

use super::{
    animation::GridMoveBlocked,
    direction::{GridDirection, GridDirectionMove},
    position::{GridPosition, GridPositionMove},
    GridSystemSet,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_grid_movement_input_system.in_set(GridSystemSet::HandleInput),
    );
}

pub fn handle_grid_movement_input_system(
    mut commands: Commands,
    mut camera_in_grid: Query<(Entity, &GridPosition, &GridDirection), With<PlayerCamera>>,
    walls: Query<&GridPosition, With<Wall>>,
    ramps: Query<(&GridPosition, &GridDirection), With<Ramp>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (entity, grid_position, grid_direction) = camera_in_grid.single_mut();

    for press in input.get_just_pressed() {
        match press {
            KeyCode::KeyW => {
                let new_position = grid_position.forward(grid_direction);
                validated_move(
                    commands.reborrow(),
                    entity,
                    grid_position,
                    new_position,
                    &walls,
                    &ramps,
                )
            }
            KeyCode::KeyD => {
                let new_position = grid_position.right(grid_direction);
                validated_move(
                    commands.reborrow(),
                    entity,
                    grid_position,
                    new_position,
                    &walls,
                    &ramps,
                )
            }
            KeyCode::KeyS => {
                let new_position = grid_position.back(grid_direction);
                validated_move(
                    commands.reborrow(),
                    entity,
                    grid_position,
                    new_position,
                    &walls,
                    &ramps,
                )
            }
            KeyCode::KeyA => {
                let new_position = grid_position.left(grid_direction);
                validated_move(
                    commands.reborrow(),
                    entity,
                    grid_position,
                    new_position,
                    &walls,
                    &ramps,
                )
            }
            KeyCode::KeyQ => {
                commands
                    .entity(entity)
                    .trigger(GridDirectionMove(grid_direction.left()));
            }
            KeyCode::KeyE => {
                commands
                    .entity(entity)
                    .trigger(GridDirectionMove(grid_direction.right()));
            }
            _ => {}
        }
    }
}

fn validated_move(
    mut commands: Commands,
    entity: Entity,
    current_position: &GridPosition,
    mut next_position: GridPosition,
    walls: &Query<&GridPosition, With<Wall>>,
    ramps: &Query<(&GridPosition, &GridDirection), With<Ramp>>,
) {
    // TODO: Refactor ramp/wall handling to work based on "connected edges", for example:
    // 1. Determine based on the current position and whether there is a ramp here, the exact
    //    positions we are trying to move to (can be singular position if we rule out up-ramp into
    //    down-ramp case).
    // 2. Determine the type of edge between the current and target position (its left altitude and
    //    right altitude).
    // 3. Examine the space we are trying to move to to see if has the same edge, on the side
    //    facing the mover.

    if let Some(occupied_ramp) = find_ramp(current_position, ramps) {
        let uphill_move = is_uphill_move(&next_position, occupied_ramp);
        let downhill_move = is_downhill_move(&next_position, occupied_ramp);

        if uphill_move {
            next_position = next_position.up();
        } else if !downhill_move {
            let compatible_ramp = is_compatible_ramp(&next_position, occupied_ramp, ramps);

            if compatible_ramp {
                commands
                    .entity(entity)
                    .trigger(GridPositionMove(next_position));
                return;
            } else {
                commands
                    .entity(entity)
                    .trigger(GridMoveBlocked(next_position));
                return;
            }
        }
    }

    let blocked = walls.iter().any(|wall| wall.eq(&next_position));

    if blocked {
        commands
            .entity(entity)
            .trigger(GridMoveBlocked(next_position));
        return;
    }

    let no_floor = !walls.iter().any(|wall| wall.eq(&next_position.down()));

    if no_floor {
        let ramp_below = find_ramp(&next_position.down(), ramps);

        if let Some(ramp_below) = ramp_below {
            let ramp_aligned = is_ramp_top_aligned(current_position, ramp_below);

            if ramp_aligned {
                next_position = next_position.down();
                commands
                    .entity(entity)
                    .trigger(GridPositionMove(next_position));
                return;
            } else {
                commands
                    .entity(entity)
                    .trigger(GridMoveBlocked(next_position));
                return;
            }
        } else {
            commands
                .entity(entity)
                .trigger(GridMoveBlocked(next_position));
            return;
        }
    }

    if let Some(onto_ramp) = find_ramp(&next_position, ramps) {
        let ramp_aligned = is_ramp_bottom_aligned(current_position, onto_ramp);

        if !ramp_aligned {
            commands
                .entity(entity)
                .trigger(GridMoveBlocked(next_position));
            return;
        }
    }

    commands
        .entity(entity)
        .trigger(GridPositionMove(next_position));
}

fn find_ramp<'a>(
    position: &GridPosition,
    ramps: &'a Query<(&GridPosition, &GridDirection), With<Ramp>>,
) -> Option<(&'a GridPosition, &'a GridDirection)> {
    ramps
        .iter()
        .find(|(ramp_position, _)| position.eq(ramp_position))
}

fn is_uphill_move(
    next_position: &GridPosition,
    (ramp_position, ramp_direction): (&GridPosition, &GridDirection),
) -> bool {
    ramp_position.forward(ramp_direction).eq(next_position)
}

fn is_downhill_move(
    next_position: &GridPosition,
    (ramp_position, ramp_direction): (&GridPosition, &GridDirection),
) -> bool {
    ramp_position.back(ramp_direction).eq(next_position)
}

fn is_compatible_ramp(
    next_position: &GridPosition,
    (_, current_ramp_direction): (&GridPosition, &GridDirection),
    ramps: &Query<(&GridPosition, &GridDirection), With<Ramp>>,
) -> bool {
    if let Some((_, next_ramp_direction)) = find_ramp(next_position, ramps) {
        return next_ramp_direction.eq(current_ramp_direction);
    }

    false
}

fn is_ramp_bottom_aligned(
    current_position: &GridPosition,
    (next_ramp_position, next_ramp_direction): (&GridPosition, &GridDirection),
) -> bool {
    next_ramp_position
        .back(next_ramp_direction)
        .eq(current_position)
        || next_ramp_position
            .back(next_ramp_direction)
            .down()
            .eq(current_position)
}

fn is_ramp_top_aligned(
    current_position: &GridPosition,
    (next_ramp_position, next_ramp_direction): (&GridPosition, &GridDirection),
) -> bool {
    next_ramp_position
        .forward(next_ramp_direction)
        .up()
        .eq(current_position)
}
