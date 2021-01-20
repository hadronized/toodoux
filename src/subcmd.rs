use crate::{
  cli::NoteCommand,
  cli::{
    add_task, date_time_to_string, edit_task, list_tasks, show_task, show_task_history, SubCommand,
  },
  config::Config,
  interactive_editor::interactively_edit,
  task::{Status, Task, TaskManager, UID},
  term::Term,
};
use colored::Colorize as _;
use itertools::Itertools as _;
use std::{error::Error, fmt};

const PREVIOUS_NOTES_HELP_END_MARKER: &str = "---------------------- >8 ----------------------\n";

#[derive(Debug)]
enum SubCmdError {
  CannotEditNote(String),
  EmptyNote,
}

impl fmt::Display for SubCmdError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      SubCmdError::CannotEditNote(ref reason) => write!(f, "cannot edit note: {}", reason),
      SubCmdError::EmptyNote => f.write_str("the note was empty; nothing added"),
    }
  }
}

impl Error for SubCmdError {}

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

/// Interactively edit a note for a given task.
///
/// The note will be pre-populated by the note history if the config allows for it. The `prefill` argument allows to
/// pre-populate the content of the note.
///
/// The note is returned as a [`String`].
fn interactively_edit_note(
  config: &Config,
  with_history: bool,
  task: &Task,
  prefill: &str,
) -> Result<String, Box<dyn Error>> {
  let prefill = if with_history {
    // if we have the previously recorded note help, pre-populate the file with the previous notes
    let mut new_prefill = task
      .notes()
      .into_iter()
      .enumerate()
      .map(|(i, note)| {
        let modified_date_str = if note.last_modification_date >= note.creation_date {
          format!(
            ", modified on {}",
            date_time_to_string(&note.last_modification_date)
          )
        } else {
          String::new()
        };

        format!(
          "> Note #{nb}, on {creation_date}{modification_date}\n{content}",
          nb = i + 1,
          creation_date = date_time_to_string(&note.creation_date),
          modification_date = modified_date_str,
          content = note.content,
        )
      })
      .join("\n\n");

    new_prefill +=
      "> Above are the previously recorded notes. You are free to temper with them if you want.\n";
    new_prefill += "> You can add the content of your note under the following line. However, do not remove this line!\n";
    new_prefill += PREVIOUS_NOTES_HELP_END_MARKER;
    new_prefill += prefill;
    new_prefill
  } else {
    prefill.to_owned()
  };

  let note_content = interactively_edit(&config, "NEW_NOTE.md", &prefill)?;

  if with_history {
    if let Some(marker_index) = note_content.find(PREVIOUS_NOTES_HELP_END_MARKER) {
      let content = note_content
        .get(marker_index + PREVIOUS_NOTES_HELP_END_MARKER.len()..)
        .unwrap();

      if content.trim().is_empty() {
        Err(Box::new(SubCmdError::EmptyNote))
      } else {
        Ok(content.to_owned())
      }
    } else {
      Err(Box::new(SubCmdError::CannotEditNote(
        "I told you not to temper with this line!".to_owned(),
      )))
    }
  } else {
    if note_content.trim().is_empty() {
      Err(Box::new(SubCmdError::EmptyNote))
    } else {
      Ok(note_content)
    }
  }
}
