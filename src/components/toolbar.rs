use crate::components::icons::Icons;
use crate::config::constants::{BUTTON_WIDTH, TOOLBAR_HEIGHT};
use eframe::egui;

#[derive(Default)]
pub struct Toolbar {
    pub on_new: Option<Box<dyn Fn()>>,
    pub on_list: Option<Box<dyn Fn()>>,
    pub on_delete: Option<Box<dyn Fn()>>,
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            on_new: None,
            on_list: None,
            on_delete: None,
        }
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let total_spacing = ui.available_width() - (BUTTON_WIDTH * 3.0);
            let spacing = total_spacing / 4.0;

            ui.add_space(spacing);

            // New button
            if ui
                .add_sized(
                    egui::vec2(BUTTON_WIDTH, TOOLBAR_HEIGHT - 8.0),
                    egui::Button::new("new"),
                )
                .clicked()
            {
                if let Some(on_new) = &self.on_new {
                    on_new();
                }
            }

            ui.add_space(spacing);

            // List button
            if ui
                .add_sized(
                    egui::vec2(BUTTON_WIDTH, TOOLBAR_HEIGHT - 8.0),
                    egui::Button::new("list"),
                )
                .clicked()
            {
                if let Some(on_list) = &self.on_list {
                    on_list();
                }
            }

            ui.add_space(spacing);

            // Delete button
            if ui
                .add_sized(
                    egui::vec2(BUTTON_WIDTH, TOOLBAR_HEIGHT - 8.0),
                    egui::Button::new("delete"),
                )
                .clicked()
            {
                if let Some(on_delete) = &self.on_delete {
                    on_delete();
                }
            }

            ui.add_space(spacing);
        });
    }
}
