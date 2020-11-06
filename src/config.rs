//! Initiate the configuration file creation when not present.

use colored::{Color as Col, ColoredString, Colorize};
use core::fmt::Formatter;
use serde::{
  de::{self, Visitor},
  Deserialize, Serialize,
};
use std::{
  error::Error,
  fmt, fs,
  ops::Deref,
  path::{Path, PathBuf},
  str::FromStr,
};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
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

  /// Maximum number of warping lines of task description before breaking it (and adding the ellipsis character).
  max_description_lines: usize,
}

impl Default for MainConfig {
  fn default() -> Self {
    Self {
      tasks_file: dirs::config_dir().unwrap().join("toodoux"),
      todo_alias: "TODO".to_owned(),
      wip_alias: "WIP".to_owned(),
      done_alias: "DONE".to_owned(),
      cancelled_alias: "CANCELLED".to_owned(),
      uid_col_name: "UID".to_owned(),
      age_col_name: "Age".to_owned(),
      spent_col_name: "Spent".to_owned(),
      prio_col_name: "Prio".to_owned(),
      project_col_name: "Project".to_owned(),
      status_col_name: "Status".to_owned(),
      description_col_name: "Description".to_owned(),
      display_empty_cols: false,
      max_description_lines: 2,
    }
  }
}

impl MainConfig {
  #[allow(dead_code)]
  pub fn new(
    tasks_file: impl Into<PathBuf>,
    todo_alias: impl Into<String>,
    wip_alias: impl Into<String>,
    done_alias: impl Into<String>,
    cancelled_alias: impl Into<String>,
    uid_col_name: impl Into<String>,
    age_col_name: impl Into<String>,
    spent_col_name: impl Into<String>,
    prio_col_name: impl Into<String>,
    project_col_name: impl Into<String>,
    status_col_name: impl Into<String>,
    description_col_name: impl Into<String>,
    display_empty_cols: bool,
    max_description_lines: usize,
  ) -> Self {
    Self {
      tasks_file: tasks_file.into(),
      todo_alias: todo_alias.into(),
      wip_alias: wip_alias.into(),
      done_alias: done_alias.into(),
      cancelled_alias: cancelled_alias.into(),
      uid_col_name: uid_col_name.into(),
      age_col_name: age_col_name.into(),
      spent_col_name: spent_col_name.into(),
      prio_col_name: prio_col_name.into(),
      project_col_name: project_col_name.into(),
      status_col_name: status_col_name.into(),
      description_col_name: description_col_name.into(),
      display_empty_cols,
      max_description_lines,
    }
  }
}

impl Config {
  #[allow(dead_code)]
  pub fn new(main: MainConfig, colors: ColorConfig) -> Self {
    Config { main, colors }
  }

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

  pub fn max_description_lines(&self) -> usize {
    self.main.max_description_lines
  }

  pub fn get() -> Result<Option<Self>, Box<dyn Error>> {
    let path = Self::get_config_path()?;
    Self::from_dir(path)
  }

