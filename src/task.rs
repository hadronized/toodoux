//! Tasks related code.

use crate::{config::Config, metadata::Metadata, metadata::Priority};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json as json;
use std::{collections::HashMap, error::Error, fmt, fs, str::FromStr};
use unicase::UniCase;

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

  pub fn get_mut(&mut self, uid: UID) -> Option<&mut Task> {
    self.tasks.get_mut(&uid)
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
  /// Name of the task.
  name: String,
  /// Event history.
  history: Vec<Event>,
}

impl Task {
  /// Create a new [`Task`] and populate automatically its history with creation date and status.
  pub fn new(name: impl Into<String>) -> Self {
    let date = Utc::now();

    Task {
      name: name.into(),
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

  /// Get the age of the [`Task`]; i.e. the duration since its creation date.
  pub fn age(&self) -> Duration {
    Utc::now().signed_duration_since(self.creation_date().copied().unwrap_or_else(|| Utc::now()))
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

  /// Add a new note to the [`Task`].
  pub fn add_note(&mut self, note: impl Into<String>) {
    self.history.push(Event::NoteAdded {
      event_date: Utc::now(),
      note: note.into(),
    });
  }

  /// Iterate over the notes, if any.
  pub fn notes(&self) -> impl Iterator<Item = (&DateTime<Utc>, &str)> {
    self.history.iter().filter_map(|event| match event {
      Event::NoteAdded {
        ref event_date,
        ref note,
      } => Some((event_date, note.as_str())),
      _ => None,
    })
  }

  /// Compute the time spent on this task.
  pub fn spent_time(&self) -> Duration {
    let (spent, last_wip) =
      self
        .history
        .iter()
        .fold((Duration::zero(), None), |(spent, last_wip), event| {
          match event {
            Event::StatusChanged { event_date, status } => match (status, last_wip) {
              // We go from any status to WIP status; return the spent time untouched and set the new “last_wip” with the
              // time at which the status change occurred
              (Status::Ongoing, _) => (spent, Some(event_date.clone())),
              // We go to anything but WIP while the previous status was WIP; accumulate.
              (_, Some(last_wip)) => (spent + (event_date.signed_duration_since(last_wip)), None),
              // We go between inactive status, ignore
              _ => (spent, last_wip),
            },
            _ => (spent, last_wip),
          }
        });

    if let Some(last_wip) = last_wip {
      // last status was WIP; accumulate moaaar
      spent + Utc::now().signed_duration_since(last_wip)
    } else {
      spent
    }
  }

  /// Mark this task as part of the input project.
  ///
  /// If a project was already present, this method overrides it. Passing an empty string puts that task into the
  /// _orphaned_ project.
  pub fn set_project(&mut self, project: impl Into<String>) {
    self.history.push(Event::SetProject {
      event_date: Utc::now(),
      project: project.into(),
    });
  }

  /// Set the priority of this task.
  ///
  /// If a priority was already set, this method overrides it. Passing [`None`] removes the priority.
  pub fn set_priority(&mut self, priority: Priority) {
    self.history.push(Event::SetPriority {
      event_date: Utc::now(),
      priority,
    });
  }

  /// Add a tag to task.
  pub fn add_tag(&mut self, tag: impl Into<String>) {
    self.history.push(Event::AddTag {
      event_date: Utc::now(),
      tag: tag.into(),
    });
  }

  /// Apply a list of metadata.
  pub fn apply_metadata(&mut self, metadata: impl IntoIterator<Item = Metadata>) {
    for md in metadata {
      match md {
        Metadata::Project(project) => self.set_project(project),
        Metadata::Priority(priority) => self.set_priority(priority),
        Metadata::Tag(tag) => self.add_tag(tag),
      }
    }
  }

  /// Check all metadata against this I have no idea how to express the end of this sentence so good luck.
  pub fn check_metadata<'a>(
    &self,
    metadata: impl IntoIterator<Item = &'a Metadata>,
    case_insensitive: bool,
  ) -> bool {
    if case_insensitive {
      let own_project = self.project().map(UniCase::new);
      let own_tags = self.tags().map(UniCase::new).collect::<Vec<_>>();
      metadata.into_iter().all(|md| match md {
        Metadata::Project(ref project) => own_project == Some(UniCase::new(project)),
        Metadata::Priority(priority) => self.priority() == Some(*priority),
        Metadata::Tag(ref tag) => own_tags.contains(&UniCase::new(tag)),
      })
    } else {
      metadata.into_iter().all(|md| match md {
        Metadata::Project(ref project) => self.project() == Some(project),
        Metadata::Priority(priority) => self.priority() == Some(*priority),
        Metadata::Tag(ref tag) => self.tags().any(|t| t == tag),
      })
    }
  }

  /// Get the current project.
  pub fn project(&self) -> Option<&str> {
    self
      .history
      .iter()
      .filter_map(|event| match event {
        Event::SetProject { ref project, .. } => Some(project.as_str()),
        _ => None,
      })
      .last()
  }

  /// Get the current project.
  pub fn priority(&self) -> Option<Priority> {
    self
      .history
      .iter()
      .filter_map(|event| match event {
        Event::SetPriority { priority, .. } => Some(*priority),
        _ => None,
      })
      .last()
  }

  /// Get the current tags of a task.
  pub fn tags<'a>(&'a self) -> impl Iterator<Item = &'a str> {
    self.history.iter().filter_map(|event| match event {
      Event::AddTag { ref tag, .. } => Some(tag.as_str()),
      _ => None,
    })
  }
}

/// Unique task identifier.
#[derive(Clone, Copy, Debug, Deserialize, Hash, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UID(u32);

impl UID {
  pub fn val(self) -> u32 {
    self.0
  }
}

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

impl fmt::Display for UID {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    self.0.fmt(f)
  }
}

/// State of a task.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Status {
  /// An “ongoing” state.
  ///
  /// Users will typically have “ONGOING”, “WIP”, etc.
  Ongoing,
  /// A “todo” state.
  ///
  /// Users will typically have “TODO“, “PLANNED”, etc.
  Todo,
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
  /// Event generated when a task is created.
  Created(DateTime<Utc>),

  /// Event generated when the status of a task changes.
  StatusChanged {
    event_date: DateTime<Utc>,
    status: Status,
  },

  /// Event generated when a note is added to a task.
  NoteAdded {
    event_date: DateTime<Utc>,
    note: String,
  },

  /// Event generated when a project is set on a task.
  SetProject {
    event_date: DateTime<Utc>,
    project: String,
  },

  /// Event generated when a priority is set on a task.
  SetPriority {
    event_date: DateTime<Utc>,
    priority: Priority,
  },

  /// Event generated when a tag is added to a task.
  AddTag {
    event_date: DateTime<Utc>,
    tag: String,
  },
}
