use pancurses;
use std::vec::Vec;

use curses_input::CursesInput;
use curses_renderer::CursesRenderer;
use input::{Input, Key};
use renderer::{Renderable, Renderer};
use subsystem::Subsystem;

mod board;
mod polyomino;


type Point = (u8, u8);

pub struct Game<'a> {
    is_running: bool,
    board: board::Board,
    complexity: u8,

    pieces: Vec<polyomino::Polyomino>,

    window: pancurses::Window,
    renderer: CursesRenderer<'a>,
    input: CursesInput<'a>
}


impl<'a> Game<'a> {
    pub fn new(complexity: u8) -> Game<'a> {
        let window = pancurses::initscr();

        Game {
            is_running: true,
            board: board::Board::new((10, 23)),
            window: window,
            renderer: CursesRenderer::new(&window),
            input: CursesInput::new(&window),
            complexity: complexity,
            pieces: polyomino::generate(complexity)
        }
    }

    pub fn initialize(&self) {
        self.renderer.start();
        self.input.start();
    }

    pub fn run(&mut self) {
        self.board.render((0, 0), &mut self.renderer);

        self.input.execute();

        if self.input.is_pressed(Key::Pause).unwrap() {
            self.quit();
        }

        self.renderer.text("Rocket in the sky".to_string(), (10, 10));

        self.renderer.execute();

        /*
        match self.pieces.iter().next() {
            Some(piece) => piece.render((0, 0), &self.renderer),
            None => ()
        }
        */
        self.pieces[0].render((0, 0), &mut self.renderer);

        self.input.execute();

        //clear();

        // Update the screen
        //refresh();
    }

    pub fn is_running(&self) -> bool {
        return self.is_running;
    }

    pub fn quit(&mut self) {
        println!("Shutting down");

        self.input.stop();
        self.renderer.stop();

        self.is_running = false;

        pancurses::endwin();  // Restore ncurses
    }

    /*
    fn process_keyboard(&mut self) {
        // Read a keypress
        let key = getch();

        if key == 27 {  // ESC pressed: quit the game
            self.quit();
        }

        if key == 32 {
            self.pieces.get_mut(0).rotate(true);
        }
    }
    */
}
