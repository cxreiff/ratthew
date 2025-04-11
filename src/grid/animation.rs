use bevy::prelude::*;

use crate::{
    animation::{GridAnimated, GridMoveBlockedTween, GridMoveTween},
    levels::RampBlock,
};

use super::{
    utilities::find_ramp_position_direction, Direction, GridDirection, GridPosition, GridSystemSet,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(grid_movement_blocked_observer)
        .add_systems(
            PostUpdate,
            (
                grid_animated_movement_system.in_set(GridSystemSet::Movement),
                ramp_height_correction_system.in_set(GridSystemSet::Movement),
            )
                .chain(),
        );
}

#[derive(Event, Default, Debug, Clone)]
pub struct GridMoveBlocked(pub GridPosition);

#[derive(Component, Default, Debug, Clone)]
pub struct GridMoveParent;

#[derive(Component, Default, Debug, Clone)]
pub struct GridMoveBlockedParent;

fn grid_movement_blocked_observer(
    trigger: Trigger<GridMoveBlocked>,
    mut commands: Commands,
    grid_positions: Query<(&Transform, &GridDirection, &GridAnimated)>,
) {
    let (transform, grid_direction, grid_animated) = grid_positions.get(trigger.entity()).unwrap();

    commands.entity(trigger.entity()).remove::<GridMoveTween>();

    commands
        .entity(trigger.entity())
        .insert(GridMoveBlockedTween {
            start_position: grid_animated.previous_position,
            blocked_position: trigger.event().0,
            start_rotation: transform.rotation,
            end_rotation: grid_direction.into(),
        });
}

fn grid_animated_movement_system(
    mut commands: Commands,
    mut grid_position_changed: Query<
        (
            Entity,
            &GridPosition,
            &GridDirection,
            &Transform,
            &mut GridAnimated,
        ),
        (Or<(Changed<GridPosition>, Changed<GridDirection>)>,),
    >,
) {
    for (entity, &position, direction, transform, mut animated) in &mut grid_position_changed {
        let previous = animated.update_previous(position);

        commands.entity(entity).remove::<GridMoveBlockedTween>();

        commands.entity(entity).insert(GridMoveTween {
            start_position: previous,
            end_position: position,
            start_rotation: transform.rotation,
            end_rotation: direction.into(),
        });
    }
}

fn ramp_height_correction_system(
    mut positioned: Query<(&mut Transform, &GridAnimated)>,
    ramps: Query<(&GridPosition, &GridDirection), With<RampBlock>>,
) {
    for (mut transform, animated) in &mut positioned {
        let current_position = GridPosition::from(animated.buffer_transform);
        if let Some((ramp_position, ramp_direction)) =
            find_ramp_position_direction(&current_position, &ramps)
        {
            let base_height = Vec3::from(current_position).y;
            let ramp_diff_z = animated.buffer_transform.z - ramp_position.z as f32;
            let ramp_diff_x = animated.buffer_transform.x - ramp_position.x as f32;
            let ramp_height = match ramp_direction.0 {
                Direction::North => 0.5 - ramp_diff_z,
                Direction::East => 0.5 + ramp_diff_x,
                Direction::South => 0.5 + ramp_diff_z,
                Direction::West => 0.5 - ramp_diff_x,
            };

            transform.translation.y = base_height + ramp_height;
        } else {
            transform.translation.y = current_position.y as f32;
        }

        transform.translation.x = animated.buffer_transform.x;
        transform.translation.z = animated.buffer_transform.z;
    }
}
