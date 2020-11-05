use std::collections::HashSet;
use unicase::UniCase;

#[derive(Clone)]
pub(crate) enum NameFilter<'a> {
  CaseSensitive(HashSet<&'a str>),
  IgnoreCase(HashSet<UniCase<&'a str>>),
}

impl<'a> NameFilter<'a> {
  pub fn new(name: impl Iterator<Item = &'a str>, case_insensitive: bool) -> Self {
    if case_insensitive {
      NameFilter::CaseSensitive(name.collect())
    } else {
      NameFilter::IgnoreCase(name.map(UniCase::new).collect())
    }
  }

  pub fn is_empty(&self) -> bool {
    match self {
      NameFilter::CaseSensitive(set) => set.is_empty(),
      NameFilter::IgnoreCase(set) => set.is_empty(),
    }
  }

  pub fn remove(&mut self, word: &'a str) -> bool {
    match self {
      NameFilter::CaseSensitive(set) => set.remove(word),
      NameFilter::IgnoreCase(set) => set.remove(&UniCase::new(word)),
    }
  }
}
