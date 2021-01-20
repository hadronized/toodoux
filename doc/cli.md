# Command Line Interface: User Guide

Two groups of main commands exist:

- td `cmd` **options**
- td **task_uid** `verb` **options**

This is heavily inspired by [taskwarrior]’s CLI. The first form allows to interact with tasks without specifying a
task. It is useful for listing all tasks or adding a new task. The second form acts on a specific task by using its
UID. It can be useful to change its status, add notes, tags, move it into a project, change its priority, etc.

Lots of commands accept _aliases_. For instance, the `add` command also accepts the `a` alias. When a command has
possible aliases, those will be listed when the command is introduced.

<!-- vim-markdown-toc GFM -->

* [Adding a new task](#adding-a-new-task)
* [Editing a task](#editing-a-task)
* [Describing a task](#describing-a-task)
* [Consult the history of a task](#consult-the-history-of-a-task)
* [Switch the status of a task](#switch-the-status-of-a-task)
* [Listing tasks](#listing-tasks)
* [Adding notes](#adding-notes)
* [Editing notes](#editing-notes)

<!-- vim-markdown-toc -->

## Adding a new task

```
td add <content> [options]
td a   <content> [options]
```

This command captures a new task in the system.

- **content** is the content of the task as described in the [metadata syntax] section. When creating
  a new task, you can pass the actual name of the task, such as `Do this`, but you can also mix the metadata syntax
  with it, such as `@my-project Do this +h #documentation`.
- _options_ can be zero or several of:
  - `--done`: mark the item as done.
  - `--start`: immediately start working on the task.

## Editing a task

```
td <task-uid> edit [content]
td <task-uid> ed   [content]
td <task-uid> e    [content]
```

This command edits an already registered task by registering new values or its content / metadata. You can change
its content, any of the metadata or all at the same time. If you omit the content, it will be left untouched. If you
omit a metadata, it will be left untouched.

- **task-uid** is the task UID referring to the task to edit.
- **content** is the content of the task as described in the [metadata syntax] section.

## Describing a task

```
td <task-uid> show
td <task-uid> s
```

Show the current state of a task.

This command is currently the only one showing the notes and their respective UIDs, too.

- **task-uid** is the task UID referring to the task to edit.

## Consult the history of a task

```
td <task-uid> history
```

Show the history of a task. This command will print everything that has happened to a task, with the associated time
at which the event happened.

- **task-uid** is the task UID referring to the task to edit.

## Switch the status of a task

```
td <task-uid> (todo | start | done | cancel)
```

These four commands allow to change the status of a task, whatever the previous status is. It is important to notice
that you should not have to use `todo` too often: indeed, you will only need that command when you have started working
on a task and want to “stop working on it.” This workflow is useful as it will take into account _only_ the time you
work on a task. If you care about this kind of stats, moving the task back to its _todo_ state will stop counting spent
time on it. You can then resume it later. It is also an interesting tool if you care about the change history of a
task, as it is recorded there.

- **task-uid** is the task UID referring to the task to edit.

## Listing tasks

```
td list [content] [options]
td ls   [content] [options]
td l    [content] [options]
```

Listing tasks is one of the most useful commands you will ever run. For this reason, it’s the default command if you
don’t pass any command to run to the binary. Listing tasks is currently the only way you have to know the UID of a task
(besides the output of the `add` command, which will also give you this information).

- **content** is the content of the tasks as described in the [metadata syntax] section. In this case,
  it’s used as a language query. Terms are used to refined the search by conjunction (i.e. a task must fulfill all the
  query terms). For instance, `@toodoux +h` – or `+h @toodoux`, it is the same query – will only list high priority
  tasks for the `toodoux` project; it will not list all `toodoux` tasks along with all high priority tasks.
- _options_:
  - `--todo` will list tasks still left to do.
  - `--start` will list tasks.
  - `--done` will list done tasks.
  - `--cancelled` will list cancelled tasks.
  - The flags above are additive.
  - `--all` will list all tasks and is the same as `--todo --start --done --cancelled`.
  - If you don’t specify one or more of `--all`, `--todo`, `--start`, `--done` and/or `--cancelled`, then the
    listing will default to _active_ tasks.
  - `--case-insensitive` allows to perform search inside the name of tasks with a case-insensitive algorithm.

## Adding notes

```
td <task-uid> note add [options]
td <task-uid> note a   [options]
```

This command allows you to record a new note for a given task, referred to by **task-uid**. The note will be written
in an editor, open trying to use the first of, in order:

1. The `$EDITOR` environment variable.
2. The `interactive_editor` entry in the user configuration file.

If none of those choices ended up with a working editor, an error is emitted and it’s not possible to add the note.
Otherwise, the editor is open and let the user write their note in it. Once the note is written, quitting the editor
after having saved the file will make **toodoux** record this note for the task referred to by **task-uid**.

Several possibilities can arise when the editor opens, though:

- If the user has the `previous_notes_help` flag set to `true` in the user configuration file, then the content of the
  file will be open with is pre-filled with the note history of the given task. That pre-fill is part of a header
  separated from the user note with a line that must not be removed. Anything in the header can be modified as it will
  be discarded before recording the new note.
- If that flag is set to `false`, no history will be shown – hence no marker line either.
- If `--no-history` is passed to this command, the previous two points above are overridden and no history will be
  shown.

Saving an empty note (with or without the header) aborts the operation.

- **task-uid** is the task UID referring to the task to edit.
- _options_:
  - `--no-history`: override user configuration and do not see the note history help.

## Editing notes

```
td <task-uid> note <note-uid> edit [options]
td <task-uid> note <note-uid> ed   [options]
td <task-uid> note <note-uid> e    [options]
```

This command is very similar to `note add` but expects a note — referred to by `<note-uid>` – to operate on. When
opening the editor, the same rules apply as with `note add`, but the pre-fill is also appended the current note
you are editing.

Saving an empty note (with or without the header) aborts the operation.

- **task-uid** is the task UID referring to the task to edit.
- **note-uid** is the note UID referring to the note to edit.
- _options_:
  - `--no-history`: override user configuration and do not see the note history help.

[metadata syntax]: ./features.md#metadata-syntax
[taskwarrior]: https://taskwarrior.org
[contributing guide]: CONTRIBUTING.md
[XDG Base Directory specification]: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
