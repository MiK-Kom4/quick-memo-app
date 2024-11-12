use eframe::egui;

pub struct Icons;

impl Icons {
    pub fn render_icon(ui: &mut egui::Ui, icon: &str, size: f32) -> egui::Response {
        let desired_size = egui::vec2(size, size);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let color = visuals.text_color();

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                match icon {
                    "search" => "🔍",
                    "back" => "←",
                    "add" => "+",
                    "list" => "≡",
                    "delete" => "×",
                    _ => "•",
                },
                egui::FontId::proportional(size),
                color,
            );
        }

        response
    }

    pub fn search(ui: &mut egui::Ui, size: f32) -> egui::Response {
        Self::render_icon(ui, "search", size)
    }

    pub fn back(ui: &mut egui::Ui, size: f32) -> egui::Response {
        Self::render_icon(ui, "back", size)
    }
}
