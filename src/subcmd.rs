use std::error::Error;

use colored::Colorize;

use crate::{cli::list_tasks, cli::SubCommand, config::Config, task::TaskManager, task::UID};

pub fn run_subcmd(
  config: Config,
  subcmd: Option<SubCommand>,
  task_uid: Option<UID>,
) -> Result<(), Box<dyn Error>> {
  match subcmd {
    // default subcommand
    None => {}
    Some(subcmd) => match subcmd {
      SubCommand::Add { start, done, name } => {
        if task_uid.is_none() {
          add_task(config, start, done, name.join(" "))?;
        } else {
          println!(
            "{}",
            "cannot add a task to another one; maybe you were looking for dependencies instead?"
              .red()
          );
        }
      }

      SubCommand::Edit { name } => {}

      SubCommand::Remove { all } => {}

      SubCommand::List {
        mut todo,
        mut start,
        mut done,
        mut cancelled,
        all,
        ..
      } => {
        // handle filtering logic
        if all {
          todo = true;
          start = true;
          done = true;
          cancelled = true;
        } else if !(todo || start || done || cancelled) {
          // if nothing is set, we use “sensible” defaults by listing only “active” tasks (todo and ongoing)
          todo = true;
          start = true;
        }

        list_tasks(config, todo, start, cancelled, done)?;
      }
    },
  }

  Ok(())
}

/// Add a new task.
fn add_task(config: Config, start: bool, done: bool, name: String) -> Result<(), Box<dyn Error>> {
  let mut task_mgr = TaskManager::new_from_config(&config)?;
  let mut task = Task::new(name, Vec::new());

  // determine if we need to switch to another status
  if start {
    task.change_status(Status::Ongoing);
  } else if done {
    task.change_status(Status::Done);
  }

  let uid = task_mgr.register_task(task.clone());
  task_mgr.save(&config)?;

  let task_uid_width = guess_task_uid_width(uid);
  let status_width = guess_task_status_width(&config, task.status());
  display_task_header(task_uid_width, status_width);
  display_task_inline(&config, uid, &task, true, task_uid_width, status_width);

  Ok(())
}
