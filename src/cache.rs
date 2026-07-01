use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

const CACHE_FILE: &str = ".raudit-cache.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    version: u32,
    files: HashMap<String, u128>,
}

impl Cache {
    pub fn load() -> Self {
        let content = match std::fs::read_to_string(CACHE_FILE) {
            Ok(c) => c,
            Err(_) => return Self::empty(),
        };
        serde_json::from_str(&content).unwrap_or_else(|_| Self::empty())
    }

    pub fn save(&self) {
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(CACHE_FILE, content);
        }
    }

    fn empty() -> Self {
        Self {
            version: 1,
            files: HashMap::new(),
        }
    }

    pub fn is_unchanged_clean(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy().to_string();
        match self.files.get(&path_str) {
            Some(&cached_mtime) => cached_mtime == mtime_nanos(path).unwrap_or(0),
            None => false,
        }
    }

    pub fn mark_clean(&mut self, path: &Path) {
        if let Some(mtime) = mtime_nanos(path) {
            self.files.insert(path.to_string_lossy().to_string(), mtime);
        }
    }

    pub fn mark_dirty(&mut self, path: &Path) {
        self.files.remove(&path.to_string_lossy().to_string());
    }
}

fn mtime_nanos(path: &Path) -> Option<u128> {
    let meta = std::fs::metadata(path).ok()?;
    let mtime = meta.modified().ok()?;
    Some(mtime.duration_since(std::time::UNIX_EPOCH).ok()?.as_nanos())
}
