# Changelog

# 0.2.6

> Nov 6th, 2020

- Task listing now accepts description filtering. It works the same way tasks are created: meta data can be found at the
  beginning and/or the end of the content: what’s in the middle is the description part. It is comprised of _search_
  terms. By default, these are sorted uniquely and are case-insensitive. It is possible to change the case behavior
  with the `-C` switch. Right now, it is not possible to enforce term order, nor cardinality (i.e. duplicated terms
  resolve to a single one, and terms are lexicographically sorted).


# 0.2.5

> Nov 2nd, 2020

- Replace the `m` suffix for month durations with `mth`, which is less confusing.

# 0.2.4

> Oct 28th, 2020

- Fix alignments for task descriptions while listing them.

# 0.2.3

> Oct 24th, 2020

- Fix configuration errors when colors are not present in the file.

# 0.2.2

> Oct 23th, 2020

- Add the `show` / `s` command, enabling to show details about a task.

# 0.2.1

> Oct 23th, 2020

- Fix a panic when the terminal’s width was too short.
- Fix a visual glitch related to cutting sentences when the final cut word ends up at the end of the terminal.
- Add support for metadata filtering. You can now fuzzy filter tasks in the `ls` command by adding the regular metadata
  (`+l`, `+m`, `+h`, `+c`, `#a-tag` or `@a-project`).

# 0.2

> Oct 19th, 2020

## Breaking changes

- Add support for color and style configuration. This is a breaking change as configuration files will need to be
  updated. Typical migration path is to simply remove your `~/.config/toodoux/config.toml` and run `td` again to
  re-create a default configuration file.

## Additions

- Add support for color and style configuration in the TOML file. This currently works for the task listing.

## Patch

- Bump `env_logger`.
- Fix the description output

# 0.1

> Oct 10th, 2020

- Initial revision.
