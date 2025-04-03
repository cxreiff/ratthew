use bevy::prelude::*;

use super::{
    direction::{Direction, GridDirection},
    GridAnimated, GridSystemSet,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(grid_position_setup_observer).add_systems(
        Update,
        grid_static_position_movement_system.in_set(GridSystemSet::Movement),
    );
}

#[derive(Component, Clone, Copy, Debug, Default, Deref, DerefMut)]
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

#[derive(Event, Default, Debug, Clone)]
pub struct GridPositionMove(pub GridPosition);

fn grid_position_setup_observer(
    trigger: Trigger<OnAdd, GridPosition>,
    mut commands: Commands,
    mut positioned: Query<(&GridPosition, &mut Transform)>,
) {
    let (position, mut transform) = positioned.get_mut(trigger.entity()).unwrap();
    transform.translation = position.into();

    commands
        .entity(trigger.entity())
        .observe(grid_movement_observer);
}

fn grid_movement_observer(
    trigger: Trigger<GridPositionMove>,
    mut grid_position: Query<&mut GridPosition>,
) {
    if let Ok(mut grid_position) = grid_position.get_mut(trigger.entity()) {
        *grid_position = trigger.0;
    }
}

fn grid_static_position_movement_system(
    mut grid_position_changed: Query<
        (&GridPosition, &mut Transform),
        (Changed<GridPosition>, Without<GridAnimated>),
    >,
) {
    for (position, mut transform) in &mut grid_position_changed {
        transform.translation = position.into();
    }
}
