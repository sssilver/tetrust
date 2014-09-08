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

        start_color();               // Use color

        // Create the ncurses color palette
        //init_color(16i16, 0, 43 * 4, 54 * 4);
        //init_color(17i16, 142 * 4, 161 * 4, 161 * 4);
        init_color(1i16, 130, 130, 120);
        init_color(2i16, 500, 500, 500);
        init_pair(1i16, 1i16, 1i16);
        init_pair(2i16, 2i16, 2i16);
    }

    pub fn block(&self, pos: Coord, color: int) {
        // Draw a full block in pos
        self.pixel(Coord::new(pos.x, pos.y), color);
        self.pixel(Coord::new(pos.x + 1, pos.y), color);
    }

    pub fn pixel(&self, pos: Coord, color: int) {
        // Draw half a block in pos
        attron(COLOR_PAIR(color as i16));  // Set the color

        move(pos.y as i32, pos.x as i32);  // Move in position
        printw("#");  // Render
    }

    pub fn text(&self, text: String, pos: Coord) {
        move((pos.y) as i32, (pos.x * 2) as i32);

        printw(text.as_slice());
    }
}
