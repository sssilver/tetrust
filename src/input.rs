use error::Result;
use subsystem::Subsystem;


#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Select,
    Pause
}


pub trait Input: Subsystem {
    fn is_pressed(&self, key: Key) -> Result<bool>;
}