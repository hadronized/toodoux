//! Initiate the configuration file creation when not present.

use colored::{ColoredString, Colorize};
use core::fmt::Formatter;
use scarlet::color::RGBColor;
use serde::de;
use serde::de::Visitor;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
// just for avoiding some typing
use colored::Color as Col;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
  pub main: MainConfig,
  pub colors: ColorConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MainConfig {
  /// Path to the folder containing all the tasks.
  tasks_file: PathBuf,

  /// Name of the “TODO” state.
  todo_alias: String,

  /// Name of the “ONGOING” state.
  wip_alias: String,

  /// Name of the “DONE” state.
  done_alias: String,

  /// Name of the “CANCELLED” state.
  cancelled_alias: String,

  /// “UID” column name.
  uid_col_name: String,

  /// “Age” column name.
  age_col_name: String,

  /// “Spent” column name.
  spent_col_name: String,

  /// “Prio” column name.
  prio_col_name: String,

  /// “Project” column name.
  project_col_name: String,

  /// “Status” column name.
  status_col_name: String,

  /// “Description” column name.
  description_col_name: String,

  /// Should we display empty columns?
  display_empty_cols: bool,
}

impl Config {
  fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    log::trace!("getting configuration root path from the environment");
    let home = dirs::config_dir().ok_or("cannot find configuration directory")?;
    let path = Path::new(&home).join("toodoux");

    Ok(path)
  }

  pub fn from_dir(path: impl AsRef<Path>) -> Result<Option<Self>, Box<dyn Error>> {
    let path = path.as_ref().join("config.toml");

    log::trace!("reading configuration from {}", path.display());
    if path.is_file() {
      let content = fs::read_to_string(&path)?;
      let parsed = toml::from_str(&content)?;
      Ok(Some(parsed))
    } else {
      Ok(None)
    }
  }

  pub fn root_dir(&self) -> &Path {
    &self.main.tasks_file
  }

  pub fn config_toml_path(&self) -> PathBuf {
    self.main.tasks_file.join("config.toml")
  }

  pub fn tasks_path(&self) -> PathBuf {
    self.main.tasks_file.join("tasks.json")
  }

  pub fn todo_alias(&self) -> &str {
    &self.main.todo_alias
  }

  pub fn wip_alias(&self) -> &str {
    &self.main.wip_alias
  }

  pub fn done_alias(&self) -> &str {
    &self.main.done_alias
  }

  pub fn cancelled_alias(&self) -> &str {
    &self.main.cancelled_alias
  }

  pub fn uid_col_name(&self) -> &str {
    &self.main.uid_col_name
  }

  pub fn age_col_name(&self) -> &str {
    &self.main.age_col_name
  }

  pub fn spent_col_name(&self) -> &str {
    &self.main.spent_col_name
  }

  pub fn prio_col_name(&self) -> &str {
    &self.main.prio_col_name
  }

  pub fn project_col_name(&self) -> &str {
    &self.main.project_col_name
  }

  pub fn status_col_name(&self) -> &str {
    &self.main.status_col_name
  }

  pub fn description_col_name(&self) -> &str {
    &self.main.description_col_name
  }

  pub fn display_empty_cols(&self) -> bool {
    self.main.display_empty_cols
  }

  pub fn get() -> Result<Option<Self>, Box<dyn Error>> {
    let path = Self::get_config_path()?;
    Self::from_dir(path)
  }

  pub fn create(path: Option<&Path>) -> Option<Self> {
    let tasks_file = path
      .map(|p| p.to_owned())
      .or(Self::get_config_path().ok())?;
    let todo_alias = "TODO".to_owned();
    let wip_alias = "WIP".to_owned();
    let done_alias = "DONE".to_owned();
    let cancelled_alias = "CANCELLED".to_owned();
    let uid_col_name = "UID".to_owned();
    let age_col_name = "Age".to_owned();
    let spent_col_name = "Spent".to_owned();
    let prio_col_name = "Prio".to_owned();
    let project_col_name = "Project".to_owned();
    let status_col_name = "Status".to_owned();
    let description_col_name = "Description".to_owned();
    let display_empty_cols = false;

    let main = MainConfig {
      tasks_file,
      todo_alias,
      wip_alias,
      done_alias,
      cancelled_alias,
      uid_col_name,
      age_col_name,
      spent_col_name,
      prio_col_name,
      project_col_name,
      status_col_name,
      description_col_name,
      display_empty_cols,
    };

    let config = Config {
      main,
      colors: Default::default(),
    };

    log::trace!("creating new configuration:\n{:#?}", config);

    Some(config)
  }

  pub fn save(&self) -> Result<(), Box<dyn Error>> {
    let root_dir = self.root_dir();
    fs::create_dir_all(root_dir)?;

    let serialized = toml::to_string_pretty(self)?;
    let _ = fs::write(self.config_toml_path(), serialized)?;

    Ok(())
  }
}

