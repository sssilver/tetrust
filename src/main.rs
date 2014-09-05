#![feature(globs)]

extern crate ncurses;

use ncurses::*;

fn main()
{
    // Start ncurses
    initscr();

    // Print to the back buffer
    printw("Hello, world!");

    // Update the screen
    refresh();

    let mut game = Game {  // Make the game mutable
        is_running: true
    };

    game.initialize();
    
    while game.is_running() {
        game.run();

        // Read a key press
        let key = getch();
        if key == 27 {  // ESC pressed; quit the game
            game.quit();
        }

        printw(format!("{:d} pressed", key).as_slice());
    }

    // Terminate ncurses
    endwin();
}


struct Game {
    is_running: bool
}


impl Game {
    fn initialize(&self) {
        printw("Initializing the game");
    }
    
    fn run(&self) {
        printw("Running the game");
    }

    fn is_running(&self) -> bool {
        return self.is_running;
    }

    fn quit(&mut self) {
        self.is_running = false;
    }
}
