use bevy::prelude::*;

use crate::{camera::PlayerCamera, levels::loading::Collider};

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
    walls: Query<&GridPosition, (With<Collider>, Without<PlayerCamera>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (entity, grid_position, grid_direction) = camera_in_grid.single_mut();

    for press in input.get_just_pressed() {
        match press {
            KeyCode::KeyW => {
                let new_position = grid_position.forward(grid_direction);
                validated_move(commands.reborrow(), entity, new_position, &walls)
            }
            KeyCode::KeyD => {
                let new_position = grid_position.right(grid_direction);
                validated_move(commands.reborrow(), entity, new_position, &walls)
            }
            KeyCode::KeyS => {
                let new_position = grid_position.back(grid_direction);
                validated_move(commands.reborrow(), entity, new_position, &walls)
            }
            KeyCode::KeyA => {
                let new_position = grid_position.left(grid_direction);
                validated_move(commands.reborrow(), entity, new_position, &walls)
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
    new_position: GridPosition,
    walls: &Query<&GridPosition, (With<Collider>, Without<PlayerCamera>)>,
) {
    if walls.iter().any(|wall| wall.eq(&new_position)) {
        commands
            .entity(entity)
            .trigger(GridMoveBlocked(new_position));
    } else {
        commands
            .entity(entity)
            .trigger(GridPositionMove(new_position));
    }
}
