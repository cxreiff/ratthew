use bevy::prelude::*;

use super::direction::{Direction, GridDirection};

#[derive(Component, Clone, Copy, Debug, Default, Deref, DerefMut)]
pub struct GridPosition(pub IVec3);

impl GridPosition {
    const DIRECTION_VECTORS: [IVec3; 4] = [
        IVec3::new(0, 1, 0),
        IVec3::new(1, 0, 0),
        IVec3::new(0, -1, 0),
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
