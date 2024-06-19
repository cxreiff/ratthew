use std::io;
use std::time::Duration;

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
use bevy_ratatui_render::{RatatuiRenderContext, RatatuiRenderPlugin};
use camera::{move_camera_system, KeysDown, ViewCameraPlugin};
use collisions::collisions_system;
use crossterm::event::{KeyCode, KeyEventKind, KeyEventState, KeyModifiers};
use dotenv::dotenv;
use loading::{GameStates, LoadingPlugin};
use ratatui::layout::Alignment;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::widgets::Block;

mod camera;
mod collisions;
mod cube;
mod loading;
mod spawning;

#[derive(Component)]
pub struct Cube;

#[derive(Resource, Default)]
pub struct Flags {
    debug: bool,
}

fn main() {
    dotenv().ok();

    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0)),
            FrameTimeDiagnosticsPlugin,
            RatatuiPlugins::default(),
            RatatuiRenderPlugin::new("main", (640, 400)),
            ViewCameraPlugin,
            LoadingPlugin,
        ))
        .insert_resource(Flags::default())
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_systems(Update, draw_scene.map(error))
        .add_systems(Update, handle_keys.map(error))
        .add_systems(
            Update,
            collisions_system
                .run_if(in_state(GameStates::Playing))
                .after(move_camera_system),
        )
        .add_systems(Update, passthrough_keyboard_events)
        .run();
}

fn draw_scene(
    mut ratatui: ResMut<RatatuiContext>,
    ratatui_render: Res<RatatuiRenderContext>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
    kitty_enabled: Option<Res<KittyEnabled>>,
) -> io::Result<()> {
    ratatui.draw(|frame| {
        let mut block = Block::bordered()
            .bg(ratatui::style::Color::Rgb(0, 0, 0))
            .border_style(Style::default().bg(ratatui::style::Color::Rgb(0, 0, 0)));
        let inner = block.inner(frame.size());

        if flags.debug {
            block = block
                .title_top(format!(
                    "[kitty protocol: {}]",
                    if kitty_enabled.is_some() {
                        "enabled"
                    } else {
                        "disabled"
                    }
                ))
                .title_alignment(Alignment::Right);

            if let Some(value) = diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|fps| fps.smoothed())
            {
                block = block
                    .title_top(format!("[fps: {value:.0}]"))
                    .title_alignment(Alignment::Right);
            }
        }

        frame.render_widget(block, frame.size());

        if let Some(widget) = ratatui_render.widget("main") {
            frame.render_widget(widget, inner);
        }
    })?;

    Ok(())
}

pub fn handle_keys(
    mut ratatui_events: EventReader<KeyEvent>,
    mut exit: EventWriter<AppExit>,
    mut flags: ResMut<Flags>,
    mut keys_down: ResMut<KeysDown>,
) -> io::Result<()> {
    for KeyEvent(key_event) in ratatui_events.read() {
        match key_event.kind {
            KeyEventKind::Press => match key_event.code {
                KeyCode::Char('q') => {
                    exit.send(AppExit);
                }

                KeyCode::Char('d') => {
                    flags.debug = !flags.debug;
                }

                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Char(' ') => {
                    if !keys_down.contains(&key_event.code) {
                        keys_down.push(key_event.code);
                    }
                }

                _ => {}
            },
            KeyEventKind::Release => match key_event.code {
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Char(' ') => {
                    keys_down.retain(|key| *key != key_event.code);
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
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
