//! Markup language used to represent a task.

pub mod markdown;

use std::error::Error;
use std::fmt;
use std::io::Write;

use crate::config::Config;
use crate::task::{State, Task, TaskManager, UID};

/// A markup language.
pub trait Markup {
  /// Extension files using this markup language have.
  const EXT: &'static [&'static str];

  /// Deserialize a task from the markup language.
  fn from_str(
    self,
    src: impl AsRef<str>,
    config: &Config,
    task_mgr: &mut TaskManager,
  ) -> Result<(UID, Task), MarkupError>;

  /// Serialize a task to the markup language.
  fn to_write(
    self,
    f: &mut impl Write,
    config: &Config,
    task_mgr: &mut TaskManager,
    task: &Task,
  ) -> Result<(), MarkupError>;
}

#[derive(Debug)]
pub enum MarkupError {
  CannotFindTitle,
  UnknownMarkupFormat(String),
  AmbiguousTitle(String),
  BadState(String),
  BadLabel(String),
  CannotSerialize(String),
  Unknown(String),
}

impl fmt::Display for MarkupError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      MarkupError::CannotFindTitle => f.write_str("cannot find title"),
      MarkupError::UnknownMarkupFormat(ref e) => write!(f, "unknown markup format: {}", e),
      MarkupError::AmbiguousTitle(ref e) => write!(f, "ambiguous title: {}", e),
      MarkupError::BadState(ref e) => write!(f, "wrong / unknown state: {}", e),
      MarkupError::BadLabel(ref e) => write!(f, "wrong / unknown label: {}", e),
      MarkupError::CannotSerialize(ref e) => write!(f, "cannot serialize: {}", e),
      MarkupError::Unknown(ref e) => write!(f, "unknown markup error: {}", e),
    }
  }
}

impl Error for MarkupError {}
