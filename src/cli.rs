//! Command line interface.

use chrono::{Duration, Utc};
use colored::Colorize;
use std::{cmp::Reverse, error::Error, fmt::Display, iter::once, path::PathBuf};
use structopt::StructOpt;
use unicode_width::UnicodeWidthStr;

use crate::{
  config::{Config, HighlightedString},
  metadata::{Metadata, Priority},
  task::{Status, Task, TaskManager, UID},
  term::Term,
};

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
    content: Vec<String>,
  },

  /// Edit a task.
  #[structopt(visible_aliases = &["e", "ed"])]
  Edit {
    /// Change the name or metadata of the task.
    content: Vec<String>,
  },

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

    /// Show the content of each listed task, if any.
    #[structopt(long)]
    content: bool,

    /// Metadata filter.
    metadata_filter: Vec<String>,
  },
}

/// List tasks.
pub fn list_tasks(
  config: &Config,
  term: &impl Term,
  todo: bool,
  start: bool,
  cancelled: bool,
  done: bool,
  metadata_filter: Vec<String>,
) -> Result<(), Box<dyn Error>> {
  // extract metadata if any
  let (metadata, name) = Metadata::from_words(metadata_filter.iter().map(|s| s.as_str()));
  Metadata::validate(&metadata)?;

  if !metadata.is_empty() {
    print!("{}", "[ ".bright_black());

    print!("{}", metadata[0].filter_like());

    for md in &metadata[1..] {
      print!(", {}", md.filter_like());
    }
    println!("{}", " ]".bright_black());
  }

  if !name.is_empty() {
    println!("{}", "name filtering is not yet supported… sorry :(".red());
  }

  let task_mgr = TaskManager::new_from_config(config)?;
  let mut tasks: Vec<_> = task_mgr
    .tasks()
    .filter(|(_, task)| {
      // filter the task depending on what is passed as argument
      let status_filter = match task.status() {
        Status::Ongoing => start,
        Status::Todo => todo,
        Status::Done => done,
        Status::Cancelled => cancelled,
      };

      if metadata_filter.is_empty() {
        status_filter
      } else {
        status_filter && task.check_metadata(metadata.iter())
      }
    })
    .collect();
  tasks.sort_by_key(|&(uid, task)| Reverse((task.priority(), task.age(), task.status(), uid)));

  // precompute a bunch of data for display widths / padding / etc.
  let display_opts =
    DisplayOptions::new(config, term, tasks.iter().map(|&(uid, task)| (*uid, task)));

  // actual display
  // only display header if there are tasks to display
  if tasks.len() > 0 {
    display_task_header(config, &display_opts);
  }

  for (&uid, task) in tasks {
    display_task_inline(config, uid, task, &display_opts);
  }

  Ok(())
}

/// Add a new task.
pub fn add_task(
  config: &Config,
  term: &impl Term,
  start: bool,
  done: bool,
  content: Vec<String>,
) -> Result<(), Box<dyn Error>> {
  // validate the metadata extracted from the content, if any
  let (metadata, name) = Metadata::from_words(content.iter().map(|s| s.as_str()));
  Metadata::validate(&metadata)?;

  let mut task_mgr = TaskManager::new_from_config(config)?;
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
  task_mgr.save(config)?;

  // display options
  let display_opts = DisplayOptions::new(config, term, once((uid, &task)));

  display_task_header(config, &display_opts);
  display_task_inline(config, uid, &task, &display_opts);

  Ok(())
}

/// Edit a task’s name or metadata.
pub fn edit_task(task: &mut Task, content: Vec<String>) -> Result<(), Box<dyn Error>> {
  // validate the metadata extracted from the content, if any
  let (metadata, name) = Metadata::from_words(content.iter().map(|s| s.as_str()));
  Metadata::validate(&metadata)?;

  // apply the metadata
  task.apply_metadata(metadata);

  // if we have a new name, apply it too
  if !name.is_empty() {
    task.change_name(name);
  }

  Ok(())
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
  /// Whether any task has spent time.
  has_spent_time: bool,
  /// Whether we have a priority in at least one task.
  has_priorities: bool,
  /// Whether we have a project in at least one task.
  has_projects: bool,
  /// Offset to use for the description column.
  description_offset: usize,
  /// Maximum columns to fit in the description column.
  ///
  /// [`None`] implies that the dimension of the terminal don’t allow for descriptions.
  max_description_cols: Option<usize>,
}

