use crate::components::{editor::Editor, memo_list::MemoList, toolbar::Toolbar};
use crate::config::constants::TOOLBAR_HEIGHT;
use crate::state::AppScreen;
use crate::storage::auto_save::AutoSave;
use eframe::egui;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct QuickMemoApp {
    editor: Editor,
    toolbar: Toolbar,
    auto_save: AutoSave,
    last_content: String,
    current_screen: AppScreen,
    memo_list: MemoList,
    should_switch_to_list: Arc<AtomicBool>,
    should_switch_to_editor: Arc<AtomicBool>,
}

impl QuickMemoApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let auto_save = AutoSave::new(2);
        let saved_content = auto_save.load_last_save();

        let editor = if let Some(content) = saved_content {
            Editor::from_content(content)
        } else {
            Editor::new("non title".to_string())
        };

        let last_content = editor.get_save_content();
        let should_switch_to_list = Arc::new(AtomicBool::new(false));
        let should_switch_to_editor = Arc::new(AtomicBool::new(false));

        let should_switch_list_clone = should_switch_to_list.clone();
        let should_switch_editor_clone = should_switch_to_editor.clone();

        let mut toolbar = Toolbar::new();
        toolbar.on_list = Some(Box::new(move || {
            should_switch_list_clone.store(true, Ordering::SeqCst);
        }));

        let mut memo_list = MemoList::new();
        memo_list.on_back = Some(Box::new(move || {
            should_switch_editor_clone.store(true, Ordering::SeqCst);
        }));

        Self {
            editor,
            toolbar,
            auto_save,
            last_content,
            current_screen: AppScreen::Editor,
            memo_list,
            should_switch_to_list,
            should_switch_to_editor,
        }
    }

    // check_changesメソッドを追加
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
        // 画面遷移のチェック
        if self.should_switch_to_list.load(Ordering::SeqCst) {
            self.current_screen = AppScreen::MemoList;
            self.should_switch_to_list.store(false, Ordering::SeqCst);
        }

        if self.should_switch_to_editor.load(Ordering::SeqCst) {
            self.current_screen = AppScreen::Editor;
            self.should_switch_to_editor.store(false, Ordering::SeqCst);
        }

        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| match self.current_screen {
            AppScreen::Editor => {
                ui.vertical(|ui| {
                    let available_height = ui.available_height() - TOOLBAR_HEIGHT;

                    ui.allocate_ui(egui::vec2(ui.available_width(), available_height), |ui| {
                        self.editor.ui(ui);
                    });

                    self.check_changes();
                    self.auto_save
                        .check_and_save(&self.editor.get_save_content());

                    ui.add_space(4.0);
                    self.toolbar.ui(ui);
                });
            }
            AppScreen::MemoList => {
                self.memo_list.ui(ui);
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.auto_save.save(&self.editor.get_save_content());
    }
}
