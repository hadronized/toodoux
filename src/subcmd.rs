use std::error::Error;

use colored::Colorize;

use crate::{
  cli::SubCommand,
  cli::{add_task, list_tasks},
  config::Config,
  task::Status,
  task::TaskManager,
  task::UID,
};

pub fn run_subcmd(
  config: Config,
  subcmd: Option<SubCommand>,
  task_uid: Option<UID>,
) -> Result<(), Box<dyn Error>> {
  match subcmd {
    // default subcommand
    None => {}
    Some(subcmd) => {
      let mut task_mgr = TaskManager::new_from_config(&config)?;

      match subcmd {
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

        SubCommand::Edit { .. } => {}

        SubCommand::Todo => {
          if let Some(task) = task_uid.and_then(|uid| task_mgr.get_mut(uid)) {
            task.change_status(Status::Todo);
            task_mgr.save(&config)?;
          } else {
            println!("{}", "missing or unknown task UID".red());
          }
        }

        SubCommand::Start => {
          if let Some(task) = task_uid.and_then(|uid| task_mgr.get_mut(uid)) {
            task.change_status(Status::Ongoing);
            task_mgr.save(&config)?;
          } else {
            println!("{}", "missing or unknown task UID".red());
          }
        }

        SubCommand::Done => {
          if let Some(task) = task_uid.and_then(|uid| task_mgr.get_mut(uid)) {
            task.change_status(Status::Done);
            task_mgr.save(&config)?;
          } else {
            println!("{}", "missing or unknown task UID".red());
          }
        }

        SubCommand::Cancel => {
          if let Some(task) = task_uid.and_then(|uid| task_mgr.get_mut(uid)) {
            task.change_status(Status::Cancelled);
            task_mgr.save(&config)?;
          } else {
            println!("{}", "missing or unknown task UID".red());
          }
        }

        SubCommand::Remove { .. } => {}

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
      }
    }
  }

  Ok(())
}
