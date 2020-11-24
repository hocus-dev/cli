use crate::core::dir::get_app_dir;
use anyhow::Result;
use lazy_static::lazy_static;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::path::PathBuf;

lazy_static! {
    static ref LOG_FILE_PATH: PathBuf = get_app_dir().unwrap().join("hocus.log");
}

pub fn init_logging() -> Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} [{l}] - {m}\n",
        )))
        .build(LOG_FILE_PATH.as_path())?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::sync::Once;
    use uuid::Uuid;

    static INIT: Once = Once::new();

    fn init() {
        INIT.call_once(|| init_logging().unwrap());
    }

    #[test]
    fn log_appends_to_file() {
        init();
        let message = format!("test: log_appends_to_file - {}", Uuid::new_v4());
        log::info!("{}", message);
        let mut logfile = File::open(LOG_FILE_PATH.as_path()).unwrap();
        let mut logs = String::new();
        logfile.read_to_string(&mut logs).unwrap();
        message.find(&message).unwrap();
    }
}
