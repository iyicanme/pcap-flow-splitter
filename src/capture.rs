use std::error::Error;
use std::path::Path;

use crate::capture_file::{ReadFile, WriteFile};
use crate::capture_header::{CaptureHeader, TimestampPrecision};
use crate::endianness_aware_cursor::Endianness;
use crate::packet::Packet;
use crate::packet_header::PacketHeader;

#[derive(Debug)]
pub struct ReadOnlyCapture {
    file: ReadFile,
    endianness: Endianness,
    timestamp_precision: TimestampPrecision,
}

impl ReadOnlyCapture {
    pub fn open(path: impl AsRef<Path>) -> Result<(CaptureHeader, Self), Box<dyn Error>> {
        let mut file = ReadFile::open(path)?;
        let header_buffer = file.read(CaptureHeader::LENGTH)?;
        let header = CaptureHeader::parse(&header_buffer)?;

        let capture = Self { file, endianness: header.endianness, timestamp_precision: header.timestamp_precision };

        Ok((header, capture))
    }

    pub fn get(&mut self) -> Result<(PacketHeader, Packet), Box<dyn Error>> {
        let header_buffer = self.file.read(PacketHeader::LENGTH)?;
        let packet_header = PacketHeader::parse(
            &header_buffer,
            self.endianness,
            self.timestamp_precision,
        );

        let packet_length = packet_header.captured_length;
        let packet_buffer = self.file.read(packet_length.into())?;
        let packet: Packet = packet_buffer.into();

        Ok((packet_header, packet))
    }
}

impl Iterator for ReadOnlyCapture {
    type Item = (PacketHeader, Packet);

    fn next(&mut self) -> Option<Self::Item> {
        self.get().ok()
    }
}

#[derive(Debug)]
pub struct WriteOnlyCapture {
    file: WriteFile,
    endianness: Endianness,
}

impl WriteOnlyCapture {
    pub fn create(path: impl AsRef<Path>, header: CaptureHeader) -> Result<Self, Box<dyn Error>> {
        let mut file = WriteFile::create(path)?;
        let header_buffer = CaptureHeader::compose(&header);
        file.write(header_buffer.as_slice())?;

        let capture = Self { file, endianness: header.endianness };

        Ok(capture)
    }

    pub fn put(
        &mut self,
        packet_header: PacketHeader,
        packet: &Packet,
    ) -> Result<(), Box<dyn Error>> {
        let header_buffer = packet_header.compose(self.endianness);
        self.file.write(header_buffer.as_slice())?;
        self.file.write(packet.as_slice())?;

        Ok(())
    }
}
