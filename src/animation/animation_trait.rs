use bevy::prelude::*;
use bevy_tween::{
    combinator::AnimationBuilder,
    prelude::AnimationBuilderExt,
    tween::{IntoTarget, TargetComponent},
};

pub trait AutoTween: Component + Sized {
    type Holder: Component + Default;

    fn insert_tween(&self, animation: AnimationBuilder, target: TargetComponent);

    fn autotween_plugin(app: &mut App) {
        app.add_observer(Self::insert_observer)
            .add_observer(Self::remove_observer);
    }

    fn insert_observer(
        trigger: Trigger<OnInsert, Self>,
        mut commands: Commands,
        self_query: Query<&Self>,
    ) {
        let target = trigger.entity().into_target();
        let component = self_query.get(trigger.entity()).unwrap();

        commands.entity(trigger.entity()).with_children(|children| {
            let mut holder = children.spawn(Self::Holder::default());

            component.insert_tween(holder.animation(), target);
        });
    }

    fn remove_observer(
        trigger: Trigger<OnReplace, Self>,
        mut commands: Commands,
        children: Query<&Children, With<Self>>,
        holders: Query<&Self::Holder>,
    ) {
        if let Ok(children) = children.get(trigger.entity()) {
            for &child in children.iter() {
                if holders.contains(child) {
                    commands.entity(child).despawn_recursive();
                }
            }
        }
    }
}
