use std::io;
use std::time::Duration;

use bevy::app::ScheduleRunnerPlugin;
use bevy::diagnostic::DiagnosticsStore;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::log::tracing_subscriber;
use bevy::log::tracing_subscriber::layer::SubscriberExt;
use bevy::log::tracing_subscriber::util::SubscriberInitExt;
use bevy::prelude::*;
use bevy::utils::error;
use bevy_ratatui::kitty::KittyEnabled;
use bevy_ratatui::terminal::RatatuiContext;
use bevy_ratatui::RatatuiPlugins;
use bevy_ratatui_camera::RatatuiCameraWidget;
use crossterm::event::KeyEventKind;

use crate::camera::{BackgroundCamera, PlayerCamera, WorldCamera};
use crate::grid::{GridDirection, GridPosition};
use crate::widgets::debug_frame::debug_frame;
use crate::Flags;
use crate::GameStates;

pub(super) fn plugin(app: &mut App) {
    // send logs to tui-logger
    tracing_subscriber::registry()
        .with(Some(tui_logger::tracing_subscriber_layer()))
        .init();
    tui_logger::init_logger(tui_logger::LevelFilter::Info).unwrap();

    app.add_plugins((
        ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(1. / 90.)),
        RatatuiPlugins::default(),
    ))
    .add_systems(
        Update,
        (
            draw_scene_system.map(error),
            temporary_terminal_forward_system,
        )
            .run_if(in_state(GameStates::Playing)),
    );
}

#[allow(clippy::too_many_arguments)]
fn draw_scene_system(
    mut commands: Commands,
    mut ratatui: ResMut<RatatuiContext>,
    player_widget: Query<&RatatuiCameraWidget, With<PlayerCamera>>,
    world_widget: Query<&RatatuiCameraWidget, With<WorldCamera>>,
    background_widget: Query<&RatatuiCameraWidget, With<BackgroundCamera>>,
    player: Query<(&GridPosition, &GridDirection), With<PlayerCamera>>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
    kitty_enabled: Option<Res<KittyEnabled>>,
) -> io::Result<()> {
    ratatui.draw(|frame| {
        let area = debug_frame(
            frame,
            &flags,
            &diagnostics,
            kitty_enabled.as_deref(),
            player.get_single().ok(),
            true,
        );

        if let Ok(w) = background_widget.get_single() {
            w.render_autoresize(area, frame.buffer_mut(), &mut commands);
        }
        if let Ok(w) = world_widget.get_single() {
            w.render_autoresize(area, frame.buffer_mut(), &mut commands);
        }
        if let Ok(w) = player_widget.get_single() {
            w.render_autoresize(area, frame.buffer_mut(), &mut commands);
        }
    })?;

    Ok(())
}

//TODO: fix input forwarding in bevy_ratatui
pub fn temporary_terminal_forward_system(
    mut commands: Commands,
    mut ratatui_input: EventReader<bevy_ratatui::event::KeyEvent>,
    mut bevy_input: EventWriter<KeyboardInput>,
    window: Query<Entity, With<Window>>,
    dummy_window: Query<Entity, With<DummyWindow>>,
) {
    let window_entity = window
        .get_single()
        .or(dummy_window.get_single())
        .unwrap_or_else(|_| commands.spawn(DummyWindow).id());

    for bevy_ratatui::event::KeyEvent(kc) in ratatui_input.read() {
        if let KeyEventKind::Release = kc.kind {
            continue;
        }

        let mut send_key = |character: char, bevy_keycode| {
            bevy_input.send(KeyboardInput {
                key_code: bevy_keycode,
                logical_key: Key::Character(character.to_string().into()),
                state: ButtonState::Pressed,
                repeat: false,
                window: window_entity,
            });
            bevy_input.send(KeyboardInput {
                key_code: bevy_keycode,
                logical_key: Key::Character(character.to_string().into()),
                state: ButtonState::Released,
                repeat: false,
                window: window_entity,
            });
        };

        match kc.code {
            crossterm::event::KeyCode::Char('w') => send_key('w', KeyCode::KeyW),
            crossterm::event::KeyCode::Char('d') => send_key('d', KeyCode::KeyD),
            crossterm::event::KeyCode::Char('s') => send_key('s', KeyCode::KeyS),
            crossterm::event::KeyCode::Char('a') => send_key('a', KeyCode::KeyA),
            crossterm::event::KeyCode::Char('q') => send_key('q', KeyCode::KeyQ),
            crossterm::event::KeyCode::Char('e') => send_key('e', KeyCode::KeyE),
            crossterm::event::KeyCode::Char('o') => send_key('o', KeyCode::KeyO),
            crossterm::event::KeyCode::Char('p') => send_key('p', KeyCode::KeyP),
            crossterm::event::KeyCode::Tab => send_key('t', KeyCode::Tab),
            crossterm::event::KeyCode::Esc => send_key('x', KeyCode::Escape),
            _ => {}
        };
    }
}

#[derive(Component)]
pub struct DummyWindow;
