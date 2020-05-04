//! Tasks related code.

use chrono::{DateTime, Utc};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json as json;
use std::error::Error;
use std::fmt;
use std::fs;

use crate::config::Config;

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskManager {
  next_uid: UID,
  tasks: Vec<Task>,
}

impl TaskManager {
  pub fn new_from_config(config: &Config) -> Result<Self, Box<dyn Error>> {
    let path = config.tasks_path();

    if path.is_file() {
      Ok(json::from_reader(fs::File::open(path)?)?)
    } else {
      let task_mgr = TaskManager {
        next_uid: UID::default(),
        tasks: Vec::new(),
      };
      Ok(task_mgr)
    }
  }

  fn increment_uid(&mut self) {
    let uid = self.next_uid.0 + 1;
    self.next_uid = UID(uid);
  }

  pub fn create_task<N, C, L>(&mut self, name: N, content: C, state: State, labels: L) -> Task
  where
    N: Into<String>,
    C: Into<String>,
    L: Into<Vec<String>>,
  {
    let uid = self.next_uid;

    self.increment_uid();

    let task = Task {
      uid,
      name: name.into(),
      content: content.into(),
      state,
      labels: labels.into(),
      history: vec![Event::Created(Utc::now())],
    };

    self.tasks.push(task.clone());
    task
  }

  pub fn create_task_from_editor(&mut self, config: &Config) -> Result<Task, Box<dyn Error>> {
    // spawn an editor if available and if not, simply return an error
    let editor = std::env::var("EDITOR")?;
    let task_path = config.editor_task_path();
    let _ = std::process::Command::new(editor)
      .arg(&task_path)
      .spawn()?
      .wait()?;

    // read the content of the file containing the task and delete it
    let content = fs::read_to_string(&task_path)?;
    fs::remove_file(task_path)?;

    Ok(self.create_task("<no name yet>", content, State::Todo, Vec::new()))
  }

  pub fn save(&mut self, config: &Config) -> Result<(), Box<dyn Error>> {
    Ok(json::to_writer_pretty(
      fs::File::create(config.tasks_path())?,
      self,
    )?)
  }

  pub fn tasks(&self) -> impl Iterator<Item = &Task> {
    self.tasks.iter()
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
  /// UID.
  uid: UID,
  /// Name of the task.
  name: String,
  /// Optional content of the task.
  content: String,
  /// State of the task.
  state: State,
  /// Optional list of labels.
  labels: Vec<String>,
  /// Event history.
  history: Vec<Event>,
}

impl Task {
  pub fn state(&self) -> &State {
    &self.state
  }

  pub fn change_state(&mut self, state: State) {
    self.state = state.clone();
    self.history.push(Event::StateChanged(Utc::now(), state));
  }
}

impl fmt::Display for Task {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let output = format!(
      "{uid} {state} {name} {labels}",
      uid = self.uid,
      state = self.state,
      name = self.name.italic(),
      labels = "" // TODO
    );

    if let State::Done(_) = self.state {
      write!(f, "{}", output.strikethrough())
    } else {
      f.write_str(&output)
    }
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

impl fmt::Display for UID {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "{}{:0>4}", "#".dimmed(), self.0.to_string().dimmed())
  }
}

/// State of a task.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum State {
  /// A “todo” state.
  ///
  /// Users will typically have “TODO“, “PLANNED”, etc.
  Todo,
  /// An “ongoing” state.
  ///
  /// Users will typically have “ONGOING”, “WIP”, etc.
  Ongoing(String),
  /// A “done” state.
  ///
  /// Users will typically have "DONE", "CANCELLED", "WONTFIX", etc.
  Done(String),
}

impl fmt::Display for State {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      State::Todo => write!(f, "{:8}", "TODO".purple().bold()),
      State::Ongoing(ref s) => write!(f, "{:8}", s.green().bold()),
      State::Done(ref s) => write!(f, "{:8}", s.dimmed()),
    }
  }
}

/// Task event.
///
/// Such events occurred when a change is made to a task (created, edited, scheduled, state
/// changed, etc.).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Event {
  Created(DateTime<Utc>),
  StateChanged(DateTime<Utc>, State),
  // AddSchedule {
  //   event_date: DateTime<Utc>,
  //   scheduled_date: DateTime<Utc>,
  // },
  // RemoveSchedule(DateTime<Utc>),
  // AddDeadline {
  //   event_date: DateTime<Utc>,
  //   scheduled_date: DateTime<Utc>,
  // },
  // RemoveDeadline(DateTime<Utc>),
}

// #542 TODO Name of the task here :label1,label2:
// Eventually a description
