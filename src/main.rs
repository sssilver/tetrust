extern crate pancurses;

mod curses_input;
mod curses_renderer;
mod error;
//mod game;
mod input;
mod renderer;
mod subsystem;

//use game::Game;
use input::{Input, Key};
use renderer::Renderer;
use subsystem::Subsystem;


fn main()
{
    let window = pancurses::initscr();

    let mut renderer = curses_renderer::CursesRenderer::new(&window);
    let mut input = curses_input::CursesInput::new(&window);

    renderer.start();
    input.start();

    loop {
        input.execute();

        if input.is_pressed(Key::Pause).unwrap() {
            break;
        }
        renderer.text("Rocket in the sky".to_string(), (10, 10));

        renderer.execute();
    }

    println!("Shutting down");

    input.stop();
    renderer.stop();

    pancurses::endwin();

    //let tetris = 4u8;  // Regular Tetris
    /*let mut game = Game::new(tetris);

    game.initialize();

    while game.is_running() {
        game.run();
    }
    */
}
