use crate::models::memo::Memo;
use eframe::egui;

pub struct MemoList {
    pub on_back: Option<Box<dyn Fn()>>,
    pub on_select: Option<Box<dyn Fn(usize)>>,
    pub memos: Vec<Memo>,
    search_query: String, // 検索用の文字列を追加
}

impl MemoList {
    pub fn new() -> Self {
        Self {
            on_back: None,
            on_select: None,
            memos: Vec::new(),
            search_query: String::new(),
        }
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
                ui.heading("memo list");
            });

            ui.separator();

            // 検索バー
            ui.horizontal(|ui| {
                let search_icon = "🔍";
                ui.label(search_icon);
                // TextEditを直接追加
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_query)
                        .desired_width(ui.available_width())
                        .hint_text("search..."),
                );
            });

            ui.separator();

            // メモ一覧
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (index, memo) in self.memos.iter().enumerate() {
                    ui.vertical(|ui| {
                        if ui
                            .add(
                                egui::Label::new(format!(
                                    "{}\n{}",
                                    memo.title,
                                    memo.display_date()
                                ))
                                .sense(egui::Sense::click()),
                            )
                            .clicked()
                        {
                            if let Some(on_select) = &self.on_select {
                                on_select(index);
                            }
                        }
                        if index < self.memos.len() - 1 {
                            ui.separator();
                        }
                    });
                }
            });
        });
    }
}
