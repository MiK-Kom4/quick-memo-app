use eframe::egui;

pub struct Editor {
    pub title: String,
    pub content: String,
}

impl Editor {
    pub fn new(title: String) -> Self {
        Self {
            title,
            content: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
            ui.add(
                egui::TextEdit::singleline(&mut self.title)
                    .desired_width(ui.available_width())
                    .hint_text("input title..."),
            );
        });

        ui.separator();

        ui.add_sized(
            egui::vec2(ui.available_width(), ui.available_height()),
            egui::TextEdit::multiline(&mut self.content).hint_text("input memo..."),
        );
    }
}
