extern crate pancurses;

mod curses_input;
mod error;
//mod game;
mod input;
//mod renderer;
mod subsystem;

//use game::Game;
use input::{Input, Key};
use subsystem::Subsystem;


fn main()
{
    let window = pancurses::initscr();

    let mut input = curses_input::CursesInput::new(&window);

    input.start();

    loop {
        input.execute();

        if input.is_pressed(Key::Pause).unwrap() {
            println!("Pause");
        }
    }

    input.stop();

    //let tetris = 4u8;  // Regular Tetris
    /*let mut game = Game::new(tetris);

    game.initialize();

    while game.is_running() {
        game.run();
    }
    */
}
