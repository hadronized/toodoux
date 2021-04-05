use crate::task::UID;
use serde_json as json;
use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
  CannotOpenFile(io::Error),
  CannotSave(io::Error),
  CannotDeserializeFromJSON(json::Error),
  CannotDeserializeFromTOML(toml::de::Error),
  CannotSerializeToTOML(toml::ser::Error),
  CannotDeserializeFromSerde(serde::de::value::Error),
  NoConfigDir,
  UnknownNote(UID),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      Error::CannotOpenFile(ref e) => {
        write!(f, "cannot open file: {}", e)
      }

      Error::CannotSave(ref e) => write!(f, "cannot save: {}", e),

      Error::CannotDeserializeFromJSON(ref e) => {
        write!(f, "cannot deserialize from JSON: {}", e)
      }

      Error::CannotDeserializeFromTOML(ref e) => {
        write!(f, "cannot deserialize from TOML: {}", e)
      }

      Error::CannotSerializeToTOML(ref e) => {
        write!(f, "cannot serialize to TOML: {}", e)
      }

      Error::CannotDeserializeFromSerde(ref e) => {
        write!(f, "cannot deserialize: {}", e)
      }

      Error::NoConfigDir => f.write_str("cannot find configuration directory"),

      Error::UnknownNote(uid) => write!(f, "note {} doesn’t exist", uid),
    }
  }
}

impl From<json::Error> for Error {
  fn from(err: json::Error) -> Self {
    Self::CannotDeserializeFromJSON(err)
  }
}

impl From<toml::de::Error> for Error {
  fn from(err: toml::de::Error) -> Self {
    Self::CannotDeserializeFromTOML(err)
  }
}

impl From<toml::ser::Error> for Error {
  fn from(err: toml::ser::Error) -> Self {
    Self::CannotSerializeToTOML(err)
  }
}

impl From<serde::de::value::Error> for Error {
  fn from(err: serde::de::value::Error) -> Self {
    Self::CannotDeserializeFromSerde(err)
  }
}
