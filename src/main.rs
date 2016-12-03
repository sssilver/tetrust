extern crate pancurses;

mod curses_input;
mod curses_renderer;
mod error;
mod game;
mod input;
mod renderer;
mod subsystem;

use game::Game;
use input::{Input, Key};
use renderer::Renderer;
use subsystem::Subsystem;


fn main()
{
    let complexity = 4u8;  // Regular Tetris
    let mut game = Game::new(complexity);

    game.initialize();

    while game.is_running() {
        game.run();
    }

    game.quit();
}
