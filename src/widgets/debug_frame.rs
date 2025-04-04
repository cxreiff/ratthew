use bevy::diagnostic::{
    DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy_ratatui::kitty::KittyEnabled;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::Block,
    Frame,
};
use tui_logger::TuiLoggerWidget;

use crate::Flags;

pub fn debug_frame(
    frame: &mut Frame,
    flags: &Flags,
    diagnostics: &DiagnosticsStore,
    kitty_enabled: Option<&KittyEnabled>,
) -> ratatui::layout::Rect {
    let mut block = Block::bordered()
        .bg(ratatui::style::Color::Rgb(0, 0, 0))
        .border_style(Style::default().bg(ratatui::style::Color::Black))
        .title_bottom("[esc for quit]")
        .title_bottom("[tab for debug]")
        .title_alignment(Alignment::Center);

    if flags.debug {
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(66), Constraint::Fill(1)],
        )
        .split(frame.area());

        block = block.title_top(format!(
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
            block = block.title_top(format!("[entities: {value}]"));
        }

        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            block = block.title_top(format!("[fps: {value:.0}]"));
        }

        let inner = block.inner(layout[0]);
        frame.render_widget(block, layout[0]);
        frame.render_widget(
            TuiLoggerWidget::default()
                .block(Block::bordered())
                .style(Style::default().bg(ratatui::style::Color::Reset)),
            layout[1],
        );

        inner
    } else {
        let inner = block.inner(frame.area());
        frame.render_widget(block, frame.area());

        inner
    }
}
