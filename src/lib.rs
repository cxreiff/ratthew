use std::io;
use std::time::Duration;

use animation::sword_bob_system;
use bevy::app::AppExit;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::utils::error;
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_ratatui::event::KeyEvent;
use bevy_ratatui::kitty::KittyEnabled;
use bevy_ratatui::terminal::RatatuiContext;
use bevy_ratatui::RatatuiPlugins;
use bevy_ratatui_camera::{RatatuiCameraPlugin, RatatuiCameraWidget};
use camera::{move_camera_system, KeysDown, PlayerCamera, ViewCameraPlugin, WorldCamera};
use collisions::collisions_system;
use crossterm::event::{KeyCode, KeyEventKind, KeyEventState, KeyModifiers};
use loading::{GameStates, LoadingPlugin};
use widgets::debug_frame::debug_frame;

mod animation;
mod camera;
mod collisions;
mod cube;
mod loading;
mod spawning;
mod widgets;

#[derive(Component)]
pub struct Cube;

#[derive(Resource, Default)]
pub struct Flags {
    debug: bool,
    supports_key_release: bool,
}

pub fn plugin(app: &mut App) {
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0)),
        FrameTimeDiagnosticsPlugin,
        RatatuiPlugins::default(),
        RatatuiCameraPlugin,
        ViewCameraPlugin,
        LoadingPlugin,
    ))
    .insert_resource(Flags::default())
    .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
    .add_systems(
        Update,
        draw_scene.map(error).run_if(in_state(GameStates::Playing)),
    )
    .add_systems(Update, handle_keyboard_system)
    .add_systems(Update, expire_keys_system)
    .add_systems(
        Update,
        (
            collisions_system.after(move_camera_system),
            sword_bob_system,
        )
            .run_if(in_state(GameStates::Playing)),
    )
    .add_systems(Update, passthrough_keyboard_events);
}

fn draw_scene(
    mut commands: Commands,
    mut ratatui: ResMut<RatatuiContext>,
    player_widget: Query<&RatatuiCameraWidget, With<PlayerCamera>>,
    world_widget: Query<&RatatuiCameraWidget, With<WorldCamera>>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
    kitty_enabled: Option<Res<KittyEnabled>>,
) -> io::Result<()> {
    ratatui.draw(|frame| {
        let area = debug_frame(frame, &flags, &diagnostics, kitty_enabled.as_deref());

        if let Ok(w) = world_widget.get_single() {
            w.render_autoresize(area, frame.buffer_mut(), &mut commands);
        }
        if let Ok(w) = player_widget.get_single() {
            w.render_autoresize(area, frame.buffer_mut(), &mut commands);
        }
    })?;

    Ok(())
}

fn handle_keyboard_system(
    mut ratatui_events: EventReader<KeyEvent>,
    mut exit: EventWriter<AppExit>,
    mut flags: ResMut<Flags>,
    mut keys_down: ResMut<KeysDown>,
) {
    for key_event in ratatui_events.read() {
        match key_event.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => match key_event.code {
                KeyCode::Char('q') => {
                    exit.send_default();
                }

                KeyCode::Char('d') => {
                    flags.debug = !flags.debug;
                }

                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Char(' ') => {
                    keys_down.entry(key_event.code).insert(0.5);
                }

                _ => {}
            },
            KeyEventKind::Release => match key_event.code {
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Char(' ') => {
                    keys_down.remove(&key_event.code);
                }
                _ => {}
            },
        }
    }
}

fn expire_keys_system(flags: Res<Flags>, mut keys_down: ResMut<KeysDown>, time: Res<Time>) {
    if flags.supports_key_release {
        return;
    }

    keys_down.iter_mut().for_each(|(_, remaining)| {
        *remaining -= time.delta_secs();
    });
    keys_down.retain(|_, remaining| *remaining > 0.);
}

fn passthrough_keyboard_events(
    mut read_keyboard: EventReader<KeyboardInput>,
    mut write_crossterm: EventWriter<KeyEvent>,
) {
    for ev in read_keyboard.read() {
        write_crossterm.send(KeyEvent(crossterm::event::KeyEvent {
            code: match ev.key_code {
                bevy::prelude::KeyCode::ArrowUp => KeyCode::Up,
                bevy::prelude::KeyCode::ArrowDown => KeyCode::Down,
                bevy::prelude::KeyCode::ArrowLeft => KeyCode::Left,
                bevy::prelude::KeyCode::ArrowRight => KeyCode::Right,
                bevy::prelude::KeyCode::Space => KeyCode::Char(' '),
                _ => KeyCode::Null,
            },
            kind: match ev.state {
                ButtonState::Pressed => KeyEventKind::Press,
                ButtonState::Released => KeyEventKind::Release,
            },
            state: KeyEventState::NONE,
            modifiers: KeyModifiers::NONE,
        }));
    }
}
