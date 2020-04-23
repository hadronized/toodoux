//! Initiate the configuration file creation when not present.

use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::task::UID;

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFile {
  /// Path to the folder containing all the tasks.
  root_dir: PathBuf,
}

impl ConfigFile {
  fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let home = env::var("HOME")?;
    let path = Path::new(&home).join(".toodoux");

    Ok(path)
  }

  pub fn root_dir(&self) -> &Path {
    &self.root_dir
  }

  pub fn config_toml_path(&self) -> PathBuf {
    self.root_dir.join("config.toml")
  }

  pub fn taskuid_path(&self) -> PathBuf {
    self.root_dir.join(".taskuid")
  }

  pub fn task_path(&self, uid: UID) -> PathBuf {
    self.root_dir.join(format!("task_{}.json", u32::from(uid)))
  }

  pub fn get() -> Option<Self> {
    let path = Self::get_config_path().ok()?;

    if path.is_dir() {
      Some(ConfigFile {
        root_dir: path.to_owned(),
      })
    } else {
      None
    }
  }

  pub fn create() -> Option<Self> {
    let root_dir = Self::get_config_path().ok()?;
    Some(ConfigFile { root_dir })
  }

  pub fn save(&self) -> Result<(), Box<dyn Error>> {
    let serialized = toml::to_string_pretty(self)?;

    let root_dir = self.root_dir();
    fs::create_dir_all(root_dir)?;

    let _ = fs::write(self.config_toml_path(), serialized)?;

    // create UID tracker
    let _ = fs::write(self.taskuid_path(), "0");

    Ok(())
  }
}
