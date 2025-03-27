use std::io;
use std::time::Duration;

use bevy::app::ScheduleRunnerPlugin;
use bevy::diagnostic::DiagnosticsStore;
use bevy::log::tracing_subscriber;
use bevy::log::tracing_subscriber::layer::SubscriberExt;
use bevy::log::tracing_subscriber::util::SubscriberInitExt;
use bevy::prelude::*;
use bevy::utils::error;
use bevy_ratatui::kitty::KittyEnabled;
use bevy_ratatui::terminal::RatatuiContext;
use bevy_ratatui::RatatuiPlugins;
use bevy_ratatui_camera::RatatuiCameraWidget;

use crate::camera::{ParticleCamera, PlayerCamera, WorldCamera};
use crate::widgets::debug_frame::debug_frame;
use crate::Flags;
use crate::GameStates;

pub(super) fn plugin(app: &mut App) {
    // Send logs to tui-logger
    tracing_subscriber::registry()
        .with(Some(tui_logger::tracing_subscriber_layer()))
        .init();
    tui_logger::init_logger(tui_logger::LevelFilter::Info).unwrap();

    app.add_plugins((
        ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(1. / 90.)),
        RatatuiPlugins {
            enable_input_forwarding: true,
            ..default()
        },
    ))
    .add_systems(
        Update,
        draw_scene_system
            .map(error)
            .run_if(in_state(GameStates::Playing)),
    );
}

#[allow(clippy::too_many_arguments)]
fn draw_scene_system(
    mut commands: Commands,
    mut ratatui: ResMut<RatatuiContext>,
    player_widget: Query<&RatatuiCameraWidget, With<PlayerCamera>>,
    world_widget: Query<&RatatuiCameraWidget, With<WorldCamera>>,
    particle_widget: Query<&RatatuiCameraWidget, With<ParticleCamera>>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
    kitty_enabled: Option<Res<KittyEnabled>>,
) -> io::Result<()> {
    ratatui.draw(|frame| {
        let area = debug_frame(frame, &flags, &diagnostics, kitty_enabled.as_deref());

        if let Ok(w) = world_widget.get_single() {
            w.render_autoresize(area, frame.buffer_mut(), &mut commands);
        }
        if let Ok(w) = particle_widget.get_single() {
            w.render_autoresize(area, frame.buffer_mut(), &mut commands);
        }
        if let Ok(w) = player_widget.get_single() {
            w.render_autoresize(area, frame.buffer_mut(), &mut commands);
        }
    })?;

    Ok(())
}
