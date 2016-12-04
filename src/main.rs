extern crate pancurses;

mod curses_input;
mod curses_renderer;
mod error;
mod game;
mod input;
mod renderer;
mod subsystem;

use curses_input::CursesInput;
use curses_renderer::CursesRenderer;
use game::Game;
use input::{Action, Input};
use renderer::Renderer;


fn main()
{
    let complexity = 4u8;  // Regular Tetris

    let window = pancurses::initscr();  // Initialize ncurses

    let mut input= CursesInput::new(&window);
    let mut renderer = CursesRenderer::new(&window);

    let mut game = Game::new(complexity, &mut renderer, &mut input);

    game.initialize();

    while game.is_running() {
        game.run();
    }

    game.quit();

    pancurses::endwin();  // Restore ncurses
}
