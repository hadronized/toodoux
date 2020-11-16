//! Interactive editor session.
//!
//! This module provides a way to open an editor based on the `$EDITOR` environment variable or what is defined in the
//! configuration.

use crate::config::Config;
use std::{env, error, fmt, fs, io, path::Path, process, string::FromUtf8Error};

/// Errors that can happen while interactively editing files.
#[derive(Debug)]
pub enum InteractiveEditingError {
  FileError(io::Error),
  MissingInteractiveEditor,
  InteractiveEditorError(io::Error),
  Utf8Error(FromUtf8Error),
}

impl fmt::Display for InteractiveEditingError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      InteractiveEditingError::FileError(ref err) => write!(f, "unable to open file: {}", err),
      InteractiveEditingError::MissingInteractiveEditor => f.write_str(
        "no interactive editor was found; consider configuring either $EDITOR or the configuration",
      ),
      InteractiveEditingError::InteractiveEditorError(ref err) => {
        write!(f, "interactive editor error: {}", err)
      }
      InteractiveEditingError::Utf8Error(ref err) => {
        write!(f, "error while decoding UTF-8: {}", err)
      }
    }
  }
}

impl error::Error for InteractiveEditingError {}

impl From<io::Error> for InteractiveEditingError {
  fn from(err: io::Error) -> Self {
    Self::FileError(err)
  }
}

impl From<FromUtf8Error> for InteractiveEditingError {
  fn from(err: FromUtf8Error) -> Self {
    Self::Utf8Error(err)
  }
}

/// Open an interactive editor for the file named `file_name` and once the file is saved and the editor
/// exits, returns what the file contains.
///
/// If `content` contains a non-empty [`String`], its content will be automatically inserted in the file before opening
/// the editor.
pub fn interactively_edit(
  config: &Config,
  file_name: &str,
  content: &str,
) -> Result<String, InteractiveEditingError> {
  log::debug!("creating temporary directory for interactive session");
  let dir = tempdir::TempDir::new("")?;
  let file_path = dir.path().join(Path::new(file_name));

  log::debug!("creating temporary file {}", file_path.display());
  fs::write(&file_path, content)?;

  let editor;
  if let Some(env_editor) = env::var("EDITOR").ok() {
    if env_editor.is_empty() {
      return Err(InteractiveEditingError::MissingInteractiveEditor);
    }

    log::debug!("editing via $EDITOR ({})", env_editor);
    editor = env_editor;
  } else if let Some(conf_editor) = config.interactive_editor() {
    if conf_editor.is_empty() {
      return Err(InteractiveEditingError::MissingInteractiveEditor);
    }

    log::debug!("editing via configuration editor ({})", conf_editor);
    editor = conf_editor.to_owned();
  } else {
    log::error!("cannot find a suitable interactive editor");
    return Err(InteractiveEditingError::MissingInteractiveEditor);
  }

  let _ = process::Command::new(editor)
    .arg(&file_path)
    .spawn()
    .map_err(InteractiveEditingError::InteractiveEditorError)?
    .wait()
    .map_err(InteractiveEditingError::InteractiveEditorError)?;
  let content = fs::read_to_string(file_path)?;

  Ok(content)
}
