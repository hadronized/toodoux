//! Tasks related code.

use chrono::{DateTime, Utc};
use colored::Colorize;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::Write as _;

use crate::config_file::ConfigFile;

pub struct TaskManager {
  next_uid: UID,
}

impl TaskManager {
  pub fn new_from_config(config: &ConfigFile) -> Result<Self, Box<dyn Error>> {
    let next_uid = fs::read_to_string(config.taskuid_path())?
      .parse()
      .map(UID)?;

    Ok(TaskManager { next_uid })
  }

  fn increment_save_uid(&mut self, config: &ConfigFile) -> Result<(), Box<dyn Error>> {
    let uid = self.next_uid.0 + 1;
    //let _ = fs::write(config.taskuid_path(), uid)?;
    let _ = write!(fs::File::create(config.taskuid_path())?, "{}", uid)?;

    self.next_uid = UID(uid);

    Ok(())
  }

  pub fn create_task<N, C, L>(
    &mut self,
    config: &ConfigFile,
    name: N,
    content: C,
    labels: L,
  ) -> Result<Task, Box<dyn Error>>
  where
    N: Into<String>,
    C: Into<String>,
    L: Into<Vec<String>>,
  {
    let uid = self.next_uid;
    self.increment_save_uid(config)?;

    let task = Task {
      uid,
      name: name.into(),
      content: content.into(),
      state: State::Todo,
      labels: labels.into(),
      history: vec![Event::Created(Utc::now())],
    };

    Ok(task)
  }
}

#[derive(Debug)]
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
  //pub fn new<N, C, L>(name: N, content: C, labels: L) -> Self
  //where
  //  N: Into<String>,
  //  C: Into<String>,
  //  L: Into<Vec<String>>,
  //{
  //  Task {
  //    name: name.into(),
  //    content: content.into(),
  //    state: State::Todo,
  //    labels: labels.into(),
  //    history: vec![Event::Created(Utc::now())],
  //  }
  //}

  // pub fn change_state(&mut self, state: State) {
  //   self.state = state.clone();
  //   self.history.push(Event::StateChanged(Utc::now(), state));
  // }
}

impl fmt::Display for Task {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(
      f,
      "{uid} {state} {name} {labels}",
      uid = self.uid,
      state = self.state,
      name = self.name.italic(),
      labels = "" // TODO
    )
  }
}

/// Unique task identifier.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct UID(u32);

impl fmt::Display for UID {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "{}{:0>4}", "#".dimmed(), self.0.to_string().dimmed())
  }
}

/// State of a task.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
  /// A “todo” state.
  ///
  /// Users will typically have “TODO“, “PLANNED”, etc.
  Todo,
  // /// An “ongoing” state.
  // ///
  // /// Users will typically have “ONGOING”, “WIP”, etc.
  // Ongoing(String),
  // /// A “done” state.
  // ///
  // /// Users will typically have "DONE", "CANCELLED", "WONTFIX", etc.
  // Done(String),
}

impl fmt::Display for State {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      State::Todo => write!(f, "{}", "TODO".purple().bold()),
    }
  }
}

/// Task event.
///
/// Such events occurred when a change is made to a task (created, edited, scheduled, state
/// changed, etc.).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
  Created(DateTime<Utc>),
  //StateChanged(DateTime<Utc>, State),
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
