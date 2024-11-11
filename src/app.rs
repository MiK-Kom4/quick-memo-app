use crate::components::{editor::Editor, memo_list::MemoList, toolbar::Toolbar};
use crate::config::constants::TOOLBAR_HEIGHT;
use crate::models::memo::Memo;
use crate::state::AppScreen;
use crate::storage::auto_save::AutoSave;
use crate::storage::memo_storage::MemoStorage;
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
    memo_storage: MemoStorage,
    should_switch_to_list: Arc<AtomicBool>,
    should_switch_to_editor: Arc<AtomicBool>,
    should_create_new_memo: Arc<AtomicBool>, // 新規メモ作成フラグを追加
    current_memo: Memo,
}

impl QuickMemoApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let auto_save = AutoSave::new(2);
        let saved_content = auto_save.load_last_save();
        let memo_storage = MemoStorage::new();

        let editor = if let Some(content) = saved_content {
            Editor::from_content(content)
        } else {
            Editor::new("non title".to_string())
        };

        let last_content = editor.get_save_content();
        let should_switch_to_list = Arc::new(AtomicBool::new(false));
        let should_switch_to_editor = Arc::new(AtomicBool::new(false));
        let should_create_new_memo = Arc::new(AtomicBool::new(false));

        let should_switch_list_clone = should_switch_to_list.clone();
        let should_switch_editor_clone = should_switch_to_editor.clone();
        let should_create_new_memo_clone = should_create_new_memo.clone();

        let mut toolbar = Toolbar::new();

        // メモ一覧ボタンのハンドラ
        toolbar.on_list = Some(Box::new(move || {
            should_switch_list_clone.store(true, Ordering::SeqCst);
        }));

        // 新規メモボタンのハンドラ
        toolbar.on_new = Some(Box::new(move || {
            should_create_new_memo_clone.store(true, Ordering::SeqCst);
        }));

        let mut memo_list = MemoList::new();
        memo_list.on_back = Some(Box::new(move || {
            should_switch_editor_clone.store(true, Ordering::SeqCst);
        }));
        memo_list.memos = memo_storage.load_all_memos();

        // 初期メモの作成
        let mut current_memo = Memo::new();
        current_memo.title = editor.title.clone();
        current_memo.content = editor.content.clone();

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
            should_create_new_memo,
            current_memo,
        }
    }

    fn check_changes(&mut self) {
        let current_content = self.editor.get_save_content();
        if current_content != self.last_content {
            self.auto_save.mark_dirty();
            self.last_content = current_content.clone();

            // 現在のメモを更新
            self.current_memo
                .update_content(self.editor.title.clone(), self.editor.content.clone());

            // 現在のメモを保存
            if let Err(e) = self.memo_storage.save_memo(&mut self.current_memo) {
                eprintln!("Failed to save memo: {}", e);
            }
        }
    }

    fn create_new_memo(&mut self) {
        // 現在のメモを保存
        self.memo_storage.save_memo(&mut self.current_memo).ok();

        // 新しいメモを作成
        self.current_memo = Memo::new();
        self.editor.title = "non title".to_string();
        self.editor.content.clear();
        self.last_content = self.editor.get_save_content();

        // メモリストを更新
        self.memo_list.memos = self.memo_storage.load_all_memos();
    }
}

impl eframe::App for QuickMemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 新規メモ作成のチェック
        if self.should_create_new_memo.load(Ordering::SeqCst) {
            self.create_new_memo();
            self.should_create_new_memo.store(false, Ordering::SeqCst);
        }

        // 画面遷移のチェック
        if self.should_switch_to_list.load(Ordering::SeqCst) {
            self.current_screen = AppScreen::MemoList;
            self.should_switch_to_list.store(false, Ordering::SeqCst);
            // リスト表示前に現在のメモを保存し、リストを更新
            self.memo_storage.save_memo(&mut self.current_memo).ok();
            self.memo_list.memos = self.memo_storage.load_all_memos();
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
        // 終了時に現在のメモを保存
        self.memo_storage.save_memo(&mut self.current_memo).ok();
        self.auto_save.save(&self.editor.get_save_content());
    }
}
