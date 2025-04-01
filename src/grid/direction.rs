use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, Default)]
pub enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn right(&self) -> Direction {
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
