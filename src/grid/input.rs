use bevy::prelude::*;

use crate::{camera::PlayerCamera, GameStates};

use super::{direction::GridDirection, position::GridPosition};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (handle_grid_movement_input_system,).run_if(in_state(GameStates::Playing)),
    );
}

pub fn handle_grid_movement_input_system(
    mut camera_in_grid: Query<(&mut GridPosition, &mut GridDirection), With<PlayerCamera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (mut grid_position, mut grid_direction) = camera_in_grid.single_mut();

    for press in input.get_just_pressed() {
        match press {
            KeyCode::KeyW => {
                **grid_position = grid_position.forward(&grid_direction);
            }
            KeyCode::KeyD => {
                **grid_position = grid_position.right(&grid_direction);
            }
            KeyCode::KeyS => {
                **grid_position = grid_position.back(&grid_direction);
            }
            KeyCode::KeyA => {
                **grid_position = grid_position.left(&grid_direction);
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
