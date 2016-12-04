use error::Result;
use input::{Action, Input};
use pancurses;
use std::collections::HashSet;
use subsystem::Subsystem;


pub struct CursesInput<'a> {
    window: &'a pancurses::Window,
    performed_actions: HashSet<Action>
}


impl<'a> CursesInput<'a> {
    pub fn new(window: &'a pancurses::Window) -> CursesInput<'a> {
        CursesInput {
            window: window,
            performed_actions: HashSet::new()
        }
    }
}

impl<'a> Input for CursesInput<'a> {
    fn actions(&self) -> HashSet<Action> {
        self.performed_actions.clone()
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
        self.performed_actions.clear();

        match self.window.getch() {
            Some(pancurses::Input::KeyLeft) => { self.performed_actions.insert(Action::Left); },
            Some(pancurses::Input::KeyRight) => { self.performed_actions.insert(Action::Right); },
            Some(pancurses::Input::KeyUp) => { self.performed_actions.insert(Action::Up); },
            Some(pancurses::Input::KeyDown) => { self.performed_actions.insert(Action::Down); },
            Some(pancurses::Input::Character(' ')) => { self.performed_actions.insert(Action::Select); },
            Some(pancurses::Input::Character('$')) => { self.performed_actions.insert(Action::Pause); },
            Some(pancurses::Input::Character('q')) => { self.performed_actions.insert(Action::Pause); },
            Some(pancurses::Input::Character('Q')) => { self.performed_actions.insert(Action::Pause); },
            Some(_) => { },
            None => ()
        }

        Ok(())
    }
}
