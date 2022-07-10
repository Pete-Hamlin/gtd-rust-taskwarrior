# gtd-rust-taskwarrior

A small wrapper CLI app to assist with using taskwarrior for the GTD methodology. Written in rust

## Justification - Why does this exist?

[Taskwarrior](https://taskwarrior.org/) is a great tool for support the [gtd](https://gettingthingsdone.com/) methodology, with great support form the following features:
- A next task list (with priority)
- Tickle folder/tasks
- Someday/maybe lists
- Projects(ish - see below)
- Contexts (via tags e.g. @home, @work)
- Delegate/waiting tasks

However certain aspects of the methodology I find are not as easy to interact with as, such as:
- Projects are only tracked if they have pending tasks associated with them by default, with the only other option to show all projects that have **ever existed** making project review a convoluted process
- I believe GTD relies heavily on reducing friction for creating/updating tasks & projects as much as possible in order to keep the user (me) motivated/up to date.
- By personal preference, pen and paper lists are not an ideal solution for me - I want something in software.
- It would be great to have an app to work with taskwarrior to assist with filling in the gaps/friction points with GTD.
- I have had a burning desire to try and write something in rust for some time


## Installation/Usage

This project is currently in pre-release status, so please treat accordingly. That being said, if you still want to try it out, run the following from the root of the repo:

```
cargo build --release
```

From there copy `./target/release/gtd` to somewhere within your `$PATH`. Then simply invoke the command using `gtd`.

Once I'm satisfied with the base level behaviour, I hope to publish via cargo to make installation easier.

## Specification

`gtd-rust 1.0.0` will provide support for the following:

 - [ ] Have an easily customisable CLI interface to adapt to changing use cases
 - [x] Keep a separate inventory to taskwarrior for active projects 
    - [x] Assign an ID for each active project
    - [x]Provide a method to import tasks from taskwarrior via `gtd init`
 - [x] Provide a method to manually add tasks to the active projects via `gtd add <PROJECT>` 
 - [ ] Provide a method to implement a taskwarrior hook to add projects to the inventory via `gtd hooks`
 - [ ] Provide an easy review process for **all** current projects via `gtd project review`
     - [ ] Assign next actions for those that do not currently have them  - [ ] mark finished projects as completed.
     - [ ] Provide  a `--quick` quick option, to only review projects without a `next` action
    - [x] Provide a `--short` option to only display a list of currently active projects for review
- [x] Provide closing command for open projects via `gtd <ID> done`

Additional considerations:
- [ ] If context is not `none either:
  - [x] Warn the user with regards to currently set context
  - [ ] Switch context to none for duration of the command, then revert back

## Assumptions

- Will treat all projects as WYSIWYG - i.e. taskwarrior subproject syntax will just be treated as is (e.g `shopping.boots` will be treated as a separate project unrelated to `shopping.aldi`)
    - The reason for this is that (for me) sub projects are dealt with just fine via taskwarrior. The parent project (in this example `shopping`) fits more in the horizon 3 category of GTD, which this tool is not aiming to implement (yet...?)
- No duplicate projects are allowed
- When a project is marked as completed, for `gtd` it no longer exists (i.e deleted). 
    - History/reopening projects is handled fine by taskwarrior. The purpose of this piece is just to track ongoing projects (specifically those with no current next actions - which taskwarrior by default sees as completed)

## Config

- `next_action_tag`: The tag used to represent next action in taskwarrior (default: `next`)
