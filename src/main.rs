mod app;
mod components;
mod config;
mod storage;

use app::QuickMemoApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Quick Memo",
        options,
        Box::new(|cc| Ok(Box::new(QuickMemoApp::new(cc)))),
    )
}
