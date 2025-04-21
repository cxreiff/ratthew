use bevy::prelude::*;
use bevy_asset_loader::{
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
    standard_dynamic_asset::StandardDynamicAssetCollection,
};

use crate::{camera::PlayerAssets, levels::LevelAssets, sound::SfxAssets, GameStates};

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(GameStates::Loading)
            .continue_to_state(GameStates::Playing)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("assets.ron")
            .load_collection::<PlayerAssets>()
            .load_collection::<LevelAssets>()
            .load_collection::<SfxAssets>(),
    );
}
