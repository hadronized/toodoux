# toodoux, a task manager in CLI / TUI

**toodoux** is a CLI task manager based on several famous softwares (among [Org Mode], [taskwarrior] and [git]).
Even though heavily influenced by Org Mode, it doesn’t plan to implement Org Mode but rather be a
solid and fun task manager to work with. It is made for people who want to to quickly create task
notes, sort them, and add, edit, remove and list what’s around.

## Feature list

- [ ] Basic task management.
  - [x] Add new tasks.
    - [x] CLI mode.
    - [x] Editor mode.
    - [x] Name support.
    - [x] Content support.
    - [x] State support.
    - [ ] Labels support.
    - [ ] Priorities support.
    - [ ] Projects support.
  - [ ] List tasks.
    - [x] List all todo and on-going tasks.
    - [x] List tasks by filtering by states.
    - [ ] List tasks by filtering by labels.
    - [ ] List tasks by filtering by priorities.
    - [ ] Interactive TUI.
  - [ ] View tasks.
    - [ ] Display the content of a task by UID.
    - [ ] Display the content of a task via fuzzy-searching.
    - [ ] Display the content of a task in TUI.
  - [ ] Remove tasks.
    - [ ] Remove by their UID.
    - [ ] Remove via fuzzy-searching.
    - [ ] Interactive TUI.
  - [ ] Edit tasks.
    - [x] Change name in CLI.
    - [x] Change state in CLI.
    - [ ] Change labels in CLI.
    - [ ] Change priorities in CLI.
    - [ ] Change schedule in CLI.
    - [ ] Change deadline in CLI.
    - [ ] Change everything in editor.
    - [ ] Change everything in TUI.
- [ ] Capture mode and projects.
  - [ ] Projects-driven tasks.
  - [ ] Implement the capture special project.
  - [ ] Implement moving a task from one project to another.
  - [ ] Re-make the listing / filtering to use capture first.
- [ ] Styles and configuration.
  - [x] Change the tag names of states.
  - [ ] Change the style of UIDs.
  - [ ] Change the style of names.
  - [ ] Change the style of states.
  - [ ] Change the style of priorities.
  - [ ] Change the style of labels.

[Org Mode]: https://orgmode.org
[taskwarrior]: https://taskwarrior.org
[git]: https://git-scm.com
