//! Command line interface.

use crate::{
  interactive_editor::{interactively_edit, InteractiveEditingError},
  term::Terminal,
};
use chrono::{DateTime, Duration, Utc};
use colored::Colorize as _;
use itertools::Itertools;
use std::{fmt, fmt::Display, iter::once, path::PathBuf};
use structopt::StructOpt;
use toodoux::{
  config::Config,
  error::Error,
  filter::TaskDescriptionFilter,
  metadata::{Metadata, MetadataValidationError, Priority},
  task::{Event, Status, Task, TaskManager, UID},
};
use unicode_width::UnicodeWidthStr;

const PREVIOUS_NOTES_HELP_END_MARKER: &str = "---------------------- >8 ----------------------\n";

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

    /// Log a note after creating the item.
    #[structopt(short, long)]
    note: bool,

    /// Content of the task.
    ///
    /// If nothing is set, an interactive prompt is spawned for you to enter the content
    /// of what to do.
    content: Vec<String>,
  },

  /// Edit a task.
  #[structopt(visible_aliases = &["e", "ed"])]
  Edit {
    /// Change the name or metadata of the task.
    content: Vec<String>,
  },

  /// Show the details of a task.
  #[structopt(visible_aliases = &["s"])]
  Show,

  /// Mark a task as todo.
  Todo,

  /// Mark a task as started.
  Start,

  /// Mark a task as done.
  Done,

  /// Mark a task as cancelled.
  Cancel,

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

    /// Apply filters ignoring case.
    #[structopt(short = "C", long)]
    case_insensitive: bool,

    /// Metadata filter.
    metadata_filter: Vec<String>,
  },

  /// List, add and edit notes.
  Note {
    /// UID of a note to operate on.
    note_uid: Option<UID>,

    #[structopt(subcommand)]
    subcmd: NoteCommand,
  },

  /// Show the edit history of a task.
  History,

  /// Manipulate projects.
  #[structopt(visible_aliases = &["proj"])]
  Project(ProjectCommand),
}

#[derive(Debug, StructOpt)]
pub enum NoteCommand {
  /// Add a new note.
  ///
  /// You will be prompted to write a note within an editor.
  #[structopt(visible_aliases = &["a"])]
  Add {
    /// Edit the note without note history.
    ///
    /// Overrides the user configuration.
    #[structopt(long)]
    no_history: bool,
  },

  /// Edit a note.
  ///
  /// You will be prompted to edit the node within an editor.
  #[structopt(visible_aliases = &["ed", "e"])]
  Edit {
    /// Edit the note without note history.
    ///
    /// Overrides the user configuration.
    #[structopt(long)]
    no_history: bool,
  },
}

#[derive(Debug, StructOpt)]
pub enum ProjectCommand {
  /// Rename a project.
  ///
  /// This has the effect of renamming the project used for all tasks if their current project is the one to rename.
  Rename {
    /// Project to rename.
    current_project: String,

    /// New name of the project.
    new_project: String,
  },
}

#[derive(Debug)]
pub enum SubCmdError {
  MetadataValidationError(MetadataValidationError),
  CannotEditNote(String),
  EmptyNote,
  InteractiveEditingError(InteractiveEditingError),
  ToodouxError(Error),
}

impl fmt::Display for SubCmdError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      SubCmdError::MetadataValidationError(ref e) => write!(f, "metadata validation error: {}", e),
      SubCmdError::CannotEditNote(ref reason) => write!(f, "cannot edit note: {}", reason),
      SubCmdError::EmptyNote => f.write_str("the note was empty; nothing added"),
      SubCmdError::InteractiveEditingError(ref e) => write!(f, "interactive edit error: {}", e),
      SubCmdError::ToodouxError(ref e) => write!(f, "toodoux error: {}", e),
    }
  }
}

impl std::error::Error for SubCmdError {}

impl From<MetadataValidationError> for SubCmdError {
  fn from(err: MetadataValidationError) -> Self {
    Self::MetadataValidationError(err)
  }
}

impl From<InteractiveEditingError> for SubCmdError {
  fn from(err: InteractiveEditingError) -> Self {
    Self::InteractiveEditingError(err)
  }
}

impl From<Error> for SubCmdError {
  fn from(err: Error) -> Self {
    Self::ToodouxError(err)
  }
}

pub struct CLI<Term> {
  config: Config,
  term: Term,
}

