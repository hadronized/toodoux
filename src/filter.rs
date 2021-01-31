//! Various types used to filter tasks in listings.

use std::collections::HashSet;
use unicase::UniCase;

/// A filter based on tasks’ description.
#[derive(Clone)]
pub enum TaskDescriptionFilter<'a> {
  /// Case-sensitive filter.
  ///
  /// Searching for `foo` is not the same as searching for `Foo`.
  CaseSensitive(HashSet<&'a str>),
  /// Case-insensitive filter.
  ///
  /// Searching for `foo` is the same as searching for `Foo`.
  CaseInsensitive(HashSet<UniCase<&'a str>>),
}

impl<'a> TaskDescriptionFilter<'a> {
  /// Create a new task description filter based on an iterator on strings.
  ///
  /// If `case_insensitive` is `true`, the resulting filter will ignore case.
  pub fn new(name: impl Iterator<Item = &'a str>, case_insensitive: bool) -> Self {
    if case_insensitive {
      TaskDescriptionFilter::CaseInsensitive(name.map(UniCase::new).collect())
    } else {
      TaskDescriptionFilter::CaseSensitive(name.collect())
    }
  }

  /// Check whether the filter contains any term.
  pub fn is_empty(&self) -> bool {
    match self {
      TaskDescriptionFilter::CaseSensitive(set) => set.is_empty(),
      TaskDescriptionFilter::CaseInsensitive(set) => set.is_empty(),
    }
  }

  /// Remove a search term from the filter.
  pub fn remove(&mut self, word: &'a str) -> bool {
    match self {
      TaskDescriptionFilter::CaseSensitive(set) => set.remove(word),
      TaskDescriptionFilter::CaseInsensitive(set) => set.remove(&UniCase::new(word)),
    }
  }

  /// Get an iterator on the carried search terms.
  pub fn terms(&'a self) -> Box<dyn 'a + Iterator<Item = &'a str>> {
    match self {
      TaskDescriptionFilter::CaseSensitive(ref set) => Box::new(set.iter().copied()),
      TaskDescriptionFilter::CaseInsensitive(ref set) => Box::new(set.iter().map(AsRef::as_ref)),
    }
  }
}
