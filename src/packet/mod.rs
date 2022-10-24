use std::fmt::{Display, Formatter};

/// Represents actual network traffic packet.
/// It contains the array of bytes that belong to the packet.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Packet {
    buffer: Vec<u8>,
}

impl From<Vec<u8>> for Packet {
    fn from(buffer: Vec<u8>) -> Self {
        Packet { buffer }
    }
}

impl Packet {
    /// Returns a slice to packet buffer
    pub fn as_slice(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    /// Returns a mutable slice to packet buffer
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.buffer.as_mut_slice()
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Packet: [ Length: {} ]", self.buffer.len())
    }
}
