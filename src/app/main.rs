mod cli;
mod interactive_editor;
mod term;

use crate::{
  cli::{Command, SubCmdError, SubCommand},
  term::DefaultTerm,
};
use cli::CLI;
use colored::Colorize as _;

use std::{
  io::{self, Write as _},
  path::Path,
};
use structopt::StructOpt;
use toodoux::task::UID;
use toodoux::{config::Config, task::TaskManager};

fn print_introduction_text() {
  println!(
    "Hello! It seems like you’re new to {toodoux}!

{toodoux} is a modern take on task / todo lists, mostly based on the amazing emacs’ {org_mode} and {taskwarrior}. Instead of recreating the same plugin inside everybody’s favorite editors over and over, {toodoux} takes it the UNIX way and just does one thing good: {editing_tasks}.

You will first be able to {add_tasks} new tasks, {edit_tasks} existing tasks, {remove_tasks} some and {list_tasks} them all. Then, you will be able to enjoy more advanced features, such as {capturing} and {refiling} tasks, {filtering} them, as well as {putting_deadlines}. Time metadata are automatically handled for you to follow along.",
    toodoux = "toodoux".purple().bold(),
    org_mode = "Org-Mode".purple().bold(),
    taskwarrior = "taskwarrior".purple().bold(),
    editing_tasks = "editing tasks".bold(),
    add_tasks = "add".green().bold(),
    edit_tasks = "edit".green().bold(),
    remove_tasks = "remove".green().bold(),
    list_tasks = "list".green().bold(),
    capturing = "capturing".green().bold(),
    refiling = "refiling".green().bold(),
    filtering = "filtering".green().bold(),
    putting_deadlines = "putting deadlines".green().bold(),
  );
}

fn print_wizard_question() {
  print!(
    "\n{wizard_question} ({Y}/{n}) ➤ ",
    wizard_question =
      "You don’t seem to have a configuration set up…\nWould you like to set it up?".blue(),
    Y = "Y".green().bold(),
    n = "n".red(),
  );

  io::stdout().flush().unwrap();
}

fn print_no_file_information() {
  println!("\n{toodoux} {rest}", toodoux = "toodoux".purple().bold(), rest = "won’t work without a configuration file. If you don’t want to generate it via this interactive wizard, you can create it by hand and put it in the right folder, depending on the platform you run on.".red());
}

fn main() {
  if let Err(err) = entry_point() {
    eprintln!("{}", err.to_string().red().bold())
  }
}

fn entry_point() -> Result<(), SubCmdError> {
  let Command {
    subcmd,
    config,
    task_uid,
  } = Command::from_args(); // TODO: use the task_uid

  // initialize the logger
  log::debug!("initializing logger");
  env_logger::init();

  // override the config if explicitly passed a configuration path; otherwise, use the one by provided by default
  log::debug!("initializing configuration");
  match config {
    Some(path) => initiate_explicit_config(path, subcmd, task_uid),
    None => initiate(subcmd, task_uid),
  }
}

/// Initiate configuration with an explicitly provided path.
fn initiate_explicit_config(
  config_path: impl AsRef<Path>,
  subcmd: Option<SubCommand>,
  task_uid: Option<UID>,
) -> Result<(), SubCmdError> {
  let path = config_path.as_ref();
  let config = Config::from_dir(path)?;

  initiate_with_config(Some(path), config, subcmd, task_uid)
}

/// Initiate configuration by using the default configuration path.
fn initiate(subcmd: Option<SubCommand>, task_uid: Option<UID>) -> Result<(), SubCmdError> {
  let config = Config::get()?;
  initiate_with_config(None, config, subcmd, task_uid)
}

fn initiate_with_config(
  path: Option<&Path>,
  config: Option<Config>,
  subcmd: Option<SubCommand>,
  task_uid: Option<UID>,
) -> Result<(), SubCmdError> {
  let term = DefaultTerm;

  match config {
    // explicit configuration
    Some(config) => {
      log::info!(
        "running on configuration at {}",
        config.root_dir().display()
      );

      let mut task_mgr = TaskManager::new_from_config(&config)?;
      CLI::new(config, term).run(&mut task_mgr, subcmd, task_uid)
    }

    // no configuration; create it
    None => {
      log::warn!("no configuration detected");

      let mut input = String::new();

      // initiate configuration file creation wizard and create the configuration file
      print_introduction_text();

      let must_create_config_file = loop {
        input.clear();

        print_wizard_question();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim_end() {
          "Y" | "y" | "" => {
            break true;
          }

          "N" | "n" => {
            break false;
          }

          _ => {
            println!("{}", "I’m so sorry, but I didn’t quite get that.".red());
          }
        }
      };

      if must_create_config_file {
        let config = Config::create(path)?;
        config.save()?;

        let mut task_mgr = TaskManager::new_from_config(&config)?;
        CLI::new(config, term).run(&mut task_mgr, subcmd, task_uid)
      } else {
        print_no_file_information();
        Ok(())
      }
    }
  }
}
