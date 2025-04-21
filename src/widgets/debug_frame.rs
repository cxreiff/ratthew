use bevy::diagnostic::{
    DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy_persistent::Persistent;
use bevy_ratatui::kitty::KittyEnabled;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Padding},
    Frame,
};
use tui_logger::TuiLoggerWidget;

use crate::{
    camera::PlayerPersist,
    config::{PLAYER_STARTING_DIRECTION, PLAYER_STARTING_POSITION},
    grid::{GridDirection, GridPosition},
    Flags,
};

pub fn debug_frame(
    frame: &mut Frame,
    flags: &Flags,
    diagnostics: &DiagnosticsStore,
    kitty_enabled: Option<&KittyEnabled>,
    player: Option<(&GridPosition, &GridDirection)>,
    persist: &Persistent<PlayerPersist>,
    show_log_panel: bool,
) -> ratatui::layout::Rect {
    let main_block = Block::bordered()
        .bg(Color::Rgb(0, 0, 0))
        .border_style(Style::default().bg(ratatui::style::Color::Black));
    let undertab_block = Block::default()
        .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
        .padding(Padding::horizontal(2))
        .bg(Color::Black);

    let layout = Layout::new(
        Direction::Vertical,
        [Constraint::Fill(1), Constraint::Length(2)],
    )
    .split(frame.area());

    let name_string = "ratthew";
    let name_line = Line::from(name_string).centered();

    let mut settings_strings = vec![format!("sound: {}", if flags.sound { "ON" } else { "OFF" })];
    if !persist.position.eq(&PLAYER_STARTING_POSITION)
        || !persist.direction.0.eq(&PLAYER_STARTING_DIRECTION)
    {
        settings_strings.push(format!(
            "persist: {}, {}, {}, {}",
            persist.position.x,
            persist.position.y,
            persist.position.z,
            format!("{:?}", persist.direction.0).to_lowercase()
        ));
    }
    let settings_string = settings_strings.join("  |  ");
    let settings_line = Line::from(settings_string).centered();

    let controls_string = [
        "WASD to move",
        "Q/E to turn",
        "M to toggle sound",
        "ESC to quit",
        "TAB to debug",
    ]
    .join("  |  ");
    let controls_line = Line::from(controls_string.clone()).centered();

    let bottom_area = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length(name_string.len() as u16 + 8),
            Constraint::Fill(1),
            Constraint::Length(controls_string.len() as u16 + 8),
        ],
    )
    .split(layout[1]);
    frame.render_widget(name_line, undertab_block.inner(bottom_area[0]));
    frame.render_widget(undertab_block.clone(), bottom_area[0]);
    frame.render_widget(settings_line, undertab_block.inner(bottom_area[1]));
    frame.render_widget(undertab_block.clone(), bottom_area[1]);
    frame.render_widget(controls_line, undertab_block.inner(bottom_area[2]));
    frame.render_widget(undertab_block.clone(), bottom_area[2]);

    if flags.debug {
        let debug_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Fill(2),
                Constraint::Fill(if show_log_panel { 1 } else { 0 }),
                Constraint::Length(2),
            ],
        )
        .split(layout[0]);

        let inner = main_block.inner(debug_layout[0]);
        frame.render_widget(main_block, debug_layout[0]);

        if show_log_panel {
            frame.render_widget(
                TuiLoggerWidget::default()
                    .block(undertab_block.clone().padding(Padding::uniform(1)))
                    .style(Style::default().bg(ratatui::style::Color::Reset)),
                debug_layout[1],
            );
        }

        let mut debug_strings_left = vec![];
        let mut debug_strings_right = vec![];

        if let Some((position, direction)) = player {
            debug_strings_left.push(format!(
                "xyz: {}, {}, {}",
                position.x, position.y, position.z
            ));

            debug_strings_left.push(format!(
                "direction: {}",
                format!("{:?}", direction.0).to_lowercase()
            ));
        }

        debug_strings_right.push(format!(
            "kitty protocol: {}",
            if kitty_enabled.is_some() {
                "enabled"
            } else {
                "disabled"
            }
        ));

        if let Some(value) = diagnostics
            .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
            .and_then(|count| count.value())
        {
            debug_strings_right.push(format!("entities: {value}"));
        }

        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            debug_strings_right.push(format!("fps: {value:3.0}"));
        }

        let debug_string_left = debug_strings_left.join("  |  ");
        let debug_string_right = debug_strings_right.join("  |  ");

        let debug_line_layout = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Length(debug_string_left.len() as u16 + 8),
                Constraint::Fill(1),
                Constraint::Length(debug_string_right.len() as u16 + 8),
            ],
        )
        .split(debug_layout[2]);

        let debug_line_left = Line::from(debug_string_left).centered();
        let debug_line_right = Line::from(debug_string_right).centered();

        frame.render_widget(debug_line_left, undertab_block.inner(debug_line_layout[0]));
        frame.render_widget(undertab_block.clone(), debug_line_layout[0]);
        frame.render_widget(undertab_block.clone(), debug_line_layout[1]);
        frame.render_widget(debug_line_right, undertab_block.inner(debug_line_layout[2]));
        frame.render_widget(undertab_block.clone(), debug_line_layout[2]);

        inner
    } else {
        let inner = main_block.inner(layout[0]);
        frame.render_widget(main_block, layout[0]);

        inner
    }
}
