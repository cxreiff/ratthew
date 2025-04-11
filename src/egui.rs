use std::io;

use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_ratatui::kitty::KittyEnabled;
use bevy_ratatui_camera::RatatuiCameraWidget;
use egui::{CentralPanel, Frame};
use egui_ratatui::RataguiBackend;
use ratatui::Terminal;

use crate::{
    animations::GridAnimated,
    camera::{PlayerCamera, WorldCamera},
    grid::GridPosition,
    widgets::debug_frame::debug_frame,
    Flags,
};

#[allow(dead_code)]
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EguiPlugin)
        .init_resource::<EguiTerminal>()
        .add_systems(Update, draw_scene_system.map(bevy::utils::error));
}

#[derive(Resource, Deref, DerefMut)]
struct EguiTerminal(Terminal<RataguiBackend>);

impl Default for EguiTerminal {
    fn default() -> Self {
        let mut backend = RataguiBackend::new(512, 512);
        backend.set_font_size(12);
        Self(Terminal::new(backend).unwrap())
    }
}

#[allow(dead_code, clippy::too_many_arguments)]
fn draw_scene_system(
    mut commands: Commands,
    mut ratagui: ResMut<EguiTerminal>,
    mut egui: EguiContexts,
    player_widget: Query<&RatatuiCameraWidget, With<PlayerCamera>>,
    world_widget: Query<&RatatuiCameraWidget, With<WorldCamera>>,
    player: Query<(&GridPosition, &Transform, &GridAnimated), With<PlayerCamera>>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
    kitty_enabled: Option<Res<KittyEnabled>>,
) -> io::Result<()> {
    ratagui.draw(|frame| {
        let area = debug_frame(
            frame,
            &flags,
            &diagnostics,
            kitty_enabled.as_deref(),
            player.get_single().ok(),
            false,
        );

        if let Ok(w) = world_widget.get_single() {
            w.render_autoresize(area, frame.buffer_mut(), &mut commands);
        }
        if let Ok(w) = player_widget.get_single() {
            w.render_autoresize(area, frame.buffer_mut(), &mut commands);
        }
    })?;

    CentralPanel::default()
        .frame(Frame::none())
        .show(egui.ctx_mut(), |ui| {
            ui.add(ratagui.backend_mut());
        });

    Ok(())
}
