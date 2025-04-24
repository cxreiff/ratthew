use std::{fmt::Debug, ops::Deref};

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use super::block_layer::BlockSource;

pub type BlockMeshHandle<T> = <<T as Block>::BlockMeshType as BlockMesh>::Handle;

pub trait Block: From<BlockSource> {
    type MarkerType: Default + Component;
    type BlockMeshType: BlockMesh;

    fn specialize(
        &self,
        _entity: EntityCommands,
        _mesh: &BlockMeshHandle<Self>,
        _material: &Handle<StandardMaterial>,
    ) {
    }
}

pub trait BlockMesh: Debug + Clone {
    type Handle: Resource + Deref<Target = Handle<Mesh>> + From<Handle<Mesh>> + Clone + Debug;

    fn positions() -> Vec<[f32; 3]>;
    fn uvs() -> Vec<[f32; 2]>;
    fn normals() -> Vec<[f32; 3]>;
    fn indices() -> Vec<u32>;

    fn generate_mesh(sprite_xy: IVec2, sprite_size: IVec2, tileset_size: IVec2) -> Mesh {
        let uv_tile_width = sprite_size.x as f32 / tileset_size.x as f32;
        let uv_tile_height = sprite_size.y as f32 / tileset_size.y as f32;
        let x_zero = sprite_xy.x as f32 * uv_tile_width + 0.0001;
        let y_zero = sprite_xy.y as f32 * uv_tile_height + 0.0001;

        let uvs = Self::uvs()
            .iter()
            .map(|[x, y]| {
                [
                    x_zero + (uv_tile_width - 0.0002) * x,
                    y_zero + (uv_tile_height - 0.0002) * y,
                ]
            })
            .collect::<Vec<_>>();

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, Self::positions())
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, Self::normals())
        .with_inserted_indices(Indices::U32(Self::indices()))
    }
}
