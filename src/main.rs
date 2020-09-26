mod cli;
mod config;
mod task;

use crate::{
  cli::{Command, SubCommand},
  config::Config,
  task::{Status, TaskManager},
};
use chrono::Utc;
use colored::Colorize as _;
use std::{
  error::Error,
  io::{self, Write as _},
  path::Path,
};
use structopt::StructOpt;
use task::{Task, UID};

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

fn print_wizzard_question() {
  print!(
    "\n{wizzard_question} ({Y}/{n}) ➤ ",
    wizzard_question =
      "You don’t seem to have a configuration set up…\nWould you like to set it up?".blue(),
    Y = "Y".green().bold(),
    n = "n".red(),
  );

  io::stdout().flush().unwrap();
}

fn print_no_file_information() {
  println!("\n{toodoux} {rest}", toodoux = "toodoux".purple().bold(), rest = "won’t work without a configuration file. If you don’t want to generate it via this interactive wizzard, you can create it by hand and put it in the right folder, depending on the platform you run on.".red());
}

fn main() -> Result<(), Box<dyn Error>> {
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
) -> Result<(), Box<dyn Error>> {
  let path = config_path.as_ref();
  let config = Config::from_dir(path)?;

  initiate_with_config(Some(path), config, subcmd, task_uid)
}

/// Initiate configuration by using the default configuration path.
fn initiate(subcmd: Option<SubCommand>, task_uid: Option<UID>) -> Result<(), Box<dyn Error>> {
  let config = Config::get()?;
  initiate_with_config(None, config, subcmd, task_uid)
}

fn initiate_with_config(
  path: Option<&Path>,
  config: Option<Config>,
  subcmd: Option<SubCommand>,
  task_uid: Option<UID>,
) -> Result<(), Box<dyn Error>> {
  match config {
    // explicit configuration
    Some(config) => {
      log::info!(
        "running on configuration at {}",
        config.root_dir().display()
      );
      run_subcmd(config, subcmd, task_uid)
    }

    // no configuration; create it
    None => {
      log::warn!("no configuration detected");

      let mut input = String::new();

      // initiate configuration file creation wizzard and create the configuration file
      print_introduction_text();

      let must_create_config_file = loop {
        input.clear();

        print_wizzard_question();
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
        let config = Config::create(path).ok_or_else(|| "cannot create config file")?;
        config.save()?;

        run_subcmd(config, subcmd, task_uid)
      } else {
        print_no_file_information();
        Ok(())
      }
    }
  }
}

fn run_subcmd(
  config: Config,
  subcmd: Option<SubCommand>,
  task_uid: Option<UID>,
) -> Result<(), Box<dyn Error>> {
  match subcmd {
    // default subcommand
    None => {}
    Some(subcmd) => match subcmd {
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

      SubCommand::Edit { name } => {}

      SubCommand::Remove { all } => {}

      SubCommand::List {
        todo,
        start,
        done,
        cancelled,
        all,
        content,
      } => list_tasks(config, todo, start, cancelled, done)?,
    },
  }

  Ok(())
}

/// Add a new task.
fn add_task(config: Config, start: bool, done: bool, name: String) -> Result<(), Box<dyn Error>> {
  let mut task_mgr = TaskManager::new_from_config(&config)?;
  let mut task = Task::new(name, "", Vec::new());

  // determine if we need to switch to another status
  if start {
    task.change_status(Status::Ongoing);
  } else if done {
    task.change_status(Status::Done);
  }

  let uid = task_mgr.register_task(task.clone());
  task_mgr.save(&config)?;

  display_task_header();
  display_task(&config, uid, &task, true);

  Ok(())
}

/// List tasks.
fn list_tasks(
  config: Config,
  todo: bool,
  start: bool,
  cancelled: bool,
  done: bool,
) -> Result<(), Box<dyn Error>> {
  let task_mgr = TaskManager::new_from_config(&config)?;
  let mut tasks: Vec<_> = task_mgr.tasks().collect();
  tasks.sort_by_key(|(_, task)| task.status());

  display_task_header();

  let mut parity = true;
  for (&uid, task) in tasks {
    display_task(&config, uid, task, parity);
    parity = !parity;
  }

  Ok(())
}

/// Display the header of tasks.
fn display_task_header() {
  println!(
    " {uid:<5} {age:8} {status:<12}  {name:<120}",
    uid = "UID".underline(),
    age = "Age".underline(),
    status = "Status".underline(),
    name = "Description".underline()
  );
}

/// Display a task to the user.
fn display_task(config: &Config, uid: UID, task: &Task, parity: bool) {
  let (name, status);
  match task.status() {
    Status::Todo => {
      if parity {
        name = task.name().bright_white().on_black();
      } else {
        name = task.name().bright_white().on_bright_black();
      }
      status = config.todo_alias().clone().bold().magenta();
    }

    Status::Ongoing => {
      name = task.name().black().on_bright_green();
      status = config.wip_alias().clone().bold().green();
    }

    Status::Done => {
      name = task.name().bright_black().dimmed().on_black();
      status = config.done_alias().clone().dimmed().bright_black();
    }

    Status::Cancelled => {
      name = task
        .name()
        .bright_black()
        .dimmed()
        .strikethrough()
        .on_black();
      status = config.cancelled_alias().clone().dimmed().bright_red();
    }
  }

  let output = format!(
    " {uid:^5} {age:^8} {status:^12}  {name:<120}",
    uid = uid,
    age = friendly_age(task),
    status = status,
    name = name,
  );

  println!("{:<120}", output);
}

/// Find out the age of a task and get a friendly representation.
fn friendly_age(task: &Task) -> String {
  let dur =
    Utc::now().signed_duration_since(task.creation_date().cloned().unwrap_or_else(|| Utc::now()));

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
