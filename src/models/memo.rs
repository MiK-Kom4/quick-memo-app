use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
pub struct Memo {
    pub id: String, // メモの一意のID
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    #[serde(skip)] // シリアライズ対象から除外
    pub file_path: Option<PathBuf>, // メモファイルのパス
}

impl Memo {
    pub fn new() -> Self {
        let now = Local::now();
        Self {
            id: now.timestamp_millis().to_string(), // 一意のIDを生成
            title: String::from("non title"),
            content: String::new(),
            created_at: now,
            updated_at: now,
            file_path: None,
        }
    }

    pub fn update_content(&mut self, title: String, content: String) {
        self.title = title;
        self.content = content;
        self.updated_at = Local::now();
    }

    pub fn display_date(&self) -> String {
        self.updated_at.format("%Y年%m月%d日 %H:%M:%S").to_string()
    }
}
