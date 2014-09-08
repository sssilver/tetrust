use ncurses::*;


pub struct Coord {
    x: int,
    y: int
}


pub trait Renderable {
    fn render(&self, pos: Coord, renderer: &Renderer);
}


pub struct Renderer;


impl Coord {
    pub fn new(x: int, y: int) -> Coord {
        Coord {x: x, y: y}
    }

    pub fn x(&self) -> int {
        self.x
    }

    pub fn y(&self) -> int {
        self.y
    }
}


impl Renderer {
    pub fn new() -> Renderer {
        Renderer
    }

    pub fn initialize(&self) {
        initscr();                   // Start ncurses
        cbreak();                    // Disable TTY input buffering; we need fast input
        keypad(stdscr, true);        // Enable expended keyboard buttons
        noecho();                    // No output upon keypress
        nodelay(stdscr, true);       // Enable non-blocking getch()
        curs_set(CURSOR_INVISIBLE);  // Hide the cursor

        start_color();
        init_color(16i16, 0, 43 * 4, 54 * 4);
        init_color(17i16, 142 * 4, 161 * 4, 161 * 4);
        init_pair(1i16, 16i16, 17i16);
    }

    pub fn point(&self, pos: Coord) {
        // Draw a point in x, y
        move((pos.y) as i32, (pos.x * 2) as i32);

        printw("#.");
        
    }
}