impl DisplayOptions {
  /// Create a new renderer for a set of tasks.
  fn new<'a>(
    config: &Config,
    term: &impl Term,
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
      has_spent_time,
      has_priorities,
      has_projects,
    ) = tasks.into_iter().fold(
      (0, 0, 0, 0, 0, 0, false, false, false),
      |(
        task_uid_width,
        age_width,
        spent_width,
        status_width,
        description_width,
        project_width,
        has_spent_time,
        has_priorities,
        has_projects,
      ),
       (uid, task)| {
        let task_uid_width = task_uid_width.max(Self::guess_task_uid_width(uid));
        let age_width = age_width.max(Self::guess_duration_width(&task.age()));
        let spent_width = spent_width.max(Self::guess_duration_width(&task.spent_time()));
        let status_width = status_width.max(Self::guess_task_status_width(&config, task.status()));
        let description_width = description_width.max(task.name().width());
        let project_width = project_width.max(Self::guess_task_project_width(&task).unwrap_or(0));
        let has_spent_time = has_spent_time || task.spent_time() != Duration::zero();
        let has_priorities = has_priorities || task.priority().is_some();
        let has_projects = has_projects || task.project().is_some();

        (
          task_uid_width,
          age_width,
          spent_width,
          status_width,
          description_width,
          project_width,
          has_spent_time,
          has_priorities,
          has_projects,
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
      has_spent_time,
      has_priorities,
      has_projects,
      description_offset: 0,
      max_description_cols: None,
    };

    opts.description_offset = opts.guess_description_col_offset(config);
    opts.max_description_cols = term.dimensions().unwrap()[0].checked_sub(opts.description_offset);

    opts
  }

  /// Guess the number of characters needed to represent a number.
  ///
  /// We limit ourselves to number < 100000.
  fn guess_number_width(val: usize) -> usize {
    if val < 10 {
      1
    } else if val < 100 {
      2
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
      // months, encoded with "Nm"
      Self::guess_number_width(dur.num_weeks() as usize / 4) + "m".len()
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

  /// Compute the column offset at which descriptions can start.
  ///
  /// The way we compute this is by summing all the display width and adding the require padding.
  fn guess_description_col_offset(&self, config: &Config) -> usize {
    let spent_width;
    let prio_width;
    let project_width;

    if config.display_empty_cols() {
      spent_width = self.spent_width + 1;
      prio_width = config.prio_col_name().width() + 1;
      project_width = self.project_width + 1;
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
    }

    // The “+ 1” are there because of the blank spaces we have in the output to separate columns.
    1 + self.task_uid_width
      + 1
      + self.age_width
      + 1
      + spent_width
      + prio_width
      + project_width
      + self.status_width
      + 1 // to end up on the first column in the description
  }
}

/// Display the header of tasks.
fn display_task_header(config: &Config, opts: &DisplayOptions) {
  print!(
    " {uid:<uid_width$} {age:<age_width$}",
    uid = config.uid_col_name().underline(),
    uid_width = opts.task_uid_width,
    age = config.age_col_name().underline(),
    age_width = opts.age_width,
  );

  let display_empty_cols = config.display_empty_cols();

  if display_empty_cols || opts.has_spent_time {
    print!(
      " {spent:<spent_width$}",
      spent = config.spent_col_name().underline(),
      spent_width = opts.spent_width,
    );
  }

  if display_empty_cols || opts.has_priorities {
    print!(
      " {priority:<prio_width$}",
      priority = config.prio_col_name().underline(),
      prio_width = config.prio_col_name().width(),
    );
  }

  if display_empty_cols || opts.has_projects {
    print!(
      " {project:<project_width$}",
      project = config.project_col_name().underline(),
      project_width = opts.project_width,
    );
  }

  if let Some(max_description_cols) = opts.max_description_cols {
    println!(
      " {status:<status_width$} {description:<description_width$}",
      status = config.status_col_name().underline(),
      status_width = opts.status_width,
      description = config.description_col_name().underline(),
      description_width = opts.description_width.min(max_description_cols),
    );
  }
}

/// Display a task to the user.
fn display_task_inline(config: &Config, uid: UID, task: &Task, opts: &DisplayOptions) {
  let status;
  let task_name = task.name();
  let task_status = task.status();

  match task_status {
    Status::Todo => {
      status = config.colors.status.todo.highlight(config.todo_alias());
    }

    Status::Ongoing => {
      status = config.colors.status.ongoing.highlight(config.wip_alias());
    }

    Status::Done => {
      status = config.colors.status.done.highlight(config.done_alias());
    }

    Status::Cancelled => {
      status = config
        .colors
        .status
        .cancelled
        .highlight(config.cancelled_alias());
    }
  }

  let spent_time = friendly_spent_time(task.spent_time(), task_status);

  print!(
    " {uid:<uid_width$} {age:<age_width$}",
    uid = uid,
    uid_width = opts.task_uid_width,
    age = friendly_task_age(task),
    age_width = opts.age_width,
  );

  let display_empty_cols = config.display_empty_cols();

  if display_empty_cols || opts.has_spent_time {
    print!(
      " {spent:<spent_width$}",
      spent = spent_time,
      spent_width = opts.spent_width,
    );
  }

  if display_empty_cols || opts.has_priorities {
    print!(
      " {priority:<prio_width$}",
      priority = friendly_priority(task, config),
      prio_width = config.prio_col_name().width(),
    );
  }

  if display_empty_cols || opts.has_projects {
    print!(
      " {project:<project_width$}",
      project = friendly_project(task),
      project_width = opts.project_width,
    );
  }

  print!(
    " {status:<status_width$}",
    status = status,
    status_width = opts.status_width,
  );

  display_description(config, opts, task_status, task_name);
}

/// Display a description by respecting the allowed description column size.
///
/// The description is not displayed if no space is available on screen.
fn display_description(config: &Config, opts: &DisplayOptions, status: Status, description: &str) {
  if let Some(max_description_cols) = opts.max_description_cols {
    let mut line_index = 0;
    let mut rel_offset = 0;
    let mut line_buffer = String::new();
    let description_width = opts.description_width.min(max_description_cols);

    print!(" ");
    for word in description.split_ascii_whitespace() {
      let word_size = word.width(); // TODO: check what to do about CJK
      let next_offset = rel_offset + word_size + 1;

      if next_offset > max_description_cols {
        // we’ve passed the end of the line; break into another line
        line_index += 1;
        rel_offset = 0;

        if line_index >= config.max_description_lines() {
          line_buffer.push('…');
          break;
        } else {
          let hl_description = highlight_description_line(config, status, &line_buffer);
          print!("{:<width$}", hl_description, width = description_width);

          line_buffer.clear();
          print!("{:<width$}", "", width = opts.description_offset);
        }
      }

      if rel_offset > 0 {
        line_buffer.push(' ');
      }

      line_buffer.push_str(word);
      rel_offset = next_offset;
    }

    let hl_description = highlight_description_line(config, status, &line_buffer);
    println!("{:<width$}", hl_description, width = description_width);
  }
}

/// Highlight a description line
fn highlight_description_line(config: &Config, status: Status, line: &str) -> impl Display {
  match status {
    Status::Todo => config.colors.description.todo.highlight(line),
    Status::Ongoing => config.colors.description.ongoing.highlight(line),
    Status::Done => config.colors.description.done.highlight(line),
    Status::Cancelled => config.colors.description.cancelled.highlight(line),
  }
}

/// Find out the age of a task and get a friendly representation.
fn friendly_task_age(task: &Task) -> String {
  let dur =
    Utc::now().signed_duration_since(task.creation_date().cloned().unwrap_or_else(|| Utc::now()));
  friendly_duration(dur)
}

pub fn friendly_duration(dur: Duration) -> String {
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

fn friendly_priority(task: &Task, config: &Config) -> impl Display {
  if let Some(prio) = task.priority() {
    match prio {
      Priority::Low => config.colors.priority.low.highlight("LOW"),
      Priority::Medium => config.colors.priority.medium.highlight("MED"),
      Priority::High => config.colors.priority.high.highlight("HIGH"),
      Priority::Critical => config.colors.priority.critical.highlight("CRIT"),
    }
  } else {
    HighlightedString::regular("")
  }
}

fn friendly_project(task: &Task) -> impl Display {
  if let Some(project) = task.project() {
    project.italic()
  } else {
    "".into()
  }
}

/// String representation of a spent-time.
///
/// If no time has been spent on this task, an empty string is returned.
fn friendly_spent_time(dur: Duration, status: Status) -> impl Display {
  if dur == Duration::zero() {
    return String::new().normal();
  }

  let output = friendly_duration(dur);

  match status {
    Status::Ongoing => output.blue(),
    _ => output.bright_black(),
  }
}

#[cfg(test)]
mod unit_tests {
  use super::*;

  use crate::config::{ColorConfig, MainConfig};

  struct DummyTerm {
    dimensions: [usize; 2],
  }

  impl DummyTerm {
    pub fn new(dimensions: [usize; 2]) -> Self {
      Self { dimensions }
    }
  }

  impl crate::term::Term for DummyTerm {
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
    let main_config = MainConfig::new(
      "N/A",
      "TODO",
      "WIP",
      "DONE",
      "CANCELLED",
      "UID",
      "Age",
      "Spent",
      "Prio",
      "Project",
      "Status",
      "Desc",
      false,
      2,
    );
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
    let main_config = MainConfig::new(
      "N/A",
      "TODO",
      "WIP",
      "DONE",
      "CANCELLED",
      "UID",
      "Age",
      "Spent",
      "Prio",
      "Project",
      "Status",
      "Desc",
      false,
      2,
    );
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
