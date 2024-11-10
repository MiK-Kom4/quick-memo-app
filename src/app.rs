use eframe::egui;
use crate::components::{toolbar::Toolbar, editor::Editor};
use crate::config::constants::TOOLBAR_HEIGHT;
use crate::storage::auto_save::AutoSave;

pub struct QuickMemoApp {
    editor: Editor,
    toolbar: Toolbar,
    auto_save: AutoSave,
    last_content: String,
}

impl QuickMemoApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // 保存データがあれば読み込む
        let auto_save = AutoSave::new(2); // 2秒間隔で保存
        let saved_content = auto_save.load_last_save();
        
        let editor = if let Some(content) = saved_content {
            Editor::from_content(content)
        } else {
            Editor::new("non title".to_string())
        };

        let last_content = editor.get_save_content();

        Self {
            editor,
            toolbar: Toolbar::new(),
            auto_save,
            last_content,
        }
    }

    fn check_changes(&mut self) {
        let current_content = self.editor.get_save_content();
        if current_content != self.last_content {
            self.auto_save.mark_dirty();
            self.last_content = current_content.clone();
        }
    }
}

impl eframe::App for QuickMemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                let available_height = ui.available_height() - TOOLBAR_HEIGHT;
                
                // エディター領域
                ui.allocate_ui(egui::vec2(ui.available_width(), available_height), |ui| {
                    self.editor.ui(ui);
                });
                
                // 内容の変更をチェック
                self.check_changes();
                
                // 必要に応じて保存
                self.auto_save.check_and_save(&self.editor.get_save_content());
                
                // ツールバー
                ui.add_space(4.0);
                self.toolbar.ui(ui);
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // 終了時に強制保存
        self.auto_save.save(&self.editor.get_save_content());
    }
}
