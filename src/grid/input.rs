use bevy::prelude::*;

use crate::{camera::PlayerCamera, levels::loading::Collider, GameStates};

use super::{direction::GridDirection, movement::GridBlockedMove, position::GridPosition};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (handle_grid_movement_input_system,).run_if(in_state(GameStates::Playing)),
    );
}

pub fn handle_grid_movement_input_system(
    mut commands: Commands,
    mut camera_in_grid: Query<(Entity, &mut GridPosition, &mut GridDirection), With<PlayerCamera>>,
    walls: Query<&GridPosition, (With<Collider>, Without<PlayerCamera>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (entity, mut grid_position, mut grid_direction) = camera_in_grid.single_mut();

    for press in input.get_just_pressed() {
        match press {
            KeyCode::KeyW => {
                let new_position = grid_position.forward(&grid_direction);
                validated_move(
                    commands.reborrow(),
                    entity,
                    &mut grid_position,
                    new_position,
                    &walls,
                )
            }
            KeyCode::KeyD => {
                let new_position = grid_position.right(&grid_direction);
                validated_move(
                    commands.reborrow(),
                    entity,
                    &mut grid_position,
                    new_position,
                    &walls,
                )
            }
            KeyCode::KeyS => {
                let new_position = grid_position.back(&grid_direction);
                validated_move(
                    commands.reborrow(),
                    entity,
                    &mut grid_position,
                    new_position,
                    &walls,
                )
            }
            KeyCode::KeyA => {
                let new_position = grid_position.left(&grid_direction);
                validated_move(
                    commands.reborrow(),
                    entity,
                    &mut grid_position,
                    new_position,
                    &walls,
                )
            }
            KeyCode::KeyQ => {
                **grid_direction = grid_direction.left();
            }
            KeyCode::KeyE => {
                **grid_direction = grid_direction.right();
            }
            _ => {}
        }
    }
}

fn validated_move(
    mut commands: Commands,
    entity: Entity,
    position: &mut GridPosition,
    new_position: GridPosition,
    walls: &Query<&GridPosition, (With<Collider>, Without<PlayerCamera>)>,
) {
    if walls.iter().any(|wall| wall.eq(&new_position)) {
        commands
            .entity(entity)
            .trigger(GridBlockedMove(new_position));
    } else {
        *position = new_position;
    }
}
