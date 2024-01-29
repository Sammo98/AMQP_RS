use std::{error::Error, fmt, io::Write};
#[derive(Debug)]
pub enum FlowError {
    ConnectionError,
}

pub struct ConnectionError(String);

impl fmt::Display for FlowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test")
    }
}
impl Error for FlowError {}
