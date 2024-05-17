# Rime

Rime is intended to be a Vim-like text editor.
Main frameworks are Ratatui, Crossterm and Clap.

## Features

Right now, Rime has :
- simple Undo/ Redo
- some basic commands such as tabnew, write, quit, all...
- some synthax highlighting for .rs files
- cursor and scrolling still have some issues

## Usage

It's imporatant to notice that you need cargo to build the project. If you did not installed Rust, you should take a look on there website.

```bash
cargo run -- [FILE_NAME...]

# Note that you can use clap feature for help
cargo run --help
```
