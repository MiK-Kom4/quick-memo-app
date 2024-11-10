use crate::components::{editor::Editor, memo_list::MemoList, toolbar::Toolbar};
use crate::config::constants::TOOLBAR_HEIGHT;
use crate::models::memo::Memo;
use crate::state::AppScreen;
use crate::storage::auto_save::AutoSave;
use crate::storage::memo_storage::MemoStorage;
use eframe::egui;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub struct QuickMemoApp {
    editor: Editor,
    toolbar: Toolbar,
    auto_save: AutoSave,
    last_content: String,
    current_screen: AppScreen,
    memo_list: MemoList,
    memo_storage: MemoStorage,
    should_switch_to_list: Arc<AtomicBool>,
    should_switch_to_editor: Arc<AtomicBool>,
    should_update_memo_list: Arc<AtomicBool>, // メモリスト更新フラグを追加
    current_memo: Memo,
}

impl QuickMemoApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let auto_save = AutoSave::new(2);
        let saved_content = auto_save.load_last_save();
        let memo_storage = MemoStorage::new();
        let current_memo = Memo::new();

        let editor = if let Some(content) = saved_content {
            Editor::from_content(content)
        } else {
            Editor::new("non title".to_string())
        };

        let last_content = editor.get_save_content();
        let should_switch_to_list = Arc::new(AtomicBool::new(false));
        let should_switch_to_editor = Arc::new(AtomicBool::new(false));
        let should_update_memo_list = Arc::new(AtomicBool::new(false));

        let should_switch_list_clone = should_switch_to_list.clone();
        let should_switch_editor_clone = should_switch_to_editor.clone();
        let should_update_list_clone = should_update_memo_list.clone();

        let editor_clone = editor.clone();
        let mut toolbar = Toolbar::new();

        let memo_storage_clone = Arc::new(Mutex::new(memo_storage.clone()));
        toolbar.on_new = Some(Box::new(move || {
            if let Ok(mut storage) = memo_storage_clone.lock() {
                let memo = Memo::new(editor_clone.title.clone(), editor_clone.content.clone());
                if let Err(e) = storage.save_memo(&memo) {
                    eprintln!("Failed to save memo: {}", e);
                } else {
                    // メモ保存成功時にメモリストの更新フラグを設定
                    should_update_list_clone.store(true, Ordering::SeqCst);
                }
            }
        }));

        toolbar.on_list = Some(Box::new(move || {
            should_switch_list_clone.store(true, Ordering::SeqCst);
        }));

        let mut memo_list = MemoList::new();
        memo_list.on_back = Some(Box::new(move || {
            should_switch_editor_clone.store(true, Ordering::SeqCst);
        }));
        memo_list.memos = memo_storage.load_all_memos();

        let storage_clone = Arc::new(Mutex::new(memo_storage.clone()));
        let should_update_list_clone = should_update_memo_list.clone();
        toolbar.on_new = Some(Box::new(move || {
            if let Ok(storage) = storage_clone.lock() {
                // 新しいメモを作成
                let new_memo = Memo::new();
                // メモリストの更新をトリガー
                should_update_list_clone.store(true, Ordering::SeqCst);
            }
        }));

        Self {
            editor,
            toolbar,
            auto_save,
            last_content,
            current_screen: AppScreen::Editor,
            memo_list,
            memo_storage,
            should_switch_to_list,
            should_switch_to_editor,
            should_update_memo_list,
            current_memo,
        }
    }

    fn check_changes(&mut self) {
        let current_content = self.editor.get_save_content();
        if current_content != self.last_content {
            // 自動保存の処理
            self.auto_save.mark_dirty();
            self.last_content = current_content.clone();

            // 現在のメモを更新
            self.current_memo
                .update_content(self.editor.title.clone(), self.editor.content.clone());

            // 現在のメモを保存（上書き）
            if let Err(e) = self.memo_storage.save_memo(&mut self.current_memo) {
                eprintln!("Failed to save memo: {}", e);
            }
        }
    }

    fn update_memo_list(&mut self) {
        self.memo_list.memos = self.memo_storage.load_all_memos();
    }

    fn load_memo(&mut self, memo: Memo) {
        self.current_memo = memo;
        self.editor.title = self.current_memo.title.clone();
        self.editor.content = self.current_memo.content.clone();
        self.last_content = self.editor.get_save_content();
    }
}

impl eframe::App for QuickMemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // メモリストの更新チェック
        if self.should_update_memo_list.load(Ordering::SeqCst) {
            self.update_memo_list();
            self.should_update_memo_list.store(false, Ordering::SeqCst);
        }

        // 画面遷移のチェック
        if self.should_switch_to_list.load(Ordering::SeqCst) {
            self.current_screen = AppScreen::MemoList;
            self.should_switch_to_list.store(false, Ordering::SeqCst);
            self.update_memo_list(); // リスト表示時にも更新
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
