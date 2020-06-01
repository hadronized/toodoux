//! The CLI view.

use colored::Colorize;

use crate::config::Config;
use crate::task::{State, Task, UID};
use crate::view::View;

/// The CLI view.
pub struct CLIView {
  todo_state_name: String,
  ongoing_state_name: String,
  done_state_name: String,
  max_state_width: usize,
}

impl CLIView {
  pub fn new(config: &Config) -> Self {
    let todo_state_name = config.todo_state_name().to_owned();
    let ongoing_state_name = config.ongoing_state_name().to_owned();
    let done_state_name = config.done_state_name().to_owned();
    let max_state_width = todo_state_name
      .len()
      .max(ongoing_state_name.len())
      .max(done_state_name.len());

    CLIView {
      todo_state_name,
      ongoing_state_name,
      done_state_name,
      max_state_width,
    }
  }
}

impl View for CLIView {
  fn display_task_summary(&mut self, uid: &UID, task: &Task) {
    let state = task.state();

    let output = format!(
      "{uid} {state} {name} {labels}",
      uid = uid,
      state = *state,
      name = task.name().italic(),
      labels = "" // TODO
    );

    if let State::Done(_) = *state {
      println!("{}", output.strikethrough())
    } else {
      println!("{}", output);
    }
  }

  fn display_filtered_tasks<'a, T, P>(&mut self, tasks: T, mut pred: P)
  where
    T: Iterator<Item = (&'a UID, &'a Task)>,
    P: FnMut(&UID, &Task) -> bool,
  {
    // list by state first
    let mut ongoing_tasks = Vec::new();
    let mut todo_tasks = Vec::new();
    let mut done_tasks = Vec::new();

    for (uid, task) in tasks {
      match task.state() {
        State::Todo(_) => todo_tasks.push((uid, task)),
        State::Ongoing(_) => ongoing_tasks.push((uid, task)),
        State::Done(_) => done_tasks.push((uid, task)),
      }
    }

    for (uid, task) in ongoing_tasks {
      if pred(uid, task) {
        self.display_task_summary(uid, task);
      }
    }

    for (uid, task) in todo_tasks {
      if pred(uid, task) {
        self.display_task_summary(uid, task);
      }
    }

    for (uid, task) in done_tasks {
      if pred(uid, task) {
        self.display_task_summary(uid, task);
      }
    }
  }
}
