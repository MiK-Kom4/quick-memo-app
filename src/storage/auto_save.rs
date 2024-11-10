use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub struct AutoSave {
    last_save: Instant,
    save_interval: Duration,
    is_dirty: bool,
    file_path: PathBuf,
}

impl AutoSave {
    pub fn new(save_interval_secs: u64) -> Self {
        let file_path = Self::get_save_path();
        Self {
            last_save: Instant::now(),
            save_interval: Duration::from_secs(save_interval_secs),
            is_dirty: false,
            file_path,
        }
    }

    fn get_save_path() -> PathBuf {
        let home = dirs::home_dir().expect("Could not find home directory");
        home.join(".quick-memo").join("autosave.txt")
    }

    pub fn check_and_save(&mut self, content: &str) {
        if self.is_dirty && self.last_save.elapsed() >= self.save_interval {
            self.save(content);
        }
    }

    pub fn save(&mut self, content: &str) {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).ok();
        }
        if fs::write(&self.file_path, content).is_ok() {
            self.last_save = Instant::now();
            self.is_dirty = false;
        }
    }

    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }

    pub fn load_last_save(&self) -> Option<String> {
        fs::read_to_string(&self.file_path).ok()
    }
}
