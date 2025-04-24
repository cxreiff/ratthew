use bevy::prelude::*;

use crate::blocks::BlockMesh;

#[derive(Clone, Debug)]
pub struct RampFlippedMesh;

#[derive(Resource, Deref, Clone, Debug, Default)]
pub struct RampFlippedMeshHandle(Handle<Mesh>);

impl From<Handle<Mesh>> for RampFlippedMeshHandle {
    fn from(value: Handle<Mesh>) -> Self {
        Self(value)
    }
}

impl BlockMesh for RampFlippedMesh {
    type Handle = RampFlippedMeshHandle;

    fn indices() -> Vec<u32> {
        vec![
            0, 1, 3, 1, 2, 3, // up (+y)
            4, 6, 5, // east (+x)
            7, 8, 9, // west (-x)
            10, 13, 11, 11, 13, 12, // down/north (-y, -z)
            14, 15, 17, 15, 16, 17, // south (+z)
        ]
    }

    fn positions() -> Vec<[f32; 3]> {
        vec![
            // up (+y)
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, -0.5],
            // east (+x)
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, 0.5],
            // west (-x)
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, -0.5, 0.5],
            // down/north (-y, -z)
            [-0.5, 0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, -0.5],
            // south (+z)
            [-0.5, 0.5, 0.5],
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
        ]
    }

    fn uvs() -> Vec<[f32; 2]> {
        vec![
            // up (+y)
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
            [0.0, 0.0],
            // east (+x)
            [0.0, 0.0],
            [1.0, 0.0],
            [0.0, 1.0],
            // west (-x)
            [1.0, 0.0],
            [0.0, 0.0],
            [1.0, 1.0],
            // down/north (-y, -z)
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            [0.0, 0.0],
            // south (+z)
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
        ]
    }

    fn normals() -> Vec<[f32; 3]> {
        vec![
            // up (+y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // east (+x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // west (-x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // down/north (-y, -z)
            [0.0, -1.0, -1.0],
            [0.0, -1.0, -1.0],
            [0.0, -1.0, -1.0],
            [0.0, -1.0, -1.0],
            // south (+z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ]
    }
}
