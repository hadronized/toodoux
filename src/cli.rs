//! Command line interface.

use chrono::Utc;
use colored::Colorize;
use std::{error::Error, path::PathBuf};
use structopt::StructOpt;

use crate::{config::Config, task::Status, task::Task, task::TaskManager, task::UID};

#[derive(Debug, StructOpt)]
#[structopt(
  name = "toodoux",
  about = "A modern task / todo / note management tool."
)]
pub struct Command {
  /// UID of a task to operate on.
  pub task_uid: Option<UID>,

  #[structopt(subcommand)]
  pub subcmd: Option<SubCommand>,

  /// Non-default config root to read data and configuration from.
  #[structopt(long, short)]
  pub config: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
  /// Add a task.
  #[structopt(visible_aliases = &["a"])]
  Add {
    /// Mark the item as ONGOING.
    #[structopt(long)]
    start: bool,

    /// Mark the item as DONE.
    #[structopt(long)]
    done: bool,

    /// Content of the task.
    ///
    /// If nothing is set, an interactive prompt is spawned for you to enter the content
    /// of what to do.
    name: Vec<String>,
  },
  /// Edit a task.
  #[structopt(visible_aliases = &["e", "ed"])]
  Edit {
    /// Change the name of the task.
    name: Vec<String>,
  },
  /// Remove a task.
  #[structopt(visible_aliases = &["r", "rm"])]
  Remove {
    /// Remove all the tasks.
    #[structopt(short, long)]
    all: bool,
  },
  /// List all the tasks.
  #[structopt(visible_aliases = &["l", "ls"])]
  List {
    /// Filter with todo items.
    #[structopt(short, long)]
    todo: bool,

    /// Filter with started items.
    #[structopt(short, long)]
    start: bool,

    /// Filter with done items.
    #[structopt(short, long)]
    done: bool,

    /// Filter with cancelled items.
    #[structopt(short, long)]
    cancelled: bool,

    /// Do not filter the items and show them all.
    #[structopt(short, long)]
    all: bool,

    /// Show the content of each listed task, if any.
    #[structopt(long)]
    content: bool,
  },
}

/// List tasks.
pub fn list_tasks(
  config: Config,
  todo: bool,
  start: bool,
  cancelled: bool,
  done: bool,
) -> Result<(), Box<dyn Error>> {
  let task_mgr = TaskManager::new_from_config(&config)?;
  let mut tasks: Vec<_> = task_mgr
    .tasks()
    .filter(|(_, task)| {
      // filter the task depending on what is passed as argument
      match task.status() {
        Status::Ongoing => start,
        Status::Todo => todo,
        Status::Done => done,
        Status::Cancelled => cancelled,
      }
    })
    .collect();
  tasks.sort_by_key(|(_, task)| task.status());

  // precompute a bunch of data for display widths / padding / etc.
  let (task_uid_width, status_width) =
    tasks
      .iter()
      .fold((0, 0), |(task_uid_width, status_width), (&uid, task)| {
        let task_uid_width = task_uid_width.max(guess_task_uid_width(uid));
        let status_width = status_width.max(guess_task_status_width(&config, task.status()));

        (task_uid_width, status_width)
      });

  // actual display
  display_task_header(task_uid_width, status_width);

  let mut parity = true;
  for (&uid, task) in tasks {
    display_task_inline(&config, uid, task, parity, task_uid_width, status_width);
    parity = !parity;
  }

  Ok(())
}

/// Display the header of tasks.
fn display_task_header(task_uid_width: usize, status_width: usize) {
  let output = format!(
    " {uid:<width$}",
    uid = "UID".underline(),
    width = task_uid_width
  ) + &format!(
    " {age:5} {status:<width$} {name}",
    age = "Age".underline(),
    status = "Status".underline(),
    name = "Description".underline(),
    width = status_width
  );

  println!("{:<120}", output);
}

/// Display a task to the user.
fn display_task_inline(
  config: &Config,
  uid: UID,
  task: &Task,
  parity: bool,
  task_uid_width: usize,
  status_width: usize,
) {
  let (name, status);
  match task.status() {
    Status::Todo => {
      if parity {
        name = task.name().bright_white().on_black();
      } else {
        name = task.name().bright_white().on_bright_black();
      }
      status = config.todo_alias().clone().bold().magenta();
    }

    Status::Ongoing => {
      name = task.name().black().on_bright_green();
      status = config.wip_alias().clone().bold().green();
    }

    Status::Done => {
      name = task.name().bright_black().dimmed().on_black();
      status = config.done_alias().clone().dimmed().bright_black();
    }

    Status::Cancelled => {
      name = task
        .name()
        .bright_black()
        .dimmed()
        .strikethrough()
        .on_black();
      status = config.cancelled_alias().clone().dimmed().bright_red();
    }
  }

  let output = format!(" {uid:<width$}", uid = uid, width = task_uid_width)
    + &format!(
      " {age:<5} {status:<width$} {name}",
      age = friendly_age(task),
      status = status,
      name = name,
      width = status_width
    );

  println!("{:<120}", output);
}

/// Find out the age of a task and get a friendly representation.
fn friendly_age(task: &Task) -> String {
  let dur =
    Utc::now().signed_duration_since(task.creation_date().cloned().unwrap_or_else(|| Utc::now()));

  if dur.num_minutes() < 1 {
    format!("{}s", dur.num_seconds())
  } else if dur.num_hours() < 1 {
    format!("{}min", dur.num_minutes())
  } else if dur.num_days() < 1 {
    format!("{}h", dur.num_hours())
  } else if dur.num_weeks() < 2 {
    format!("{}d", dur.num_days())
  } else if dur.num_weeks() < 4 {
    // less than four weeks
    format!("{}w", dur.num_weeks())
  } else {
    format!("{}m", dur.num_weeks() / 4)
  }
}

/// Guess the width required to represent the task UID.
fn guess_task_uid_width(uid: UID) -> usize {
  let val = uid.val();

  // minimum is 3 because of “UID” (three chars)
  if val < 10 {
    3
  } else if val < 100 {
    3
  } else if val < 1000 {
    3
  } else if val < 10000 {
    4
  } else if val < 100000 {
    5
  } else {
    6
  }
}

/// Guess the width required to represent the task status.
fn guess_task_status_width(config: &Config, status: Status) -> usize {
  let width = match status {
    Status::Ongoing => config.wip_alias().len(),
    Status::Todo => config.todo_alias().len(),
    Status::Done => config.done_alias().len(),
    Status::Cancelled => config.cancelled_alias().len(),
  };

  width.max("Status".len())
}