impl<Term> CLI<Term>
where
  Term: Terminal,
{
  /// Create a CLI.
  pub fn new(config: Config, term: Term) -> Self {
    Self { config, term }
  }

  /// Run a subcommand of the CLI.
  pub fn run(
    &mut self,
    task_mgr: &mut TaskManager,
    subcmd: Option<SubCommand>,
    task_uid: Option<UID>,
  ) -> Result<(), SubCmdError> {
    match subcmd {
      // default subcommand
      None => {
        self.list_active_tasks(task_mgr, true, true, false, false, false, false, vec![])?;
      }

      Some(subcmd) => {
        match subcmd {
          SubCommand::Add {
            start,
            done,
            note: with_note,
            content,
          } => {
            if task_uid.is_none() {
              let uid = self.add_task(task_mgr, start, done, content)?;

              // TODO: rework this while refactoring
              if with_note {
                if let Some(task) = task_mgr.get_mut(uid) {
                  let note = interactively_edit_note(&self.config, false, &task, "")?;
                  task.add_note(note);
                  task_mgr.save(&self.config)?;
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
            if let Some(task) = task_uid.and_then(|uid| task_mgr.get_mut(uid)) {
              Self::edit_task(task, content.iter().map(String::as_str))?;
              task_mgr.save(&self.config)?;
            } else {
              println!("{}", "missing or unknown task to edit".red());
            }
          }

          SubCommand::Show => {
            if let Some((uid, task)) =
              task_uid.and_then(|uid| task_mgr.get(uid).map(|task| (uid, task)))
            {
              self.show_task(uid, task);
            } else {
              println!("{}", "missing or unknown task to show".red());
            }
          }

          SubCommand::Todo => {
            if let Some(task) = task_uid.and_then(|uid| task_mgr.get_mut(uid)) {
              task.change_status(Status::Todo);
              task_mgr.save(&self.config)?;
            } else {
              println!("{}", "missing or unknown task".red());
            }
          }

          SubCommand::Start => {
            if let Some(task) = task_uid.and_then(|uid| task_mgr.get_mut(uid)) {
              task.change_status(Status::Ongoing);
              task_mgr.save(&self.config)?;
            } else {
              println!("{}", "missing or unknown task to start".red());
            }
          }

          SubCommand::Done => {
            if let Some(task) = task_uid.and_then(|uid| task_mgr.get_mut(uid)) {
              task.change_status(Status::Done);
              task_mgr.save(&self.config)?;
            } else {
              println!("{}", "missing or unknown task to finish".red());
            }
          }

          SubCommand::Cancel => {
            if let Some(task) = task_uid.and_then(|uid| task_mgr.get_mut(uid)) {
              task.change_status(Status::Cancelled);
              task_mgr.save(&self.config)?;
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
            self.list_active_tasks(
              task_mgr,
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
            if let Some((uid, task)) =
              task_uid.and_then(|uid| task_mgr.get_mut(uid).map(|task| (uid, task)))
            {
              match subcmd {
                NoteCommand::Add { no_history } => {
                  let note = interactively_edit_note(
                    &self.config,
                    !no_history && self.config.previous_notes_help(),
                    &task,
                    "\n",
                  )?;
                  task.add_note(note);
                  task_mgr.save(&self.config)?;
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
                      &self.config,
                      !no_history && self.config.previous_notes_help(),
                      &task,
                      prenote,
                    )?;
                    task.replace_note(note_uid, note)?;
                    task_mgr.save(&self.config)?;
                  } else {
                    println!(
                      "{}",
                      format!("cannot edit task {}’s note: no note UID provided", uid).red()
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
            if let Some((uid, task)) =
              task_uid.and_then(|uid| task_mgr.get(uid).map(|task| (uid, task)))
            {
              self.show_task_history(uid, task);
            } else {
              println!("{}", "missing or unknown task to display history".red());
            }
          }

          SubCommand::Project(ProjectCommand::Rename {
            current_project,
            new_project,
          }) => {
            Self::rename_project(task_mgr, current_project, new_project);
            task_mgr.save(&self.config)?;
          }
        }
      }
    }

    Ok(())
  }

  /// Extract metadata and print them (if any) on screen to help the user know what they are using.
  fn extract_metadata(
    metadata_filter: &[String],
  ) -> Result<(Vec<Metadata>, String), MetadataValidationError> {
    let (metadata, name) = Metadata::from_words(metadata_filter.iter().map(String::as_str));
    Metadata::validate(&metadata)?;

    if !metadata.is_empty() {
      print!(
        "{} {} {}",
        "[".bright_black(),
        metadata.iter().map(Metadata::filter_like).format(", "),
        "]".bright_black()
      );
    }

    Ok((metadata, name))
  }

  /// Extract name filters and print them (if any) on screen to help the user know what they are using.
  fn extract_name_filters<'a>(name: &'a str, case_insensitive: bool) -> TaskDescriptionFilter<'a> {
    let name_filter = TaskDescriptionFilter::new(name.split_ascii_whitespace(), case_insensitive);

    if !name_filter.is_empty() {
      println!(
        "{} {}: {} {}",
        "[".bright_black(),
        "contains".italic(),
        name_filter.terms().format(", "),
        "]".bright_black()
      );
    } else {
      println!();
    }

    name_filter
  }

  /// List all tasks.
  ///
  /// The various arguments allow to refine the listing.
  pub fn list_tasks(
    &self,
    task_mgr: &TaskManager,
    todo: bool,
    start: bool,
    cancelled: bool,
    done: bool,
    case_insensitive: bool,
    metadata_filter: Vec<String>,
  ) -> Result<(), SubCmdError> {
    // extract metadata if any and build the name filter
    let (metadata, name) = Self::extract_metadata(&metadata_filter)?;

    // put an extra space between sections (metadata and name filter) if they are both present
    if !metadata.is_empty() && !name.is_empty() {
      print!(" ");
    }

    let name_filter = Self::extract_name_filters(&name, case_insensitive);

    // get the filtered tasks
    let tasks = task_mgr.filtered_task_listing(
      metadata,
      name_filter,
      todo,
      start,
      done,
      cancelled,
      case_insensitive,
    );

    // precompute a bunch of data for display widths / padding / etc.
    let display_opts = DisplayOptions::new(
      &self.config,
      &self.term,
      tasks.iter().map(|&(uid, task)| (*uid, task)),
    );

    // actual display
    // only display header if there are tasks to display
    if !tasks.is_empty() {
      self.display_task_header(&display_opts);
    }

    for (&uid, task) in tasks {
      self.display_task_inline(uid, task, &display_opts);
    }

    Ok(())
  }

  pub fn list_active_tasks(
    &self,
    task_mgr: &TaskManager,
    mut todo: bool,
    mut start: bool,
    mut cancelled: bool,
    mut done: bool,
    all: bool,
    case_insensitive: bool,
    metadata_filter: Vec<String>,
  ) -> Result<(), SubCmdError> {
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

    self.list_tasks(
      task_mgr,
      todo,
      start,
      cancelled,
      done,
      case_insensitive,
      metadata_filter,
    )
  }

  /// Display the header of tasks.
  fn display_task_header(&self, opts: &DisplayOptions) {
    print!(
      " {uid:<uid_width$} {age:<age_width$}",
      uid = self.config.uid_col_name().underline(),
      uid_width = opts.task_uid_width,
      age = self.config.age_col_name().underline(),
      age_width = opts.age_width,
    );

    let display_empty_cols = self.config.display_empty_cols();

    if display_empty_cols || opts.has_spent_time {
      print!(
        " {spent:<spent_width$}",
        spent = self.config.spent_col_name().underline(),
        spent_width = opts.spent_width,
      );
    }

    if display_empty_cols || opts.has_priorities {
      print!(
        " {priority:<prio_width$}",
        priority = self.config.prio_col_name().underline(),
        prio_width = self.config.prio_col_name().width(),
      );
    }

    if display_empty_cols || opts.has_projects {
      print!(
        " {project:<project_width$}",
        project = self.config.project_col_name().underline(),
        project_width = opts.project_width,
      );
    }

    if self.config.display_tags_listings() && (display_empty_cols || opts.has_tags) {
      print!(
        " {tags:<tags_width$}",
        tags = self.config.tags_col_name().underline(),
        tags_width = opts.tags_width,
      );
    }

    let notes_nb_width = opts.notes_nb_width;
    if notes_nb_width != 0 {
      print!(
        " {notes_nb:<notes_nb_width$}",
        notes_nb = self.config.notes_nb_col_name().underline(),
        notes_nb_width = opts
          .notes_nb_width
          .max(self.config.notes_nb_col_name().len())
      );
    }

    if let Some(max_description_cols) = opts.max_description_cols {
      println!(
        " {status:<status_width$} {description:<description_width$}",
        status = self.config.status_col_name().underline(),
        status_width = opts.status_width,
        description = self.config.description_col_name().underline(),
        description_width = opts.description_width.min(max_description_cols),
      );
    }
  }

  /// Display a task to the user.
  fn display_task_inline(&self, uid: UID, task: &Task, opts: &DisplayOptions) {
    let task_name = task.name();
    let status = task.status();

    print!(
      " {uid:<uid_width$} {age:<age_width$}",
      uid = uid,
      uid_width = opts.task_uid_width,
      age = Self::friendly_task_age(task),
      age_width = opts.age_width,
    );

    let display_empty_cols = self.config.display_empty_cols();

    if display_empty_cols || opts.has_spent_time {
      print!(
        " {spent:<spent_width$}",
        spent = Self::friendly_spent_time(task.spent_time(), status),
        spent_width = opts.spent_width,
      );
    }

    if display_empty_cols || opts.has_priorities {
      if let Some(prio) = task.priority() {
        print!(
          " {priority:<prio_width$}",
          priority = self.friendly_priority(prio),
          prio_width = self.config.prio_col_name().width(),
        );
      } else {
        print!(
          " {prio:<prio_width$}",
          prio = "",
          prio_width = self.config.prio_col_name().width(),
        );
      }
    }

    if display_empty_cols || opts.has_projects {
      print!(
        " {project:<project_width$}",
        project = Self::friendly_project(task.project().unwrap_or("")),
        project_width = opts.project_width,
      );
    }

    if self.config.display_tags_listings() && (display_empty_cols || opts.has_tags) {
      Self::display_tags(task, opts);
    }

    let notes_nb_width = opts.notes_nb_width;
    let notes_nb = task.notes().len();
    if notes_nb_width != 0 {
      print!(
        " {notes_nb:<notes_nb_width$}",
        notes_nb = Self::friendly_notes_nb(notes_nb),
        notes_nb_width = opts
          .notes_nb_width
          .max(self.config.notes_nb_col_name().len())
      );
    }

    print!(
      " {status:<status_width$}",
      status = self.highlight_status(status),
      status_width = opts.status_width,
    );

    self.display_description(opts, status, task_name);
  }

  /// Display the tags by respecting the allowed tags column size.
  fn display_tags(task: &Task, opts: &DisplayOptions) {
    print!(
      " {tags:<tags_width$}",
      tags = Itertools::intersperse(task.tags(), ", ")
        .collect::<String>()
        .yellow(),
      tags_width = opts.tags_width,
    );
  }

  /// Display a description by respecting the allowed description column size.
  ///
  /// The description is not displayed if no space is available on screen.
  fn display_description(&self, opts: &DisplayOptions, status: Status, description: &str) {
    if let Some(max_description_cols) = opts.max_description_cols {
      let mut line_index = 0; // line number we are currently at; cannot exceed config.max_description_lines()
      let mut rel_offset = 0; // unicode offset in the current line; cannot exceed the description width
      let mut line_buffer = String::new(); // buffer for the current line
      let description_width = opts.description_width.min(max_description_cols);

      // The algorithm is a bit convoluted, so here’s a bit of explanation. It’s an iterative algorithm that splits the
      // description into an iterator over words. Each word has a unicode width, which is used to determine whether
      // appending it to the buffer line will make it longer than the description width. The tricky part comes in with
      // the fact that we want to display a ellipsis character if the next word is too long (…) and that we would end up
      // on more line than required.
      //
      // Before adding a new word, we check that its size + 1 added to the current unicode offset is still smaller than
      // the acceptable description width. If it is not the case, it means that adding this word would be out of sight,
      // so it has to be put on another line. However, if we cannot add another line, we simply add “…” to the current
      // line buffer and we are done. Otherwise, we just go to the next line, reset the offset and output the word. If we
      // haven’t passed the end of the line, we simply output the word.
      print!(" ");
      for word in description.split_ascii_whitespace() {
        let word_size = word.width() + 1; // TODO: check what to do about CJK

        if rel_offset + word_size > description_width {
          // we’ve passed the end of the line; break into another line
          line_index += 1;

          if line_index >= self.config.max_description_lines() {
            // we reserve the last column for …
            // we cannot create another line; add the ellipsis (…) character and stop
            line_buffer.push('…');
            break;
          }

          // we can create another line; display the line buffer first
          let hl_description = self.highlight_description_line(status, &line_buffer);
          println!("{:<width$}", hl_description, width = description_width);
          print!("{:<width$}", "", width = opts.description_offset);

          // reset the line buffer and the relative offset
          line_buffer.clear();
          line_buffer.push_str(word);
          rel_offset = word_size;
        } else {
          // we still have room; simply add the word and go on
          if rel_offset > 0 {
            line_buffer.push(' ');
          }

          line_buffer.push_str(word);
          rel_offset += word_size;
        }
      }

      let hl_description = self.highlight_description_line(status, &line_buffer);
      println!("{:<width$}", hl_description, width = description_width);
    }
  }

  /// Find out the age of a task and get a friendly representation.
  fn friendly_task_age(task: &Task) -> impl Display {
    let dur =
      Utc::now().signed_duration_since(task.creation_date().cloned().unwrap_or_else(Utc::now));
    Self::friendly_duration(dur)
  }

  /// Friendly representation of duration.
  fn friendly_duration(dur: Duration) -> String {
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
      format!("{}mth", dur.num_weeks() / 4)
    }
  }

  /// String representation of a spent-time.
  ///
  /// If no time has been spent on this task, an empty string is returned.
  fn friendly_spent_time(dur: Duration, status: Status) -> impl Display {
    if dur == Duration::zero() {
      return String::new().normal();
    }

    let output = Self::friendly_duration(dur);

    match status {
      Status::Ongoing => output.blue(),
      _ => output.bright_black(),
    }
  }

  /// Friendly representation of priorities.
  fn friendly_priority(&self, prio: Priority) -> impl Display {
    match prio {
      Priority::Low => self.config.colors.priority.low.highlight("LOW"),
      Priority::Medium => self.config.colors.priority.medium.highlight("MED"),
      Priority::High => self.config.colors.priority.high.highlight("HIGH"),
      Priority::Critical => self.config.colors.priority.critical.highlight("CRIT"),
    }
  }

  /// Friendly representation of a project name.
  fn friendly_project(project: impl AsRef<str>) -> impl Display {
    project.as_ref().italic()
  }

  /// Friendly representation of a number of notes.
  fn friendly_notes_nb(nb: usize) -> impl Display {
    if nb != 0 {
      nb.to_string().blue().italic()
    } else {
      "".normal()
    }
  }

  /// Friendly representation of a status.
  fn highlight_status(&self, status: Status) -> impl Display {
    match status {
      Status::Todo => self
        .config
        .colors
        .status
        .todo
        .highlight(self.config.todo_alias()),
      Status::Ongoing => self
        .config
        .colors
        .status
        .ongoing
        .highlight(self.config.wip_alias()),
      Status::Done => self
        .config
        .colors
        .status
        .done
        .highlight(self.config.done_alias()),
      Status::Cancelled => self
        .config
        .colors
        .status
        .cancelled
        .highlight(self.config.cancelled_alias()),
    }
  }

  /// Highlight a description line
  fn highlight_description_line(&self, status: Status, line: &str) -> impl Display {
    match status {
      Status::Todo => self.config.colors.description.todo.highlight(line),
      Status::Ongoing => self.config.colors.description.ongoing.highlight(line),
      Status::Done => self.config.colors.description.done.highlight(line),
      Status::Cancelled => self.config.colors.description.cancelled.highlight(line),
    }
  }

  /// Friendly string representation of a date.
  fn friendly_date_time(date_time: &DateTime<Utc>) -> impl Display {
    date_time_to_string(date_time).italic().blue()
  }

  /// Add a new task.
  pub fn add_task(
    &mut self,
    task_mgr: &mut TaskManager,
    start: bool,
    done: bool,
    content: Vec<String>,
  ) -> Result<UID, SubCmdError> {
    // validate the metadata extracted from the content, if any
    let (metadata, name) = Metadata::from_words(content.iter().map(|s| s.as_str()));
    Metadata::validate(&metadata)?;

    let mut task = Task::new(name);

    // apply the metadata
    task.apply_metadata(metadata);

    // determine if we need to switch to another status
    if start {
      task.change_status(Status::Ongoing);
    } else if done {
      task.change_status(Status::Done);
    }

    let uid = task_mgr.register_task(task.clone());
    task_mgr.save(&self.config)?;

    // display options
    let display_opts = DisplayOptions::new(&self.config, &self.term, once((uid, &task)));

    self.display_task_header(&display_opts);
    self.display_task_inline(uid, &task, &display_opts);

    Ok(uid)
  }

  /// Edit a task’s name or metadata.
  pub fn edit_task<'a>(
    task: &mut Task,
    content: impl IntoIterator<Item = &'a str>,
  ) -> Result<(), SubCmdError> {
    // validate the metadata extracted from the content, if any
    let (metadata, name) = Metadata::from_words(content);
    Metadata::validate(&metadata)?;

    // apply the metadata
    task.apply_metadata(metadata);

    // if we have a new name, apply it too
    if !name.is_empty() {
      task.change_name(name);
    }

    Ok(())
  }

  /// Show a task.
  pub fn show_task(&self, uid: UID, task: &Task) {
    let header_hl = &self.config.colors.show_header;
    let status = task.status();

    println!(
      " {}: {}",
      header_hl.highlight(self.config.description_col_name()),
      task.name().bold()
    );
    println!(
      " {}: {}",
      header_hl.highlight(self.config.uid_col_name()),
      uid
    );
    println!(
      " {}: {}",
      header_hl.highlight(self.config.age_col_name()),
      Self::friendly_task_age(task)
    );

    let spent_time = task.spent_time();
    if spent_time == Duration::zero() {
      println!(
        " {}: {}",
        header_hl.highlight(self.config.spent_col_name()),
        "not started yet".bright_black().italic()
      );
    } else {
      println!(
        " {}: {}",
        header_hl.highlight(self.config.spent_col_name()),
        Self::friendly_spent_time(task.spent_time(), status)
      );
    }

    if let Some(prio) = task.priority() {
      println!(
        " {}: {}",
        header_hl.highlight(self.config.prio_col_name()),
        self.friendly_priority(prio)
      );
    }

    if let Some(project) = task.project() {
      println!(
        " {}: {}",
        header_hl.highlight(self.config.project_col_name()),
        Self::friendly_project(project)
      );
    }

    let mut tags = task.tags();

    if let Some(first_tag) = tags.next() {
      let hash = "#".bright_black();

      print!(" {}: ", header_hl.highlight("Tags"));
      print!("{}{}", hash, first_tag.yellow());

      for tag in tags {
        print!(", {}{}", hash, tag.yellow());
      }

      println!();
    }

    println!(
      " {}: {}",
      header_hl.highlight(self.config.status_col_name()),
      self.highlight_status(status)
    );

    println!();

    // show the notes
    for (nb, note) in task.notes().into_iter().enumerate() {
      print!(
        "{}{}{}{}",
        " Note #".bright_black().italic(),
        (nb + 1).to_string().blue().italic(),
        ", on ".bright_black().italic(),
        Self::friendly_date_time(&note.creation_date)
      );

      if note.last_modification_date != note.creation_date {
        print!(
          "{}{}",
          ", edited on ".bright_black().italic(),
          Self::friendly_date_time(&note.last_modification_date)
        );
      }
      println!();

      println!("{}", note.content.trim());
      println!();
    }
  }

  pub fn show_task_history(&self, uid: UID, task: &Task) {
    for event in task.history() {
      // Extract event date from all variants
      match event {
        Event::Created(event_date)
        | Event::StatusChanged { event_date, .. }
        | Event::NoteAdded { event_date, .. }
        | Event::NoteReplaced { event_date, .. }
        | Event::SetProject { event_date, .. }
        | Event::SetPriority { event_date, .. }
        | Event::AddTag { event_date, .. } => {
          print!("{}: ", Self::friendly_date_time(event_date));
        }
      }

      match event {
        Event::Created(_) => {
          println!("{} {}", "Task created with uid".bright_black(), uid);
        }

        Event::StatusChanged { status, .. } => {
          println!(
            "{} {}",
            "Status changed to".bright_black(),
            self.highlight_status(*status)
          );
        }

        Event::NoteAdded { content, .. } => {
          println!("{} {}", "Note added".bright_black(), content);
        }

        Event::NoteReplaced {
          content, note_uid, ..
        } => {
          println!(
            "{} {} {} {}",
            "Note".bright_black(),
            note_uid.to_string().blue(),
            "updated".bright_black(),
            content
          );
        }

        Event::SetProject { project, .. } => {
          println!(
            "{} {}",
            "Project set to".bright_black(),
            Self::friendly_project(project)
          );
        }

        Event::SetPriority { priority, .. } => {
          println!(
            "{} {}",
            "Priority set to".bright_black(),
            self.friendly_priority(*priority)
          );
        }

        Event::AddTag { tag, .. } => {
          println!("{}{}", "Tag added #".bright_black(), tag.yellow());
        }
      }
    }
  }

  pub fn rename_project(
    task_mgr: &mut TaskManager,
    current_project: impl AsRef<str>,
    new_project: impl AsRef<str>,
  ) {
    let current_project = current_project.as_ref();
    let new_project = new_project.as_ref();
    let mut count = 0;

    task_mgr.rename_project(&current_project, &new_project, |_| {
      count += 1;
    });

    if count != 0 {
      println!("updated {} tasks", count);
    } else {
      println!("{}", "no task for this project".yellow());
    }
  }
}

/// Display options to use when rendering in CLI.
struct DisplayOptions {
  /// Width of the task UID column.
  task_uid_width: usize,
  /// Width of the task age column.
  age_width: usize,
  /// Width of the task spent column.
  spent_width: usize,
  /// Width of the task status column.
  status_width: usize,
  /// Width of the task description column.
  description_width: usize,
  /// Width of the task project column.
  project_width: usize,
  /// Width of the task tags column.
  tags_width: usize,
  /// Whether any task has spent time.
  has_spent_time: bool,
  /// Whether we have a priority in at least one task.
  has_priorities: bool,
  /// Whether we have a project in at least one task.
  has_projects: bool,
  /// Whether we have a tag in at least one task.
  has_tags: bool,
  /// Offset to use for the description column.
  description_offset: usize,
  /// Maximum columns to fit in the description column.
  ///
  /// [`None`] implies that the dimension of the terminal don’t allow for descriptions.
  max_description_cols: Option<usize>,
  /// With of the number of notes column.
  ///
  /// `0` indicates no data.
  notes_nb_width: usize,
}

impl DisplayOptions {
  /// Create a new renderer for a set of tasks.
  fn new<'a>(
    config: &Config,
    term: &impl Terminal,
    tasks: impl IntoIterator<Item = (UID, &'a Task)>,
  ) -> Self {
    // FIXME: switch to a builder pattern here, because it’s starting to becoming a mess
    let (
      task_uid_width,
      age_width,
      spent_width,
      status_width,
      description_width,
      project_width,
      tags_width,
      has_spent_time,
      has_priorities,
      has_projects,
      has_tags,
      notes_nb_width,
    ) = tasks.into_iter().fold(
      (0, 0, 0, 0, 0, 0, 0, false, false, false, false, 0),
      |(
        task_uid_width,
        age_width,
        spent_width,
        status_width,
        description_width,
        project_width,
        tags_width,
        has_spent_time,
        has_priorities,
        has_projects,
        has_tags,
        notes_nb_width,
      ),
       (uid, task)| {
        let task_uid_width = task_uid_width.max(Self::guess_task_uid_width(uid));
        let age_width = age_width.max(Self::guess_duration_width(&task.age()));
        let spent_width = spent_width.max(Self::guess_duration_width(&task.spent_time()));
        let status_width = status_width.max(Self::guess_task_status_width(&config, task.status()));
        let description_width = description_width.max(task.name().width());
        let project_width = project_width.max(Self::guess_task_project_width(&task).unwrap_or(0));
        let tags_width = tags_width.max(Self::guess_tags_width(&task));
        let has_spent_time = has_spent_time || task.spent_time() != Duration::zero();
        let has_priorities = has_priorities || task.priority().is_some();
        let has_projects = has_projects || task.project().is_some();
        let has_tags = has_tags || task.tags().next().is_some();
        let notes_nb_width = notes_nb_width.max(Self::guess_notes_width(
          task.notes().iter().map(|note| note.content.as_str()),
        ));

        (
          task_uid_width,
          age_width,
          spent_width,
          status_width,
          description_width,
          project_width,
          tags_width,
          has_spent_time,
          has_priorities,
          has_projects,
          has_tags,
          notes_nb_width,
        )
      },
    );

    let mut opts = Self {
      task_uid_width: task_uid_width.max(config.uid_col_name().width()),
      age_width: age_width.max(config.age_col_name().width()),
      spent_width: spent_width.max(config.spent_col_name().width()),
      status_width: status_width.max(config.status_col_name().width()),
      description_width: description_width.max(config.description_col_name().width()),
      project_width: project_width.max(config.project_col_name().width()),
      tags_width: tags_width.max(config.tags_col_name().width()),
      has_spent_time,
      has_priorities,
      has_projects,
      has_tags,
      description_offset: 0,
      max_description_cols: None,
      notes_nb_width,
    };

    opts.description_offset = opts.guess_description_col_offset(config);

    if let Some(term_dims) = term.dimensions() {
      opts.max_description_cols = term_dims[0].checked_sub(opts.description_offset);
    } else {
      println!(
        "{}",
        "⚠ You’re using a terminal that doesn’t expose its dimensions; expect broken output ⚠"
          .yellow()
          .bold()
      );
    }

    opts
  }

  /// Guess the number of characters needed to represent a number.
  ///
  /// We limit ourselves to number < 100000.
  fn guess_number_width(mut val: usize) -> usize {
    let mut w = 1;

    while val >= 10 {
      val /= 10;
      w += 1;
    }

    w
  }

  /// Guess the width required to represent the task UID.
  fn guess_task_uid_width(uid: UID) -> usize {
    Self::guess_number_width(uid.val() as _)
  }

  /// Guess the width required to represent a duration.
  ///
  /// The width is smart enough to take into account the unit (s, min, h, d, w, m or y) as well as the number.
  fn guess_duration_width(dur: &Duration) -> usize {
    if dur.num_minutes() < 1 {
      // seconds, encoded with "Ns"
      Self::guess_number_width(dur.num_seconds() as _) + "s".len()
    } else if dur.num_hours() < 1 {
      // minutes, encoded with "Nmin"
      Self::guess_number_width(dur.num_minutes() as _) + "min".len()
    } else if dur.num_days() < 1 {
      // hours, encoded with "Nh"
      Self::guess_number_width(dur.num_hours() as _) + "h".len()
    } else if dur.num_weeks() < 2 {
      // days, encoded with "Nd"
      Self::guess_number_width(dur.num_days() as _) + "d".len()
    } else if dur.num_weeks() < 4 {
      // weeks, encoded with "Nw"
      Self::guess_number_width(dur.num_weeks() as _) + "w".len()
    } else {
      // months, encoded with "Nmth"
      Self::guess_number_width(dur.num_weeks() as usize / 4) + "mth".len()
    }
  }

  /// Guess the width required to represent the task status.
  fn guess_task_status_width(config: &Config, status: Status) -> usize {
    let width = match status {
      Status::Ongoing => config.wip_alias().width(),
      Status::Todo => config.todo_alias().width(),
      Status::Done => config.done_alias().width(),
      Status::Cancelled => config.cancelled_alias().width(),
    };

    width.max("Status".len())
  }

  fn guess_task_project_width(task: &Task) -> Option<usize> {
    task.project().map(UnicodeWidthStr::width)
  }

  /// Guess the width required to represent the task tags.
  fn guess_tags_width(task: &Task) -> usize {
    Itertools::intersperse(task.tags(), ", ")
      .map(UnicodeWidthStr::width)
      .sum()
  }

  /// Compute the column offset at which descriptions can start.
  ///
  /// The way we compute this is by summing all the display width and adding the require padding.
  fn guess_description_col_offset(&self, config: &Config) -> usize {
    let spent_width;
    let prio_width;
    let project_width;
    let tags_width;
    let notes_nb_width;

    if config.display_empty_cols() {
      spent_width = self.spent_width + 1;
      prio_width = config.prio_col_name().width() + 1;
      project_width = self.project_width + 1;
      tags_width = self.tags_width + 1;
      notes_nb_width = self.notes_nb_width + 1;
    } else {
      // compute spent time if any
      if self.has_spent_time {
        spent_width = self.spent_width + 1;
      } else {
        spent_width = 0;
      }

      // compute priority width if any
      if self.has_priorities {
        prio_width = config.prio_col_name().width() + 1;
      } else {
        prio_width = 0;
      }

      // compute project width if any
      if self.has_projects {
        project_width = self.project_width + 1; // FIXME
      } else {
        project_width = 0;
      }

      // compute tags width if any
      if config.display_tags_listings() && self.has_tags {
        tags_width = self.tags_width + 1; // FIXME
      } else {
        tags_width = 0;
      }

      // compute notes number width if any
      if self.notes_nb_width != 0 {
        notes_nb_width = config.notes_nb_col_name().width() + 1;
      } else {
        notes_nb_width = 0;
      }
    }

    // The “+ 1” are there because of the blank spaces we have in the output to separate columns.
    1 + self.task_uid_width
      + 1
      + self.age_width
      + 1
      + spent_width
      + prio_width
      + project_width
      + tags_width
      + notes_nb_width
      + self.status_width
      + 1 // to end up on the first column in the description
  }

  /// Guess the maximum width to align notes.
  fn guess_notes_width<'a>(notes: impl Iterator<Item = &'a str>) -> usize {
    let nb = notes.count();

    if nb == 0 {
      0
    } else {
      Self::guess_number_width(nb)
    }
  }
}

/// Friendly string representation of a date.
pub fn date_time_to_string(date_time: &DateTime<Utc>) -> String {
  date_time.format("%a, %d %b %Y at %H:%M").to_string()
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
) -> Result<String, SubCmdError> {
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

  let note_content = interactively_edit(config, "NEW_NOTE.md", &prefill)?;

  if with_history {
    if let Some(marker_index) = note_content.find(PREVIOUS_NOTES_HELP_END_MARKER) {
      let content = note_content
        .get(marker_index + PREVIOUS_NOTES_HELP_END_MARKER.len()..)
        .unwrap();

      if content.trim().is_empty() {
        Err(SubCmdError::EmptyNote)
      } else {
        Ok(content.to_owned())
      }
    } else {
      Err(SubCmdError::CannotEditNote(
        "I told you not to temper with this line!".to_owned(),
      ))
    }
  } else {
    if note_content.trim().is_empty() {
      Err(SubCmdError::EmptyNote)
    } else {
      Ok(note_content)
    }
  }
}

#[cfg(test)]
mod unit_tests {
  use super::*;

  use toodoux::config::{ColorConfig, MainConfig};

  struct DummyTerm {
    dimensions: [usize; 2],
  }

  impl DummyTerm {
    pub fn new(dimensions: [usize; 2]) -> Self {
      Self { dimensions }
    }
  }

  impl Terminal for DummyTerm {
    fn dimensions(&self) -> Option<[usize; 2]> {
      Some(self.dimensions)
    }
  }

  #[test]
  fn guess_number_width() {
    for i in 0..10 {
      assert_eq!(DisplayOptions::guess_number_width(i), 1);
    }

    for i in 10..100 {
      assert_eq!(DisplayOptions::guess_number_width(i), 2);
    }

    for i in 100..1000 {
      assert_eq!(DisplayOptions::guess_number_width(i), 3);
    }
  }

  #[test]
  fn guess_duration_width() {
    assert_eq!(
      DisplayOptions::guess_duration_width(&Duration::seconds(5)),
      2
    ); // 5s
    assert_eq!(
      DisplayOptions::guess_duration_width(&Duration::seconds(10)),
      3
    ); // 10s
    assert_eq!(
      DisplayOptions::guess_duration_width(&Duration::seconds(60)),
      4
    ); // 1min
    assert_eq!(
      DisplayOptions::guess_duration_width(&Duration::minutes(59)),
      5
    ); // 59min
  }

  #[test]
  fn display_options_term_width() {
    let main_config = MainConfig::default();
    let config = Config::new(main_config, ColorConfig::default());
    let tasks = &[(UID::default(), &Task::new("Foo"))];
    let term = DummyTerm::new([100, 1]);
    let opts = DisplayOptions::new(&config, &term, tasks.iter().copied());

    let description_offset = " UID ".len() + "Age ".len() + "Status ".len();
    assert_eq!(opts.description_offset, description_offset,);
    assert_eq!(
      opts.max_description_cols,
      Some(term.dimensions().unwrap()[0] - description_offset)
    );
  }

  #[test]
  fn display_options_should_yield_no_description_if_too_short() {
    let main_config = MainConfig::default();
    let config = Config::new(main_config, ColorConfig::default());
    let tasks = &[(UID::default(), &Task::new("Foo"))];
    let term = DummyTerm::new([100, 1]);
    let opts = DisplayOptions::new(&config, &term, tasks.iter().copied());

    let description_offset = " UID ".len() + "Age ".len() + "Status ".len();
    assert_eq!(opts.description_offset, description_offset,);
    assert_eq!(
      opts.max_description_cols,
      Some(term.dimensions().unwrap()[0] - description_offset)
    );
  }
}
