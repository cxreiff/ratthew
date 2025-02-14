use bevy::prelude::*;

use crate::camera::Sword;

pub fn sword_bob_system(time: Res<Time>, mut query_sword: Query<&mut Transform, With<Sword>>) {
    let mut sword_transform = query_sword.single_mut();
    sword_transform.translation.y = -0.15 + 0.01 * (time.elapsed_secs() * 2.).sin();
}
