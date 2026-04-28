use egui::{Color32, RichText};

use crate::painting::Painting;

#[derive(serde::Serialize, serde::Deserialize, Default)]
enum AppState {
    #[default]
    Doodle,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TemplateApp {
    mode: AppState,
    painting: Painting,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            mode: Default::default(),
            painting: Default::default(),
        }
    }
}

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for TemplateApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {

        // ──────── TOP PANEL ────────
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            ui.horizontal(|ui| {

                // Left side: File menu
                ui.menu_button("File", |ui| {
                    if ui.button("Save as...").clicked() {
                        // TODO: save logic here
                        ui.close();
                    }
                });
            
                // Right side: Clock (digital)
                // ui.add_space pushes content, but for far-right alignment, we use:
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // TODO: digital clock goes here
                    // For now, a placeholder:
                    ui.label(RichText::new("12:34 PM").color(Color32::MAGENTA));
                });
            });
        });

        // ──────── BOTTOM PANEL (footer) ────────
        egui::Panel::bottom("bottom_panel").show_inside(ui, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // TODO: global_theme_preference_buttons go here
                // For now, a placeholder:
                ui.label(RichText::new("Theme: [ ]").color(Color32::LIGHT_BLUE));
            });
        });

        // ──────── CENTRAL PANEL ────────
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.painting.ui_control(ui);
            ui.separator();
            self.painting.ui_content(ui);
        });
    }
}

