//! An abstracton of a terminal.

pub trait Terminal {
  /// Get the dimension (in characters / columns) of the terminal.
  fn dimensions(&self) -> Option<[usize; 2]>;
}

/// Default terminal abstraction..
pub struct DefaultTerm;

impl Terminal for DefaultTerm {
  fn dimensions(&self) -> Option<[usize; 2]> {
    term_size::dimensions().map(|(w, h)| [w, h])
  }
}
