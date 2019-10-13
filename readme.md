# Amazeing

Are you still sure that every acyclic maze can be easily solved by sticking
to the left (or right) wall? Try this one!

## Screenshots
![Screenshot](screenshots/0.png)

## Dependencies
If you have never compiled Rust before, visit [rustup.rs](https://rustup.rs/) to install
compiler toolchain.

You will also need `SDL2` and `SDL2_ttf` libraries.
You can obtain them using your favourite package manager or download from official website:
[SDL2](https://www.libsdl.org/download-2.0.php),
[SDL2_ttf](https://www.libsdl.org/projects/SDL_ttf/).

#### Ubuntu
```
# apt-get install libsdl2-dev libsdl2-ttf-dev
```

#### Arch Linux
```
# pacman -S sdl2 sdl2_ttf
```


## Running
After that compiling and running Amazeing is as easy as this:
```
$ cargo run --release
```

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

## Credits
Used font - [mystyle](https://www.fontspace.com/ashleyeden/mystyle) by
AshleyEden (Tamlyn Nicholson).
