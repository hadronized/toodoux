# User Configuration

Configuration is done by following the [XDG Base Directory specification] by default but can be overridden by the user
if required. The configuration root directory is `$XDG_CONFIG_DIR/toodoux` — it should be `~/.config/toodoux` for most
people on Linux, for instance.

The configuration file, `config.toml`, is a TOML file that contains several sections:

- `[main]`, containing the main configuration of **toodoux**.
- `[colors]`, containing all the configuration keys to customize the colors and styles used by **toodoux**.

> We reserve the right to use other sections for further, more precise configuration.


<!-- vim-markdown-toc GFM -->

* [Main configuration](#main-configuration)
  * [`interactive_editor`](#interactive_editor)
  * [`tasks_file`](#tasks_file)
  * [`todo_alias`](#todo_alias)
  * [`wip_alias`](#wip_alias)
  * [`done_alias`](#done_alias)
  * [`cancelled_alias`](#cancelled_alias)
  * [`uid_col_name`](#uid_col_name)
  * [`age_col_name`](#age_col_name)
  * [`spent_col_name`](#spent_col_name)
  * [`prio_col_name`](#prio_col_name)
  * [`project_col_name`](#project_col_name)
  * [`tags_col_name`](#tags_col_name)
  * [`status_col_name`](#status_col_name)
  * [`description_col_name`](#description_col_name)
  * [`notes_nb_col_name`](#notes_nb_col_name)
  * [`display_empty_cols`](#display_empty_cols)
  * [`max_description_lines`](#max_description_lines)
  * [`display_tags_listings`](#display_tags_listings)
  * [`previous_notes_help`](#previous_notes_help)
* [Colors configuration](#colors-configuration)
  * [`[colors.description.todo]`](#colorsdescriptiontodo)
  * [`[colors.description.ongoing]`](#colorsdescriptionongoing)
  * [`[colors.description.done]`](#colorsdescriptiondone)
  * [`[colors.description.cancelled]`](#colorsdescriptioncancelled)
  * [`[colors.status.todo]`](#colorsstatustodo)
  * [`[colors.status.ongoing]`](#colorsstatusongoing)
  * [`[colors.status.done]`](#colorsstatusdone)
  * [`[colors.status.cancelled]`](#colorsstatuscancelled)
  * [`[colors.priority.low]`](#colorsprioritylow)
  * [`[colors.priority.medium]`](#colorsprioritymedium)
  * [`[colors.priority.high]`](#colorspriorityhigh)
  * [`[colors.priority.critical]`](#colorsprioritycritical)
  * [`[colors.show_header]`](#colorsshow_header)

<!-- vim-markdown-toc -->

## Main configuration

The `[main]` section contains the following keys.

### `interactive_editor`

- Editor to use for interactive editing.
- Defaults to none.

### `tasks_file`

- Path to the folder containing all the tasks.
- Defaults to `"$XDG_CONFIG_DIR/toodoux"`.

### `todo_alias`

- Name of the _tood_ state.
- Defaults to `"TODO"`.

### `wip_alias`

- Name of the _on-going_ state.
- Defaults to `"WIP"`.

### `done_alias`

- Name of the _done_ state.
- Defaults to `"DONE"`.

### `cancelled_alias`

- Name of the _cancelled_ state.
- Defaults to `"CANCELLED"`.

### `uid_col_name`

- UID column name.
- Defaults to `"IUD"`.

### `age_col_name`

- Age column name.
- Defaults to `"Age"`.

### `spent_col_name`

- Spent column name.
- Defaults to `"Spent"`.

### `prio_col_name`

- Priority column name.
- Defaults to `"Prio"`.

### `project_col_name`

- Project column name.
- Defaults to `"Project"`.

### `tags_col_name`

- Tags column name.
- Defaults to `"Tags"`.

### `status_col_name`

- Status column name.
- Defaults to `"Status"`.

### `description_col_name`

- Description column name.
- Defaults to `"Description"`.

### `notes_nb_col_name`

- Number of notes column name.
- Defaults to `"Notes"`.

### `display_empty_cols`

- Whether or not display empty columns in listing views.
- Defaults to `false`.

### `max_description_lines`

- Maximum number of warping lines of task description before breaking it (and adding the ellipsis character) if it’s
  too long.
- Defaults to `2`.

### `display_tags_listings`

- Display tags in listings.
- Defaults to `true`.

### `previous_notes_help`

- Show the previously recorded notes when adding a new note for a given task.
- Defaults to `true`.

## Colors configuration

Colors are configured via several sub-sections:

- `[colors.description.*]` contains all the styles for changing the description content in listing depending on the
  status of the task.
- `[colors.status.*]` contains all the styles for changing the status content in listing depending on the
  status of the task.
- `[colors.priority.*]` contains all the styles for changing the priority content in listing depending on the
  priority of the task.
- `[colors.show_header]` contains the style to apply on headers while describing notes.

Colors can be encoded via several formats:

- Regular RGB hexadecimal strings — `"#rrggbb"` or `"#rgb"`.
- Terminal colors are supported with the following names:
  - `"black"`.
  - `"red"`.
  - `"green"`.
  - `"yellow"`.
  - `"blue"`.
  - `"magenta"`.
  - `"cyan"`.
  - `"white"`.
  - `"bright black"`.
  - `"bright red"`.
  - `"bright green"`.
  - `"bright yellow"`.
  - `"bright blue"`.
  - `"bright magenta"`.
  - `"bright cyan"`.
  - `"bright white"`.

Style attributes are applied above colors to implement a specific style. They are:

- `"bold"`.
- `"dimmed"`.
- `"underline"`.
- `"reversed"`.
- `"italic"`.
- `"blink"`.
- `"hidden"`.
- `"strikethrough"`.

A _style_ is an object composed of three keys:

- `foreground` is the color to use as foreground.
- `background` is the color to use as foreground.
- `style` is a list of zero or more style attributes to apply.

### `[colors.description.todo]`

- Style to apply on description content of a task still left to do.
- Defaults to:
  - Foreground is `"bright white"`.
  - Background is `"black"`.
  - Style is `[]`.

### `[colors.description.ongoing]`

- Style to apply on description content of an on-going task.
- Defaults to:
  - Foreground is `"black"`.
  - Background is `"bright green"`.
  - Style is `[]`.

### `[colors.description.done]`

- Style to apply on description content of a done task.
- Defaults to:
  - Foreground is `"bright black"`.
  - Background is `"black"`.
  - Style is `["dimmed"]`.

### `[colors.description.cancelled]`

- Style to apply on description content of a cancelled task.
- Defaults to:
  - Foreground is `"bright black"`.
  - Background is `"black"`.
  - Style is `["dimmed", "strikethrough"]`.

### `[colors.status.todo]`

- Style to apply on status content of a task still left to do.
- Defaults to:
  - Foreground is `"magenta"`.
  - Background is none.
  - Style is `["bold"]`.

### `[colors.status.ongoing]`

- Style to apply on status content of an on-going task.
- Defaults to:
  - Foreground is `"green"`.
  - Background is none.
  - Style is `["bold"]`.

### `[colors.status.done]`

- Style to apply on status content of a done task.
- Defaults to:
  - Foreground is `"bright black"`.
  - Background is none.
  - Style is `["dimmed"]`.

### `[colors.status.cancelled]`

- Style to apply on status content of a cancelled task.
- Defaults to:
  - Foreground is `"bright red"`.
  - Background is none.
  - Style is `["dimmed"]`.

### `[colors.priority.low]`

- Style to apply on priority content of a low priority task.
- Defaults to:
  - Foreground is `"bright black"`.
  - Background is none.
  - Style is `["dimmed"]`.

### `[colors.priority.medium]`

- Style to apply on priority content of a medium priority task.
- Defaults to:
  - Foreground is `"blue"`.
  - Background is none.
  - Style is `[]`.

### `[colors.priority.high]`

- Style to apply on priority content of a high priority task.
- Defaults to:
  - Foreground is `"red"`.
  - Background is none.
  - Style is `[]`.

### `[colors.priority.critical]`

- Style to apply on priority content of a high priority task.
- Defaults to:
  - Foreground is `"black"`.
  - Background is `"bright red"`.
  - Style is `[]`.

### `[colors.show_header]`

- Style to apply on headers while showing tasks.
- Defaults to:
  - Foreground is `"bright black"`.
  - Background is none.
  - Style is `[]`.

[XDG Base Directory specification]: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
