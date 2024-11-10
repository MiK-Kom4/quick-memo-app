use eframe::egui;

const TOOLBAR_HEIGHT: f32 = 32.0;
const BUTTON_WIDTH: f32 = 80.0;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Quick Memo",
        options,
        Box::new(|_cc| Ok(Box::new(QuickMemoApp::default())))
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
        // ダークモードを設定
        ctx.set_visuals(egui::Visuals::dark());
        
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
                
                ui.separator();
                
                let available_height = ui.available_height() - TOOLBAR_HEIGHT;
                
                // メモ入力エリア
                ui.add_sized(
                    egui::vec2(ui.available_width(), available_height),
                    egui::TextEdit::multiline(&mut self.content).hint_text("input memo..."),
                );
                
                // 下部フッター
                ui.add_space(4.0);
                ui.allocate_space(egui::vec2(ui.available_width(), 1.0));
                
                // ツールバー
                ui.horizontal(|ui| {
                    let total_spacing = ui.available_width() - (BUTTON_WIDTH * 3.0);
                    let spacing = total_spacing / 4.0; // 両端とボタン間に均等に配置

                    // 左の余白
                    ui.add_space(spacing);

                    // New button
                    if ui.add_sized(
                        egui::vec2(BUTTON_WIDTH, TOOLBAR_HEIGHT - 8.0),
                        egui::Button::new("new")
                    ).clicked() {
                        self.title = String::from("non title");
                        self.content.clear();
                    }

                    // ボタン間の余白
                    ui.add_space(spacing);

                    // List button
                    if ui.add_sized(
                        egui::vec2(BUTTON_WIDTH, TOOLBAR_HEIGHT - 8.0),
                        egui::Button::new("list")
                    ).clicked() {
                        println!("メモ一覧ボタンがクリックされました");
                    }

                    // ボタン間の余白
                    ui.add_space(spacing);

                    // Delete button
                    if ui.add_sized(
                        egui::vec2(BUTTON_WIDTH, TOOLBAR_HEIGHT - 8.0),
                        egui::Button::new("delete")
                    ).clicked() {
                        println!("delete!");
                    }

                    // 右の余白
                    ui.add_space(spacing);
                });
            });
        });
    }
}
