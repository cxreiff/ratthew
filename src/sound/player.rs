use std::ops::DerefMut;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_rand::prelude::{Entropy, WyRand};
use rand::seq::SliceRandom;

use crate::{
    grid::{GridDirectionMove, GridPositionMoveAttempt},
    Flags,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(move_position_sfx_observer)
        .add_observer(move_direction_sfx_observer);
}

#[derive(AssetCollection, Resource)]
pub struct SfxAssets {
    #[asset(key = "sfx.grass", collection(typed))]
    _grass_sfx: Vec<Handle<AudioSource>>,
    #[asset(key = "sfx.gravel", collection(typed))]
    gravel_sfx: Vec<Handle<AudioSource>>,
    #[asset(key = "sfx.snow", collection(typed))]
    snow_sfx: Vec<Handle<AudioSource>>,
    #[asset(key = "sfx.wood", collection(typed))]
    _wood_sfx: Vec<Handle<AudioSource>>,
}

fn move_position_sfx_observer(
    trigger: Trigger<GridPositionMoveAttempt>,
    flags: Res<Flags>,
    commands: Commands,
    sfx_assets: Res<SfxAssets>,
    mut rng: Local<Entropy<WyRand>>,
) {
    if !flags.sound {
        return;
    }

    let sfx = sfx_assets
        .snow_sfx
        .choose(&mut rng.deref_mut())
        .unwrap()
        .clone();

    play_sfx(commands, trigger.entity(), sfx);
}

fn move_direction_sfx_observer(
    trigger: Trigger<GridDirectionMove>,
    flags: Res<Flags>,
    commands: Commands,
    sfx_assets: Res<SfxAssets>,
    mut rng: Local<Entropy<WyRand>>,
) {
    if !flags.sound {
        return;
    }

    let sfx = sfx_assets
        .gravel_sfx
        .choose(&mut rng.deref_mut())
        .unwrap()
        .clone();

    play_sfx(commands, trigger.entity(), sfx);
}

fn play_sfx(mut commands: Commands, entity: Entity, sfx: Handle<AudioSource>) {
    let mut entity = commands.entity(entity);
    entity.remove::<AudioSink>();
    entity.insert(AudioPlayer::new(sfx));
}
