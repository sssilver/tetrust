use renderer::Renderable;
use renderer::Renderer;


type Point = (u8, u8);


pub struct Board {
    field: [[u8; 22]; 10]
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
    fn render(&self, pos: Point, renderer: &Renderer) {
        let mut color = 0;

        for y in range(0u8, 21) {
            for x in range(0u8, 12) {
                if y == 20 || x == 0 || x == 11 {  // Edge
                    color = 2;
                } else {
                    color = 1;
                }

                renderer.block(
                    (
                        (x * 2 + pos.val0() as u8) as int,
                        (y + pos.val1() as u8) as int
                    ), color
                );
            }
        }
    }
}
