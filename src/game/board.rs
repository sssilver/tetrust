use renderer::*;


pub struct Board {
    field: [[u8, ..22], ..10]
}


impl Board {
    pub fn new() -> Board {
        Board {field: [[0, ..22], ..10]}
    }

    /*
    pub fn render(&self, pos: (i32, i32)) {
        bkgd(' ' as u32 | COLOR_PAIR(1i16) as u32);
    }
    */
}


impl Renderable for Board {
    fn render(&self, pos: Coord, renderer: &Renderer) {
        for y in range(0u8, 22) {
            for x in range(0u8, 10) {
                renderer.point(
                    Coord::new(
                        (x + pos.x() as u8) as int,
                        (y + pos.y() as u8) as int
                    )
                );
            }
        }
    }
}
