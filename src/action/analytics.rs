use super::Action;
use crate::cmd::analytics::AnalyticsCmd;
use crate::core::storage::STORAGE;
use anyhow::Result;

impl Action for AnalyticsCmd {
    fn run(&self) -> Result<()> {
        let client = reqwest::blocking::Client::new();

        let os_type = sys_info::os_type().unwrap_or("unknown".to_string());
        let os_release = sys_info::os_release().unwrap_or("unknown".to_string());
        let os_info = format!("{}-{}", os_type, os_release);

        // Reference:
        // https://developers.google.com/analytics/devguides/collection/protocol/v1/devguide
        // https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters
        let params = [
            ("v", "1"),
            ("t", "event"),
            ("tid", "UA-111652152-3"),
            ("cid", &STORAGE.lock().unwrap().analytics_uuid),
            ("aip", "1"),
            ("ds", "app"),
            ("an", "Hocus CLI"),
            ("av", crate_version!()),
            ("ec", "usage"),
            ("ea", "tick"),
            ("el", &os_info),
        ];

        client
            .post("https://www.google-analytics.com/collect")
            .form(&params)
            .send()?;

        Ok(())
    }
}
