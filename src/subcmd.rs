use crate::{
  cli::NoteCommand,
  cli::{
    add_task, date_time_to_string, edit_task, list_tasks, rename_project, show_task,
    show_task_history, ProjectCommand, SubCommand,
  },
  config::Config,
  interactive_editor::interactively_edit,
  task::{Status, Task, TaskManager, UID},
  term::Term,
};
use colored::Colorize as _;
use std::{error::Error, fmt};

// TODO: break this function into small parts.
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
          note: with_note,
          content,
        } => {
          if task.is_none() {
            let uid = add_task(&config, &mut task_mgr, &term, start, done, content)?;

            // TODO: rework this while refactoring
            if with_note {
              if let Some(task) = task_mgr.get_mut(uid) {
                let note = interactively_edit_note(&config, false, &task, "")?;
                task.add_note(note);
                task_mgr.save(&config)?;
              }
            }
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

        // TODO: simplify this pile of shit.
        SubCommand::Note { note_uid, subcmd } => {
          if let Some((task_uid, task)) = task {
            match subcmd {
              NoteCommand::Add { no_history } => {
                let note = interactively_edit_note(
                  &config,
                  !no_history && config.previous_notes_help(),
                  &task,
                  "\n",
                )?;
                task.add_note(note);
                task_mgr.save(&config)?;
              }

              NoteCommand::Edit { no_history } => {
                if let Some(note_uid) = note_uid {
                  // get the note so that we can put it in the temporary file
                  let notes = task.notes();
                  let note_uid = note_uid.dec();
                  let prenote = notes
                    .get(usize::from(note_uid))
                    .map(|note| note.content.as_str())
                    .unwrap_or_default();

                  // open an interactive editor and replace the previous note
                  let note = interactively_edit_note(
                    &config,
                    !no_history && config.previous_notes_help(),
                    &task,
                    prenote,
                  )?;
                  task.replace_note(note_uid, note)?;
                  task_mgr.save(&config)?;
                } else {
                  println!(
                    "{}",
                    format!("cannot edit task {}’s note: no note UID provided", task_uid).red()
                  );
                }
              }
            }
          } else {
            println!(
              "{}",
              "missing or unknown task to add, edit or list notes about".red()
            );
          }
        }

        SubCommand::History => {
          if let Some((uid, task)) = task {
            show_task_history(&config, uid, task);
          } else {
            println!("{}", "missing or unknown task to display history".red());
          }
        }

        SubCommand::Project(ProjectCommand::Rename {
          current_project,
          new_project,
        }) => {
          rename_project(&mut task_mgr, current_project, new_project);
          task_mgr.save(&config)?;
        }
      }
    }
  }

  Ok(())
}
