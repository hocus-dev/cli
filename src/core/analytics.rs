use crate::core::config::CONFIG;
use crate::core::storage::STORAGE;
use anyhow::Result;
use std::env::current_exe;
use std::process::{Command, Stdio};
use std::time::Duration;

pub fn spawn_analytics_process() {
    let wrapper = || -> Result<()> {
        if !CONFIG.analytics_enabled {
            return Ok(());
        }

        let storage = STORAGE.lock().unwrap();
        let min_time_between_ticks = Duration::from_secs(60 * 60 * 12); // 12 hours
        if storage.last_analytics_tick.elapsed()? < min_time_between_ticks {
            return Ok(());
        }

        Command::new(current_exe()?)
            .arg("analytics")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        Ok(())
    };

    match wrapper() {
        Ok(_) => (),
        Err(_) => (),
    }
}
