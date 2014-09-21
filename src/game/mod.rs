use ncurses::*;
use std::vec::Vec;

use renderer::Renderer;
use renderer::Renderable;

mod board;
mod polyomino;


type Point = (int, int);

pub struct Game {
    is_running: bool,
    board: board::Board,
    renderer: Renderer,
    complexity: u8,

    pieces: Vec<polyomino::Polyomino>,
}


impl Game {
    pub fn new(complexity: u8) -> Game {
        Game {
            is_running: true,
            board: board::Board::new(),
            renderer: Renderer::new(),
            complexity: complexity,
            pieces: polyomino::generate(complexity)
        }
    }

    pub fn initialize(&self) {
        self.renderer.initialize();
    }

    pub fn run(&mut self) {
        self.board.render((2, 2), &self.renderer);

        let mut x = 1u8;

        match self.pieces.iter().next() {
            Some(piece) => piece.render((0, 0), &self.renderer),
            None => ()
        }

        self.process_keyboard();

        // Update the screen
        refresh();
    }

    pub fn is_running(&self) -> bool {
        return self.is_running;
    }

    fn quit(&mut self) {
        self.is_running = false;

        endwin();
    }

    fn process_keyboard(&mut self) {
        // Read a keypress
        let key = getch();

        if key == 27 {  // ESC pressed: quit the game
            self.quit();
        }
    }
}
