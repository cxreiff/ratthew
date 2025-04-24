use bevy::{prelude::*, render::view::RenderLayers};

use crate::grid::GridPosition;

use super::{block_layer::BlockSource, block_traits::Block, BlockLayer, BlockMeshMap};

pub struct BlockInstance<B: Block> {
    pub tile_xy: IVec2,
    pub sprite_xy: IVec2,
    pub block: B,
}

#[derive(Component, Clone, Debug, Default)]
pub struct BlockSpawnedFromLdtk;

impl<B: Block> From<BlockSource> for BlockInstance<B> {
    fn from(value: BlockSource) -> Self {
        let (tile_xy, sprite_xy) = match value {
            BlockSource::Entity(ref entity_instance) => {
                let tile_xy = entity_instance.px;
                let sprite_xy = entity_instance
                    .tile
                    .map(|t| IVec2::new(t.x, t.y))
                    .expect("ENTITY MISSING A SPRITESHEET TILE");

                (tile_xy, sprite_xy)
            }
            BlockSource::Tile(ref tile_instance) => {
                let tile_xy = tile_instance.px;
                let sprite_xy = tile_instance.src;

                (tile_xy, sprite_xy)
            }
        };

        let block = B::from(value);

        Self {
            tile_xy,
            sprite_xy,
            block,
        }
    }
}

impl<B: Block> BlockInstance<B> {
    pub fn spawn(
        &self,
        block_layer: &BlockLayer<B>,
        mut commands: Commands,
        mesh_map: &BlockMeshMap<B>,
        material: &Handle<StandardMaterial>,
    ) {
        let entity = commands.spawn((
            B::MarkerType::default(),
            BlockSpawnedFromLdtk,
            RenderLayers::layer(1),
            GridPosition(IVec3::new(
                (self.tile_xy.x + block_layer.offset.x) / block_layer.sprite_size.x,
                block_layer.offset.y,
                (self.tile_xy.y + block_layer.offset.z) / block_layer.sprite_size.y,
            )),
        ));

        let mesh = mesh_map
            .get(&(self.sprite_xy.x, self.sprite_xy.y))
            .expect("MISSING MESH FOR SPRITE COORDS");

        self.block.specialize(entity, mesh, material);
    }
}
