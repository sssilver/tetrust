use ncurses::*;
use std::char;


pub struct Board {
    field: [[u8, ..22], ..10]
}


impl Board {
    pub fn new() -> Board {
        Board {field: [[0, ..22], ..10]}
    }

    pub fn render(&self, pos: (i32, i32)) {
        bkgd(' ' as u32 | COLOR_PAIR(1i16) as u32);

        for i in range(0i32, 22) {
            for j in range(0i32, 10) {
                move(pos.val0() + i, pos.val1() + j);
                printw("x");
            }
        }
    }
}
