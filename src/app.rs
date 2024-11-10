use crate::components::{editor::Editor, toolbar::Toolbar};
use crate::config::constants::TOOLBAR_HEIGHT;
use eframe::egui;

pub struct QuickMemoApp {
    editor: Editor,
    toolbar: Toolbar,
}

impl QuickMemoApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            editor: Editor::new("non title".to_string()),
            toolbar: Toolbar::new(),
        };

        // コールバックの設定
        app.toolbar.on_new = Some(Box::new(move || {
            println!("New memo");
            // self.editor.clear(); // ここでは直接アクセスできないので別の方法が必要
        }));

        app.toolbar.on_delete = Some(Box::new(move || {
            println!("Delete memo");
        }));

        app
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

                // ツールバー
                ui.add_space(4.0);
                self.toolbar.ui(ui);
            });
        });
    }
}
