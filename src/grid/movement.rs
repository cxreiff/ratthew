use bevy::prelude::*;
use bevy_tween::{
    combinator::{event, sequence, tween},
    prelude::*,
    tween::AnimationTarget,
    tween_event::TweenEventPlugin,
};
use interpolate::{rotation, translation};

use crate::GameStates;

use super::{direction::GridDirection, GridPosition};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TweenEventPlugin::<GridTweenCleanup>::default())
        .add_event::<GridTweenCleanup>()
        .add_systems(
            Update,
            (
                grid_animated_movement_system,
                grid_static_position_movement_system,
                grid_static_direction_movement_system,
                grid_tween_cleanup_handler,
            )
                .run_if(in_state(GameStates::Playing)),
        )
        .add_observer(grid_position_setup_observer)
        .add_observer(grid_direction_setup_observer)
        .add_observer(grid_animated_setup_observer);
}

#[derive(Component, Debug)]
#[require(AnimationTarget(|| AnimationTarget))]
pub struct GridAnimated;

#[derive(Event, Default, Debug, Clone)]
pub struct GridBlockedMove(pub GridPosition);

#[derive(Component, Default, Debug, Clone)]
pub struct GridBlockedMoveParent;

#[derive(Event, Debug, Clone)]
pub struct GridTweenCleanup(pub Entity);

impl Default for GridTweenCleanup {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

fn grid_position_setup_observer(
    trigger: Trigger<OnAdd, GridPosition>,
    mut positioned: Query<(&GridPosition, &mut Transform)>,
) {
    let (position, mut transform) = positioned.get_mut(trigger.entity()).unwrap();
    transform.translation = position.into();
}

fn grid_direction_setup_observer(
    trigger: Trigger<OnAdd, GridDirection>,
    mut directed: Query<(&GridDirection, &mut Transform)>,
) {
    let (direction, mut transform) = directed.get_mut(trigger.entity()).unwrap();
    transform.rotation = direction.into();
}

fn grid_animated_setup_observer(trigger: Trigger<OnAdd, GridAnimated>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(grid_blocked_movement_observer);
}

fn grid_blocked_movement_observer(
    trigger: Trigger<GridBlockedMove>,
    mut commands: Commands,
    grid_positions: Query<(&Transform, &GridPosition)>,
) {
    let target = trigger.entity().into_target();
    let (transform, grid_position) = grid_positions.get(trigger.entity()).unwrap();
    let grid_position = Vec3::from(grid_position);
    let attempted_position = trigger.event().0;
    let distance = grid_position.distance(attempted_position.into());
    let bump_position = grid_position.move_towards(attempted_position.into(), distance / 3.);

    commands.entity(trigger.entity()).with_children(|children| {
        let mut tween_parent = children.spawn(GridBlockedMoveParent);
        let tween_parent_id = tween_parent.id();
        tween_parent.animation().insert(sequence((
            tween(
                Duration::from_millis(150),
                EaseKind::ExponentialOut,
                target.with(translation(transform.translation, bump_position)),
            ),
            tween(
                Duration::from_millis(250),
                EaseKind::ExponentialOut,
                target.with(translation(bump_position, grid_position)),
            ),
            event(GridTweenCleanup(tween_parent_id)),
        )));
    });
}

fn grid_animated_movement_system(
    mut commands: Commands,
    grid_position_changed: Query<
        (Entity, &GridPosition, &GridDirection, &Transform, &Children),
        (
            Or<(Changed<GridPosition>, Changed<GridDirection>)>,
            With<GridAnimated>,
        ),
    >,
    block_animations: Query<&GridBlockedMoveParent>,
    mut cleanup_event: EventWriter<GridTweenCleanup>,
) {
    for (entity, position, direction, transform, children) in &grid_position_changed {
        let target = entity.into_target();

        for child in children {
            if block_animations.get(*child).is_ok() {
                cleanup_event.send(GridTweenCleanup(*child));
            }
        }

        commands.entity(entity).animation().insert_tween_here(
            Duration::from_millis(500),
            EaseKind::ExponentialOut,
            (
                target.with(translation(transform.translation, position.into())),
                target.with(rotation(transform.rotation, direction.into())),
            ),
        );
    }
}

fn grid_static_position_movement_system(
    mut grid_position_changed: Query<
        (&GridPosition, &mut Transform),
        (Changed<GridPosition>, Without<GridAnimated>),
    >,
) {
    for (position, mut transform) in &mut grid_position_changed {
        transform.translation = position.into();
    }
}

fn grid_static_direction_movement_system(
    mut grid_position_changed: Query<
        (&GridDirection, &mut Transform),
        (Changed<GridDirection>, Without<GridAnimated>),
    >,
) {
    for (direction, mut transform) in &mut grid_position_changed {
        transform.rotation = direction.into();
    }
}

fn grid_tween_cleanup_handler(
    mut commands: Commands,
    mut cleanup_events: EventReader<GridTweenCleanup>,
) {
    for GridTweenCleanup(parent) in cleanup_events.read() {
        if let Some(parent_commands) = commands.get_entity(*parent) {
            parent_commands.despawn_recursive();
        }
    }
}
