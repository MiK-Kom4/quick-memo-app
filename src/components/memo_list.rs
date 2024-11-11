use crate::models::memo::Memo;
use eframe::egui;

pub struct MemoList {
    pub on_back: Option<Box<dyn Fn()>>,
    pub on_select: Option<Box<dyn Fn(usize)>>,
    pub memos: Vec<Memo>,
    search_query: String,
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
                ui.heading("メモ一覧");
            });

            ui.separator();
            // 検索バー
            ui.horizontal(|ui| {
                let search_icon = "🔍";
                ui.label(search_icon);
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_query)
                        .desired_width(ui.available_width())
                        .hint_text("検索..."),
                );
            });

            ui.separator();

            // メモ一覧
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (index, memo) in self.memos.iter().enumerate() {
                    // 検索フィルター
                    if !self.search_query.is_empty()
                        && !memo
                            .title
                            .to_lowercase()
                            .contains(&self.search_query.to_lowercase())
                        && !memo
                            .content
                            .to_lowercase()
                            .contains(&self.search_query.to_lowercase())
                    {
                        continue;
                    }

                    ui.vertical(|ui| {
                        ui.add_space(4.0);
                        let layout = egui::Layout::top_down(egui::Align::LEFT);
                        ui.with_layout(layout, |ui| {
                            let title_label =
                                egui::Label::new(egui::RichText::new(&memo.title).strong())
                                    .sense(egui::Sense::click());
                            if ui.add(title_label).clicked() {
                                if let Some(on_select) = &self.on_select {
                                    on_select(index);
                                }
                            }
                            ui.label(egui::RichText::new(memo.display_date()).weak().size(14.0));
                            // プレビュー表示（最初の100文字まで）
                            let preview = memo.content.chars().take(100).collect::<String>();
                            ui.label(egui::RichText::new(preview).weak().size(14.0));
                        });
                        ui.add_space(4.0);
                        if index < self.memos.len() - 1 {
                            ui.separator();
                        }
                    });
                }
            });
        });
    }
}
