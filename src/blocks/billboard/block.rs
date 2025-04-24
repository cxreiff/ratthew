use std::ops::Deref;

use crate::{
    blocks::{block_layer::BlockSource, block_traits::Block},
    camera::PlayerCamera,
    grid::{Direction, GridCollides, GridDirection},
    GameStates,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkFields;

use super::mesh::BillboardMesh;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(billboard_setup_observer).add_systems(
        Update,
        billboard_movement_system.run_if(in_state(GameStates::Playing)),
    );
}

#[derive(Clone, Debug)]
pub struct BillboardBlock {
    direction: Direction,
    face_camera: bool,
}

#[derive(Component, Clone, Debug, Default)]
pub struct BillboardBlockMarker;

impl From<BlockSource> for BillboardBlock {
    fn from(value: BlockSource) -> Self {
        match value {
            BlockSource::Entity(entity_instance) => {
                let direction = Direction::from(&entity_instance);

                let face_camera = entity_instance
                    .get_bool_field("face_camera")
                    .cloned()
                    .unwrap_or_default();

                Self {
                    direction,
                    face_camera,
                }
            }
            BlockSource::Tile(_tile_instance) => todo!(),
        }
    }
}

impl Block for BillboardBlock {
    type MarkerType = BillboardBlockMarker;
    type BlockMeshType = BillboardMesh;

    fn specialize(
        &self,
        mut entity: EntityCommands,
        mesh: &crate::blocks::block_traits::BlockMeshHandle<Self>,
        material: &Handle<StandardMaterial>,
    ) {
        entity.insert((
            GridDirection(self.direction),
            Mesh3d(mesh.deref().clone()),
            MeshMaterial3d(material.clone()),
            GridCollides,
        ));

        if self.face_camera {
            entity.insert(BillboardFaceCamera);
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct BillboardFaceCamera;

fn billboard_setup_observer(
    trigger: Trigger<OnInsert, BillboardFaceCamera>,
    player_camera: Query<&GridDirection, With<PlayerCamera>>,
    mut billboards: Query<&mut GridDirection, (Without<PlayerCamera>, With<BillboardFaceCamera>)>,
) {
    let Ok(camera_direction) = player_camera.get_single() else {
        return;
    };
    let Ok(mut billboard_direction) = billboards.get_mut(trigger.entity()) else {
        return;
    };

    *billboard_direction = camera_direction.reverse();
}

fn billboard_movement_system(
    player_camera: Query<&GridDirection, (With<PlayerCamera>, Changed<GridDirection>)>,
    mut billboards: Query<&mut GridDirection, (Without<PlayerCamera>, With<BillboardFaceCamera>)>,
) {
    if let Ok(camera_direction) = player_camera.get_single() {
        for mut billboard_direction in &mut billboards {
            *billboard_direction = camera_direction.reverse().reverse();
        }
    }
}
