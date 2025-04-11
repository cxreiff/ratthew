use bevy::prelude::*;
use bevy_tween::tween_event::TweenEventPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins(TweenEventPlugin::<GridTweenCleanup>::default())
        .add_event::<GridTweenCleanup>()
        .add_observer(grid_tween_cleanup_observer);
}

#[derive(Event, Debug, Clone)]
pub struct GridTweenCleanup(pub Entity);

impl Default for GridTweenCleanup {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

fn grid_tween_cleanup_observer(trigger: Trigger<GridTweenCleanup>, mut commands: Commands) {
    commands.entity(trigger.event().0).despawn_recursive();
}
