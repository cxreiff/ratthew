use bevy::prelude::*;

mod billboard;
mod blank_mesh;
mod block_instances;
mod block_layer;
mod block_traits;
mod column;
mod ldtk_loading;
mod ramp;
mod ramp_flipped;
mod torch;
mod wall;

pub use billboard::BillboardBlock;
pub use block_instances::BlockSpawnedFromLdtk;
pub use block_layer::{BlockLayer, BlockMeshMap};
pub use block_traits::BlockMesh;
pub use ldtk_loading::LevelAssets;
pub use ramp::{RampBlock, RampBlockMarker};
pub use ramp_flipped::RampFlippedBlock;
pub use torch::TorchBlock;
pub use wall::WallBlock;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((billboard::plugin, ldtk_loading::plugin, torch::plugin));
}
