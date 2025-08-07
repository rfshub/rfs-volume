use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json::{json, Value};
use toml::de;

use crate::config;

#[derive(Debug, Deserialize)]
struct RfsConfig {
    volumes: BTreeMap<String, String>,
}

pub fn get_volumes_json() -> Value {
    let path = match config::get_rfs_toml_path() {
        Some(p) => p,
        None => return json!({ "error": "rfs.toml path not set" }),
    };

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return json!({ "error": "failed to read rfs.toml" }),
    };

    let parsed: RfsConfig = match de::from_str(&content) {
        Ok(cfg) => cfg,
        Err(e) => return json!({ "error": "failed to parse rfs.toml", "detail": e.to_string() }),
    };

    let volumes: Vec<Value> = parsed
        .volumes
        .into_iter()
        .filter_map(|(id, path)| {
            let pb = PathBuf::from(&path);
            if pb.is_dir() {
                Some(json!({ "id": id, "path": path }))
            } else {
                None
            }
        })
        .collect();

    json!({ "volumes": volumes })
}