  pub fn create(path: Option<&Path>) -> Option<Self> {
    let default_config = Self::default();
    let tasks_file = path
      .map(|p| p.to_owned())
      .or(Self::get_config_path().ok())?;

    let main = MainConfig {
      tasks_file,
      ..default_config.main
    };
    let config = Self {
      main,
      ..default_config
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StyleAttribute {
  Bold,
  Dimmed,
  Underline,
  Reversed,
  Italic,
  Blink,
  Hidden,
  Strikethrough,
}

impl StyleAttribute {
  /// Apply this style attribute to the input colored string.
  fn apply_style(&self, s: ColoredString) -> ColoredString {
    match self {
      StyleAttribute::Bold => s.bold(),
      StyleAttribute::Dimmed => s.dimmed(),
      StyleAttribute::Underline => s.underline(),
      StyleAttribute::Reversed => s.reversed(),
      StyleAttribute::Italic => s.italic(),
      StyleAttribute::Blink => s.blink(),
      StyleAttribute::Hidden => s.hidden(),
      StyleAttribute::Strikethrough => s.strikethrough(),
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct ColorConfig {
  pub description: TaskDescriptionColorConfig,
  pub status: TaskStatusColorConfig,
  pub priority: PriorityColorConfig,
  pub show_header: ShowHeaderColorConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskDescriptionColorConfig {
  pub ongoing: Highlight,
  pub todo: Highlight,
  pub done: Highlight,
  pub cancelled: Highlight,
}

impl Default for TaskDescriptionColorConfig {
  fn default() -> Self {
    Self {
      ongoing: Highlight {
        foreground: Some(Color(Col::Black)),
        background: Some(Color(Col::BrightGreen)),
        style: vec![],
      },
      todo: Highlight {
        foreground: Some(Color(Col::BrightWhite)),
        background: Some(Color(Col::Black)),
        style: vec![],
      },
      done: Highlight {
        foreground: Some(Color(Col::BrightBlack)),
        background: Some(Color(Col::Black)),
        style: vec![StyleAttribute::Dimmed],
      },
      cancelled: Highlight {
        foreground: Some(Color(Col::BrightBlack)),
        background: Some(Color(Col::Black)),
        style: vec![StyleAttribute::Dimmed, StyleAttribute::Strikethrough],
      },
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskStatusColorConfig {
  pub ongoing: Highlight,
  pub todo: Highlight,
  pub done: Highlight,
  pub cancelled: Highlight,
}

impl Default for TaskStatusColorConfig {
  fn default() -> Self {
    Self {
      ongoing: Highlight {
        foreground: Some(Color(Col::Green)),
        background: None,
        style: vec![StyleAttribute::Bold],
      },
      todo: Highlight {
        foreground: Some(Color(Col::Magenta)),
        background: None,
        style: vec![StyleAttribute::Bold],
      },
      done: Highlight {
        foreground: Some(Color(Col::BrightBlack)),
        background: None,
        style: vec![StyleAttribute::Dimmed],
      },
      cancelled: Highlight {
        foreground: Some(Color(Col::BrightRed)),
        background: None,
        style: vec![StyleAttribute::Dimmed],
      },
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PriorityColorConfig {
  pub low: Highlight,
  pub medium: Highlight,
  pub high: Highlight,
  pub critical: Highlight,
}

impl Default for PriorityColorConfig {
  fn default() -> Self {
    Self {
      low: Highlight {
        foreground: Some(Color(Col::BrightBlack)),
        background: None,
        style: vec![StyleAttribute::Dimmed],
      },
      medium: Highlight {
        foreground: Some(Color(Col::Blue)),
        background: None,
        style: vec![],
      },
      high: Highlight {
        foreground: Some(Color(Col::Red)),
        background: None,
        style: vec![],
      },
      critical: Highlight {
        foreground: Some(Color(Col::Black)),
        background: Some(Color(Col::BrightRed)),
        style: vec![],
      },
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShowHeaderColorConfig(Highlight);

impl Default for ShowHeaderColorConfig {
  fn default() -> Self {
    Self(Highlight {
      foreground: Some(Color(Col::BrightBlack)),
      background: None,
      style: vec![],
    })
  }
}

impl Deref for ShowHeaderColorConfig {
  type Target = Highlight;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

/// Highlight definition.
///
/// Contains foreground and background colors as well as the style to use.
#[derive(Debug, Deserialize, Serialize)]
pub struct Highlight {
  /// Foreground color.
  ///
  /// Leaving it empty implies using the default foreground color of your terminal
  pub foreground: Option<Color>,

  /// Background color.
  ///
  /// Leaving it empty implies using the default background color of your terminal
  pub background: Option<Color>,

  /// Style attributes to use.
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub style: Vec<StyleAttribute>,
}

impl Highlight {
  /// Apply the highlight to an input string.
  pub fn highlight(&self, input: impl AsRef<str>) -> HighlightedString {
    let mut colored: ColoredString = input.as_ref().into();

    if let Some(foreground) = &self.foreground {
      colored = colored.color(foreground.0);
    }

    if let Some(background) = &self.background {
      colored = colored.on_color(background.0);
    }

    for s in &self.style {
      colored = s.apply_style(colored);
    }

    HighlightedString(colored)
  }
}

/// Highlighted string — i.e. all color information and styles have been applied.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HighlightedString(ColoredString);

impl fmt::Display for HighlightedString {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    self.0.fmt(f)
  }
}

/// a wrapper around colored::Color in order to implement serialization
#[derive(Debug, PartialEq)]
pub struct Color(pub Col);

impl Color {
  /// Parse a [`Color`] from a hexadecimal string.
  ///
  /// Supports two simple formats (uppercase / lowercase supported in either case):
  ///
  /// - `#rrggbb`. Example: `#34f1c8`.
  /// - `#rgb`, which desugars to repeating each channel once. Example: `#3fc`.
  pub fn from_hex(hex: impl AsRef<str>) -> Option<Color> {
    let hex = hex.as_ref();
    let bytes = hex.as_bytes();
    let (mut r, mut g, mut b);

    if hex.len() == 4 && bytes[0] == b'#' {
      // triplet form (#rgb)
      let mut h = u16::from_str_radix(&hex[1..], 16).ok()?;

      b = (h & 0xf) as _;
      b += b << 4;

      h >>= 4;
      g = (h & 0xf) as _;
      g += g << 4;

      h >>= 4;
      r = (h & 0xf) as _;
      r += r << 4;
    } else if hex.len() == 7 && bytes[0] == b'#' {
      // #rrggbb form
      let mut h = u32::from_str_radix(&hex[1..], 16).ok()?;

      b = (h & 0xff) as _;

      h >>= 8;
      g = (h & 0xff) as _;

      h >>= 8;
      r = (h & 0xff) as _;
    } else {
      return None;
    }

    Some(Color(Col::TrueColor { r, g, b }))
  }
}

impl<'de> Deserialize<'de> for Color {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct ColorVisitor;

    const EXPECTING: &str = "a color name or hexadecimal color";

    impl<'de> Visitor<'de> for ColorVisitor {
      type Value = Color;

      fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(EXPECTING)
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        // try to use from_str to get color; if this doesn't work we try to parse it as hex
        Col::from_str(value)
          .ok()
          .map(Color)
          .or_else(|| Color::from_hex(value))
          .ok_or_else(|| {
            // in the case we were unable to parse either a color name or hexadecimal color, we emit a serde error
            E::invalid_value(de::Unexpected::Str(value), &EXPECTING)
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
    let true_color;
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
        true_color = format!("#{:02x}{:02x}{:02x}", r, g, b);
        &true_color
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
    assert_eq!(
      Color::from_hex("#123"),
      Some(Color(Col::TrueColor {
        r: 0x11,
        g: 0x22,
        b: 0x33
      }))
    );

    assert_eq!(
      Color::from_hex("#112234"),
      Some(Color(Col::TrueColor {
        r: 0x11,
        g: 0x22,
        b: 0x34
      }))
    );
  }

  #[test]
  fn color_colored_name() {
    let c = Color(Col::White);
    assert_tokens(&c, &[Token::Str("white")])
  }

  #[test]
  fn apply_color_options() {
    // with color
    {
      let expected = HighlightedString("test".on_black().white().bold());
      let opts = Highlight {
        background: Some(Color(Col::Black)),
        foreground: Some(Color(Col::White)),
        style: vec![StyleAttribute::Bold],
      };
      assert_eq!(expected, opts.highlight("test"));
    }

    // only styles
    {
      let expected = HighlightedString("test".italic().bold());
      let opts = Highlight {
        background: None,
        foreground: None,
        style: vec![StyleAttribute::Bold, StyleAttribute::Italic],
      };
      assert_eq!(expected, opts.highlight("test"));
    }
  }
}
