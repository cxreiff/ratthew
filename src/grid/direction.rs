use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::LdtkFields, EntityInstance};
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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

    pub fn reverse(&self) -> Self {
        self.right().right()
    }
}

impl From<&EntityInstance> for Direction {
    fn from(value: &EntityInstance) -> Self {
        value
            .get_enum_field("direction")
            .map(|dir| match dir.as_str() {
                "north" => Direction::North,
                "east" => Direction::East,
                "south" => Direction::South,
                "west" => Direction::West,
                _ => unreachable!(),
            })
            .unwrap_or(Direction::North)
    }
}

#[derive(
    Component, Clone, Copy, Debug, Default, PartialEq, Eq, Deref, DerefMut, Serialize, Deserialize,
)]
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

    pub fn reverse(&self) -> Self {
        Self(self.0.reverse())
    }
}
