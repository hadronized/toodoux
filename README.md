# toodoux, a task manager in CLI / TUI

**toodoux** – English/French pun between _todo_ (EN) and _doux_ (FR, _soft_) — is a task management system that aims to
be _super simple_ to operate but yet provide access to powerful features. It is heavily mainly based on [taskwarrior],
for its powerful CLI and presentation. However, the opinionated task workflow in **toodoux** is rather different from
what you would find in [taskwarrior].

Just like [taskwarrior], **toodoux** is a CLI application and not a plugin for an editor. It will remain a CLI
application and contributions are welcomed as long as they keep that in mind (see the [contributing guide]). No support
for any editor will be added directly into **toodoux**. It doesn’t prevent us to provide libraries and helpers to help
external applications integrate **toodoux** directly, but it will not be on our side.

**toodoux** is made out of two main components:

- The `toodoux` Rust crate. This library crate allows other Rust developer to manipulate and use **toodoux**
  capabilities from within a developer perspective. It also ships the binary version.
- The `td` binary, which uses the `toodoux` crate. It is what people will most likely use.

Some other components might be shipped, such as _completions_ for typical shells (**bash**, **zsh** and **fish**), man
pages, etc.

## Features

The exhaustive feature set is available [here](./doc/features.md).

## CLI User Guide

The user guide is availble [here](./doc/cli.md).

## User configuration

The user configuration is available [here](./doc/config.md).

[taskwarrior]: https://taskwarrior.org
[contributing guide]: CONTRIBUTING.md
