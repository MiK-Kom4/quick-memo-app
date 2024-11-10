use crate::components::{editor::Editor, memo_list::MemoList, toolbar::Toolbar};
use crate::config::constants::TOOLBAR_HEIGHT;
use crate::state::AppScreen;
use crate::storage::auto_save::AutoSave;
use eframe::egui;

pub struct QuickMemoApp {
    editor: Editor,
    toolbar: Toolbar,
    auto_save: AutoSave,
    last_content: String,
    current_screen: AppScreen,
    memo_list: MemoList,
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
        let memo_list = MemoList::new();
        let mut toolbar = Toolbar::new();

        // MemoListへの遷移コールバック（この時点では空）
        toolbar.on_list = Some(Box::new(|| {
            println!("List button clicked");
        }));

        Self {
            editor,
            toolbar,
            auto_save,
            last_content,
            current_screen: AppScreen::Editor,
            memo_list,
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
        // クロージャで画面遷移を行うためのセットアップ
        let app_ptr = self as *mut QuickMemoApp;

        // 安全でないブロックを最小限に抑える
        unsafe {
            self.toolbar.on_list = Some(Box::new(move || {
                (*app_ptr).current_screen = AppScreen::MemoList;
            }));

            self.memo_list.on_back = Some(Box::new(move || {
                (*app_ptr).current_screen = AppScreen::Editor;
            }));
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
