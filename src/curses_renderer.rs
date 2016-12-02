use error::Result;
use pancurses;
use renderer::{Point, Renderer};
use std::collections::HashMap;
use subsystem::Subsystem;


pub struct CursesRenderer<'a> {
    window: &'a pancurses::Window,

    texts: HashMap<Point, String>
}


impl<'a> CursesRenderer<'a> {
    pub fn new(window: &'a pancurses::Window) -> CursesRenderer<'a> {
        CursesRenderer {
            window: window,
            texts: HashMap::new()
        }
    }

    fn render_texts(&self) {
        for (pos, text) in &self.texts {
            self.window.mvaddstr(pos.1, pos.0, &text);
        }
    }
}


impl<'a> Renderer for CursesRenderer<'a> {
    fn block(&mut self, pos: Point) {

    }

    fn text(&mut self, text: String, pos: Point) {
        self.texts.insert(pos, text);
    }
}


impl<'a> Subsystem for CursesRenderer<'a> {
    fn start(&mut self) -> Result<()> {
        self.window.nodelay(true);
        pancurses::curs_set(0);  // Hide the cursor
        pancurses::start_color();
        pancurses::use_default_colors();
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        pancurses::curs_set(1);  // Show the cursor
        self.window.nodelay(false);
        Ok(())
    }

    fn execute(&mut self) -> Result<()> {
        pancurses::init_pair(1, pancurses::COLOR_WHITE, pancurses::COLOR_BLUE);
        self.window.bkgd(pancurses::COLOR_PAIR(1));
        self.window.erase();

        // Render the texts last
        self.render_texts();

        self.window.refresh();

        Ok(())
    }
}
