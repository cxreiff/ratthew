use bevy::prelude::*;

use std::ops::Deref;

use crate::{
    blocks::{block_layer::BlockSource, block_traits::Block},
    grid::GridCollides,
};

use super::mesh::ColumnMesh;

#[derive(Clone, Debug)]
pub struct ColumnBlock;

#[derive(Component, Clone, Debug, Default)]
pub struct ColumnBlockMarker;

impl From<BlockSource> for ColumnBlock {
    fn from(value: BlockSource) -> Self {
        match value {
            BlockSource::Entity(_entity_instance) => todo!(),
            BlockSource::Tile(_tile_instance) => Self,
        }
    }
}

impl Block for ColumnBlock {
    type BlockMeshType = ColumnMesh;
    type MarkerType = ColumnBlockMarker;

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
