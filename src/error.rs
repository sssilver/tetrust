use std::result;


pub type Result<T> = result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
    SubsystemStartupFailure
}
