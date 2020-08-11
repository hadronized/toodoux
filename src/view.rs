//! Module responsible in rendering tasks via various methods.

pub mod cli;

use crate::task::{Task, UID};

/// Task renderer.
pub trait View {
  /// Display a single task’s summary on screen.
  fn display_task_summary(&mut self, uid: &UID, task: &Task);

  /// Display all the tasks that satisfy a predicate.
  fn display_filtered_tasks<'a, T, P>(&mut self, tasks: T, p: P)
  where
    T: Iterator<Item = (&'a UID, &'a Task)>,
    P: FnMut(&UID, &Task) -> bool;
}
