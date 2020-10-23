# Changelog

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
