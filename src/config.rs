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
    log::trace!("getting configuration root path from the environment");
    let home = dirs::config_dir().ok_or("cannot find configuration directory")?;
    let path = Path::new(&home).join("toodoux");

    Ok(path)
  }

  pub fn from_dir(path: impl AsRef<Path>) -> Result<Option<Self>, Box<dyn Error>> {
    let path = path.as_ref().join("config.toml");

    log::trace!("reading configuration from {}", path.display());
    if path.is_file() {
      let content = fs::read_to_string(&path)?;
      let parsed = toml::from_str(&content)?;
      Ok(Some(parsed))
    } else {
      Ok(None)
    }
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

  pub fn todo_state_name(&self) -> &str {
    &self.main.todo_state_name
  }

  pub fn ongoing_state_name(&self) -> &str {
    &self.main.ongoing_state_name
  }

  pub fn done_state_name(&self) -> &str {
    &self.main.done_state_name
  }

  pub fn get() -> Result<Option<Self>, Box<dyn Error>> {
    let path = Self::get_config_path()?;
    Self::from_dir(path)
  }

  pub fn create(path: Option<&Path>) -> Option<Self> {
    let root_dir = path
      .map(|p| p.to_owned())
      .or(Self::get_config_path().ok())?;
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

    log::trace!("creating new configuration:\n{:#?}", config);

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
