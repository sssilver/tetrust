use error::Result;
use input::{Input, Key};
use pancurses;
use std::collections::HashSet;
use subsystem::Subsystem;


pub struct CursesInput<'a> {
    window: &'a pancurses::Window,
    pressed_keys: HashSet<Key>
}


impl<'a> CursesInput<'a> {
    pub fn new(window: &'a pancurses::Window) -> CursesInput<'a> {
        CursesInput {
            window: window,
            pressed_keys: HashSet::new()
        }
    }
}

impl<'a> Input for CursesInput<'a> {
    fn is_pressed(&self, key: Key) -> Result<bool> {
        Ok(self.pressed_keys.contains(&key))
    }
}


impl<'a> Subsystem for CursesInput<'a> {
    fn start(&mut self) -> Result<()> {
        self.window.keypad(true);
        pancurses::noecho();

        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        self.window.keypad(false);
        Ok(())
    }

    fn execute(&mut self) -> Result<()> {
        self.pressed_keys.clear();

        match self.window.getch() {
            Some(pancurses::Input::KeyLeft) => { self.pressed_keys.insert(Key::Left); },
            Some(pancurses::Input::KeyRight) => { self.pressed_keys.insert(Key::Right); },
            Some(pancurses::Input::KeyUp) => { self.pressed_keys.insert(Key::Up); },
            Some(pancurses::Input::KeyDown) => { self.pressed_keys.insert(Key::Down); },
            Some(pancurses::Input::Character(' ')) => { self.pressed_keys.insert(Key::Select); },
            Some(pancurses::Input::Character('$')) => { self.pressed_keys.insert(Key::Pause); },
            Some(input) => { },
            None => ()
        }

        Ok(())
    }
}
