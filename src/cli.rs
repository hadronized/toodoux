//! CLI options.

use std::path::PathBuf;
use structopt::StructOpt;

use crate::task::UID;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "toodoux",
  about = "A modern task / todo / note management tool."
)]
pub struct Command {
  #[structopt(subcommand)]
  pub subcmd: Option<SubCommand>,
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
    ongoing: bool,

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
    /// UID of the task.
    uid: UID,

    /// Change the name of the task.
    #[structopt(short, long)]
    name: Option<Vec<String>>,

    /// Change the state of the task to TODO.
    #[structopt(short, long)]
    todo: bool,

    /// Change the state of the task to ONGOING.
    #[structopt(short, long)]
    ongoing: bool,

    /// Change the state of the task to DONE.
    #[structopt(short, long)]
    done: bool,
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
    /// Filter with TODO items.
    #[structopt(short, long)]
    todo: bool,

    /// Filter with ONGOING items.
    #[structopt(short, long)]
    ongoing: bool,

    /// Filter with DONE items.
    #[structopt(short, long)]
    done: bool,

    /// Do not filter the items and show them all.
    #[structopt(short, long)]
    all: bool,

    /// Show the content of each listed task, if any.
    #[structopt(short, long)]
    content: bool,
  },
}
