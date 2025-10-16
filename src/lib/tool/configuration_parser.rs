use crate::config::Config;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct ConfigurationParser {
    project_directory: PathBuf,
}

impl ConfigurationParser {
    pub fn new(project_directory: PathBuf) -> Self {
        Self {
            project_directory,
        }
    }

    pub fn parse(&self) -> Result<Config> {
        let config_path: PathBuf = self.project_directory.join("build.yaml");

        log::info!("Parsing configuration file {:?}", &config_path);

        let config_str: String = fs::read_to_string(config_path)
            .context("Failed to read 'build.yaml' file")?;

        let config: Config = serde_yaml::from_str(&config_str)
            .context("Failed to parse 'build.yaml' file")?;

        Ok(config)
    }
}
