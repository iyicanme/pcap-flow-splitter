use std::error::Error;
use std::path::Path;
use crate::endianness_aware_cursor::Endianness;

use crate::packet::Packet;
use crate::pcap::capture_file::CaptureFile;
use crate::pcap::capture_header::{CaptureHeader, TimestampPrecision};
use crate::pcap::packet_header::PacketHeader;

mod capture_file;

mod capture_header;
mod endian_aware_buffer;
mod error;

mod endianness;
mod packet_header;

#[derive(Debug)]
pub struct ReadOnlyCapture {
    file: CaptureFile,
    endianness: Endianness,
    timestamp_precision: TimestampPrecision,
}

impl ReadOnlyCapture {
    pub fn open(path: impl AsRef<Path>) -> Result<(CaptureHeader, Self), Box<dyn Error>> {
        let mut file = CaptureFile::open(path)?;
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
    file: CaptureFile,
    endianness: Endianness,
}

impl WriteOnlyCapture {
    pub fn create(path: impl AsRef<Path>, header: CaptureHeader) -> Result<Self, Box<dyn Error>> {
        let mut file = CaptureFile::create(path)?;
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
