mod cli;
mod config_file;

use colored::Colorize;
use std::error::Error;
use std::io::{self, Write as _};
use structopt::StructOpt;

use crate::config_file::ConfigFile;

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
  match ConfigFile::get() {
    Some(config) => Ok(()),

    None => {
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
        let config = ConfigFile::create().ok_or_else(|| "cannot create config file")?;
        config.save()?;
        Ok(())
      } else {
        print_no_file_information();
        Ok(())
      }
    }
  }
}
