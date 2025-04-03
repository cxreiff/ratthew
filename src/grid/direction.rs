use std::f32::consts::PI;

use bevy::prelude::*;

use super::{GridAnimated, GridSystemSet};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(grid_direction_setup_observer).add_systems(
        Update,
        grid_static_direction_movement_system.in_set(GridSystemSet::Movement),
    );
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Default, Deref, DerefMut)]
pub struct GridDirection(pub Direction);

impl From<GridDirection> for Quat {
    fn from(value: GridDirection) -> Self {
        match *value {
            Direction::North => Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
            Direction::East => Quat::from_euler(EulerRot::XYZ, 0., 3. * PI / 2., 0.),
            Direction::South => Quat::from_euler(EulerRot::XYZ, 0., PI, 0.),
            Direction::West => Quat::from_euler(EulerRot::XYZ, 0., PI / 2., 0.),
        }
    }
}

impl From<&GridDirection> for Quat {
    fn from(value: &GridDirection) -> Self {
        Quat::from(*value)
    }
}

impl GridDirection {
    pub fn left(&self) -> Self {
        Self(self.0.left())
    }

    pub fn right(&self) -> Self {
        Self(self.0.right())
    }
}

#[derive(Event, Default, Debug, Clone)]
pub struct GridDirectionMove(pub GridDirection);

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
