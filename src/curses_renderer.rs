use error::Result;
use game::board::Board;
use pancurses;
use renderer::{Point, Renderer};
use std::collections::BinaryHeap;
use subsystem::Subsystem;


pub struct CursesRenderer<'a> {
    window: &'a pancurses::Window,

    texts: BinaryHeap<(Point, String)>,
    blocks: BinaryHeap<(Point, bool)>
}


impl<'a> CursesRenderer<'a> {
    pub fn new(window: &'a pancurses::Window) -> CursesRenderer<'a> {

        CursesRenderer {
            window: window,
            texts: BinaryHeap::new(),
            blocks: BinaryHeap::new()
        }
    }

    fn render_texts(&mut self) {
        while let Some((pos, text)) = self.texts.pop() {
            self.window.mvaddstr(pos.1, pos.0, &text);
        }
    }

    fn render_blocks(&mut self) {
        let saved_attr = self.window.attrget();

        self.window.attrset(pancurses::COLOR_PAIR(2));
        while let Some((pos, block)) = self.blocks.pop() {
            self.window.mvaddch(pos.1, pos.0, ' ');
        }

        self.window.attrset(saved_attr.1 as u32);
    }
}


impl<'a> Renderer for CursesRenderer<'a> {
    fn block(&mut self, pos: Point) {
        self.blocks.push((pos, true));
    }

    fn text(&mut self, pos: Point, text: String) {
        self.texts.push((pos, text));
    }

    fn board(&mut self, pos: Point, board: Board) {
        for y in 0..board.size.1 {
            for x in 0..board.size.0 {
                if y == board.size.1 - 1 || x == 0 || x == board.size.0 {  // Edge
                    self.block((
                        x as i32 * 2 + pos.0,
                        y as i32 + pos.1
                    ));
                }
            }
        }
    }
}


impl<'a> Subsystem for CursesRenderer<'a> {
    fn start(&mut self) -> Result<()> {
        self.window.nodelay(true);
        pancurses::curs_set(0);  // Hide the cursor
        pancurses::start_color();
        pancurses::use_default_colors();
        pancurses::init_pair(1, pancurses::COLOR_WHITE, pancurses::COLOR_BLUE);
        pancurses::init_pair(2, pancurses::COLOR_WHITE, pancurses::COLOR_RED);
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        pancurses::curs_set(1);  // Show the cursor
        self.window.nodelay(false);
        Ok(())
    }

    fn execute(&mut self) -> Result<()> {
        self.window.bkgd(pancurses::COLOR_PAIR(1));
        self.window.erase();
        self.window.clear();

        // Render the texts last
        self.render_texts();
        self.render_blocks();


        self.window.refresh();  // Update the screen
        Ok(())
    }
}
