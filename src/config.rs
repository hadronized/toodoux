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
  tasks_file: PathBuf,

  /// Name of the “TODO” state.
  todo_alias: String,

  /// Name of the “ONGOING” state.
  wip_alias: String,

  /// Name of the “DONE” state.
  done_alias: String,

  /// Name of the “CANCELLED” state.
  cancelled_alias: String,

  /// “UID” column name.
  uid_col_name: String,

  /// “Age” column name.
  age_col_name: String,

  /// “Spent” column name.
  spent_col_name: String,

  /// “Prio” column name.
  prio_col_name: String,

  /// “Project” column name.
  project_col_name: String,

  /// “Status” column name.
  status_col_name: String,

  /// “Description” column name.
  description_col_name: String,

  /// Should we display empty columns?
  display_empty_cols: bool,
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
    &self.main.tasks_file
  }

  pub fn config_toml_path(&self) -> PathBuf {
    self.main.tasks_file.join("config.toml")
  }

  pub fn tasks_path(&self) -> PathBuf {
    self.main.tasks_file.join("tasks.json")
  }

  pub fn todo_alias(&self) -> &str {
    &self.main.todo_alias
  }

  pub fn wip_alias(&self) -> &str {
    &self.main.wip_alias
  }

  pub fn done_alias(&self) -> &str {
    &self.main.done_alias
  }

  pub fn cancelled_alias(&self) -> &str {
    &self.main.cancelled_alias
  }

  pub fn uid_col_name(&self) -> &str {
    &self.main.uid_col_name
  }

  pub fn age_col_name(&self) -> &str {
    &self.main.age_col_name
  }

  pub fn spent_col_name(&self) -> &str {
    &self.main.spent_col_name
  }

  pub fn prio_col_name(&self) -> &str {
    &self.main.prio_col_name
  }

  pub fn project_col_name(&self) -> &str {
    &self.main.project_col_name
  }

  pub fn status_col_name(&self) -> &str {
    &self.main.status_col_name
  }

  pub fn description_col_name(&self) -> &str {
    &self.main.description_col_name
  }

  pub fn display_empty_cols(&self) -> bool {
    self.main.display_empty_cols
  }

  pub fn get() -> Result<Option<Self>, Box<dyn Error>> {
    let path = Self::get_config_path()?;
    Self::from_dir(path)
  }

  pub fn create(path: Option<&Path>) -> Option<Self> {
    let tasks_file = path
      .map(|p| p.to_owned())
      .or(Self::get_config_path().ok())?;
    let todo_alias = "TODO".to_owned();
    let wip_alias = "WIP".to_owned();
    let done_alias = "DONE".to_owned();
    let cancelled_alias = "CANCELLED".to_owned();
    let uid_col_name = "UID".to_owned();
    let age_col_name = "Age".to_owned();
    let spent_col_name = "Spent".to_owned();
    let prio_col_name = "Prio".to_owned();
    let project_col_name = "Project".to_owned();
    let status_col_name = "Status".to_owned();
    let description_col_name = "Description".to_owned();
    let display_empty_cols = false;

    let main = MainConfig {
      tasks_file,
      todo_alias,
      wip_alias,
      done_alias,
      cancelled_alias,
      uid_col_name,
      age_col_name,
      spent_col_name,
      prio_col_name,
      project_col_name,
      status_col_name,
      description_col_name,
      display_empty_cols,
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
