//! Initiate the configuration file creation when not present.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
  main: MainConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MainConfig {
  /// Path to the folder containing all the tasks.
  root_dir: PathBuf,
  /// Name of the “TODO” state.
  todo_state_name: String,
  /// Name of the “ONGOING” state.
  ongoing_state_name: String,
  /// Name of the “DONE” state.
  done_state_name: String,
}

impl Config {
  fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let home = std::env::var("HOME")?;
    let path = Path::new(&home).join(".toodoux");

    Ok(path)
  }

  pub fn root_dir(&self) -> &Path {
    &self.main.root_dir
  }

  pub fn config_toml_path(&self) -> PathBuf {
    self.main.root_dir.join("config.toml")
  }

  pub fn tasks_path(&self) -> PathBuf {
    self.main.root_dir.join("tasks.json")
  }

  pub fn editor_task_path(&self) -> PathBuf {
    self.main.root_dir.join(".NEW_TASK")
  }

  pub fn get() -> Result<Option<Self>, Box<dyn Error>> {
    let path = Self::get_config_path()?;

    if path.is_dir() {
      let content = fs::read_to_string(path.join("config.toml"))?;
      let parsed = toml::from_str(&content)?;
      Ok(Some(parsed))
    } else {
      Ok(None)
    }
  }

  pub fn create() -> Option<Self> {
    let root_dir = Self::get_config_path().ok()?;
    let todo_state_name = "TODO".to_owned();
    let ongoing_state_name = "ONGOING".to_owned();
    let done_state_name = "DONE".to_owned();

    let main = MainConfig {
      root_dir,
      todo_state_name,
      ongoing_state_name,
      done_state_name,
    };

    let config = Config { main };

    Some(config)
  }

  pub fn save(&self) -> Result<(), Box<dyn Error>> {
    let root_dir = self.root_dir();
    fs::create_dir_all(root_dir)?;

    let serialized = toml::to_string_pretty(self)?;
    let _ = fs::write(self.config_toml_path(), serialized)?;

    Ok(())
  }
}
