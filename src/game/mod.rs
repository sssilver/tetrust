use ncurses::*;

use renderer::Renderer;
use renderer::Renderable;
use renderer::Coord;
mod board;


pub struct Game {
    is_running: bool,
    board: board::Board,
    renderer: Renderer
}


impl Game {
    pub fn new() -> Game {
        Game {
            is_running: true,
            board: board::Board::new(),
            renderer: Renderer::new(),
        }
    }

    pub fn initialize(&self) {
        self.renderer.initialize();

    }
    
    pub fn run(&mut self) {
        self.board.render(Coord::new(5, 7), &self.renderer);

        self.process_keyboard();

        // Update the screen
        //clear();
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
