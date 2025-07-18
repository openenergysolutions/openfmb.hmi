// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use fern::colors::{Color, ColoredLevelConfig};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ModuleLogLevel {
    pub module: String,
    pub level: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct OESAppSettings {
    pub log_level: String,
    pub log_file: Option<String>,
    pub colored: Option<bool>,
    pub app_name: String,
    pub module_log_levels: Option<Vec<ModuleLogLevel>>,
}

impl OESAppSettings {
    pub fn new() -> Self {
        Self {
            log_level: "info".to_string(),
            log_file: None,
            colored: Some(false),
            app_name: "oes_app".to_string(),
            module_log_levels: None,
        }
    }

    pub fn colored(&self) -> bool {
        self.colored.unwrap_or(false)
    }
}

pub fn setup_logger(settings: OESAppSettings) -> Result<(), fern::InitError> {
    let colors_level = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Magenta)
        // depending on the terminals color scheme, this is the same as the background color
        .trace(Color::BrightBlack);

    let colored = settings.colored.unwrap_or_default();
    let app_name = settings.app_name.clone();

    // TIMESTAMP [TARGET] [LEVEL] [APP_NAME] - MESSAGE
    let mut dispatch = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}] [{}] - {}",
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.target(),
                if colored {
                    colors_level.color(record.level()).to_string()
                } else {
                    record.level().to_string()
                },
                app_name,
                message
            ));
        })
        .level(log::LevelFilter::from_str(&settings.log_level).unwrap_or(log::LevelFilter::Error));

    // Set the log level for the specified modules, good when you want to customize the log level for deps
    if let Some(module_log_levels) = settings.module_log_levels {
        for module_log_level in module_log_levels {
            let level = log::LevelFilter::from_str(&module_log_level.level)
                .unwrap_or(log::LevelFilter::Off);
            dispatch = dispatch.level_for(module_log_level.module, level);
        }
    }

    dispatch = dispatch.chain(std::io::stdout());

    if let Some(file) = &settings.log_file {
        dispatch = dispatch.chain(fern::log_file(file.clone())?);
    }

    dispatch.apply()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_setup_logger() {
        let settings = OESAppSettings {
            log_level: "info".to_string(),
            log_file: Some("test.log".to_string()),
            colored: Some(true),
            app_name: "test".to_string(),
            module_log_levels: Some(vec![
                ModuleLogLevel {
                    module: "test_module".to_string(),
                    level: "debug".to_string(),
                },
                ModuleLogLevel {
                    module: "another_module".to_string(),
                    level: "error".to_string(),
                },
            ]),
        };

        // Create a test log file
        let _ = File::create("test.log").unwrap();

        // Setup logger
        setup_logger(settings).unwrap();

        // Log a test message
        log::info!("This is a test log message.");

        // Clean up the test log file
        let _ = std::fs::remove_file("test.log");
    }
}
