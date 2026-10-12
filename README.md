# ed

[![language](https://img.shields.io/badge/language-Rust-DEA584)](https://www.rust-lang.org)

SIMPLE, lightweight terminal text editor.

<!-- TODO: add GIF here (asciinema) -->
<!-- ![demo](assets/demo.gif) -->

## Tech stack
 
Here's a brief high-level overview of the tech stack ed uses:
 
- This project is written in [Rust](https://www.rust-lang.org/) (2024 edition).
- For everything that touches the terminal, ed uses the [crossterm](https://crates.io/crates/crossterm) crate. It handles raw mode, the alternate screen, cursor movement, text styling, window titles, and reading keyboard and resize events.
- To parse command-line arguments, ed uses [clap](https://crates.io/crates/clap) with its derive API.
- To handle text correctly, ed uses [unicode-segmentation](https://crates.io/crates/unicode-segmentation) to split lines into grapheme clusters and [unicode-width](https://crates.io/crates/unicode-width) to measure how many terminal columns each character takes up.
- The UI is drawn without a TUI framework. Each part of the screen (text view, status bar, message bar, command bar) implements a small `UiComponent` trait and only redraws when something has changed.

## Features

- Multiple editor windows: open several files at once and switch between them without leaving the terminal
- Incremental search: results update as you type, jump forward/backward through matches, and the search wraps around the file
- Unicode-aware editing: the cursor moves by grapheme cluster and respects character width, so emoji and wide characters behave correctly
- "Save as" prompt for unnamed buffers, and a warning (press `^Q` again) before quitting with unsaved changes
- Status bar: file name, line count, modified indicator and current line position
- Message bar: short-lived notifications that fade after a few seconds
- Optional line numbers
- Clean terminal handling: uses the alternate screen, adapts to terminal resizes, and restores your terminal even if the editor panics
- Cross platform: compatible with Linux, Windows and MacOS

## Installation

Requires a recent stable [Rust toolchain](https://rustup.rs) (the project uses the 2024 edition).

```bash
git clone https://github.com/Balcus/ed.git
cd ed
cargo build --release
```

The binary will be at `target/release/ed`. To install it to `~/.cargo/bin`:

```bash
cargo install --path .
```

## Usage

Open files inside ed:
```bash
# Open one file inside ed
ed open main.rs

# Open multiple files, each in it's owm editor windo
- Cross platform: the apcompapompatible with Linux, Windows and MaxOcOS
ed open main.rs size.rs
```

Ask for help:
```bash
ed --help
```

You can also open files from inside the editor, see the keybindings below.

## Keybindings

**General**

| Key | Action |
|-----|--------|
| `Ctrl-S` | Save (asks for a file name if the buffer has none) |
| `Ctrl-Q` | Quit the current window (press twice if there are unsaved changes) |
| `Ctrl-F` | Find |
| `Ctrl-L` | Toggle line numbers |
| `Esc` | Cancel the current action (search, save, open) |

**Windows**

| Key | Action |
|-----|--------|
| `Ctrl-N` | New empty window |
| `Ctrl-O` | Open a file in a new window |
| `Ctrl-J` | Switch to the window on the left |
| `Ctrl-K` | Switch to the window on the right |

**Editing**

| Key | Action |
|-----|--------|
| `Ctrl-X` | Delete the current line |
| `Tab` | Insert a tab character |
| `Backspace` / `Delete` | Delete the character before / after the cursor |
| `Enter` | Insert a new line |

**Navigation**

| Key | Action |
|-----|--------|
| `←` `→` `↑` `↓` | Move the cursor |
| `Ctrl-←` / `Ctrl-→` | Jump one word left / right |
| `Home` / `End` | Start / end of line |
| `PageUp` / `PageDown` | Scroll one page up / down |

**While searching**

| Key | Action |
|-----|--------|
| *keyword* | Update the search as you type |
| `↑` / `↓` | Previous / next match |
| `Enter` | Accept the current match and exit search |
| `Esc` | Cancel and return to where you were |

## Todos

- [ ] Undo / redo
- [ ] Syntax highlighting
- [ ] Adding unit tests
- [ ] Better keyboard shortcuts
- [ ] Support for modes (similar to Vim)
