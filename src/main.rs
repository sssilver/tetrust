#![feature(globs)]

extern crate ncurses;

mod renderer;
mod game;


fn main()
{
	let tetris = 4u8;  // Regular Tetris
    let mut game = game::Game::new(tetris);

    game.initialize();

    while game.is_running() {
        game.run();
    }
}
