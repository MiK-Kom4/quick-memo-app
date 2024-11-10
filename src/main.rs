use eframe::egui;

const TOOLBAR_HEIGHT: f32 = 32.0;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Quick Memo",
        options,
        Box::new(|_cc| Ok(Box::new(QuickMemoApp::default()))),
    )
}

struct QuickMemoApp {
    title: String,
    content: String,
}

impl Default for QuickMemoApp {
    fn default() -> Self {
        Self {
            title: String::from("non title"),
            content: String::new(),
        }
    }
}

impl eframe::App for QuickMemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // タイトル編集エリア
                ui.horizontal(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);

                    ui.add(
                        egui::TextEdit::singleline(&mut self.title)
                            .desired_width(ui.available_width())
                            .hint_text("input title..."),
                    );
                });

                // 区切り線
                ui.separator();

                // 残りの高さを計算
                let available_height = ui.available_height() - TOOLBAR_HEIGHT;

                // メモ入力エリア
                ui.add_sized(
                    egui::vec2(ui.available_width(), available_height),
                    egui::TextEdit::multiline(&mut self.content).hint_text("input memo..."),
                );

                // 下部フッター
                ui.allocate_space(egui::vec2(ui.available_width(), TOOLBAR_HEIGHT));
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("new").clicked() {
                            self.title = String::from("non title");
                            self.content.clear();
                        }
                    });
                });
            });
        });
    }
}
