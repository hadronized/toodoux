# Features

This document provides a comprehensive list of the features as currently implemented in the binary form of **toodoux**.

<!-- vim-markdown-toc GFM -->

* [Overview](#overview)
* [Concepts](#concepts)
  * [Metadata](#metadata)
  * [Lifecycle](#lifecycle)
    * [The four status](#the-four-status)
    * [Status customization](#status-customization)
    * [Implicit / computed metadata](#implicit--computed-metadata)
  * [Priorities](#priorities)
  * [Tags / labels](#tags--labels)
  * [Notes](#notes)
  * [Metadata syntax](#metadata-syntax)
    * [Operators](#operators)
    * [Inline syntax](#inline-syntax)
* [User interface](#user-interface)
  * [Adding a new task](#adding-a-new-task)
  * [Editing a task](#editing-a-task)
  * [Describing a task](#describing-a-task)
  * [Consult the history of a task](#consult-the-history-of-a-task)
  * [Switch the status of a task](#switch-the-status-of-a-task)
  * [Listing tasks](#listing-tasks)
  * [Adding notes](#adding-notes)
  * [Editing notes](#editing-notes)
* [Configuration](#configuration)
  * [`[main]`](#main)
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
  * [`[colors]`](#colors)
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

## Overview

**toodoux** helps users with capturing _tasks_, listing them, edit them and manipulating various metadata related to
task management, such as status, dates and times, tags, projects, notes, etc. It is a CLI application that expects a
working terminal. Color support is highly recommended for a better experience.

## Concepts

### Metadata

Tasks have _metadata_. A metadata is an information that is associated with a task, such as its creation date,
the project it belongs to, its creation / modification dates, priority, etc. The complete list:

- _Unique identifier (UID)_: a number that uniquely identifies the task and is used to manipulate it.
- _Project_: a task belongs to either no project, or a single project. Tasks without project are considered _orphaned_.
  Orphaned tasks are useful to quickly capture an idea and move it to a project later.
- _Creation date_: the date when the task was captured into the system.
- _Modification dates_: the dates when the task was modified.
- _Status_: the status of the task.
- _Priority_: four priorities are supported and help sorting the tasks.
- _Tags_: free and user-defined tags that can be used to filter and sort tasks more easily. A task can have as many tags
  as wanted.
- _Notes_: an optional set of ordered texts users can use to add more details to a task; for instance while working on a
  task, a user can recorde some notes about the resolution of a problem, what they tried, what worked, etc. Notes are
  formatted in Markdown.
- _Event history_: a set of ordered events that have happened to the task. It gathers all the other metadata and pins
  them to a date to provide a proper historical view of what happened to a project.

### Lifecycle

#### The four status

The four _status_ define the lifecycle of a task. Without explicit setting, a task starts in the _todo_ status. It will
then evolve through different status, depending on what is happening:

1. `TODO`: the initial and default status of a task; means that the task was recorded in the system is not currently
  on-going.
2. `WIP`: status of a task when it is been started.
3. `DONE`: the task has been completely done.
4. `CANCELLED`: the task has been cancelled. This status is useful to keep track of the task even if not done.

On those four status, `TODO` and `WIP` are considered _active_ and `DONE` and `CANCELLED` are considered _inactive_.
Those considerations define the default behavior when listing tasks: only _active_ tasks are shown. Inactive tasks can
still be listed by providing the right filtering options.

Switching from one status to another is logged in history. It is possible to go from any status to any other.

#### Status customization

The real names the status will use is a user-defined setting. See the [Configuration](#configuration) section for
further information.

#### Implicit / computed metadata

Besides all the metadata and notes, a task is also added some metadata related to its life cycle. Those information,
automatically computed, are:

- Its _creation date_, which allows to show its _age_. This information allows to know how old a task is and can be
  used to re-prioritize it.
- Its _activation duration_. Activation duration is a measure that is done by computing the time user has been spending on
  it. The way it is done is rather simple: it is the sum of the durations the user passed on the task while in `WIP`
  status. Switching its status back to `TODO`, to `DONE` or `CANCELLED` will not make the duration impacted anymore. If
  a task has some _activation duration_ and is moved back to `TODO`, the activation duration should still be visible in
  listings, but greyed out.
- Its _completion duration_. When a task is moved to `DONE` or `CANCELLED` status, its _activation duration_ is
  automatically transformed into a _completion duration_.

### Priorities

Priorities are a simple way to sort tasks in a way that shows urgent ones first. Four level of priorities are provided:

1. `LOW`: low-priority task that will be shown after all higher ones.
2. `MEDIUM`: medium-priority task that will be shown after all higher ones.
3. `HIGH`: high-priority task that will be shown after all higher ones.
4. `CRITICAL`: task that will be shown with emphasis after all others.

A task can have only one priority, but it is possible to change it whenever wanted.

### Tags / labels

Tags / labels are a way to add additional filtering flags to tasks to group and classify them. For instance, inside a
_“project-alpha”_ project, one might want to classify tasks regarding whether they are about documentation, bugs,
features or regressions, for instance. Tags are free objects users can set on tasks, and a task can have as many tags
as wanted.

### Notes

Tasks can be added notes, which are Markdown entries associated with a timestamp. A task is always added notes one by
one — i.e. there is no rich text editing where several notes can be edited all at once. Task edition is performed by
opening an interactive editor instead of typing the note on the command line.

When editing notes, it is possible to ask for the history help – i.e. previously recorded notes – for the task you are
adding a note for. See the `--note` switch.

### Metadata syntax

The metadata syntax is a simple yet powerful mechanism to quickly add metadata to a task or to refine a query. Several
metadata are associated with a prefix operator that represents the class of the metadata:

#### Operators

| Class        | Operator | Example          |
| =====        | ======== | =======          |
| **Project**  | `@`      | `@toodoux`       |
| **Priority** | `+`      | `+h`             |
| **Tags**     | `#`      | `#documentation` |

Each operator is expected to be in a prefix position behind a string, representing the value for this class. For
instance, `@toodoux` means “the toodoux project.” `+h` means the high priority. Etc. etc.

Priorities are a bit special as they do not accept arbitrary strings. Refer to this table to know which string to use
regarding the kind of priority you want to use:

| Priority   | String |
| ========   | ====== |
| `CRITICAL` | `c`  |
| `HIGH`     | `h`  |
| `MEDIUM`   | `m`  |
| `LOW`      | `l`  |

#### Inline syntax

Metadata operators can be inlined and combined while adding, editing or quering tasks. For instance, the following
string means “tasks in the toodoux project, high priority, with tags _#foo_ and _#bar_:”

```
@toodoux +h #foo #bar
```

The order is not important, for any of the metadata. All of the following strings are equivalent:

```
@toodoux +h #foo #bar
+h @toodoux #foo #bar
#foo +h #bar @toodoux
```

Adding _free terms_ allow to, depending on the command, fill the task name or refine a query:

```
@toodoux +h #foo #bar reduce
```

In the context of a query, this string will match any task containing `reduce` for “the toodoux project, high
priority with tags _#foo_ and _#bar_.” Free text can be placed anywhere.

## User interface

Two groups of main commands exist:

> `td            <verb> <options>`

> `td <task_uid> <verb> <options>`

This is heavily inspired by [taskwarrior]’s CLI. The first form allows to interact with tasks without specifying a
task. It is useful for listing all tasks or adding a new task. The second form acts on a specific task by using its
UID. It can be useful to change its status, add notes, tags, move it into a project, change its priority, etc.

Lots of commands accept _aliases_. For instance, the `add` command also accepts the `a` alias. When a command has
possible aliases, those will be listed when the command is introduced.

### Adding a new task

> `td add <content> [options]`

> `td a   <content> [options]`

This command captures a new task in the system.

- `<content>` is the content of the task as described in the [metadata syntax](#metadata-syntax) section. When creating
  a new task, you can pass the actual name of the task, such as `Do this`, but you can also mix the metadata syntax
  with it, such as `@my-project Do this +h #documentation`.
- `[options]` can be zero or several of:
  - `--done`: mark the item as done.
  - `--start`: immediately start working on the task.

### Editing a task

> `td <task-uid> edit [content]`

> `td <task-uid> ed   [content]`

> `td <task-uid> e    [content]`

This command edits an already registered task by registering new values or its content / metadata. You can change
its content, any of the metadata or all at the same time. If you omit the content, it will be left untouched. If you
omit a metadata, it will be left untouched.

- `<task-uid>` is the task UID referring to the task to edit.
- `<content>` is the content of the task as described in the [metadata syntax](#metadata-syntax) section.

### Describing a task

> `td <task-uid> show`

> `td <task-uid> s`

Show the current state of a task.

This command is currently the only one showing the notes and their respective UIDs, too.

- `<task-uid>` is the task UID referring to the task to edit.

### Consult the history of a task

> `td <task-uid> history`

Show the history of a task. This command will print everything that has happened to a task, with the associated time
at which the event happened.

- `<task-uid>` is the task UID referring to the task to edit.

### Switch the status of a task

> `td <task-uid> todo`

> `td <task-uid> start`

> `td <task-uid> done`

> `td <task-uid> cancel`

These four commands allow to change the status of a task, whatever the previous status is. It is important to notice
that you should not have to use `todo` too often: indeed, you will only need that command when you have started working
on a task and want to “stop working on it.” This workflow is useful as it will take into account _only_ the time you
work on a task. If you care about this kind of stats, moving the task back to its _todo_ state will stop counting spent
time on it. You can then resume it later. It is also an interesting tool if you care about the change history of a
task, as it is recorded there.

- `<task-uid>` is the task UID referring to the task to edit.

### Listing tasks

> `td list [content] [options]`

> `td ls   [content] [options]`

> `td l    [content] [options]`

Listing tasks is one of the most useful commands you will ever run. For this reason, it’s the default command if you
don’t pass any command to run to the binary. Listing tasks is currently the only way you have to know the UID of a task
(besides the output of the `add` command, which will also give you this information).

- `<content>` is the content of the tasks as described in the [metadata syntax](#metadata-syntax) section. In this case,
  it’s used as a language query. Terms are used to refined the search by conjunction (i.e. a task must fulfill all the
  query terms). For instance, `@toodoux +h` – or `+h @toodoux`, it is the same query – will only list high priority
  tasks for the `toodoux` project; it will not list all `toodoux` tasks along with all high priority tasks.
- `[options]`:
  - `--todo` will list tasks still left to do.
  - `--start` will list tasks.
  - `--done` will list done tasks.
  - `--cancelled` will list cancelled tasks.
  - The flags above are additive.
  - `--all` will list all tasks and is the same as `--todo --start --done --cancelled`.
  - If you don’t specify one or more of `--all`, `--todo`, `--start`, `--done` and/or `--cancelled`, then the
    listing will default to _active_ tasks.
  - `--case-insensitive` allows to perform search inside the name of tasks with a case-insensitive algorithm.

### Adding notes

> `td <task-uid> note add [options]`

> `td <task-uid> note a   [options]`

- `<task-uid>` is the task UID referring to the task to edit.
- `[options]`:
  - `--no-history`: override user configuration and do not see the note history help.

This command allows you to record a new note for a given task, referred to by `<task-uid>`. The note will be written
in an editor, open trying to use the first of, in order:

1. The `$EDITOR` environment variable.
2. The `interactive_editor` entry in the user configuration file.

If none of those choices ended up with a working editor, an error is emitted and it’s not possible to add the note.
Otherwise, the editor is open and let the user write their note in it. Once the note is written, quitting the editor
after having saved the file will make **toodoux** record this note for the task referred to by `<task-uid>`.

Several possibilities can arise when the editor opens, though:

- If the user has the `previous_notes_help` flag set to `true` in the user configuration file, then the content of the
  file will be open with is pre-filled with the note history of the given task. That pre-fill is part of a header
  separated from the user note with a line that must not be removed. Anything in the header can be modified as it will
  be discarded before recording the new note.
- If that flag is set to `false`, no history will be shown – hence no marker line either.
- If `--no-history` is passed to this command, the previous two points above are overridden and no history will be
  shown.

Saving an empty note (with or without the header) aborts the operation.

### Editing notes

> `td <task-uid> note <note-uid> edit [options]`

> `td <task-uid> note <note-uid> ed   [options]`

> `td <task-uid> note <note-uid> e    [options]`

- `<task-uid>` is the task UID referring to the task to edit.
- `<note-uid>` is the note UID referring to the note to edit.
- `[options]`:
  - `--no-history`: override user configuration and do not see the note history help.

This command is very similar to `note add` but expects a note — referred to by `<note-uid>` – to operate on. When
opening the editor, the same rules apply as with `note add`, but the pre-fill is also appended the current note
you are editing.

Saving an empty note (with or without the header) aborts the operation.

## Configuration

Configuration is done by following the [XDG Base Directory specification] by default but can be overridden by the user
if required. The configuration root directory is `$XDG_CONFIG_DIR/toodoux` — it should be `~/.config/toodoux` for most
people on Linux, for instance.

The configuration file, `config.toml`, is a TOML file that contains several sections:

- `[main]`, containing the main configuration of **toodoux**.
- `[colors]`, containing all the configuration keys to customize the colors and styles used by **toodoux**.

> We reserve the right to use other sections for further, more precise configuration.

### `[main]`

The `[main]` section contains the following keys.

#### `interactive_editor`

- Editor to use for interactive editing.
- Defaults to none.

#### `tasks_file`

- Path to the folder containing all the tasks.
- Defaults to `$XDG_CONFIG_DIR/toodoux`.

#### `todo_alias`

- Name of the _tood_ state.
- Defaults to `TODO`.

#### `wip_alias`

- Name of the _on-going_ state.
- Defaults to `WIP`.

#### `done_alias`

- Name of the _done_ state.
- Defaults to `DONE`.

#### `cancelled_alias`

- Name of the _cancelled_ state.
- Defaults to `CANCELLED`.

#### `uid_col_name`

- UID column name.
- Defaults to `IUD`.

#### `age_col_name`

- Age column name.
- Defaults to `Age`.

#### `spent_col_name`

- Spent column name.
- Defaults to `Spent`.

#### `prio_col_name`

- Priority column name.
- Defaults to `Prio`.

#### `project_col_name`

- Project column name.
- Defaults to `Project`.

#### `tags_col_name`

- Tags column name.
- Defaults to `Tags`.

#### `status_col_name`

- Status column name.
- Defaults to `Status`.

#### `description_col_name`

- Description column name.
- Defaults to `Description`.

#### `notes_nb_col_name`

- Number of notes column name.
- Defaults to `Notes`.

#### `display_empty_cols`

- Whether or not display empty columns in listing views.
- Defaults to `false`.

#### `max_description_lines`

- Maximum number of warping lines of task description before breaking it (and adding the ellipsis character) if it’s
  too long.
- Defaults to `2`.

#### `display_tags_listings`

- Display tags in listings.
- Defaults to `true`.

#### `previous_notes_help`

- Show the previously recorded notes when adding a new note for a given task.
- Defaults to `true`.

### `[colors]`

Colors are configured via several sub-sections:

- `[colors.description.*]` contains all the styles for changing the description content in listing depending on the
  status of the task.
- `[colors.status.*]` contains all the styles for changing the status content in listing depending on the
  status of the task.
- `[colors.priority.*]` contains all the styles for changing the priority content in listing depending on the
  priority of the task.
- `[colors.show_header]` contains the style to apply on headers while describing notes.

Colors can be encoded via several formats:

- Regular RGB hexadecimal strings — `#rrggbb` or `#rgb`.
- Terminal colors are supported with the following names:
  - `black`.
  - `red`.
  - `green`.
  - `yellow`.
  - `blue`.
  - `magenta`.
  - `cyan`.
  - `white`.
  - `bright black`.
  - `bright red`.
  - `bright green`.
  - `bright yellow`.
  - `bright blue`.
  - `bright magenta`.
  - `bright cyan`.
  - `bright white`.

Style attributes are applied above colors to implement a specific style. They are:

- `bold`.
- `dimmed`.
- `underline`.
- `reversed`.
- `italic`.
- `blink`.
- `hidden`.
- `strikethrough`.

A _style_ is an object composed of three keys:

- `foreground` is the color to use as foreground.
- `background` is the color to use as foreground.
- `style` is a list of zero or more style attributes to apply.

#### `[colors.description.todo]`

- Style to apply on description content of a task still left to do.
- Defaults to:
  - Foreground is `bright white`.
  - Background is `black`.
  - Style is `[]`.

#### `[colors.description.ongoing]`

- Style to apply on description content of an on-going task.
- Defaults to:
  - Foreground is `black`.
  - Background is `bright green`.
  - Style is `[]`.

#### `[colors.description.done]`

- Style to apply on description content of a done task.
- Defaults to:
  - Foreground is `bright black`.
  - Background is `black`.
  - Style is `["dimmed"]`.

#### `[colors.description.cancelled]`

- Style to apply on description content of a cancelled task.
- Defaults to:
  - Foreground is `bright black`.
  - Background is `black`.
  - Style is `["dimmed", "strikethrough"]`.

#### `[colors.status.todo]`

- Style to apply on status content of a task still left to do.
- Defaults to:
  - Foreground is `magenta`.
  - Background is none.
  - Style is `["bold"]`.

#### `[colors.status.ongoing]`

- Style to apply on status content of an on-going task.
- Defaults to:
  - Foreground is `green`.
  - Background is none.
  - Style is `["bold"]`.

#### `[colors.status.done]`

- Style to apply on status content of a done task.
- Defaults to:
  - Foreground is `bright black`.
  - Background is none.
  - Style is `["dimmed"]`.

#### `[colors.status.cancelled]`

- Style to apply on status content of a cancelled task.
- Defaults to:
  - Foreground is `bright red`.
  - Background is none.
  - Style is `["dimmed"]`.

#### `[colors.priority.low]`

- Style to apply on priority content of a low priority task.
- Defaults to:
  - Foreground is `bright black`.
  - Background is none.
  - Style is `["dimmed"]`.

#### `[colors.priority.medium]`

- Style to apply on priority content of a medium priority task.
- Defaults to:
  - Foreground is `blue`.
  - Background is none.
  - Style is `[]`.

#### `[colors.priority.high]`

- Style to apply on priority content of a high priority task.
- Defaults to:
  - Foreground is `red`.
  - Background is none.
  - Style is `[]`.

#### `[colors.priority.critical]`

- Style to apply on priority content of a high priority task.
- Defaults to:
  - Foreground is `black`.
  - Background is `bright red`.
  - Style is `[]`.

#### `[colors.show_header]`

- Style to apply on headers while showing tasks.
- Defaults to:
  - Foreground is `bright black`.
  - Background is none.
  - Style is `[]`.

[taskwarrior]: https://taskwarrior.org
[contributing guide]: CONTRIBUTING.md
[XDG Base Directory specification]: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
