use crate::components::icons::Icons;
use crate::models::memo::Memo;
use eframe::egui;

pub struct MemoList {
    pub on_back: Option<Box<dyn Fn()>>,
    pub on_select: Option<Box<dyn Fn(usize)>>,
    pub memos: Vec<Memo>,
    pub selected_index: Option<usize>,
    search_query: String,
}

impl MemoList {
    pub fn new() -> Self {
        Self {
            on_back: None,
            on_select: None,
            memos: Vec::new(),
            selected_index: None,
            search_query: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // ヘッダー部分
            ui.horizontal(|ui| {
                if ui
                    .add_sized(egui::vec2(24.0, 24.0), |ui: &mut egui::Ui| {
                        Icons::back(ui, 16.0)
                    })
                    .clicked()
                {
                    if let Some(on_back) = &self.on_back {
                        on_back();
                    }
                }
                ui.heading("メモ一覧");
            });
            ui.separator();

            // 検索バー
            ui.horizontal(|ui| {
                Icons::search(ui, 16.0);
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

                    // メモアイテムのレイアウトを作成
                    ui.push_id(index, |ui| {
                        let item_height = 50.0; // メモアイテムの高さを調整
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(ui.available_width(), item_height),
                            egui::Sense::click(),
                        );

                        // 背景とインタラクション
                        if response.hovered() {
                            ui.ctx()
                                .output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                            let hover_color = ui.style().visuals.widgets.hovered.bg_fill;
                            ui.painter().rect_filled(rect, 0.0, hover_color);
                        }

                        // メモの内容を描画
                        let text_margin = 8.0;
                        let text_rect = rect.shrink(text_margin);
                        let title_height = 24.0;

                        // タイトル
                        ui.painter().text(
                            text_rect.min,
                            egui::Align2::LEFT_TOP,
                            &memo.title,
                            egui::FontId::proportional(16.0),
                            ui.style().visuals.text_color(),
                        );

                        // 日付
                        ui.painter().text(
                            text_rect.min + egui::vec2(0.0, title_height),
                            egui::Align2::LEFT_TOP,
                            memo.display_date(),
                            egui::FontId::proportional(12.0),
                            ui.style().visuals.weak_text_color(),
                        );

                        if response.clicked() {
                            self.selected_index = Some(index);
                            if let Some(on_select) = &self.on_select {
                                on_select(index);
                            }
                        }

                        if index < self.memos.len() - 1 {
                            ui.add_space(4.0);
                            ui.separator();
                            ui.add_space(4.0);
                        }
                    });
                }
            });
        });
    }
}
