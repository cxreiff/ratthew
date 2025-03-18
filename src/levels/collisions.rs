use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

use crate::{camera::PlayerCamera, GameStates};

use super::loading::Collider;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        collisions_system.run_if(in_state(GameStates::Playing)),
    );
}

enum Collision {
    North,
    East,
    South,
    West,
}

pub fn collisions_system(
    mut query_player: Query<&mut Transform, With<PlayerCamera>>,
    query_colliders: Query<&Transform, (With<Collider>, Without<PlayerCamera>)>,
) {
    let mut player_transform = query_player.single_mut();

    let player_bounds = BoundingCircle::new(player_transform.translation.xy(), 0.4);

    for collider_transform in &query_colliders {
        let collider_bounds = Aabb2d::new(collider_transform.translation.xy(), Vec2::new(0.5, 0.5));

        if let Some(collision) = collision_check(&player_bounds, &collider_bounds) {
            match collision {
                Collision::North => {
                    player_transform.translation.y = player_transform
                        .translation
                        .y
                        .max(collider_bounds.max.y + 0.4)
                }
                Collision::East => player_transform.translation.x = collider_bounds.max.x + 0.4,
                Collision::South => {
                    player_transform.translation.y = player_transform
                        .translation
                        .y
                        .min(collider_bounds.min.y - 0.4)
                }
                Collision::West => player_transform.translation.x = collider_bounds.min.x - 0.4,
            }
        }
    }
}

fn collision_check(player_bounds: &BoundingCircle, collider_bounds: &Aabb2d) -> Option<Collision> {
    if !player_bounds.intersects(collider_bounds) {
        return None;
    }

    let closest = collider_bounds.closest_point(player_bounds.center);
    let offset = player_bounds.center - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::West
        } else {
            Collision::East
        }
    } else if offset.y > 0. {
        Collision::North
    } else {
        Collision::South
    };

    Some(side)
}
