use std::error::Error;

use colored::Colorize;

use crate::{
  cli::{add_task, edit_task, list_tasks, show_task, SubCommand},
  config::Config,
  task::{Status, TaskManager, UID},
  term::Term,
};

pub fn run_subcmd(
  config: Config,
  term: impl Term,
  subcmd: Option<SubCommand>,
  task_uid: Option<UID>,
) -> Result<(), Box<dyn Error>> {
  match subcmd {
    // default subcommand
    None => {
      default_list(
        &config,
        &term,
        true,
        true,
        false,
        false,
        false,
        false,
        vec![],
      )?;
    }

    Some(subcmd) => {
      let mut task_mgr = TaskManager::new_from_config(&config)?;
      let task = task_uid.and_then(|uid| task_mgr.get_mut(uid).map(|task| (uid, task)));

      match subcmd {
        SubCommand::Add {
          start,
          done,
          content,
        } => {
          if task.is_none() {
            add_task(&config, &term, start, done, content)?;
          } else {
            println!(
              "{}",
              "cannot add a task to another one; maybe you were looking for dependencies instead?"
                .red()
            );
          }
        }

        SubCommand::Edit { content } => {
          if let Some((_, task)) = task {
            edit_task(task, content)?;
            task_mgr.save(&config)?;
          } else {
            println!("{}", "missing or unknown task to edit".red());
          }
        }

        SubCommand::Show => {
          if let Some((uid, task)) = task {
            show_task(&config, uid, task);
          } else {
            println!("{}", "missing or unknown task to show".red());
          }
        }

        SubCommand::Todo => {
          if let Some((_, task)) = task {
            task.change_status(Status::Todo);
            task_mgr.save(&config)?;
          } else {
            println!("{}", "missing or unknown task".red());
          }
        }

        SubCommand::Start => {
          if let Some((_, task)) = task {
            task.change_status(Status::Ongoing);
            task_mgr.save(&config)?;
          } else {
            println!("{}", "missing or unknown task to start".red());
          }
        }

        SubCommand::Done => {
          if let Some((_, task)) = task {
            task.change_status(Status::Done);
            task_mgr.save(&config)?;
          } else {
            println!("{}", "missing or unknown task to finish".red());
          }
        }

        SubCommand::Cancel => {
          if let Some((_, task)) = task {
            task.change_status(Status::Cancelled);
            task_mgr.save(&config)?;
          } else {
            println!("{}", "missing or unknown task to cancel".red());
          }
        }

        SubCommand::Remove { .. } => {}

        SubCommand::List {
          todo,
          start,
          done,
          cancelled,
          all,
          case_insensitive,
          metadata_filter,
          ..
        } => {
          default_list(
            &config,
            &term,
            todo,
            start,
            cancelled,
            done,
            all,
            case_insensitive,
            metadata_filter,
          )?;
        }
      }
    }
  }

  Ok(())
}

fn default_list(
  config: &Config,
  term: &impl Term,
  mut todo: bool,
  mut start: bool,
  mut cancelled: bool,
  mut done: bool,
  all: bool,
  case_insensitive: bool,
  metadata_filter: Vec<String>,
) -> Result<(), Box<dyn Error>> {
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

  list_tasks(
    config,
    term,
    todo,
    start,
    cancelled,
    done,
    case_insensitive,
    metadata_filter,
  )
}
