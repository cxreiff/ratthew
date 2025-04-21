use bevy::prelude::*;

use crate::camera::{PersistClearEvent, PersistEvent, PlayerCamera};

use super::{
    direction::GridDirection,
    movement::{GridDirectionMove, GridPositionMoveAttempt},
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
    mut camera_in_grid: Query<(Entity, &GridDirection), With<PlayerCamera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (entity, grid_direction) = camera_in_grid.single_mut();
    let mut entity = commands.entity(entity);

    for press in input.get_just_pressed() {
        match press {
            KeyCode::KeyW => {
                entity.trigger(GridPositionMoveAttempt(*grid_direction));
            }
            KeyCode::KeyD => {
                entity.trigger(GridPositionMoveAttempt(grid_direction.right()));
            }
            KeyCode::KeyS => {
                entity.trigger(GridPositionMoveAttempt(grid_direction.reverse()));
            }
            KeyCode::KeyA => {
                entity.trigger(GridPositionMoveAttempt(grid_direction.left()));
            }
            KeyCode::KeyQ => {
                entity.trigger(GridDirectionMove(grid_direction.left()));
            }
            KeyCode::KeyE => {
                entity.trigger(GridDirectionMove(grid_direction.right()));
            }
            KeyCode::KeyP => {
                entity.trigger(PersistEvent);
            }
            KeyCode::KeyO => {
                entity.trigger(PersistClearEvent);
            }
            _ => {}
        }
    }
}
