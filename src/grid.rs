use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_tween::{
    interpolate::{rotation, translation},
    prelude::{AnimationBuilderExt, EaseKind},
    tween::{AnimationTarget, IntoTarget},
};

use crate::{camera::PlayerCamera, GameStates};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (move_toward_grid_position_system, move_in_grid_system)
            .run_if(in_state(GameStates::Playing)),
    );
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct GridPosition(pub IVec3);

impl GridPosition {
    const DIRECTION_VECTORS: [IVec3; 4] = [
        IVec3::new(0, 1, 0),
        IVec3::new(1, 0, 0),
        IVec3::new(0, -1, 0),
        IVec3::new(-1, 0, 0),
    ];

    fn direction_vector_offset(dir: &GridCardinalDirection) -> usize {
        match dir {
            GridCardinalDirection::North => 0,
            GridCardinalDirection::East => 1,
            GridCardinalDirection::South => 2,
            GridCardinalDirection::West => 3,
        }
    }

    fn forward(&self, dir: &GridCardinalDirection) -> IVec3 {
        let index = Self::direction_vector_offset(dir) % Self::DIRECTION_VECTORS.len();
        **self + Self::DIRECTION_VECTORS[index]
    }

    fn right(&self, dir: &GridCardinalDirection) -> IVec3 {
        let index = (1 + Self::direction_vector_offset(dir)) % Self::DIRECTION_VECTORS.len();
        **self + Self::DIRECTION_VECTORS[index]
    }

    fn back(&self, dir: &GridCardinalDirection) -> IVec3 {
        let index = (2 + Self::direction_vector_offset(dir)) % Self::DIRECTION_VECTORS.len();
        **self + Self::DIRECTION_VECTORS[index]
    }

    fn left(&self, dir: &GridCardinalDirection) -> IVec3 {
        let index = (3 + Self::direction_vector_offset(dir)) % Self::DIRECTION_VECTORS.len();
        **self + Self::DIRECTION_VECTORS[index]
    }
}

impl From<&GridPosition> for Vec3 {
    fn from(value: &GridPosition) -> Self {
        Vec3::new(value.x as f32, value.y as f32, value.z as f32)
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub enum GridCardinalDirection {
    North,
    East,
    South,
    West,
}

impl GridCardinalDirection {
    fn left(&self) -> GridCardinalDirection {
        match self {
            GridCardinalDirection::North => GridCardinalDirection::West,
            GridCardinalDirection::East => GridCardinalDirection::North,
            GridCardinalDirection::South => GridCardinalDirection::East,
            GridCardinalDirection::West => GridCardinalDirection::South,
        }
    }

    fn right(&self) -> GridCardinalDirection {
        match self {
            GridCardinalDirection::North => GridCardinalDirection::East,
            GridCardinalDirection::East => GridCardinalDirection::South,
            GridCardinalDirection::South => GridCardinalDirection::West,
            GridCardinalDirection::West => GridCardinalDirection::North,
        }
    }
}

impl From<&GridCardinalDirection> for Quat {
    fn from(value: &GridCardinalDirection) -> Self {
        match value {
            GridCardinalDirection::North => Quat::from_euler(EulerRot::XYZ, PI / 2., 0., 0.),
            GridCardinalDirection::East => {
                Quat::from_euler(EulerRot::XYZ, PI / 2., 3. * PI / 2., 0.)
            }
            GridCardinalDirection::South => Quat::from_euler(EulerRot::XYZ, PI / 2., PI, 0.),
            GridCardinalDirection::West => Quat::from_euler(EulerRot::XYZ, PI / 2., PI / 2., 0.),
        }
    }
}

fn move_toward_grid_position_system(
    mut commands: Commands,
    has_grid_pos: Query<(Entity, &GridPosition, &GridCardinalDirection, &Transform)>,
) {
    for (entity, grid_pos, grid_cardinal_dir, transform) in &has_grid_pos {
        let jeb = AnimationTarget.into_target();
        commands.entity(entity).animation().insert_tween_here(
            Duration::from_millis(750),
            EaseKind::ExponentialOut,
            (
                jeb.with(translation(transform.translation, grid_pos.into())),
                jeb.with(rotation(transform.rotation, grid_cardinal_dir.into())),
            ),
        );
    }
}

pub fn move_in_grid_system(
    mut camera_in_grid: Query<(&mut GridPosition, &mut GridCardinalDirection), With<PlayerCamera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (mut grid_pos, mut grid_cardinal_dir) = camera_in_grid.single_mut();

    for press in input.get_just_pressed() {
        match press {
            KeyCode::KeyW => {
                **grid_pos = grid_pos.forward(&grid_cardinal_dir);
            }
            KeyCode::KeyD => {
                **grid_pos = grid_pos.right(&grid_cardinal_dir);
            }
            KeyCode::KeyS => {
                **grid_pos = grid_pos.back(&grid_cardinal_dir);
            }
            KeyCode::KeyA => {
                **grid_pos = grid_pos.left(&grid_cardinal_dir);
            }
            KeyCode::KeyQ => {
                *grid_cardinal_dir = grid_cardinal_dir.left();
            }
            KeyCode::KeyE => {
                *grid_cardinal_dir = grid_cardinal_dir.right();
            }
            _ => {}
        }
    }
}
