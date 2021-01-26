# Features

This document provides a comprehensive list of the features as currently implemented in the binary form of **toodoux**.

<!-- vim-markdown-toc GFM -->

* [Metadata](#metadata)
* [Lifecycle](#lifecycle)
  * [The four status](#the-four-status)
  * [Status customization](#status-customization)
  * [Implicit / computed metadata](#implicit--computed-metadata)
* [Projects](#projects)
* [Priorities](#priorities)
* [Tags / labels](#tags--labels)
* [Notes](#notes)
* [Metadata syntax](#metadata-syntax)
  * [Operators](#operators)
  * [Inline syntax](#inline-syntax)

<!-- vim-markdown-toc -->

## Metadata

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

## Lifecycle

### The four status

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

### Status customization

The real names the status will use is a user-defined setting. See the [Configuration](./config.md) section for
further information.

### Implicit / computed metadata

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

## Projects

Tasks are by default _orphaned_: they don’t belong to any project. You can gather tasks under a project to group them
in a more easy way.

Projects are free names acting like special tags: a task can be in either no or one project at most. Projects on their
own don’t exist currently as a specific kind of object in **toodoux**: they are just labels.

## Priorities

Priorities are a simple way to sort tasks in a way that shows urgent ones first. Four level of priorities are provided:

1. `LOW`: low-priority task that will be shown after all higher ones.
2. `MEDIUM`: medium-priority task that will be shown after all higher ones.
3. `HIGH`: high-priority task that will be shown after all higher ones.
4. `CRITICAL`: task that will be shown with emphasis after all others.

A task can have only one priority, but it is possible to change it whenever wanted.

## Tags / labels

Tags / labels are a way to add additional filtering flags to tasks to group and classify them. For instance, inside a
_“project-alpha”_ project, one might want to classify tasks regarding whether they are about documentation, bugs,
features or regressions, for instance. Tags are free objects users can set on tasks, and a task can have as many tags
as wanted.

## Notes

Tasks can be added notes, which are Markdown entries associated with a timestamp. A task is always added notes one by
one — i.e. there is no rich text editing where several notes can be edited all at once. Task edition is performed by
opening an interactive editor instead of typing the note on the command line.

When editing notes, it is possible to ask for the history help – i.e. previously recorded notes – for the task you are
adding a note for. See the `--note` switch.

## Metadata syntax

The metadata syntax is a simple yet powerful mechanism to quickly add metadata to a task or to refine a query. Several
metadata are associated with a prefix operator that represents the class of the metadata:

### Operators

| Class        | Operator | Example          |
| -----        | -------- | -------          |
| **Project**  | `@`      | `@toodoux`       |
| **Priority** | `+`      | `+h`             |
| **Tags**     | `#`      | `#documentation` |

Each operator is expected to be in a prefix position behind a string, representing the value for this class. For
instance, `@toodoux` means “the toodoux project.” `+h` means the high priority. Etc. etc.

Priorities are a bit special as they do not accept arbitrary strings. Refer to this table to know which string to use
regarding the kind of priority you want to use:

| Priority   | String |
| --------   | ------ |
| `CRITICAL` | `c`    |
| `HIGH`     | `h`    |
| `MEDIUM`   | `m`    |
| `LOW`      | `l`    |

### Inline syntax

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
