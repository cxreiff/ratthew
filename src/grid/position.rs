use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::direction::{Direction, GridDirection};

#[derive(Component, Deref, DerefMut, Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub struct GridPosition(pub IVec3);

impl GridPosition {
    const DIRECTION_VECTORS: [IVec3; 4] = [
        IVec3::new(0, 0, -1),
        IVec3::new(1, 0, 0),
        IVec3::new(0, 0, 1),
        IVec3::new(-1, 0, 0),
    ];

    fn direction_vector_offset(direction: &GridDirection) -> usize {
        match **direction {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }

    pub fn forward(&self, dir: &GridDirection) -> GridPosition {
        let index = Self::direction_vector_offset(dir) % Self::DIRECTION_VECTORS.len();
        Self(**self + Self::DIRECTION_VECTORS[index])
    }

    pub fn right(&self, dir: &GridDirection) -> GridPosition {
        let index = (1 + Self::direction_vector_offset(dir)) % Self::DIRECTION_VECTORS.len();
        Self(**self + Self::DIRECTION_VECTORS[index])
    }

    pub fn back(&self, dir: &GridDirection) -> GridPosition {
        let index = (2 + Self::direction_vector_offset(dir)) % Self::DIRECTION_VECTORS.len();
        Self(**self + Self::DIRECTION_VECTORS[index])
    }

    pub fn left(&self, dir: &GridDirection) -> GridPosition {
        let index = (3 + Self::direction_vector_offset(dir)) % Self::DIRECTION_VECTORS.len();
        Self(**self + Self::DIRECTION_VECTORS[index])
    }

    pub fn up(&self) -> GridPosition {
        Self(**self + IVec3::new(0, 1, 0))
    }

    pub fn down(&self) -> GridPosition {
        Self(**self + IVec3::new(0, -1, 0))
    }

    pub fn edge_heights(
        &self,
        edge_direction: GridDirection,
        ramp_direction: Option<GridDirection>,
    ) -> (i32, i32) {
        if let Some(ramp_direction) = ramp_direction {
            let edge_direction_index = Self::direction_vector_offset(&edge_direction) as i32;
            let ramp_direction_index = Self::direction_vector_offset(&ramp_direction) as i32;

            let offset = match (edge_direction_index - ramp_direction_index).rem_euclid(4) {
                0 => (1, 1),
                3 => (0, 1),
                2 => (0, 0),
                1 => (1, 0),
                _ => unreachable!(),
            };

            (self.y + offset.0, self.y + offset.1)
        } else {
            (self.y, self.y)
        }
    }
}

impl From<GridPosition> for Vec3 {
    fn from(value: GridPosition) -> Self {
        Vec3::new(value.x as f32, value.y as f32, value.z as f32)
    }
}

impl From<&GridPosition> for Vec3 {
    fn from(value: &GridPosition) -> Self {
        Vec3::from(*value)
    }
}

impl From<Vec3> for GridPosition {
    fn from(value: Vec3) -> Self {
        Self(IVec3::new(
            value.x.round() as i32,
            value.y.round() as i32,
            value.z.round() as i32,
        ))
    }
}
