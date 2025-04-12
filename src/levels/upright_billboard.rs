use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};

pub struct UprightBillboard;

impl From<UprightBillboard> for Mesh {
    fn from(_value: UprightBillboard) -> Self {
        Self::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                // south (+z)
                [-0.5, -0.5, 0.0],
                [-0.5, 0.5, 0.0],
                [0.5, 0.5, 0.0],
                [0.5, -0.5, 0.0],
                // north (-z)
                [-0.5, -0.5, 0.0],
                [-0.5, 0.5, 0.0],
                [0.5, 0.5, 0.0],
                [0.5, -0.5, 0.0],
            ],
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
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
            ],
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
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
            ],
        )
        .with_inserted_indices(Indices::U32(vec![
            0, 3, 1, 1, 3, 2, // south (+z)
            4, 5, 7, 5, 6, 7, // north (-z)
        ]))
    }
}
