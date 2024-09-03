use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Packet {
    buffer: Vec<u8>,
}

impl From<Vec<u8>> for Packet {
    fn from(buffer: Vec<u8>) -> Self {
        Self { buffer }
    }
}

impl Packet {
    pub fn as_slice(&self) -> &[u8] {
        self.buffer.as_slice()
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Packet: [ Length: {} ]", self.buffer.len())
    }
}
