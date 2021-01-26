# 0.3.3

> Jan 26, 2021

## Additions

- Add a new set of commands, under `td project`. The only currently available sub-command is `rename`, allowing to
  perform mass renaming of project. Have a look at [this](./doc/cli.md#mass-renaming-projects) for further details.

## Patches / fixes

- Fix the case sensitive / insensitive filters (they were incorrectly reversed) when filtering on tasks’ contents.

# 0.3.2

> Jan 21, 2021

## Additions

- Add the history view. See [this](./doc/cli.md#consult-the-history-of-a-task) for further details.
- Add support for tags display in listings (`td ls`). Because this might generate big listings, it is possible not to
  show tags via the configuration file. See [this](./doc/config.md#display_tags_listings) for further details.
- When adding / editing notes, the previously recorded notes are automatically shown as a header in the editor prompt.
  This help section can be switched off manually with the `--no-history` switch and is configurable in the configuration
  file. See [this](./doc/config.md#previous_notes_help).
- When creating a task, the `--note` argument can be passed to automatically be dropped in an editor prompt and record
  a note right way for the task that was just added. This is short cut to prevent users from having to read a task UID
  and is similar to the following:
  ```
  td add Something to do
  td <task-uid> note add
  ```
  The shortcut way is:
  ```
  td add Something to do --note
  ```
- Three new documents were added to help users get more information and guides:
  - The [features](./doc/features.md) file, describing all the features currently supported by the latest version.
  - The [CLI user guide](./doc/cli.md), providing a guide on how to use the CLI. Very similar to a _man(1)_.
  - The [configuration user guide](./doc/config.md), which describes the configuration file and all the customization
    options.

## Patches / fixes

- Fix a bug when editing note, where note UIDs were 0-based instead of being 1-based (as shown in the output of
  `td <note-uid> show`).
- Various internal refactorings.

# 0.3.1

> Jan 15, 2021

- Fix a panic where the terminal doesn’t export its size (such as `eshell`).

# 0.3

> Nov 16th, 2020

- Add note support (addition, deletion). It’s a breaking change as notes were already supported in secret (in the
  serialized form), even though the interface was not exposed. Nothing should break if you didn’t try to do sneaky
  things.

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
