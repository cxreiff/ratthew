use std::ops::Deref;

use bevy::prelude::*;

use crate::{
    blocks::{block_layer::BlockSource, block_traits::Block},
    grid::{Direction, GridDirection},
};

use super::mesh::RampMesh;

#[derive(Clone, Debug)]
pub struct RampBlock {
    direction: Direction,
}

#[derive(Component, Clone, Debug, Default)]
pub struct RampBlockMarker;

impl From<BlockSource> for RampBlock {
    fn from(value: BlockSource) -> Self {
        match value {
            BlockSource::Entity(entity_instance) => {
                let direction = Direction::from(&entity_instance);
                Self { direction }
            }
            BlockSource::Tile(_tile_instance) => todo!(),
        }
    }
}

impl Block for RampBlock {
    type MarkerType = RampBlockMarker;
    type BlockMeshType = RampMesh;

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
        ));
    }
}
