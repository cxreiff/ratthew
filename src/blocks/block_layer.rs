use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_ldtk::{
    ldtk::{self, loaded_level::LoadedLevel, LayerInstance, TileInstance},
    EntityInstance,
};
use image::DynamicImage;

use super::{
    block_instances::BlockInstance,
    block_traits::{Block, BlockMeshHandle},
    BlockMesh,
};

pub struct BlockLayer<B: Block> {
    pub sprite_size: IVec2,
    pub offset: IVec3,
    pub blocks: Vec<BlockInstance<B>>,
}

pub enum BlockSource {
    Entity(EntityInstance),
    Tile(TileInstance),
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct BlockMeshMap<B: Block>(HashMap<(i32, i32), <B::BlockMeshType as BlockMesh>::Handle>);

impl<B: Block> Default for BlockMeshMap<B> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<B: Block> BlockLayer<B> {
    pub fn build(level: &LoadedLevel, layer: &LayerInstance) -> Self {
        let sprite_size = IVec2::new(layer.c_wid, layer.c_hei);
        let offset = IVec3::new(*level.world_x(), *level.world_depth(), *level.world_y());

        let blocks = match layer.layer_instance_type {
            ldtk::Type::IntGrid => Self::collect_tile_block_instances(&layer.auto_layer_tiles),
            ldtk::Type::Tiles => Self::collect_tile_block_instances(&layer.grid_tiles),
            ldtk::Type::AutoLayer => Self::collect_tile_block_instances(&layer.auto_layer_tiles),
            ldtk::Type::Entities => Self::collect_entity_block_instances(&layer.entity_instances),
        };

        Self {
            sprite_size,
            offset,
            blocks,
        }
    }

    pub fn spawn(
        &self,
        mut commands: Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        tileset: &DynamicImage,
        material: &Handle<StandardMaterial>,
    ) {
        let mut mesh_map = BlockMeshMap::<B>::default();
        let width_in_tiles = tileset.width() as i32 / 16;
        let height_in_tiles = tileset.height() as i32 / 16;

        for x in 0..width_in_tiles {
            for y in 0..height_in_tiles {
                let mesh = B::BlockMeshType::generate_mesh(
                    IVec2::new(x, y),
                    self.sprite_size,
                    IVec2::new(tileset.width() as i32, tileset.height() as i32),
                );
                let handle = BlockMeshHandle::<B>::from(meshes.add(mesh));
                mesh_map.insert((x * self.sprite_size.x, y * self.sprite_size.y), handle);
            }
        }

        for block in &self.blocks {
            block.spawn(self, commands.reborrow(), &mesh_map, material);
        }
    }

    fn collect_tile_block_instances(tiles: &[TileInstance]) -> Vec<BlockInstance<B>> {
        let mut instances = vec![];

        for tile in tiles.iter() {
            instances.push(BlockInstance::<B>::from(BlockSource::Tile(tile.clone())));
        }

        instances
    }

    fn collect_entity_block_instances(entities: &[EntityInstance]) -> Vec<BlockInstance<B>> {
        let mut instances = vec![];

        for entity in entities.iter() {
            instances.push(BlockInstance::<B>::from(BlockSource::Entity(
                entity.clone(),
            )));
        }

        instances
    }
}
