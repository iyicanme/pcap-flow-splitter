use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct DummyError {}

impl DummyError {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Display for DummyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DUMMY ERROR - YOU SHOULDN'T BE SEEING THIS")
    }
}

impl Error for DummyError {}
