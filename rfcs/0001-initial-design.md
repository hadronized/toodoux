- RFC name: **Initial design**
- RFC type: **addition** | removal | change
- Author: Dimitri Sabadie <dimitri.sabadie@gmail.com>
- Date created: Sep 25, 2020
- Date updated: Sep 25, 2020
- Status: **draft** | review | accepted | refused

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
    * [Task UID](#task-uid)
    * [Projects](#projects)
    * [Lifecycle](#lifecycle)
  * [User interface](#user-interface)
  * [What’s next](#whats-next)
* [Impacts](#impacts)
* [Unresolved questions](#unresolved-questions)
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
- _Status_: the status of the task. We currently support four status, which define a lifecycle of the task:
  1. `TODO`: the initial and default status of a task.
  2. `WIP`: status of a task when it’s been started.
  3. `DONE`: the task has been completely done.
  4. `CANCELLED`: the task has been cancelled. This status is useful to keep track of the task even if not done.
- _Priority_: four priorities are supported and help sorting the tasks. Ascending priority:
  1. `LOW`: low-priority task that will be shown after all higher ones.
  2. `MEDIUM`: medium-priority task that will be shown after all higher ones.
  3. `HIGH`: high-priority task that will be shown after all higher ones.
  4. `CRITICAL`: task that will be shown after all others with emphasis.
- _Tags_: free and user-defined tags that can used to filter and sort tasks more easily. A task can have several tags.
- _Notes_: an optional text users can use to add notes to a task; for instance while working on a task, a user can put
  some notes about the resolution of a problem, what they tried, what worked, etc. Notes are formatted in Markdown.
- _Event history_: a set of ordered events that have happened to the task. It gathers all the other metadata and pins
  them to a date to provide a proper historical view of what happened to a project. Even changing the content of a
  task is an event.

### Task UID

A task, upon creation, is given a _unique identifier_ (UID). That UID is used later on to interact with the task.

Past tasks keep their previous UIDs, so as time passes, UID values will increase. Auto-completion and fuzzy-searching
will be required to provide a nice UX when working with tasks.

### Projects

By default, a task is automatically placed into the _orphaned_ project, which means it’s not linked to any project.
Projects allow to classify tasks by user-defined namespace.

### Lifecycle

The four _status_ define TODO

## User interface

## What’s next

# Impacts
> Does that change have any impact, and if so, which?

N/A

# Unresolved questions
> Any questions that need to be addressed before going on with implementation.

N/A

# Rationale and alternatives
> In the end.

N/A

[org-mode]: https://orgmode.org
[taskwarrior]: https://taskwarrior.org
