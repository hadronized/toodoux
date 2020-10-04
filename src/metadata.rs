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

impl From<Priority> for Metadata {
  fn from(v: Priority) -> Self {
    Metadata::Priority(v)
  }
}

impl Metadata {
  /// Find metadata in a list of words encoded as a string.
  ///
  /// This function will look metadata at the beginning of the string and the end. If you put metadata in the middle of
  /// the string, they will not be reported as metadata.
  pub fn from_words(s: &str) -> (Vec<Metadata>, String) {
    let mut metadata = Vec::new();
    let mut output = String::new();
    let mut words = s.split(" ");

    // first pass for metadata
    Self::parse_metadata_only(&mut metadata, &mut words);

    // second pass is for output only
    Self::parse_normal_only(&mut metadata, &mut words, &mut output);

    // third pass is for metadata again
    Self::parse_metadata_only(&mut metadata, &mut words);

    (metadata, output)
  }

  fn parse_metadata_only<'a>(
    metadata: &mut Vec<Metadata>,
    words: &mut impl Iterator<Item = &'a str>,
  ) {
    while let Some(word) = words.next() {
      if let Ok(md) = word.parse() {
        metadata.push(md);
      } else {
        break;
      }
    }
  }

  fn parse_normal_only<'a>(
    metadata: &mut Vec<Metadata>,
    words: &mut impl Iterator<Item = &'a str>,
    output: &mut String,
  ) {
    while let Some(word) = words.next() {
      if let Ok(md) = word.parse() {
        metadata.push(md);
        break;
      } else {
        output.push(' ');
        output.push_str(word);
      }
    }
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
}
