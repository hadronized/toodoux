//! CLI options.

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "toodoux",
  about = "A modern task / todo / note management tool."
)]
pub enum Command {
  /// Add a task.
  Add {
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
  List,
}
