//! Markup language used to represent a task.

pub mod markdown;

use std::io::Write;

use crate::task::{State, Task};

/// A markup language.
pub trait Markup {
  /// Extension files using this markup language have.
  const EXT: &'static str;

  /// Deserialize a task from the markup language.
  fn from_str<S>(src: S) -> Result<Task, MarkupError>
  where
    S: AsRef<str>;

  /// Serialize a task to the markup language.
  fn to_write<W>(f: &mut W, task: &Task) -> Result<(), MarkupError>
  where
    W: Write;
}

#[derive(Debug)]
pub enum MarkupError {
  CannotFindTitle,
  UnknownMarkupFormat(String),
  AmbiguousTitle(String),
  BadState(String),
  BadLabels(String),
  CannotSerialize(String),
  Unknown(String),
}
