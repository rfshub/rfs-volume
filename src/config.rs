use std::path::{Path, PathBuf};
use std::sync::RwLock;

use once_cell::sync::Lazy;

static RFS_TOML_PATH: Lazy<RwLock<Option<PathBuf>>> = Lazy::new(|| RwLock::new(None));

pub fn set_rfs_toml_path<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    if path.exists() && path.is_file() {
        let mut lock = RFS_TOML_PATH.write().unwrap();
        *lock = Some(path.to_path_buf());
        true
    } else {
        false
    }
}

pub fn get_rfs_toml_path() -> Option<PathBuf> {
    RFS_TOML_PATH.read().unwrap().clone()
}

pub fn find_rfs_toml(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();

    loop {
        let candidate = current.join("rfs.toml");
        if candidate.exists() && candidate.is_file() {
            return Some(candidate);
        }

        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            break;
        }
    }

    None
}
