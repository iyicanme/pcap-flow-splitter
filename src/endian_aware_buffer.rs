use bytes::{Buf, BufMut};

use crate::endianness::Endianness;

pub struct EndiannessAwareReadOnlyCursor<'a> {
    buffer: &'a [u8],
    endianness: Endianness,
}

impl<'a> EndiannessAwareReadOnlyCursor<'a> {
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

pub struct EndianAwareWriteOnlyCursor {
    buffer: Vec<u8>,
    endianness: Endianness,
}

impl EndianAwareWriteOnlyCursor {
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
