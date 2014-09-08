#![feature(globs)]

extern crate ncurses;

mod game;


fn main()
{

    let mut game = game::Game::new();

    game.initialize();
    
    while game.is_running() {
        game.run();
    }
}
