use bevy::prelude::{App, EventWriter, OrthographicProjection, Plugin, Query, ResMut, Resource, Update};
use bevy_egui::{egui, EguiPlugin, EguiContexts};

use crate::world::BoardGenerateEvent;

pub struct UIPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .insert_resource::<UiState>(UiState { board_size: 4 })
            .add_systems(Update, setup_egui);
    }
}
#[derive(Default, Resource)]
pub struct UiState {
    pub board_size: u32,
}

fn setup_egui(
    mut egui_context: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut generate_board_events: EventWriter<BoardGenerateEvent>,
    mut projection_query: Query<&mut OrthographicProjection>,
) {
    egui::Window::new("Settings").show(egui_context.ctx_mut(), |ui| {
        ui.label("Board size");
        ui.add(egui::Slider::new(&mut ui_state.board_size, 0..=25));
        if ui.button("Generate board").clicked() {
            generate_board_events.send_default();
        }

        ui.separator();
        ui.label("Zoom");
        for mut projection in projection_query.iter_mut() {
            let mut log_scale = projection.scale.ln();
            ui.add(egui::Slider::new(&mut log_scale, 1.0..=4.0));
            projection.scale = log_scale.exp();
        }
    });
}
