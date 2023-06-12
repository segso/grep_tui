# Grep Tui
This is a application for basic search in files with a Terminal User Interface (TUI), it's written in Rust and uses [crossterm](https://github.com/crossterm-rs/crossterm) and [tui-rs](https://github.com/fdehau/tui-rs) for the interface.

---

## Getting Started

In order to build the application, you will need to have installed [git](https://git-scm.com/) and [rust](https://www.rust-lang.org/).

---

## Building

To clone the project and build it, use the following commands:
```bash
git clone https://github.com/seg-mx/grep_tui
cd grep_tui
cargo build --release
```

After doing that, the binary will be in `./target/release/grep_tui`.

---

## Usage

The interface has some components, every component has the key you have to press to focus that component. To quit the focus press `Esc`. When you have a component focused, the usage changes:

- Text input
  - This element should be intuitive, but in case it's not, press any key to add that character to the input, and backspace for removing characters
- Results
  - Here will appear the search results, press the arrow keys for going up or down. It's useful when the results overflows the screen size.

---

## Quitting

Like in most terminal applications, to exit you have to press `Ctrl + C`.

---

## Troubleshooting

If you experience any problems with the application, [open an issue](https://github.com/seg-mx/grep_tui/issues/new) explaining the situation and I'll try my best to help.