macro_rules! color {
  ($name:ident) => {
    Color(Col::$name)
  };
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Style {
  Bold,
  Dimmed,
  Underline,
  Reversed,
  Italic,
  Blink,
  Hidden,
  Strikethrough,
}

impl Style {
  /// applies this Style to a ColoredString.
  /// this is not public as to not leak the implementation that uses ColoredString
  fn apply(&self, s: ColoredString) -> ColoredString {
    match self {
      Style::Bold => s.bold(),
      Style::Dimmed => s.dimmed(),
      Style::Underline => s.underline(),
      Style::Reversed => s.reversed(),
      Style::Italic => s.italic(),
      Style::Blink => s.blink(),
      Style::Hidden => s.hidden(),
      Style::Strikethrough => s.strikethrough(),
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ColorConfig {
  pub description: TaskDescriptionColorConfig,
  pub status: TaskStatusColorConfig,
  pub priority: PriorityColorConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskDescriptionColorConfig {
  pub ongoing: ColorOptions,
  pub todo: ColorOptions,
  pub done: ColorOptions,
  pub cancelled: ColorOptions,
}

impl Default for TaskDescriptionColorConfig {
  fn default() -> Self {
    Self {
      ongoing: ColorOptions {
        foreground: Some(color!(Black)),
        background: Some(color!(BrightGreen)),
        styles: vec![],
      },
      todo: ColorOptions {
        foreground: Some(color!(BrightWhite)),
        background: Some(color!(Black)),
        styles: vec![],
      },
      done: ColorOptions {
        foreground: Some(color!(BrightBlack)),
        background: Some(color!(Black)),
        styles: vec![Style::Dimmed],
      },
      cancelled: ColorOptions {
        foreground: Some(color!(BrightBlack)),
        background: Some(color!(Black)),
        styles: vec![Style::Dimmed, Style::Strikethrough],
      },
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskStatusColorConfig {
  pub ongoing: ColorOptions,
  pub todo: ColorOptions,
  pub done: ColorOptions,
  pub cancelled: ColorOptions,
}

impl Default for TaskStatusColorConfig {
  fn default() -> Self {
    Self {
      ongoing: ColorOptions {
        foreground: Some(color!(Green)),
        background: None,
        styles: vec![Style::Bold],
      },
      todo: ColorOptions {
        foreground: Some(color!(Magenta)),
        background: None,
        styles: vec![Style::Bold],
      },
      done: ColorOptions {
        foreground: Some(color!(BrightBlack)),
        background: None,
        styles: vec![Style::Dimmed],
      },
      cancelled: ColorOptions {
        foreground: Some(color!(BrightRed)),
        background: None,
        styles: vec![Style::Dimmed],
      },
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PriorityColorConfig {
  pub low: ColorOptions,
  pub medium: ColorOptions,
  pub high: ColorOptions,
  pub critical: ColorOptions,
}

impl Default for PriorityColorConfig {
  fn default() -> Self {
    Self {
      low: ColorOptions {
        foreground: Some(color!(BrightBlack)),
        background: None,
        styles: vec![Style::Dimmed],
      },
      medium: ColorOptions {
        foreground: Some(color!(Blue)),
        background: None,
        styles: vec![],
      },
      high: ColorOptions {
        foreground: Some(color!(Red)),
        background: None,
        styles: vec![],
      },
      critical: ColorOptions {
        foreground: Some(color!(Black)),
        background: Some(color!(BrightRed)),
        styles: vec![],
      },
    }
  }
}

/// an option that includes all console text formatting
#[derive(Debug, Deserialize, Serialize)]
pub struct ColorOptions {
  pub foreground: Option<Color>,
  pub background: Option<Color>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub styles: Vec<Style>,
}

impl ColorOptions {
  /// applies the color options to a string
  pub fn apply(&self, s: &str) -> String {
    let mut colored: ColoredString = s.into();
    if let Some(foreground) = &self.foreground {
      colored = colored.color(foreground.0);
    }

    if let Some(background) = &self.background {
      colored = colored.on_color(background.0);
    }

    for s in &self.styles {
      colored = s.apply(colored);
    }

    // this is the only method i could find to get a formatted String from a ColoredString
    format!("{}", colored)
  }
}

/// a wrapper around colored::Color in order to implement serialization
#[derive(Debug, PartialEq)]
pub struct Color(pub colored::Color);

impl<'de> Deserialize<'de> for Color {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct ColorVisitor;
    // the message of what the expected value is
    const EXPECTED: &str = "a hexadecimal color value, or X11 Color name";
    impl<'de> Visitor<'de> for ColorVisitor {
      type Value = Color;
      fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(EXPECTED)
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        // try to use from_str to get color
        // if this doesn't work we try to parse it as hex
        // map to option so we dont have to worry about error types
        match colored::Color::from_str(value).ok() {
          None => {
            // try to decode from hex, then from name
            let rgb = match RGBColor::from_hex_code(value) {
              Err(_) => RGBColor::from_color_name(value),
              c => c,
            };

            if let Ok(c) = rgb {
              Some(colored::Color::TrueColor {
                r: c.int_r(),
                g: c.int_g(),
                b: c.int_b(),
              })
            } else {
              None
            }
          }
          c => c,
        }
        // map to wrapper type from colored::Color
        .map(|c| Color(c))
        // map to result with serde error if color was invalid
        .ok_or_else(|| {
          E::invalid_value(
            de::Unexpected::Str(value),
            &EXPECTED,
          )
        })
      }
    }
    deserializer.deserialize_str(ColorVisitor)
  }
}

impl Serialize for Color {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    // this is a bit of a hack in order to extend the life time of a string
    // so we can return a ref to it from a match
    let s;
    // this is a reversed version of colored::Color::from_str()
    // with hex added
    let clr = match self.0 {
      Col::Black => "black",
      Col::Red => "red",
      Col::Green => "green",
      Col::Yellow => "yellow",
      Col::Blue => "blue",
      Col::Magenta => "magenta",
      Col::Cyan => "cyan",
      Col::White => "white",
      Col::BrightBlack => "bright black",
      Col::BrightRed => "bright red",
      Col::BrightGreen => "bright green",
      Col::BrightYellow => "bright yellow",
      Col::BrightBlue => "bright blue",
      Col::BrightMagenta => "bright magenta",
      Col::BrightCyan => "bright cyan",
      Col::BrightWhite => "bright white",
      Col::TrueColor { r, g, b } => {
        s = format!("#{:02x}{:02x}{:02x}", r, g, b);
        &s
      }
    };
    serializer.serialize_str(clr)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_test::*;

  #[test]
  fn color_hex() {
    let c = Color(colored::Color::TrueColor { r: 255, g: 0, b: 0 });
    assert_tokens(&c, &[Token::Str("#ff0000")]);

    // shorthands
    assert_de_tokens(
      &c,
      &[
        // heh, foo
        Token::Str("f00"),
      ],
    )
  }
  #[test]
  fn color_colored_name() {
    let c = color!(White);
    assert_tokens(&c, &[Token::Str("white")])
  }

  #[test]
  fn colored_x11_name() {
    let c = Color(colored::Color::TrueColor {
      r: 255,
      g: 0,
      b: 255,
    });
    assert_de_tokens(&c, &[Token::Str("fuchsia")])
  }

  #[test]
  fn apply_color_options() {
    // with color
    {
      let expected = "test".on_black().white().bold();
      let opts = ColorOptions {
        background: Some(color!(Black)),
        foreground: Some(color!(White)),
        styles: vec![Style::Bold],
      };
      assert_eq!(format!("{}", expected), opts.apply("test"));
    }

    // only styles
    {
      let expected = "test".italic().bold();
      let opts = ColorOptions {
        background: None,
        foreground: None,
        styles: vec![Style::Bold, Style::Italic],
      };
      assert_eq!(format!("{}", expected), opts.apply("test"));
    }
  }
}
