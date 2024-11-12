use crate::models::memo::Memo;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct MemoStorage {
    storage_dir: PathBuf,
}

impl MemoStorage {
    pub fn new() -> Self {
        let storage_dir = Self::get_storage_dir();
        fs::create_dir_all(&storage_dir).expect("Failed to create memo directory");
        Self { storage_dir }
    }

    fn get_storage_dir() -> PathBuf {
        let home = dirs::home_dir().expect("Could not find home directory");
        home.join(".quick-memo").join("memos")
    }

    // メモを保存（新規または上書き）
    pub fn save_memo(&self, memo: &mut Memo) -> Result<(), std::io::Error> {
        // ファイルパスがない場合は新規作成
        let file_path = if let Some(path) = &memo.file_path {
            path.clone()
        } else {
            let file_name = format!("{}.json", memo.id);
            let path = self.storage_dir.join(file_name);
            memo.file_path = Some(path.clone());
            path
        };

        let content = serde_json::to_string_pretty(memo)?;
        fs::write(file_path, content)
    }

    // 全メモを読み込み
    pub fn load_all_memos(&self) -> Vec<Memo> {
        let mut memos: Vec<Memo> = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.storage_dir) {
            for entry in entries.flatten() {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(mut memo) = serde_json::from_str::<Memo>(&content) {
                        memo.file_path = Some(entry.path()); // パスを保存
                        memos.push(memo);
                    }
                }
            }
        }

        memos.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        memos
    }

    // メモを削除
    pub fn delete_memo(&self, memo: &Memo) -> Result<(), std::io::Error> {
        if let Some(path) = &memo.file_path {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}
