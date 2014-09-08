use ncurses::*;

mod board;


pub struct Game {
    is_running: bool,
    board: board::Board
}


impl Game {
    pub fn new() -> Game {
        Game {
            is_running: true,
            board: board::Board::new()
        }
    }

    pub fn initialize(&self) {
        initscr();  // Start ncurses
        cbreak();  // Disable TTY input buffering; we need fast input
        keypad(stdscr, true);  // Enable expended keyboard buttons
        noecho();  // No output upon keypress
        nodelay(stdscr, true);  // Enable non-blocking getch()
        curs_set(CURSOR_INVISIBLE);  // Hide the cursor

        start_color();
        init_color(16i16, 0, 43 * 4, 54 * 4);
        init_color(17i16, 142 * 4, 161 * 4, 161 * 4);
        init_pair(1i16, 16i16, 17i16);

    }
    
    pub fn run(&mut self) {
        printw("Running the game");

        self.board.render((5, 7));

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
