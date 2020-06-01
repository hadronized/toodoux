//! Markdown support for task markup.

use pulldown_cmark::{CowStr, Event, Parser, Tag};

use crate::markup::{Markup, MarkupError};

/// The Markdown language.
#[derive(Debug)]
pub struct Markdown {
  title: String,
  content: String,
}

type EventStack<'a> = Vec<Event<'a>>;

impl Markdown {
  pub fn new() -> Self {
    Self {
      title: String::new(),
      content: String::new(),
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
