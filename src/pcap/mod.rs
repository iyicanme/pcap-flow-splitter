//! # pcap
//!
//! Provides a simple interface to read and write packet capture (pcap) files packet-by-packet
//!
//! Crate's main interfaces, `ReadOnlyCapture` and `WriteOnlyCapture` provides simple abstractions to
//! read from and write to captures.
//!
//! ## Example
//!
//! Below is an example application that reads all packets from a capture, increases their timestamps
//! by 1 second and writes them to another capture
//!
//! ```rust
//! use pcap::{ReadOnlyCapture, WriteOnlyCapture};
//!
//! # use pcap::capture_header::{CaptureHeader, LinkLayerType, MaximumPacketLength, TimestampPrecision, Version};
//! # use pcap::packet::Packet;
//! # use pcap::packet_header::{PacketHeader, Timestamp};
//! # use pcap::endianness::Endianness;
//!
//! # let mut write_capture = WriteOnlyCapture::create("example.pcap", CaptureHeader {
//! #     endianness: Endianness::Identical,
//! #     timestamp_precision: TimestampPrecision::Micro,
//! #     version: Version(2, 6),
//! #     maximum_packet_length: MaximumPacketLength(8),
//! #     frame_cyclic_sequence: None,
//! #     link_layer_type: LinkLayerType::En10Mb
//! # }).unwrap();
//!
//! # let packet_header = PacketHeader {
//! #    timestamp: Timestamp(TimestampPrecision::Micro, 123456789, 123456789),
//! #    captured_length: 8u32.into(),
//! #    actual_length: 156u32.into(),
//! # };
//!
//! # let packet: Packet = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08].into();
//!
//! # write_capture.put(packet_header, &packet).unwrap();
//!
//! let read_capture = ReadOnlyCapture::open("example.pcap").unwrap();
//! let mut write_capture = WriteOnlyCapture::create("example_modified.pcap", read_capture.header).unwrap();
//!
//! for (mut packet_header, packet) in read_capture {
//!     packet_header.timestamp.1 += 1;
//!
//!     write_capture.put(packet_header, &packet).unwrap();
//! }
//! ```
//!
use std::error::Error;
use std::path::Path;

use crate::packet::Packet;
use crate::pcap::capture_file::CaptureFile;
use crate::pcap::capture_header::CaptureHeader;
use crate::pcap::packet_header::PacketHeader;

mod capture_file;

/// Contains definitions of capture header
pub mod capture_header;
mod endian_aware_buffer;
mod error;

/// Contains definition of endianness relation
pub mod endianness;
/// Contains definitions of packet header
pub mod packet_header;

/// Represents a readable packet capture
#[derive(Debug)]
pub struct ReadOnlyCapture {
    file: CaptureFile,
    /// Capture file header
    pub header: CaptureHeader,
}

impl ReadOnlyCapture {
    /// Opens a capture in read-only mode
    ///
    /// # Errors
    /// Can fail if opening the file fails, the file is not long enough to contain capture header or parsing capture header fails
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let mut file = CaptureFile::open(path)?;
        let header_buffer = file.read(CaptureHeader::LENGTH)?;
        let header = CaptureHeader::parse(&header_buffer)?;

        let capture = Self { file, header };

        Ok(capture)
    }

    /// Reads the next packet
    ///
    /// # Errors
    /// Can fail if file is not long enough to contain packet header or the data with length
    /// described by the packet header.
    pub fn get(&mut self) -> Result<(PacketHeader, Packet), Box<dyn Error>> {
        let header_buffer = self.file.read(PacketHeader::LENGTH)?;
        let packet_header = PacketHeader::parse(
            &header_buffer,
            self.header.endianness,
            self.header.timestamp_precision,
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

/// Represents a writeable packet capture
#[derive(Debug)]
pub struct WriteOnlyCapture {
    file: CaptureFile,
    header: CaptureHeader,
}

impl WriteOnlyCapture {
    /// Opens a capture in write-only mode
    ///
    /// # Errors
    /// Can fail if opening the file, or write operation fails
    pub fn create(path: impl AsRef<Path>, header: CaptureHeader) -> Result<Self, Box<dyn Error>> {
        let mut file = CaptureFile::create(path)?;
        let header_buffer = CaptureHeader::compose(&header);
        file.write(header_buffer.as_slice())?;

        let capture = Self { file, header };

        Ok(capture)
    }

    /// Writes packet to the file
    ///
    /// # Errors
    /// Can fail if called on a capture that is opened in read-only mode (by calling open),
    /// or the write operation fails
    pub fn put(
        &mut self,
        packet_header: PacketHeader,
        packet: &Packet,
    ) -> Result<(), Box<dyn Error>> {
        let header_buffer = packet_header.compose(self.header.endianness);
        self.file.write(header_buffer.as_slice())?;
        self.file.write(packet.as_slice())?;

        Ok(())
    }
}
