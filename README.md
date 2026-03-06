# note CLI
A simple lightweight command-line notes app written in Rust.
This tool allows you to add notes, view them later, and erase them straight from the terminal.

This project was built to practice working with Rust file I/O and building command-line interfaces. 

## Features
- Append notes from the command line
- List all saved notes
- Clear all stored notes

## Installation
Clone the repo
```bash
git clone https://github.com/carmex12/note
cd note
```
Build the project
```bash
cargo build --release
```
Run the compiled binary
```bash
./target/release/note <command>
```

# Technologies used
- Rust
- clap for command-line arg parsing
- Rust standard library for file handling

# Future Improvements
- Store notes in system data directory