use error::Result;


pub trait Subsystem {
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn execute(&mut self) -> Result<()>;
}