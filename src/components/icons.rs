use eframe::egui;
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct Icons {
    svg_paths: HashMap<&'static str, &'static str>,
}

static ICONS: OnceLock<Icons> = OnceLock::new();

impl Icons {
    fn get_or_init() -> &'static Icons {
        ICONS.get_or_init(|| {
            let mut svg_paths = HashMap::new();
            // assetsからSVGファイルを読み込む
            svg_paths.insert(
                "search",
                include_str!("../../assets/icons/search_search_symbol.svg"),
            );
            svg_paths.insert(
                "back",
                include_str!("../../assets/icons/arrow_back_arrow_back_symbol.svg"),
            );
            svg_paths.insert("new", include_str!("../../assets/icons/add_add_symbol.svg"));
            svg_paths.insert(
                "list",
                include_str!("../../assets/icons/menu_menu_symbol.svg"),
            );
            svg_paths.insert(
                "delete",
                include_str!("../../assets/icons/delete_delete_symbol.svg"),
            );
            Icons { svg_paths }
        })
    }

    fn render_icon(ui: &mut egui::Ui, icon_name: &str, size: f32) -> egui::Response {
        let desired_size = egui::vec2(size, size);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let color = visuals.text_color();

            // アイコンの代替テキストを表示
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                match icon_name {
                    "search" => "🔍",
                    "back" => "a",
                    "new" => "+",
                    "list" => "≡",
                    "delete" => "×",
                    _ => "•",
                },
                egui::FontId::proportional(size),
                color,
            );
        }

        response
    }

    pub fn search(ui: &mut egui::Ui, size: f32) -> egui::Response {
        Self::render_icon(ui, "search", size)
    }

    pub fn back(ui: &mut egui::Ui, size: f32) -> egui::Response {
        Self::render_icon(ui, "back", size)
    }

    pub fn new(ui: &mut egui::Ui, size: f32) -> egui::Response {
        Self::render_icon(ui, "new", size)
    }

    pub fn list(ui: &mut egui::Ui, size: f32) -> egui::Response {
        Self::render_icon(ui, "list", size)
    }

    pub fn delete(ui: &mut egui::Ui, size: f32) -> egui::Response {
        Self::render_icon(ui, "delete", size)
    }
}
