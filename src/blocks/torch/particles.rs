use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(HanabiPlugin)
        .add_systems(Startup, torch_setup);
}

#[derive(Resource)]
pub struct TorchParticleEffect(pub Handle<EffectAsset>);

fn torch_setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.8, 0.4, 0.1, 1.0));
    gradient.add_key(1.0, Vec4::new(0.8, 0.0, 0.1, 0.4));

    let mut module = Module::default();

    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.01),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(0.1),
    };

    let init_orientation = OrientModifier {
        mode: OrientMode::FaceCameraPosition,
        ..default()
    };

    let set_size_modifier = SetSizeModifier {
        size: Vec3::splat(0.1).into(),
    };

    let lifetime = module.lit(1.);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let accel = module.lit(Vec3::new(0., 0.8, 0.));
    let update_accel = AccelModifier::new(accel);

    let effect = EffectAsset::new(32768, Spawner::rate(10.0.into()), module)
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .render(init_orientation)
        .render(set_size_modifier)
        .update(update_accel)
        .render(ColorOverLifetimeModifier { gradient });

    let effect_handle = effects.add(effect);

    commands.insert_resource(TorchParticleEffect(effect_handle));
}
