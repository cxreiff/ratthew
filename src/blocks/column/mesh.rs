use crate::blocks::block_traits::BlockMesh;
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct ColumnMesh;

#[derive(Resource, Deref, Clone, Debug)]
pub struct ColumnMeshHandle(Handle<Mesh>);

impl From<Handle<Mesh>> for ColumnMeshHandle {
    fn from(value: Handle<Mesh>) -> Self {
        Self(value)
    }
}

impl BlockMesh for ColumnMesh {
    type Handle = ColumnMeshHandle;

    fn indices() -> Vec<u32> {
        vec![
            0, 3, 1, 1, 3, 2, // up (+y)
            4, 5, 7, 5, 6, 7, // down (-y)
            8, 11, 9, 9, 11, 10, // east (+x)
            12, 13, 15, 13, 14,
            15, // west (-x)
                // 16, 19, 17, 17, 19, 18, // south (+z)
                // 20, 21, 23, 21, 22, 23, // north (-z)
        ]
    }

    fn positions() -> std::vec::Vec<[f32; 3]> {
        vec![
            // east (+x)
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
            // west (-x)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            // south (+z)
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // north (-z)
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ]
    }

    fn uvs() -> std::vec::Vec<[f32; 2]> {
        vec![
            // east (+x)
            [1.0, 1.0],
            [0.0, 1.0],
            [0.0, 0.0],
            [1.0, 0.0],
            // west (-x)
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
            [0.0, 0.0],
            // south (+z)
            [0.0, 1.0],
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            // north (-z)
            [1.0, 1.0],
            [1.0, 0.0],
            [0.0, 0.0],
            [0.0, 1.0],
        ]
    }

    fn normals() -> std::vec::Vec<[f32; 3]> {
        vec![
            // east (+x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // west (-x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
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
