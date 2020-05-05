//! CLI options.

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "toodoux",
  about = "A modern task / todo / note management tool."
)]
pub struct Command {
  #[structopt(subcommand)]
  pub subcmd: Option<SubCommand>,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
  /// Add a task.
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
  /// Remove a task.
  Remove {
    /// Remove all the tasks.
    #[structopt(short, long)]
    all: bool,
  },
  /// List all the tasks.
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
