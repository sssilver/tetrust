pub mod board;
mod polyomino;
mod state;


use game::board::Board;
use error::Result;
use input::{Action, Input};
use rand::{thread_rng};
use rand::distributions::{IndependentSample, Range};
use renderer::{Renderable, Renderer};
use std::vec::Vec;


type Point = (u8, u8);

pub struct Game<'a> {
    is_running: bool,
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
            renderer: renderer,
            state: state::State { board: Board::new((10, 23)) },
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
        self.input.execute();
        self.process_input();

        self.renderer.text((1, 1), "Tetrust".to_string());

        let mut rng = thread_rng();
        let range = Range::new(0i32, 80i32);
        self.renderer.block((range.ind_sample(&mut rng), range.ind_sample(&mut rng)));

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
                /*
                Action::Up => { self.state.top -= 1; },
                Action::Right => { self.state.left += 1; },
                Action::Down => { self.state.top += 1; },
                Action::Left => { self.state.left -= 1; },
                */
                Action::Pause => { self.is_running = false; },
                _ => {}
            }
        }
    }

    pub fn is_running(&self) -> bool {
        return self.is_running;
    }

    pub fn quit(&mut self) -> Result<()> {
        println!("Shutting down");

        self.input.stop()?;
        self.renderer.stop()?;

        Ok(())
    }
}
