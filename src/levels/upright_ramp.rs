use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};

pub struct UprightRamp;

impl From<UprightRamp> for Mesh {
    fn from(_value: UprightRamp) -> Self {
        Self::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                // down (-y)
                [-0.5, -0.5, -0.5],
                [0.5, -0.5, -0.5],
                [0.5, -0.5, 0.5],
                [-0.5, -0.5, 0.5],
                // east (+x)
                [0.5, -0.5, -0.5],
                [0.5, -0.5, 0.5],
                [0.5, 0.5, -0.5],
                // west (-x)
                [-0.5, -0.5, -0.5],
                [-0.5, -0.5, 0.5],
                [-0.5, 0.5, -0.5],
                // up/south (+y, +z)
                [-0.5, -0.5, 0.5],
                [-0.5, 0.5, -0.5],
                [0.5, 0.5, -0.5],
                [0.5, -0.5, 0.5],
                // north (-z)
                [-0.5, -0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [0.5, 0.5, -0.5],
                [0.5, -0.5, -0.5],
            ],
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                // down (-y)
                [0.0, 0.0],
                [1.0, 0.0],
                [1.0, 1.0],
                [0.0, 1.0],
                // east (+x)
                [1.0, 1.0],
                [0.0, 1.0],
                [1.0, 0.0],
                // west (-x)
                [0.0, 1.0],
                [1.0, 1.0],
                [0.0, 0.0],
                // up/south (+y, +z)
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
                // down (-y)
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                // east (+x)
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                // west (-x)
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                // up/south (+y, +z)
                [0.0, 1.0, 1.0],
                [0.0, 1.0, 1.0],
                [0.0, 1.0, 1.0],
                [0.0, 1.0, 1.0],
                // north (-z)
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
            ],
        )
        .with_inserted_indices(Indices::U32(vec![
            0, 1, 3, 1, 2, 3, // down (-y)
            4, 6, 5, // east (+x)
            7, 8, 9, // west (-x)
            10, 13, 11, 11, 13, 12, // up/south (+y, +z)
            14, 15, 17, 15, 16, 17, // north (-z)
        ]))
    }
}
