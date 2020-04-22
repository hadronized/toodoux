//! Initiate the configuration file creation when not present.

use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFile {
  /// Path to the folder containing all the tasks.
  tasks_root_dir: PathBuf,
}

impl ConfigFile {
  pub const CONFIG_PATH: &'static str = "~/.toodoux/toodoux.toml";

  pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let home = env::var("HOME")?;
    let path = Path::new(&home).join(".toodoux/config.toml");

    Ok(path)
  }

  pub fn get() -> Option<Self> {
    let path = Self::get_config_path().ok()?;

    if path.is_file() {
      Some(ConfigFile {
        tasks_root_dir: path.to_owned(),
      })
    } else {
      None
    }
  }

  pub fn create() -> Option<Self> {
    let tasks_root_dir = Self::get_config_path().ok()?;
    Some(ConfigFile { tasks_root_dir })
  }

  pub fn save(&self) -> Result<(), Box<dyn Error>> {
    let serialized = toml::to_string_pretty(self)?;

    let path = Self::get_config_path()?;

    // create parent directories if missing
    let parent = path
      .parent()
      .ok_or_else(|| "trying to save file at wrong location on the file system")?;

    if !parent.is_dir() {
      fs::create_dir_all(parent)?;
    }

    fs::write(path, serialized)?;

    Ok(())
  }
}
