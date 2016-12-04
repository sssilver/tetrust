use error::Result;
use std::collections::HashSet;
use subsystem::Subsystem;


#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Action {
    Left,
    Right,
    Up,
    Down,
    Select,
    Pause
}


pub trait Input: Subsystem {
    fn actions(&self) -> HashSet<Action>;
}