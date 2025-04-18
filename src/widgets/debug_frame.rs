use bevy::diagnostic::{
    DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy_ratatui::kitty::KittyEnabled;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::Block,
    Frame,
};
use tui_logger::TuiLoggerWidget;

use crate::{
    grid::{GridDirection, GridPosition},
    Flags,
};

pub fn debug_frame(
    frame: &mut Frame,
    flags: &Flags,
    diagnostics: &DiagnosticsStore,
    kitty_enabled: Option<&KittyEnabled>,
    player: Option<(&GridPosition, &GridDirection)>,
    show_log_panel: bool,
) -> ratatui::layout::Rect {
    let layout = Layout::new(
        Direction::Vertical,
        [Constraint::Fill(1), Constraint::Length(3)],
    )
    .split(frame.area());

    let block = Block::bordered()
        .bg(ratatui::style::Color::Rgb(0, 0, 0))
        .border_style(Style::default().bg(ratatui::style::Color::Black))
        .title_alignment(Alignment::Center);

    let controls = Line::from("[esc to quit] [tab to debug]")
        .centered()
        .bg(Color::Black);
    let controls_block = Block::bordered().bg(Color::Black);
    frame.render_widget(controls, controls_block.inner(layout[1]));
    frame.render_widget(controls_block, layout[1]);

    if flags.debug {
        let debug_layout = Layout::new(
            Direction::Vertical,
            if show_log_panel {
                &[
                    Constraint::Fill(2),
                    Constraint::Length(3),
                    Constraint::Fill(1),
                ]
            } else {
                &[Constraint::Fill(1), Constraint::Length(3)] as &[Constraint]
            },
        )
        .split(layout[0]);

        let mut debug_text = vec![];

        debug_text.push(format!(
            "[kitty protocol: {}]",
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
            debug_text.push(format!("[entities: {value}]"));
        }

        if let Some((position, direction)) = player {
            debug_text.push(format!(
                "[xyz: {}, {}, {}]",
                position.x, position.y, position.z
            ));

            debug_text.push(format!(
                "[direction: {}]",
                format!("{:?}", direction.0).to_lowercase()
            ));
        }

        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            debug_text.push(format!("[fps: {value:3.0}]"));
        }

        let inner = block.inner(debug_layout[0]);
        frame.render_widget(block, debug_layout[0]);

        let debug_block = Block::bordered().bg(Color::Black);
        frame.render_widget(
            Text::from(debug_text.join(" ")),
            debug_block.inner(debug_layout[1]),
        );
        frame.render_widget(debug_block, debug_layout[1]);

        if show_log_panel {
            frame.render_widget(
                TuiLoggerWidget::default()
                    .block(Block::bordered())
                    .style(Style::default().bg(ratatui::style::Color::Reset)),
                debug_layout[2],
            );
        }

        inner
    } else {
        let inner = block.inner(layout[0]);
        frame.render_widget(block, layout[0]);

        inner
    }
}
