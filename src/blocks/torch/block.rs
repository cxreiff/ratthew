use std::ops::Deref;

use bevy::{prelude::*, render::view::RenderLayers};
use bevy_hanabi::{ParticleEffect, ParticleEffectBundle};

use crate::blocks::{blank_mesh::BlankMesh, block_layer::BlockSource, block_traits::Block};

use super::particles::TorchParticleEffect;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, torch_setup_system)
        .add_observer(torch_setup_observer);
}

#[derive(Clone, Debug)]
pub struct TorchBlock;

#[derive(Component, Clone, Debug, Default)]
pub struct TorchBlockMarker;

impl From<BlockSource> for TorchBlock {
    fn from(value: BlockSource) -> Self {
        match value {
            BlockSource::Entity(_entity_instance) => Self,
            BlockSource::Tile(_tile_instance) => todo!(),
        }
    }
}

impl Block for TorchBlock {
    type BlockMeshType = BlankMesh;
    type MarkerType = TorchBlockMarker;
}

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct TorchMesh(Handle<Mesh>);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct TorchMaterial(Handle<StandardMaterial>);

fn torch_setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(TorchMesh(meshes.add(Cylinder::new(0.03, 0.4))));
    commands.insert_resource(TorchMaterial(materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.4, 0.3),
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..default()
    })));
}

fn torch_setup_observer(
    trigger: Trigger<OnAdd, TorchBlockMarker>,
    mut commands: Commands,
    particle_handle: Res<TorchParticleEffect>,
    torch_mesh: Res<TorchMesh>,
    torch_material: Res<TorchMaterial>,
) {
    commands
        .entity(trigger.entity())
        .insert(ParticleEffectBundle {
            effect: ParticleEffect::new(particle_handle.0.clone()),
            ..Default::default()
        })
        .with_child((
            RenderLayers::layer(1),
            Mesh3d(torch_mesh.deref().deref().clone()),
            MeshMaterial3d(torch_material.deref().deref().clone()),
            Transform::from_xyz(0., -0.25, 0.),
        ));
}
