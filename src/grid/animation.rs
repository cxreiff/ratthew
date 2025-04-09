use bevy::prelude::*;
use bevy_tween::{
    combinator::{event, sequence, tween},
    component_tween_system,
    prelude::*,
    tween::AnimationTarget,
    tween_event::TweenEventPlugin,
};
use interpolate::rotation;

use crate::levels::RampBlock;

use super::{
    utilities::find_ramp_position_direction, Direction, GridDirection, GridPosition, GridSystemSet,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TweenEventPlugin::<GridTweenCleanup>::default())
        .add_event::<GridTweenCleanup>()
        .add_observer(grid_animated_setup_observer)
        .add_tween_systems(component_tween_system::<InterpolateGridAnimated>())
        .add_systems(
            PostUpdate,
            (
                grid_animated_movement_system.in_set(GridSystemSet::Movement),
                ramp_height_correction_system.in_set(GridSystemSet::Movement),
                grid_tween_cleanup_handler.in_set(GridSystemSet::Cleanup),
            )
                .chain(),
        );
}

#[derive(Component, Debug, Default)]
#[require(AnimationTarget(|| AnimationTarget))]
pub struct GridAnimated {
    pub buffer_transform: Vec3,
    pub previous_position: GridPosition,
}

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

struct InterpolateGridAnimated {
    start: Vec3,
    end: Vec3,
}

impl Interpolator for InterpolateGridAnimated {
    type Item = GridAnimated;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.buffer_transform = self.start.lerp(self.end, value);
    }
}

fn indirect_translation(start: Vec3, end: Vec3) -> InterpolateGridAnimated {
    InterpolateGridAnimated { start, end }
}

fn grid_animated_setup_observer(
    trigger: Trigger<OnAdd, GridAnimated>,
    mut commands: Commands,
    mut grid_animated: Query<(&GridPosition, &mut GridAnimated)>,
) {
    let (grid_position, mut grid_animated) = grid_animated.get_mut(trigger.entity()).unwrap();
    grid_animated.buffer_transform = grid_position.into();
    grid_animated.previous_position = *grid_position;

    commands
        .entity(trigger.entity())
        .observe(grid_movement_blocked_observer);
}

fn grid_movement_blocked_observer(
    trigger: Trigger<GridMoveBlocked>,
    mut commands: Commands,
    grid_positions: Query<(
        &Transform,
        &GridPosition,
        &GridDirection,
        &GridAnimated,
        &Children,
    )>,
    block_animations: Query<Entity, Or<(With<GridMoveParent>, With<GridMoveBlockedParent>)>>,
    mut cleanup_event: EventWriter<GridTweenCleanup>,
) {
    let target = trigger.entity().into_target();

    let (transform, grid_position, grid_direction, grid_animated, children) =
        grid_positions.get(trigger.entity()).unwrap();
    let return_translation = Vec3::from(grid_position);
    let attempted_translation = Vec3::from(trigger.event().0);

    for child in children {
        if block_animations.get(*child).is_ok() {
            cleanup_event.send(GridTweenCleanup(*child));
        }
    }

    let distance = return_translation.distance(attempted_translation);
    let bump_position = return_translation.move_towards(attempted_translation, distance / 3.);

    commands.entity(trigger.entity()).with_children(|children| {
        let mut tween_parent = children.spawn(GridMoveBlockedParent);
        let tween_parent_id = tween_parent.id();
        tween_parent.animation().insert(sequence((
            tween(
                Duration::from_millis(200),
                EaseKind::ExponentialOut,
                (
                    target.with(indirect_translation(
                        grid_animated.previous_position.into(),
                        bump_position,
                    )),
                    target.with(rotation(transform.rotation, grid_direction.into())),
                ),
            ),
            tween(
                Duration::from_millis(250),
                EaseKind::ExponentialOut,
                target.with(indirect_translation(bump_position, return_translation)),
            ),
            event(GridTweenCleanup(tween_parent_id)),
        )));
    });
}

fn grid_animated_movement_system(
    mut commands: Commands,
    mut grid_position_changed: Query<
        (
            Entity,
            &GridPosition,
            &GridDirection,
            &Transform,
            &Children,
            &mut GridAnimated,
        ),
        (Or<(Changed<GridPosition>, Changed<GridDirection>)>,),
    >,
    block_animations: Query<Entity, Or<(With<GridMoveParent>, With<GridMoveBlockedParent>)>>,
    mut cleanup_event: EventWriter<GridTweenCleanup>,
) {
    for (entity, position, direction, transform, children, mut animated) in
        &mut grid_position_changed
    {
        let target = entity.into_target();

        let previous = animated.previous_position;
        animated.previous_position = *position;

        for child in children {
            if block_animations.get(*child).is_ok() {
                cleanup_event.send(GridTweenCleanup(*child));
            }
        }

        let target_translation = Vec3::from(position);

        commands.entity(entity).with_children(|children| {
            let mut tween_parent = children.spawn(GridMoveParent);
            let tween_parent_id = tween_parent.id();
            tween_parent.animation().insert(sequence((
                tween(
                    Duration::from_millis(500),
                    EaseKind::ExponentialOut,
                    (
                        target.with(indirect_translation(
                            Vec3::from(previous),
                            target_translation,
                        )),
                        target.with(rotation(transform.rotation, direction.into())),
                    ),
                ),
                event(GridTweenCleanup(tween_parent_id)),
            )));
        });
    }
}

fn ramp_height_correction_system(
    mut positioned: Query<(&mut Transform, &GridAnimated)>,
    ramps: Query<(&GridPosition, &GridDirection), With<RampBlock>>,
) {
    for (mut transform, animated) in &mut positioned {
        let current_position = GridPosition::from(animated.buffer_transform);
        if let Some((ramp_position, ramp_direction)) =
            find_ramp_position_direction(&current_position, &ramps)
        {
            let base_height = Vec3::from(current_position).y;
            let ramp_diff_z = animated.buffer_transform.z - ramp_position.z as f32;
            let ramp_diff_x = animated.buffer_transform.x - ramp_position.x as f32;
            let ramp_height = match ramp_direction.0 {
                Direction::North => 0.5 - ramp_diff_z,
                Direction::East => 0.5 + ramp_diff_x,
                Direction::South => 0.5 + ramp_diff_z,
                Direction::West => 0.5 - ramp_diff_x,
            };

            transform.translation.y = base_height + ramp_height;
        } else {
            transform.translation.y = current_position.y as f32;
        }

        transform.translation.x = animated.buffer_transform.x;
        transform.translation.z = animated.buffer_transform.z;
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
