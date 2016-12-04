use game::polyomino::Cell;
use renderer::{Point, Renderable, Renderer};


pub struct Board {
    size: (u8, u8),
    field: Vec<Cell>
}


impl Board {
    pub fn new(size: (u8, u8)) -> Board {
        Board {
            size: size,
            field: Vec::new()
        }
    }

    /*
    pub fn render(&self, pos: (i32, i32)) {
        bkgd(' ' as u32 | COLOR_PAIR(1i16) as u32);
    }
    */
}


impl Renderable for Board {
    fn render(&self, pos: Point, renderer: &mut Renderer) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                if y == self.size.1 - 1 || x == 0 || x == self.size.0 {  // Edge
                    renderer.block((
                        x as i32 * 2 + pos.0,
                        y as i32 + pos.1
                    ));
                }
            }
        }
    }
}
