``` 
 _____     _               _   
|_   _|___| |_ ___ _ _ ___| |_ 
  | | | -_|  _|  _| | |_ -|  _|
  |_| |___|_| |_| |___|___|_|  
                               
```

Tetrust is an implementation of [Tetris](https://en.wikipedia.org/wiki/Tetris) in
[Rust](https://www.rust-lang.org). Currently a work in progress.

The project's sole aim is to aid me in my quest of getting better at programming in Rust.

Installation
------------
- Clone this repository
- Install the [latest stable Rust](https://www.rust-lang.org/en-US/downloads.html)
- `$ cd <cloned_tetrust_directory>`
- `$ cargo run`

Interesting details
-------------------
- Polyominoes are generated; game could be customized for arbitrary lattice animal complexity
- Uses [pancurses](https://crates.io/crates/pancurses), which has the promise of supporting
both Linux/UNIX through ncurses, and Windows through whatever it is that Windows uses
- Game could be customized for arbitrary board size
