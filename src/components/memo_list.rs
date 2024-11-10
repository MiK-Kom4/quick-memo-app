use eframe::egui;

pub struct MemoList {
    pub on_back: Option<Box<dyn Fn()>>,
}

impl MemoList {
    pub fn new() -> Self {
        Self { on_back: None }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // ヘッダー部分
            ui.horizontal(|ui| {
                if ui.button("← Back").clicked() {
                    if let Some(on_back) = &self.on_back {
                        on_back();
                    }
                }
                ui.heading("Memo List");
            });

            ui.separator();

            // ここにメモ一覧を表示（次のステップで実装）
            ui.label("Memo list will be shown here...");
        });
    }
}
