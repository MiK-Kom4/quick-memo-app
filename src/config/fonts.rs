use eframe::egui::{FontData, FontDefinitions, FontFamily};
use std::collections::BTreeMap;
use std::path::Path;

pub struct Fonts {
    pub definitions: FontDefinitions,
}

impl Fonts {
    pub fn new() -> Self {
        Self {
            definitions: Self::configure_fonts(),
        }
    }

    fn configure_fonts() -> FontDefinitions {
        let mut fonts = FontDefinitions::default();
        let mut font_data: BTreeMap<String, FontData> = BTreeMap::new();

        // macOS の一般的な日本語フォントパス
        let font_paths = vec![
            "/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc",
            "/System/Library/Fonts/ヒラギノ明朝 ProN.ttc",
        ];

        // 利用可能なフォントを読み込む
        for path in font_paths {
            if let Ok(font_bytes) = std::fs::read(Path::new(path)) {
                let font_name = Path::new(path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                font_data.insert(font_name.clone(), FontData::from_owned(font_bytes));
                println!("Loaded font: {}", font_name);
            } else {
                eprintln!("Failed to load font: {}", path);
            }
        }

        // フォントデータが空の場合はデフォルトのフォントのみを使用
        if !font_data.is_empty() {
            fonts.font_data = font_data;

            // Proportionalファミリーを設定
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .clear();
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .push("ヒラギノ角ゴシック W3.ttc".to_string());

            // Monospacedファミリーを設定
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .clear();
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .push("ヒラギノ角ゴシック W3.ttc".to_string());
        }

        fonts
    }
}

impl Default for Fonts {
    fn default() -> Self {
        Self::new()
    }
}
