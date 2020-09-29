//! Metadata available to users for filtering / creating tasks.

use std::str::FromStr;

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
}
