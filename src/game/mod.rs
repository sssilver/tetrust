use error::Result;
use input::{Action, Input};
use pancurses;
use renderer::{Renderable, Renderer};
use std::vec::Vec;


mod board;
mod polyomino;
mod state;


type Point = (u8, u8);

pub struct Game<'a> {
    is_running: bool,
    board: board::Board,
    state: state::State,
    complexity: u8,

    pieces: Vec<polyomino::Polyomino>,

    renderer: &'a mut Renderer,
    input: &'a mut Input
}


impl<'a> Game<'a> {
    pub fn new(complexity: u8, renderer: &'a mut Renderer, input: &'a mut Input) -> Game<'a> {
        Game {
            is_running: true,
            board: board::Board::new((10, 23)),
            renderer: renderer,
            state: state::State { left: 0, top: 0 },
            input: input,
            complexity: complexity,
            pieces: polyomino::generate(complexity)
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        self.renderer.start()?;
        self.input.start()?;

        Ok(())
    }

    pub fn run(&mut self) {
        //self.board.render((0, 0), self.renderer);

        self.input.execute();
        self.process_input();

        self.renderer.text("Rocket in the sky".to_string(), (self.state.left, self.state.top));

        self.renderer.execute();

        /*
        match self.pieces.iter().next() {
            Some(piece) => piece.render((0, 0), &self.renderer),
            None => ()
        }
        */
        //self.pieces[0].render((0, 0), self.renderer);
    }

    fn process_input(&mut self) {
        for action in self.input.actions() {
            match action {
                Action::Up => { self.state.top -= 1; },
                Action::Right => { self.state.left += 1; },
                Action::Down => { self.state.top += 1; },
                Action::Left => { self.state.left -= 1; },
                Action::Pause => { self.is_running = false; },
                _ => {}
            }
        }
    }

    pub fn is_running(&self) -> bool {
        return self.is_running;
    }

    pub fn quit(&mut self) {
        println!("Shutting down");

        self.input.stop();
        self.renderer.stop();
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
