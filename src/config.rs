use crate::storage::Storage;
use serde::{Deserialize, Serialize};
use std::fs;

pub struct Config {
    storage: Storage,
}

impl Config {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    pub fn context(&self) -> anyhow::Result<String> {
        let config = self.read_config_file()?;
        Ok(config.context)
    }

    pub fn contexts(&self) -> anyhow::Result<Vec<String>> {
        let dir = fs::read_dir(self.storage.database())?;
        dir.map(|maybe_entry| {
            let entry = maybe_entry?;
            Ok(entry.file_name().to_string_lossy().into_owned())
        })
        .collect()
    }

    pub fn set_context(&self, context: String) -> anyhow::Result<()> {
        let mut config_file = self.read_config_file()?;
        config_file.context = context;
        Ok(())
    }

    fn read_config_file(&self) -> anyhow::Result<ConfigFile> {
        let config_str = fs::read_to_string(self.storage.config_file())?;
        let config_file = toml::from_str(config_str.as_str())?;
        Ok(config_file)
    }
}

#[derive(Deserialize, Serialize)]
struct ConfigFile {
    pub context: String,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            context: "default".to_owned(),
        }
    }
}
