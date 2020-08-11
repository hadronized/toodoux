mod cli;
mod config;
mod markup;
mod task;
mod view;

use colored::Colorize;
use std::error::Error;
use std::io::{self, Write as _};
use std::path::Path;
use structopt::StructOpt;

use crate::cli::{Command, SubCommand};
use crate::config::Config;
use crate::task::{State, TaskManager};
use crate::view::View as _;

fn print_introduction_text() {
  println!(
    "Hello! It seems like you’re new to {toodoux}!

{toodoux} is a modern take on task / todo lists, mostly based on the amazing emacs’ {org_mode}. Instead of recreating the same plugin inside everybody’s editors over and over, {toodoux} takes it the UNIX way and just does one thing good: {editing_tasks}.

You will first be able to {add_tasks} new tasks, {edit_tasks} existing tasks, {remove_tasks} some and {list_tasks} them all. Then, you will be able to enjoy more advanced features, such as {capturing} and {refiling} tasks, {filtering} them, as well as {scheduling} and {putting_deadlines}. Time metadata are automatically handled for you to follow along.",
    toodoux = "toodoux".purple().bold(),
    org_mode = "Org-Mode".purple().bold(),
    editing_tasks = "editing tasks".bold(),
    add_tasks = "add".green().bold(),
    edit_tasks = "edit".green().bold(),
    remove_tasks = "remove".green().bold(),
    list_tasks = "list".green().bold(),
    capturing = "capturing".green().bold(),
    refiling = "refiling".green().bold(),
    filtering = "filtering".green().bold(),
    scheduling = "scheduling".green().bold(),
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
  let Command { subcmd, config } = Command::from_args();

  // initialize the logger
  log::trace!("initializing logger");
  env_logger::init();

  // override the config if explicitly passed a configuration path; otherwise, use the one by
  // default
  log::trace!("initializing configuration");
  match config {
    Some(path) => initiate_explicit_config(path, subcmd),
    None => initiate(subcmd),
  }
}

fn initiate_explicit_config(
  config_path: impl AsRef<Path>,
  subcmd: Option<SubCommand>,
) -> Result<(), Box<dyn Error>> {
  let path = config_path.as_ref();
  let config = Config::from_dir(path)?;

  initiate_with_config(Some(path), config, subcmd)
}

fn initiate(subcmd: Option<SubCommand>) -> Result<(), Box<dyn Error>> {
  let config = Config::get()?;
  initiate_with_config(None, config, subcmd)
}

fn initiate_with_config(
  path: Option<&Path>,
  config: Option<Config>,
  subcmd: Option<SubCommand>,
) -> Result<(), Box<dyn Error>> {
  match config {
    Some(config) => {
      log::info!(
        "running on configuration at {}",
        config.root_dir().display()
      );
      run_subcmd(config, subcmd)
    }

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

        Ok(())
      } else {
        print_no_file_information();
        Ok(())
      }
    }
  }
}

fn run_subcmd(config: Config, subcmd: Option<SubCommand>) -> Result<(), Box<dyn Error>> {
  match subcmd {
    // default command
    None => {
      let task_mgr = TaskManager::new_from_config(&config)?;
      let mut view = view::cli::CLIView::new(&config);

      view.display_filtered_tasks(task_mgr.tasks(), |_, task| match task.state() {
        State::Todo(_) | State::Ongoing(_) => true,
        _ => false,
      });
    }

    Some(cmd) => match cmd {
      SubCommand::Add {
        content,
        ongoing,
        done,
      } => {
        let mut task_mgr = TaskManager::new_from_config(&config)?;

        // interactive mode if no content is provided
        let (uid, task) = if content.is_empty() {
          let markup = markup::markdown::Markdown::new();
          task_mgr.create_task_from_editor(&config, markup)?
        } else {
          let name = content.join(" ");

          let state = if ongoing {
            State::Ongoing(config.ongoing_state_name().to_owned())
          } else if done {
            State::Done(config.done_state_name().to_owned())
          } else {
            State::Todo(config.todo_state_name().to_owned())
          };

          task_mgr.create_task(name, "", state, Vec::new())
        };

        task_mgr.save(&config)?;
      }

      SubCommand::Edit {
        uid,
        name,
        todo,
        ongoing,
        done,
      } => {
        let mut task_mgr = TaskManager::new_from_config(&config)?;
        match task_mgr.get_mut(&uid) {
          Some(task) => {
            let mut interactive = name.is_none() && !todo && !ongoing && !done;

            if interactive {
              println!("starting interactive mode");
            }

            // update the name
            if let Some(name) = name {
              task.change_name(name.join(" "));
            }

            // update the state
            if todo {
              task.change_state(State::Todo(config.todo_state_name().to_owned()));
            }

            if ongoing {
              task.change_state(State::Ongoing(config.ongoing_state_name().to_owned()));
            }

            if done {
              task.change_state(State::Done(config.done_state_name().to_owned()));
            }

            task_mgr.save(&config);
          }

          None => {
            eprintln!("no such task {}", uid);
          }
        }
      }

      SubCommand::Remove { .. } => todo!(),

      SubCommand::List {
        mut todo,
        mut ongoing,
        mut done,
        all,
        content,
      } => {
        let task_mgr = TaskManager::new_from_config(&config)?;
        let mut view = view::cli::CLIView::new(&config);

        // filter the tasks; if no flag are passed, then we assume todo and ongoing
        if !(todo || ongoing || done) {
          todo = true;
          ongoing = true;
        }

        if all {
          todo = true;
          ongoing = true;
          done = true;
        }

        view.display_filtered_tasks(task_mgr.tasks(), |_, task| match task.state() {
          State::Todo(_) => todo,
          State::Ongoing(_) => ongoing,
          State::Done(_) => done,
        });
      }

      _ => (),
    },
  }

  Ok(())
}
