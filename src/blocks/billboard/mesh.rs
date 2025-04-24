use crate::blocks::block_traits::BlockMesh;
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct BillboardMesh;

#[derive(Resource, Deref, Clone, Debug)]
pub struct BillboardMeshHandle(pub Handle<Mesh>);

impl From<Handle<Mesh>> for BillboardMeshHandle {
    fn from(value: Handle<Mesh>) -> Self {
        Self(value)
    }
}

impl BlockMesh for BillboardMesh {
    type Handle = BillboardMeshHandle;

    fn indices() -> Vec<u32> {
        vec![
            0, 1, 2, 0, 2, 3, // south (+z)
            4, 5, 6, 4, 6, 7, // north (-z)
        ]
    }

    fn positions() -> std::vec::Vec<[f32; 3]> {
        vec![
            // south (+z)
            [0.5, 0.5, 0.0],
            [-0.5, 0.5, 0.0],
            [-0.5, -0.5, 0.0],
            [0.5, -0.5, 0.0],
            // north (-z)
            [-0.5, 0.5, 0.0],
            [0.5, 0.5, 0.0],
            [0.5, -0.5, 0.0],
            [-0.5, -0.5, 0.0],
        ]
    }

    fn uvs() -> std::vec::Vec<[f32; 2]> {
        vec![
            // south (+z)
            [1.0, 0.0],
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0],
            // north (-z)
            [1.0, 0.0],
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0],
        ]
    }

    fn normals() -> std::vec::Vec<[f32; 3]> {
        vec![
            // south (+z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // north (-z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ]
    }
}
