- RFC name: **Initial design**
- RFC type: **addition** | removal | change
- Date created: Sep 25, 2020
- Date updated: Sep 25, 2020
- Status: **draft** | review | accepted | refused | implemented

# Summary
> Brief summary of the change.

This document describes the initial MVP of the project. It contains:

- The definition of the scope of the project.
- The various components required for the first version.
- An in-depth description of each feature.
- A list of future features not included in the first versions that will be added later.

<!-- vim-markdown-toc GFM -->

* [Detailed contents](#detailed-contents)
  * [Scope of the project](#scope-of-the-project)
  * [Components](#components)
  * [Feature set](#feature-set)
    * [Metadata](#metadata)
    * [Ordering](#ordering)
    * [Task UID](#task-uid)
    * [Projects](#projects)
    * [Lifecycle](#lifecycle)
    * [Tags / labels](#tags--labels)
    * [Priorities](#priorities)
    * [Filtering](#filtering)
    * [Notes](#notes)
    * [History](#history)
    * [Time-based life cycle](#time-based-life-cycle)
  * [User interface / interaction](#user-interface--interaction)
    * [Main CLI interface](#main-cli-interface)
    * [Tags and priority syntax](#tags-and-priority-syntax)
    * [Adding a task](#adding-a-task)
    * [Listing tasks](#listing-tasks)
    * [Modifying the name of a task](#modifying-the-name-of-a-task)
    * [Modifying the priority, project or tags of a task](#modifying-the-priority-project-or-tags-of-a-task)
    * [Modifying both the name and priority, project or tags of a task](#modifying-both-the-name-and-priority-project-or-tags-of-a-task)
    * [Adding or editing notes](#adding-or-editing-notes)
    * [Describing a task](#describing-a-task)
    * [History of a task](#history-of-a-task)
  * [Configuration](#configuration)
    * [Colors and styles](#colors-and-styles)
      * [List of color sections](#list-of-color-sections)
      * [Style](#style)
      * [Color](#color)
  * [What’s next](#whats-next)
    * [Fuzzy search](#fuzzy-search)
* [Impacts](#impacts)
* [Unresolved questions](#unresolved-questions)
  * [What to do when a task goes by due date?](#what-to-do-when-a-task-goes-by-due-date)
* [Rationale and alternatives](#rationale-and-alternatives)

<!-- vim-markdown-toc -->

# Detailed contents
> Content of the RFC.

## Scope of the project

The project — named **toodoux** — English/French pun between _todo_ (EN) and _doux_ (FR) — is a task management
system that aims to be _super simple_ to operate but yet provide access to powerful features. It is heavily based on
(mainly) two softwares, among:

- [org-mode], from which it takes the concept of editing using tags to annotate tasks, like `TODO`, `DONE`, etc.
- [taskwarrior], from which it takes lots of ideas for the CLI.

However, it doesn’t have a scope as big as [org-mode]’s and adds new features over [taskwarrior]. [org-mode] is not
_just_ a task manager — it’s so much more than this — and because **toodoux** ought to follow the UNIX philosophy,
things like personal wiki / literate configuration / etc. is completely out of scope.

Just like [taskwarrior], **toodoux** is a CLI application and not a plugin to an editor. It will remain a CLI
application and contributions are welcomed as long as they keep that in mind. No support for any editor will be added
directly into **toodoux**. It doesn’t prevent us to provide libraries and helpers so that editors integrate **toodoux**
directly, but it will not be on our side.

## Components

**toodoux** is made out of two main components:

- The `toodoux` Rust crate. This library crate allows other Rust developer to manipulate and use **toodoux**
  capabilities from within a developer perspective.
- The `td` binary, which uses the `toodoux` crate. It is what people will most likely use.

Some other components might be shipped, such as _completions_ for typical shells (**bash**, **zsh** and **fish**), man
pages, etc.

## Feature set

### Metadata

Tasks have _metadata_. A metadata is an information that is associated with a task, such as its creation date,
the project it belongs to, its due date, priority, etc. The complete list:

- _Unique identifier (UID)_: a number that uniquely identifies the task and is used to manipulate it.
- _Project_: a task belongs to either no project, or a single project. Tasks without project are considered _orphaned_.
  Orphaned tasks are useful to quickly capture an idea and move it to a project later.
- _Creation date_: the date when the task was captured into the system.
- _Due date_: the date when the task is due.
- _Status_: the status of the task.
- _Priority_: four priorities are supported and help sorting the tasks.
- _Tags_: free and user-defined tags that can used to filter and sort tasks more easily. A task can have several tags.
- _Notes_: an optional text users can use to add notes to a task; for instance while working on a task, a user can put
  some notes about the resolution of a problem, what they tried, what worked, etc. Notes are formatted in Markdown.
- _Event history_: a set of ordered events that have happened to the task. It gathers all the other metadata and pins
  them to a date to provide a proper historical view of what happened to a project. Even changing the content of a
  task is an event.

### Ordering

By default, and for the first version of **toodoux** this is not going to be customizable, the task are presented sorted
to the user by following this order:

- Priorities: `CRITICAL`, `HIGH`, `MEDIUM`, `LOW`.
- Age: the older tasks first.
- Status: `WIP` first, `TODO` then for active; `CANCELLED` first, `DONE` then for inactive.
- UID.

### Task UID

A task, upon creation, is given a _unique identifier_ (UID). That UID is used later on to interact with the task.

Past tasks keep their previous UIDs, so as time passes, UID values will increase. Auto-completion and fuzzy-searching
will be required to provide a nice UX when working with tasks.

### Projects

By default, a task is automatically placed into the _orphaned_ project, which means it’s not linked to any project.
Projects allow to classify tasks by user-defined namespace. Project are simple, specific tags and not real entity.
However, it is possible to set an _active_ project. When an active project is set, all commands will automatically
filter tasks with this project.

For the initial design, a task can only belong to a single project.

### Lifecycle

The four _status_ define the lifecycle of a task. Without explicit setting, a task starts in the _todo_ status. It will
then evolve through different status, depending on what’s happening:

1. `TODO`: the initial and default status of a task.
2. `WIP`: status of a task when it’s been started.
3. `DONE`: the task has been completely done.
4. `CANCELLED`: the task has been cancelled. This status is useful to keep track of the task even if not done.

On those four status, `TODO` and `WIP` are considered _active_ and `DONE` and `CANCELLED` are considered _inactive_.
Those considerations define the default behavior when listing tasks: only _active_ tasks are shown. Inactive tasks can
still be listed by providing the right filtering options.

Switching from one status to another is logged in history. It’s possible to go from any status to any other.

### Tags / labels

Tags / labels are a way to add additional filtering flags to tasks to group and classify them. For instance, inside a
_“project-alpha”_ project, one might want to classify tasks regarding whether they are about documentation, bugs,
features or regressions, for instance. Tags are free objects users can set on tasks, and a task can have as many tags
as wanted.

### Priorities

Priorities are a simple way to sort tasks in a way that shows urgent ones first. Four level of priorities are provided:

1. `LOW`: low-priority task that will be shown after all higher ones.
2. `MEDIUM`: medium-priority task that will be shown after all higher ones.
3. `HIGH`: high-priority task that will be shown after all higher ones.
4. `CRITICAL`: task that will be shown after all others with emphasis.

A task can have only one priority, but it’s possible to change it whenever wanted.

### Filtering

Filtering allows to prevent _some_ notes from being displayed in listing. Filtering operates on several fields:

- The name of the task.
- Its priority.
- Its project.
- Its tags.

The syntax used to filter is described in the [user interface section](#user-interface--interaction).

### Notes

Tasks can be added notes, which are Markdown entries associated with a timestamp.

### History

Tasks are a collection in time of events. The only immutable property of a task is its UID. The rest is just computed
from its event history.

Most of the time, users won’t be interested into seeing the history of a task — they will simply want to have a look at
its current _state_. However, it is possible to get an exhaustive listing of what happened to a task.

### Time-based life cycle

Besides all the metadata and notes, a task can also be added some information related to its life cycle. This
information are:

- Its _creation date_, which allows to show its _age_. This information allows to know how old a task is and can be
  used to re-prioritize it.
- Its _activation duration_. Activation duration is a measure that is done by computing the time user has been spending on
  it. The way it’s done is actually simple: it is the sum of the time passed while the task is in `WIP` status.
  Switching its status back to `TODO` or to `DONE` or `CANCELLED` will not make the duration impacted anymore. If a task
  has some _activation duration_ and is moved back to `TODO`, the activation duration should still be visible in
  listings, but greyed out.
- Its _completion duration_. When a task is moved to `DONE` or `CANCELLED` status, its _activation duration_ is
  automatically transformed into a _completion duration_.
- Its _due date_, which allows to put a deadline to the task to make it more prioritized than other tasks. If the
  current time goes by the due date, the task’s priority gets boosted.

## User interface / interaction

### Main CLI interface

The initial interface is via the _command line_. Two group of main commands exist:

- `td [filters] <verb> <options>`
- `td <task_uid> [filters] <verb> <options>`

This is heavily inspired by [taskwarrior]’s CLI. The first form allows to interact with tasks without specifying a
task. It’s useful for listing all tasks, adding a new task or editing the configuration. The second form acts on a
specific task by using its UID. It can be useful to change its status, add notes, tags, move it into a project, change
its priority, etc.

### Tags and priority syntax

When creating a new task, it’s possible to add special words at the end of its name to set some metadata on:

- Tags are introduced with the `#` prefix and must be space separated with each words starting with a `#`.
- Priorities are introduced with `+` prefix:
  - Low priority is `+l`.
  - Medium priority is `+m`.
  - High priority is `+h`.
  - Critical priority is `+c`.
- Projects are introduced with `@` prefix.

Example:

```
"Do the laundry @house +m #boring #housework"
"Sweep the floor @house +m #housework"
"Take the dog to the vet @house +h #dog"
```

### Adding a task

Adding a task is done with the `add` verb, which can also be aliased `a`:

```sh
td add <…>
td a <…>
```

It can take several options:

- `--start`: upon creation, change the status of the task to `WIP`.
- `--done`: upon creation, change the status of the task to `DONE`.
- `--cancelled`: upon creation, change the status of the task to `CANCELLED` — it seems weird, but if you’re doing this
  for backtracking, it should be possible to do.

Examples:

```sh
td a --start Write the initial design of toodoux @toodoux +m #easy #design
```

### Listing tasks

Listing all tasks can be done easily by using the `list` or `ls` verb:

```sh
td list <…>
td ls <…>
```

It can take several filters as arguments:

- `--todo`: list `TODO` tasks.
- `--start`: list `WIP` tasks.
- `--done`: list `DONE` tasks.
- `--cancelled`: list `CANCELLED` tasks.
- `--all`: list all tasks.

Additionally, it is possible to use the [#filtering](#filtering) syntax to filter on:

- Projects with the `@` notation.
- Priorities with the `+` notation.
- Tags with the `#` notation.
- Any free text to search inside the content of tasks.

Filtering is allowed _before_ the text. For instance:

```sh
td ls @house`
```

Will show all the tasks for the `house` project.

```sh
td ls #dog vet
```

Will show all the tasks related to our dog and vet.

```sh
td ls +c
td ls +h #cat
```

Will show all the critical tasks, and high-priority tasks about our cat.

### Modifying the name of a task

Modifying the name of a task is done easily by using the `edit` or `ed` or `e` verb and simply using some text. You will
also need the ID of the task (that you can get from the listing right now).

```sh
td <task_uid> edit <new name>
td <task_uid> ed <new name>
td <task_uid> e <new name>
```

For instance:

```sh
td 1 e New name for this task
```

### Modifying the priority, project or tags of a task

As with the name, priorities, projects and tags can be changed with the `edit` / `ed` / `e` verb. Simply use the
filtering options as if you were creating the task, without putting any text afterwards.

Example:

```sh
td 1 e @garden
```

Will move the task which UID is 1 from its current project to the `garden` project.

```sh
td 34 e +h #late
```

Will apply (if not already present) the `late` tag and will put the `HIGH` priority on the task which UID is 34.

### Modifying both the name and priority, project or tags of a task

It is possible to modify all the information of a task by simply mixing them together. Projects, priorities and tags
go first — as in filtering. The name of the task goes afterwards.

Example:

```sh
td 34 e +h #late Finish my homework
```

### Adding or editing notes

Tasks, by default, are a collection of metadata (project, tags, priorities and name). There is, however, another
property that can be set by the user: notes.

Adding notes to a task allow to describe / detail what has been going on since trying to resolve it, for instance. That
kind of information is logged in the history, and each entry is logged as Markdown.

Notes are added with the `note` verb.

```sh
td <task_uid> note
```

It is also possible to edit the already existing notes. This is done with the `--edit` or `-e` switch:

```sh
td <task_uid> note -edit
td <task_uid> note -e
```

All those commands – whether you add or edit — will open our `$EDITOR` or, if not set, will look into the configuration
to spawn your editor so that you can add / edit notes. There is no format / syntax when adding notes: it’s just plain
Markdown. Have fun!

However, when editing notes, you will be editing all the notes at once. You are advised not to edit the metadata present
in the document, or **toodoux** will simply refuse the update.

### Describing a task

So far, we’ve been able to list tasks and operate on a single task. Another feature is to _describe_ a task, with the
`show` / `s` verb.

```sh
task <task_uid> show
```

That command will show the content of a task in a detailed way: all of its metadata along with its notes (if any).

### History of a task

The history of task can be obtained with the `history` / `hist` / `h` verb.

```sh
task <task_uid> history
task <task_uid> hist
task <task_uid> h
```

This command will provide the exhaustive history of what happened to a task.

## Configuration

Configuration is done by following the [XDG Base Directory specification] by default but can be overriden by the user
if required. The configuration root directory is `$XDG_CONFIG_DIR/toodoux` — it should be `~/.config/toodoux` for most
people on Linux, for instance.

The configuration file, `config.toml`, is a TOML file that contains a single section, called `[main]`, which contains
the main configuration.

> We reserve the right to use other sections for further, more precise configuration.

That section contains the following keys.

- `tasks_file`: path to the file to use to hold tasks.
- `todo_alias`: name to use when showing the `TODO` status.
- `wip_alias`: name to use when showing the `WIP` status.
- `done_alias`: name to use when showing the `DONE` status.
- `active_project`: project name to use to filter. Can be set to `null` or removed if no project should be used.
- `editor`: if `$EDITOR` is not set, this variable will be used to edit notes. If this variable is set while `$EDITOR`
  is set too, `$EDITOR` has predominance.

### Colors and styles

Colors can be configured by the use of several sections, in the form of:

```toml
[colors.<item>.<nested-item>]
```

#### List of color sections

Many nested items might exist and the level of nesting depends on which properties you want to set colors for.
Currently, you can change the colors of all these items:

| Item                              | Section name                     |
| ----                              | ---                              |
| Description of a _todo_ item      | `[colors.description.todo]`      |
| Description of an _ongoing_ item  | `[colors.description.ongoing]`   |
| Description of a _done_ item      | `[colors.description.done]`      |
| Description of a _cancelled_ item | `[colors.description.cancelled]` |
| _Todo status_                     | `[colors.status.todo]`           |
| _Ongoing_ status                  | `[colors.status.ongoing]`        |
| _Done_ status                     | `[colors.status.done]`           |
| _Cancelled_ status                | `[colors.status.cancelled]`      |
| Low priority                      | `[colors.priority.low]`          |
| Medium priority                   | `[colors.priority.medium]`       |
| High priority                     | `[colors.priority.high]`         |
| Critical priority                 | `[colors.priority.critical]`     |


A color section can contain three key-value pairs:

- `foreground`: foreground color to use.
- `background`: background color to use.
- `style`: a list of style attribute to set.

#### Style

A _style_ is a collection — list — of _style attributes_. A style attribute is a unique string representing how
texts should be stylized.

The exhaustive list of styles:

- `"bold"`: make the text bold.
- `"dimmed"`: dim the text.
- `"underline"`: underline the text.
- `"reversed"`: reverse the color of text (foreground color becomes background and vice versa).
- `"italic"`: emphasis with italic.
- `"blink"`: make the text blink. Note: use at your own caution, as not all terminals support that.
- `"hidden"`: hide the text.
- `"strikethrough"`: strike through style.

#### Color

A color is defined as a formatted string. For this current RFC and inherent implementation, we suggest to support these
two formats:

- Human readable version of colors for terminal colors. Those are:
  - `"black"`.
  - `"red"`.
  - `"green"`.
  - `"yellow"`.
  - `"blue"`.
  - `"magenta"`.
  - `"purple"`.
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
- RGB colors parsed as hexadecimal strings, preceded by a hash (`#`). We support both uppercase and lowercase as well
  as triplet shortcuts — i.e. `"#rrggbb"` can be rewritten as `"#rgb"`.

The above example includes all possible styles.

## What’s next

### Fuzzy search

Right now, commands expecting a `<task_uid>` require the user to go through this workflow:

1. Use `td list` or `td ls` (with, maybe, some filtering options).
2. Search for the task they want and remember its UID.
3. Issue the command they want by replacing `<task_uid>` with the UID they found previously.

While this is okay-ish, a much better interface would be to allow the user to completely omit the `<task_uid>`. In that
situation, we would spawn a fuzzy finder that would allow the user to search for their tasks by using the filtering
syntax, and that would automatically find the right UID associated with the task.

While this feature is exciting, it’s not going to be in the first version and will be added in a later version. An RFC
will be needed to describe the exact interaction with the user.

# Impacts
> Does that change have any impact, and if so, which?

N/A

# Unresolved questions
> Any questions that need to be addressed before going on with implementation.

## What to do when a task goes by due date?

Possible choices are:

- We consider it as `+h`. `+c` should be set by the user only to have “overriding” priorities on anything else, so it’s
  not a good candidate. But `+h` is.
- Display the task in a specific color to show it’s late, but do not change its priority.

# Rationale and alternatives
> In the end.

N/A

[org-mode]: https://orgmode.org
[taskwarrior]: https://taskwarrior.org
[XDG Base Directory specification]: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
