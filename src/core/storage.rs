use crate::core::dir;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

static STORAGE_FILENAME: &str = "storage.yml";

lazy_static! {
    pub static ref STORAGE: Mutex<Storage> =
        Mutex::new(Storage::load().expect("failed to load storage"));
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Storage {
    pub analytics_uuid: String,
    pub last_analytics_tick: SystemTime,
}

impl Storage {
    fn load() -> Result<Storage> {
        let app_dir = dir::get_app_dir()?;
        let storage_path = app_dir.join(STORAGE_FILENAME);
        if storage_path.exists() {
            let storage_str = fs::read_to_string(storage_path)?;
            serde_yaml::from_str(&storage_str).context("failed to parse the storage file")
        } else {
            let storage = Storage {
                analytics_uuid: Uuid::new_v4().to_string(),
                last_analytics_tick: UNIX_EPOCH,
            };

            let storage_str = serde_yaml::to_string(&storage)?;
            fs::write(storage_path, storage_str).context("failed to save the storage file")?;

            Ok(storage)
        }
    }
}
