use std::io;

use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_ratatui::kitty::KittyEnabled;
use bevy_ratatui_camera::RatatuiCameraWidget;
use egui::CentralPanel;
use egui_ratatui::RataguiBackend;
use ratatui::Terminal;

use crate::{
    camera::{ParticleCamera, PlayerCamera, WorldCamera},
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
        Self(Terminal::new(RataguiBackend::new(100, 100)).unwrap())
    }
}

#[allow(dead_code, clippy::too_many_arguments)]
fn draw_scene_system(
    mut commands: Commands,
    mut ratagui: ResMut<EguiTerminal>,
    mut egui: EguiContexts,
    player_widget: Query<&RatatuiCameraWidget, With<PlayerCamera>>,
    world_widget: Query<&RatatuiCameraWidget, With<WorldCamera>>,
    particle_widget: Query<&RatatuiCameraWidget, With<ParticleCamera>>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
    kitty_enabled: Option<Res<KittyEnabled>>,
) -> io::Result<()> {
    ratagui.draw(|frame| {
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

    CentralPanel::default().show(egui.ctx_mut(), |ui| {
        let ratagui_widget = ratagui.backend_mut();
        ui.add(ratagui_widget);
    });

    Ok(())
}
