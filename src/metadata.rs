//! Metadata available to users for filtering / creating tasks.

use std::{
  error::Error,
  fmt::{self, Display},
  str::FromStr,
};

/// Possible errors that can happen when validating metadata.
#[derive(Debug)]
pub enum MetadataValidationError {
  /// Too many projects; you should use only one or none.
  TooManyProjects(usize),

  /// Too many priorities; you should use only one or none.
  TooManyPriorities(usize),
}

impl Error for MetadataValidationError {}

impl Display for MetadataValidationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      MetadataValidationError::TooManyProjects(nb) => write!(f, "too many projects: {}", nb),
      MetadataValidationError::TooManyPriorities(nb) => write!(f, "too many priorities: {}", nb),
    }
  }
}

/// Task metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Metadata {
  /// Project name.
  Project(String),
  /// Priority.
  Priority(Priority),
  /// Tag.
  Tag(String),
}

impl From<Priority> for Metadata {
  fn from(v: Priority) -> Self {
    Metadata::Priority(v)
  }
}

impl Metadata {
  // TODO: decide what to do with duplicated tags
  /// Validate a list (set) of metadata.
  pub fn validate<'a>(
    metadata: impl IntoIterator<Item = &'a Metadata>,
  ) -> Result<(), MetadataValidationError> {
    let (proj_nb, prio_nb) = metadata
      .into_iter()
      .fold((0, 0), |(proj_nb, prio_nb), md| match md {
        Metadata::Project(_) => (proj_nb + 1, proj_nb),
        Metadata::Priority(_) => (proj_nb, prio_nb + 1),
        _ => (proj_nb, prio_nb),
      });

    if proj_nb > 1 {
      return Err(MetadataValidationError::TooManyProjects(proj_nb));
    }

    if prio_nb > 1 {
      return Err(MetadataValidationError::TooManyPriorities(prio_nb));
    }

    Ok(())
  }

  /// Create a metadata representing a project.
  pub fn project(name: impl Into<String>) -> Self {
    Metadata::Project(name.into())
  }

  /// Create a metadata representing a priority.
  pub fn priority(priority: Priority) -> Self {
    Metadata::Priority(priority)
  }

  /// Create a metadata representing a tag.
  pub fn tag(name: impl Into<String>) -> Self {
    Metadata::Tag(name.into())
  }

  /// Find metadata in a list of words encoded as a string.
  pub fn from_words<'a>(strings: impl IntoIterator<Item = &'a str>) -> (Vec<Metadata>, String) {
    let mut metadata = Vec::new();
    let mut output = Vec::new();

    for s in strings {
      let mut words = s.split(" ").filter(|s| !s.is_empty());

      for word in words {
        if let Ok(md) = word.parse() {
          metadata.push(md);
        } else {
          output.push(word);
        }
      }
    }

    log::debug!("extracted metadata:");
    log::debug!("  metadata: {:?}", metadata);
    log::debug!("  output: {:?}", output);

    (metadata, output.join(" "))
  }
}

impl FromStr for Metadata {
  type Err = MetadataParsingError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let len = s.len();
    if len < 2 {
      return Err(MetadataParsingError::Unknown(s.to_owned()));
    }

    match s.as_bytes()[0] {
      b'@' => Ok(Metadata::Project(s[1..].to_owned())),
      b'+' => {
        if len == 2 {
          match s.as_bytes()[1] {
            b'l' => Ok(Metadata::Priority(Priority::Low)),
            b'm' => Ok(Metadata::Priority(Priority::Medium)),
            b'h' => Ok(Metadata::Priority(Priority::High)),
            b'c' => Ok(Metadata::Priority(Priority::Critical)),
            _ => Err(MetadataParsingError::UnknownPriority),
          }
        } else {
          Err(MetadataParsingError::UnknownPriority)
        }
      }
      b'#' => Ok(Metadata::Tag(s[1..].to_owned())),
      _ => Err(MetadataParsingError::Unknown(s.to_owned())),
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetadataParsingError {
  /// Occurs when a priority is not recognized as valid.
  UnknownPriority,
  /// Occurs when a string is not recognized as metadata.
  Unknown(String),
}

/// Priority.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Priority {
  Low,
  Medium,
  High,
  Critical,
}

#[cfg(test)]
mod unit_tests {
  use super::*;

  #[test]
  fn project() {
    assert_eq!(
      "@foo".parse::<Metadata>(),
      Ok(Metadata::Project("foo".to_owned()))
    );

    assert_eq!(
      "@foo bar".parse::<Metadata>(),
      Ok(Metadata::Project("foo bar".to_owned()))
    );

    assert_eq!(
      "@".parse::<Metadata>(),
      Err(MetadataParsingError::Unknown("@".to_owned()))
    );
  }

  #[test]
  fn tag() {
    assert_eq!(
      "#foo".parse::<Metadata>(),
      Ok(Metadata::Tag("foo".to_owned()))
    );

    assert_eq!(
      "#foo bar".parse::<Metadata>(),
      Ok(Metadata::Tag("foo bar".to_owned()))
    );

    assert_eq!(
      "#".parse::<Metadata>(),
      Err(MetadataParsingError::Unknown("#".to_owned()))
    );
  }

  #[test]
  fn priority() {
    assert_eq!(
      "+l".parse::<Metadata>(),
      Ok(Metadata::Priority(Priority::Low))
    );

    assert_eq!(
      "+m".parse::<Metadata>(),
      Ok(Metadata::Priority(Priority::Medium))
    );

    assert_eq!(
      "+h".parse::<Metadata>(),
      Ok(Metadata::Priority(Priority::High))
    );

    assert_eq!(
      "+c".parse::<Metadata>(),
      Ok(Metadata::Priority(Priority::Critical))
    );

    assert_eq!(
      "+a".parse::<Metadata>(),
      Err(MetadataParsingError::UnknownPriority)
    );

    assert_eq!(
      "+la".parse::<Metadata>(),
      Err(MetadataParsingError::UnknownPriority)
    );
  }

  #[test]
  fn extract_metadata_output() {
    let input = "@project1 #tag1 +h Hello, this is world!  #tag2";
    let (metadata, output) = Metadata::from_words(vec![input].into_iter());

    assert_eq!(
      metadata,
      vec![
        Metadata::project("project1"),
        Metadata::tag("tag1"),
        Metadata::priority(Priority::High),
        Metadata::tag("tag2")
      ]
    );
    assert_eq!(output, "Hello, this is world!");
  }
}
