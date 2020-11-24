use crate::core::dir::get_app_dir;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

lazy_static! {
    static ref STORAGE_PATH: PathBuf = get_app_dir().unwrap().join("storage.yml");
}

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
        if STORAGE_PATH.exists() {
            let storage_str = fs::read_to_string(STORAGE_PATH.as_path())?;
            serde_yaml::from_str(&storage_str).context("failed to parse the storage file")
        } else {
            let storage = Storage {
                analytics_uuid: Uuid::new_v4().to_string(),
                last_analytics_tick: UNIX_EPOCH,
            };
            storage.save()?;
            Ok(storage)
        }
    }

    pub fn save(&self) -> Result<()> {
        let state_str = serde_yaml::to_string(self)?;
        fs::write(STORAGE_PATH.as_path(), state_str).context("failed to write to storage")
    }
}
