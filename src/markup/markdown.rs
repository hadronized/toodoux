//! Markdown support for task markup.

use pulldown_cmark::{CowStr, Event, Parser, Tag};
use std::io::Write;

use crate::config::Config;
use crate::markup::{Markup, MarkupError};
use crate::task::{State, Task, TaskManager, UID};

/// The Markdown language.
#[derive(Debug)]
pub struct Markdown {
  title: String,
  content: String,
}

type EventStack<'a> = Vec<Event<'a>>;

impl Markup for Markdown {
  const EXT: &'static [&'static str] = &["md"];

  fn from_str(
    mut self,
    src: impl AsRef<str>,
    config: &Config,
    task_mgr: &mut TaskManager,
  ) -> Result<(UID, Task), MarkupError> {
    self.parse(src)?;

    self.into_task(config, task_mgr)
  }

  fn to_write(
    self,
    f: &mut impl Write,
    config: &Config,
    task_mgr: &mut TaskManager,
    task: &Task,
  ) -> Result<(), MarkupError> {
    unimplemented!()
  }
}

impl Markdown {
  pub fn new() -> Self {
    Self {
      title: String::new(),
      content: String::new(),
    }
  }

  pub fn into_task(
    self,
    config: &Config,
    task_mgr: &mut TaskManager,
  ) -> Result<(UID, Task), MarkupError> {
    if self.title.is_empty() {
      Err(MarkupError::CannotFindTitle)
    } else {
      Ok(task_mgr.create_task(
        self.title,
        self.content,
        State::Todo(config.todo_state_name().to_owned()),
        Vec::new(),
      ))
    }
  }

  pub fn parse(&mut self, input: impl AsRef<str>) -> Result<(), MarkupError> {
    let input = input.as_ref();
    let parser = Parser::new(input);
    let mut event_stack = Vec::new();

    for event in parser {
      self.dispatch_event(&mut event_stack, event)?;
    }

    Ok(())
  }

  fn dispatch_event<'a>(
    &mut self,
    event_stack: &mut EventStack<'a>,
    event: Event<'a>,
  ) -> Result<(), MarkupError> {
    match event {
      Event::Start(ref tag) => {
        // treat tag
        println!("entering {:?}", tag);
        event_stack.push(event);
        Ok(())
      }

      Event::End(ref tag) => {
        // treat end
        println!("leaving {:?}", tag);
        event_stack.pop();
        Ok(())
      }

      Event::Text(t) => {
        println!("read text: {}", t);
        self.dispatch_text(event_stack, t)
      }

      _ => Ok(()), // ignored
    }
  }

  fn dispatch_text(
    &mut self,
    event_stack: &mut EventStack,
    text: CowStr,
  ) -> Result<(), MarkupError> {
    match event_stack.last() {
      // title
      Some(Event::Start(Tag::Heading(1))) => {
        if self.title.is_empty() {
          self.title = text.into_string();
          Ok(())
        } else {
          Err(MarkupError::AmbiguousTitle(self.title.clone()))
        }
      }

      Some(Event::Start(Tag::Heading(_))) => Err(MarkupError::UnknownMarkupFormat(
        "Only the level-1 Markdown heading is supported".to_owned(),
      )),

      // content
      Some(Event::Start(Tag::Paragraph)) => {
        if !self.content.is_empty() {
          self.content.push_str("\n");
        }

        self.content.push_str(text.as_ref());
        Ok(())
      }

      _ => Ok(()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_md() {
    let content = r"
# Title
This is a test.
Another test
";
    let mut markdown = Markdown::new();
    markdown.parse(content).unwrap();
    println!("result: {:?}", markdown);
  }
}
