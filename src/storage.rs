use anyhow::anyhow;
use directories::ProjectDirs;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Storage {
    dirs: ProjectDirs,
}

impl Storage {
    pub fn new() -> anyhow::Result<Self> {
        match ProjectDirs::from("", "Nathan Samra", "doer") {
            None => Err(anyhow!("Error finding the database directory.")),
            Some(dirs) => Ok(Storage { dirs }),
        }
    }

    pub fn database(&self) -> &Path {
        self.dirs.data_dir()
    }

    pub fn config_file(&self) -> PathBuf {
        let config_dir = self.dirs.config_dir().to_path_buf();
        config_dir.join("config.toml")
    }
}
