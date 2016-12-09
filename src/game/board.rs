use game::polyomino::Cell;
use renderer::{Point, Renderer};


pub struct Board {
    pub size: (u8, u8),
    pub field: Vec<Cell>
}


impl Board {
    pub fn new(size: (u8, u8)) -> Board {
        Board {
            size: size,
            field: Vec::new()
        }
    }
}
