//! Tasks related code.

use crate::config::Config;
use chrono::{DateTime, Utc};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json as json;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::str::FromStr;

/// Create, edit, remove and list tasks.
#[derive(Debug, Deserialize, Serialize)]
pub struct TaskManager {
  /// Next UID to use for the next task to create.
  next_uid: UID,
  /// List of known tasks.
  tasks: HashMap<UID, Task>,
}

impl TaskManager {
  /// Create a manager from a configuration.
  pub fn new_from_config(config: &Config) -> Result<Self, Box<dyn Error>> {
    let path = config.tasks_path();

    if path.is_file() {
      Ok(json::from_reader(fs::File::open(path)?)?)
    } else {
      let task_mgr = TaskManager {
        next_uid: UID::default(),
        tasks: HashMap::new(),
      };
      Ok(task_mgr)
    }
  }

  /// Increment the next UID to use.
  fn increment_uid(&mut self) {
    let uid = self.next_uid.0 + 1;
    self.next_uid = UID(uid);
  }

  /// Register a task and give it an [`UID`].
  pub fn register_task(&mut self, task: Task) -> UID {
    let uid = self.next_uid;

    self.increment_uid();
    self.tasks.insert(uid, task);

    uid
  }

  pub fn save(&mut self, config: &Config) -> Result<(), Box<dyn Error>> {
    Ok(json::to_writer_pretty(
      fs::File::create(config.tasks_path())?,
      self,
    )?)
  }

  pub fn tasks(&self) -> impl Iterator<Item = (&UID, &Task)> {
    self.tasks.iter()
  }

  pub fn get(&self, uid: &UID) -> Option<&Task> {
    self.tasks.get(uid)
  }

  pub fn get_mut(&mut self, uid: &UID) -> Option<&mut Task> {
    self.tasks.get_mut(uid)
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
  /// Name of the task.
  name: String,
  /// Optional content of the task.
  content: String,
  /// Optional list of labels.
  labels: Vec<String>,
  /// Event history.
  history: Vec<Event>,
}

impl Task {
  /// Create a new [`Task`] and populate automatically its history with creation date and status.
  pub fn new(
    name: impl Into<String>,
    content: impl Into<String>,
    labels: impl Into<Vec<String>>,
  ) -> Self {
    let date = Utc::now();

    Task {
      name: name.into(),
      content: content.into(),
      labels: labels.into(),
      history: vec![
        Event::Created(date),
        Event::StatusChanged {
          event_date: date,
          status: Status::Todo,
        },
      ],
    }
  }

  /// Get the name of the [`Task`].
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Get the (optional) content of the [`Task`].
  pub fn content(&self) -> &str {
    &self.content
  }

  /// Get the current status of the [`Task`].
  pub fn status(&self) -> Status {
    self
      .history
      .iter()
      .filter_map(|event| match event {
        Event::StatusChanged { status, .. } => Some(status),
        _ => None,
      })
      .copied()
      .last()
      .unwrap_or(Status::Todo)
  }

  /// Get the creation date of the [`Task`].
  pub fn creation_date(&self) -> Option<&DateTime<Utc>> {
    self.history.iter().find_map(|event| match event {
      Event::Created(ref date) => Some(date),
      _ => None,
    })
  }

  /// Change the name of the [`Task`].
  pub fn change_name(&mut self, name: impl Into<String>) {
    self.name = name.into()
  }

  /// Change the status of the [`Task`].
  pub fn change_status(&mut self, status: Status) {
    self.history.push(Event::StatusChanged {
      event_date: Utc::now(),
      status,
    });
  }
}

/// Unique task identifier.
#[derive(Clone, Copy, Debug, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub struct UID(u32);

impl From<UID> for u32 {
  fn from(uid: UID) -> Self {
    uid.0
  }
}

impl Default for UID {
  fn default() -> Self {
    UID(0)
  }
}

impl FromStr for UID {
  type Err = <u32 as FromStr>::Err;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    u32::from_str(s).map(UID)
  }
}

/// State of a task.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
  /// A “todo” state.
  ///
  /// Users will typically have “TODO“, “PLANNED”, etc.
  Todo,
  /// An “ongoing” state.
  ///
  /// Users will typically have “ONGOING”, “WIP”, etc.
  Ongoing,
  /// A “done” state.
  ///
  /// Users will typically have "DONE".
  Done,
  /// A “cancelled” state.
  ///
  /// Users will typically have "CANCELLED", "WONTFIX", etc.
  Cancelled,
}

/// Task event.
///
/// Such events occurred when a change is made to a task (created, edited, scheduled, state
/// changed, etc.).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Event {
  Created(DateTime<Utc>),
  StatusChanged {
    event_date: DateTime<Utc>,
    status: Status,
  },
}
