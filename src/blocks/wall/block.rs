use std::ops::Deref;

use crate::{
    blocks::{block_layer::BlockSource, block_traits::Block},
    grid::GridCollides,
};
use bevy::prelude::*;

use super::mesh::WallMesh;

#[derive(Clone, Debug)]
pub struct WallBlock;

#[derive(Component, Clone, Debug, Default)]
pub struct WallBlockMarker;

impl From<BlockSource> for WallBlock {
    fn from(value: BlockSource) -> Self {
        match value {
            BlockSource::Entity(_entity_instance) => todo!(),
            BlockSource::Tile(_tile_instance) => Self,
        }
    }
}

impl Block for WallBlock {
    type BlockMeshType = WallMesh;
    type MarkerType = WallBlockMarker;

    fn specialize(
        &self,
        mut entity: EntityCommands,
        mesh: &crate::blocks::block_traits::BlockMeshHandle<Self>,
        material: &Handle<StandardMaterial>,
    ) {
        entity.insert((
            Mesh3d(mesh.deref().clone()),
            MeshMaterial3d(material.clone()),
            GridCollides,
        ));
    }
}
