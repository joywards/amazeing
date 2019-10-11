# Amazeing

Are you still sure that every acyclic maze can be easily solved by sticking
to the left (or right) wall? Try this one!

## Screenshots
![Screenshot](screenshots/0.png)

## Controls

#### Main menu
Keyboard                | Action
----------------------- | ------------
<kbd>Enter</kbd>        | Start playing
<kbd>&uparrow;</kbd>, <kbd>&downarrow;</kbd> | Select level
<kbd>&leftarrow;</kbd>, <kbd>&rightarrow;</kbd> | Select stage of the current level
<kbd>Esc</kbd>          | Exit game

#### In-game
Keyboard                | Action
----------------------- | ------------
<kbd>&uparrow;</kbd>, <kbd>&leftarrow;</kbd>, <kbd>&rightarrow;</kbd>, <kbd>&downarrow;</kbd> | Move
<kbd>Space</kbd>        | Move one step backwards
<kbd>`</kbd>            | Use a hint (only available after a while)
<kbd>Esc</kbd>          | Return to the main menu

## Running
If you have never compiled Rust before, visit [rustup.rs](https://rustup.rs/) to install
compiler toolchain.

You will probably also need
[SDL2 development libraries](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries).

After that compiling and running Amazeing is as easy as this:
```
$ cargo run --release
```
