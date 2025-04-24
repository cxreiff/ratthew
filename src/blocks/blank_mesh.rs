use crate::blocks::block_traits::BlockMesh;
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct BlankMesh;

#[derive(Resource, Deref, Clone, Debug)]
pub struct BlankMeshHandle(Handle<Mesh>);

impl From<Handle<Mesh>> for BlankMeshHandle {
    fn from(value: Handle<Mesh>) -> Self {
        Self(value)
    }
}

impl BlockMesh for BlankMesh {
    type Handle = BlankMeshHandle;

    fn indices() -> Vec<u32> {
        vec![]
    }

    fn positions() -> std::vec::Vec<[f32; 3]> {
        vec![]
    }

    fn uvs() -> std::vec::Vec<[f32; 2]> {
        vec![]
    }

    fn normals() -> std::vec::Vec<[f32; 3]> {
        vec![]
    }
}
