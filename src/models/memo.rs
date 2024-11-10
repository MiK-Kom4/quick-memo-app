use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Memo {
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Memo {
    #[allow(dead_code)] // この関数は後で使用するため、警告を抑制
    pub fn new(title: String, content: String) -> Self {
        let now = Local::now();
        Self {
            title,
            content,
            created_at: now,
            updated_at: now,
        }
    }

    #[allow(dead_code)] // この関数は後で使用するため、警告を抑制
    pub fn update_content(&mut self, title: String, content: String) {
        self.title = title;
        self.content = content;
        self.updated_at = Local::now();
    }

    pub fn display_date(&self) -> String {
        self.updated_at.format("%Y年%m月%d日 %H:%M:%S").to_string()
    }
}
