use bevy::prelude::*;
use bevy_tween::{
    combinator::{event, sequence, tween},
    prelude::*,
    tween::AnimationTarget,
    tween_event::TweenEventPlugin,
    TweenSystemSet,
};
use interpolate::{rotation, translation};

use crate::levels::RampBlock;

use super::{
    position::GridAmbulatory,
    utilities::{find_ramp_position, find_ramp_position_direction},
    Direction, GridDirection, GridPosition, GridSystemSet,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TweenEventPlugin::<GridTweenCleanup>::default())
        .add_event::<GridTweenCleanup>()
        .add_observer(grid_animated_setup_observer)
        .add_systems(
            PostUpdate,
            (
                grid_animated_movement_system.in_set(GridSystemSet::Movement),
                grid_tween_cleanup_handler.in_set(GridSystemSet::Cleanup),
                ramp_height_correction_system.after(TweenSystemSet::ApplyTween),
            )
                .chain(),
        );
}

#[derive(Component, Debug)]
#[require(AnimationTarget(|| AnimationTarget))]
pub struct GridAnimated;

#[derive(Event, Default, Debug, Clone)]
pub struct GridMoveBlocked(pub GridPosition);

#[derive(Component, Default, Debug, Clone)]
pub struct GridMoveParent;

#[derive(Component, Default, Debug, Clone)]
pub struct GridMoveBlockedParent;

#[derive(Event, Debug, Clone)]
pub struct GridTweenCleanup(pub Entity);

impl Default for GridTweenCleanup {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

fn grid_animated_setup_observer(trigger: Trigger<OnAdd, GridAnimated>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(grid_movement_blocked_observer);
}

fn grid_movement_blocked_observer(
    trigger: Trigger<GridMoveBlocked>,
    mut commands: Commands,
    grid_positions: Query<(&Transform, &GridPosition, Option<&GridAmbulatory>)>,
    ramps: Query<(&GridPosition, &GridDirection), With<RampBlock>>,
) {
    let target = trigger.entity().into_target();
    let (transform, grid_position, ambulatory) = grid_positions.get(trigger.entity()).unwrap();
    let mut original_translation = Vec3::from(grid_position);
    let attempted_position = trigger.event().0;
    let mut attempted_translation = Vec3::from(attempted_position);

    // TODO: Consider refactoring such that "on_ramp" is a property of GridAmbulatory, that we set
    // at the time of each movement, and animate accordingly. May or may not be cleaner.

    if ambulatory.is_some() {
        if let Some((ramp_position, ramp_direction)) =
            find_ramp_position_direction(grid_position, &ramps)
        {
            original_translation.y += 0.5;

            if ramp_position
                .forward(ramp_direction)
                .eq(&attempted_position)
            {
                attempted_translation.y += 1.;
            }

            if ramp_position.left(ramp_direction).eq(&attempted_position)
                || ramp_position.right(ramp_direction).eq(&attempted_position)
            {
                attempted_translation.y += 0.5;
            }
        }
    }

    let distance = original_translation.distance(attempted_translation);
    let bump_position = original_translation.move_towards(attempted_translation, distance / 3.);

    commands.entity(trigger.entity()).with_children(|children| {
        let mut tween_parent = children.spawn(GridMoveBlockedParent);
        let tween_parent_id = tween_parent.id();
        tween_parent.animation().insert(sequence((
            tween(
                Duration::from_millis(200),
                EaseKind::ExponentialOut,
                target.with(translation(transform.translation, bump_position)),
            ),
            tween(
                Duration::from_millis(250),
                EaseKind::ExponentialOut,
                target.with(translation(bump_position, original_translation)),
            ),
            event(GridTweenCleanup(tween_parent_id)),
        )));
    });
}

fn grid_animated_movement_system(
    mut commands: Commands,
    grid_position_changed: Query<
        (
            Entity,
            &GridPosition,
            &GridDirection,
            &Transform,
            &Children,
            Option<&GridAmbulatory>,
        ),
        (
            Or<(Changed<GridPosition>, Changed<GridDirection>)>,
            With<GridAnimated>,
        ),
    >,
    ramps: Query<&GridPosition, With<RampBlock>>,
    block_animations: Query<Entity, Or<(With<GridMoveParent>, With<GridMoveBlockedParent>)>>,
    mut cleanup_event: EventWriter<GridTweenCleanup>,
) {
    for (entity, position, direction, transform, children, ambulatory) in &grid_position_changed {
        let target = entity.into_target();

        for child in children {
            if block_animations.get(*child).is_ok() {
                cleanup_event.send(GridTweenCleanup(*child));
            }
        }

        let mut target_translation = Vec3::from(position);

        if ambulatory.is_some() && find_ramp_position(position, &ramps).is_some() {
            target_translation.y += 0.5;
        }

        commands.entity(entity).with_children(|children| {
            let mut tween_parent = children.spawn(GridMoveParent);
            let tween_parent_id = tween_parent.id();
            tween_parent.animation().insert(sequence((
                tween(
                    Duration::from_millis(500),
                    EaseKind::ExponentialOut,
                    (
                        target.with(translation(transform.translation, target_translation)),
                        target.with(rotation(transform.rotation, direction.into())),
                    ),
                ),
                event(GridTweenCleanup(tween_parent_id)),
            )));
        });
    }
}

fn ramp_height_correction_system(
    mut positioned: Query<&mut Transform, With<GridAmbulatory>>,
    ramps: Query<(&GridPosition, &GridDirection), With<RampBlock>>,
) {
    for mut transform in &mut positioned {
        let current_position = GridPosition::from(transform.translation);
        if let Some((ramp_position, ramp_direction)) =
            find_ramp_position_direction(&current_position, &ramps)
        {
            let base_height = Vec3::from(ramp_position).y;
            let ramp_diff_z = transform.translation.z - ramp_position.z as f32;
            let ramp_diff_x = transform.translation.x - ramp_position.x as f32;
            let ramp_height = match ramp_direction.0 {
                Direction::North => 0.5 - ramp_diff_z,
                Direction::East => 0.5 + ramp_diff_x,
                Direction::South => 0.5 + ramp_diff_z,
                Direction::West => 0.5 - ramp_diff_x,
            };

            transform.translation.y = transform.translation.y.max(base_height + ramp_height);
        }
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
