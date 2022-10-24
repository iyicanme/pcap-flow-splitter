use std::fmt::{Display, Formatter};

use bytes::{Buf, BufMut};

pub struct ReadOnlyEndiannessAwareCursor<'a> {
    buffer: &'a [u8],
    endianness: Endianness,
}

impl<'a> ReadOnlyEndiannessAwareCursor<'a> {
    pub fn new(buffer: &'a [u8], endianness: Endianness) -> Self {
        Self { buffer, endianness }
    }

    pub fn set_endianness(&mut self, endianness: Endianness) {
        self.endianness = endianness;
    }

    pub fn advance(&mut self, advancement: usize) {
        self.buffer.advance(advancement);
    }

    pub fn get_u16(&mut self) -> u16 {
        match self.endianness {
            Endianness::Identical => self.buffer.get_u16(),
            Endianness::Swapped => self.buffer.get_u16_le(),
        }
    }

    pub fn get_u32(&mut self) -> u32 {
        match self.endianness {
            Endianness::Identical => self.buffer.get_u32(),
            Endianness::Swapped => self.buffer.get_u32_le(),
        }
    }
}

pub struct WriteOnlyEndiannessAwareCursor {
    buffer: Vec<u8>,
    endianness: Endianness,
}

impl WriteOnlyEndiannessAwareCursor {
    pub fn new(endianness: Endianness) -> Self {
        Self {
            buffer: Vec::new(),
            endianness,
        }
    }

    pub fn advance(&mut self, advancement: usize) {
        for _ in 0..advancement {
            self.buffer.put_u8(0);
        }
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.buffer
    }

    pub fn put_u16(&mut self, value: u16) {
        match self.endianness {
            Endianness::Identical => self.buffer.put_u16_le(value),
            Endianness::Swapped => self.buffer.put_u16(value),
        }
    }

    pub fn put_u32(&mut self, value: u32) {
        match self.endianness {
            Endianness::Identical => self.buffer.put_u32_le(value),
            Endianness::Swapped => self.buffer.put_u32(value),
        }
    }
}

/// Represents endianness relation between the capture file and the current machine
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Endianness {
    /// Capture has the same endianness with the current machine
    Identical,
    /// Capture has the inverse of the endianness of the current machine
    Swapped,
}

impl Display for Endianness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Endianness::Identical => "identical",
                Endianness::Swapped => "swapped",
            }
        )
    }
}
