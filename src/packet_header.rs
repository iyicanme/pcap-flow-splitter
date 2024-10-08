use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};

use crate::capture_header::TimestampPrecision;
use crate::endianness_aware_cursor::{
    Endianness, ReadOnlyEndiannessAwareCursor, WriteOnlyEndiannessAwareCursor,
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PacketHeader {
    pub timestamp: Timestamp,
    pub captured_length: PacketLength,
    pub actual_length: PacketLength,
}

impl PacketHeader {
    pub const LENGTH: usize = 16;

    pub fn parse(
        buffer: &[u8],
        endianness: Endianness,
        timestamp_precision: TimestampPrecision,
    ) -> Self {
        let mut cursor = ReadOnlyEndiannessAwareCursor::new(buffer, endianness);

        let timestamp = Timestamp(timestamp_precision, cursor.get_u32(), cursor.get_u32());
        let captured_length = PacketLength(cursor.get_u32());
        let actual_length = PacketLength(cursor.get_u32());

        Self {
            timestamp,
            captured_length,
            actual_length,
        }
    }

    pub fn compose(&self, endianness: Endianness) -> Vec<u8> {
        let mut cursor = WriteOnlyEndiannessAwareCursor::new(endianness);

        cursor.put_u32(self.timestamp.1);
        cursor.put_u32(self.timestamp.2);
        cursor.put_u32(self.captured_length.0);
        cursor.put_u32(self.actual_length.0);

        cursor.into_vec()
    }
}

impl Display for PacketHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PacketHeader: [ Timestamp: {}, Length {}]",
            self.timestamp, self.captured_length
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Timestamp(pub TimestampPrecision, pub u32, pub u32);

impl Timestamp {
    pub fn nanos(&self) -> u64 {
        u64::from(self.1)
            .mul(match self.0 {
                TimestampPrecision::Micro => 1_000_000u64,
                TimestampPrecision::Nano => 1_000_000_000u64,
            })
            .add(u64::from(self.2))
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            TimestampPrecision::Micro => write!(f, "{}.{:0>6}", self.1, self.2),
            TimestampPrecision::Nano => write!(f, "{}.{:0>9}", self.1, self.2),
        }
    }
}

/// Length of the packet
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PacketLength(pub u32);

impl PacketLength {
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl Display for PacketLength {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<PacketLength> for usize {
    fn from(packet_length: PacketLength) -> Self {
        packet_length.0 as Self
    }
}

impl From<u32> for PacketLength {
    fn from(packet_length: u32) -> Self {
        Self(packet_length)
    }
}

#[cfg(test)]
mod tests {
    use crate::capture_header::TimestampPrecision;
    use crate::endianness_aware_cursor::Endianness;
    use crate::packet_header::{PacketHeader, PacketLength, Timestamp};

    #[test]
    fn parsing_packet_header_succeeds() {
        const PCAP_BYTE_ARRAY: [u8; PacketHeader::LENGTH] = [
            0xd3, 0xf6, 0xeb, 0x5c, 0x64, 0x10, 0x01, 0x00, 0x34, 0x00, 0x00, 0x00, 0x34, 0x00,
            0x00, 0x00,
        ];

        let packet_header = PacketHeader::parse(
            &PCAP_BYTE_ARRAY,
            Endianness::Swapped,
            TimestampPrecision::Micro,
        );

        assert_eq!(
            packet_header.timestamp,
            Timestamp(TimestampPrecision::Micro, 1_558_968_019, 69_732)
        );
        assert_eq!(packet_header.actual_length, PacketLength(52));
        assert_eq!(packet_header.captured_length, PacketLength(52));
    }

    #[test]
    fn composing_packet_header_succeeds() {
        let header = PacketHeader {
            timestamp: Timestamp(TimestampPrecision::Micro, 12345, 67890),
            captured_length: PacketLength(262_144),
            actual_length: PacketLength(262_144),
        };

        let buffer = header.compose(Endianness::Identical);

        assert_eq!(buffer.split_at(4).0, 12345u32.to_le_bytes());
        assert_eq!(buffer.split_at(4).1.split_at(4).0, 67890u32.to_le_bytes());
        assert_eq!(buffer.split_at(8).1.split_at(4).0, 262_144u32.to_le_bytes());
        assert_eq!(buffer.split_at(12).1, 262_144u32.to_le_bytes());
    }
}
