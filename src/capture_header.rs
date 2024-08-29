use std::fmt::{Display, Formatter};
use std::ops::{BitAnd, Shl};

use crate::endianness_aware_cursor::{
    Endianness, ReadOnlyEndiannessAwareCursor, WriteOnlyEndiannessAwareCursor,
};
use crate::error::Error;
use crate::packet_layer::LinkLayerType;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CaptureHeader {
    pub endianness: Endianness,
    pub timestamp_precision: TimestampPrecision,
    pub version: Version,
    pub maximum_packet_length: MaximumPacketLength,
    pub frame_cyclic_sequence: Option<FrameCyclicSequence>,
    pub link_layer_type: LinkLayerType,
}

impl CaptureHeader {
    pub(crate) const LENGTH: usize = 24;

    const MAGIC_NUMBER_IDENTICAL_MICRO: u32 = 0xA1_B2_C3_D4;
    const MAGIC_NUMBER_SWAPPED_MICRO: u32 = 0xD4_C3_B2_A1;
    const MAGIC_NUMBER_IDENTICAL_NANO: u32 = 0xA1_B2_3C_4D;
    const MAGIC_NUMBER_SWAPPED_NANO: u32 = 0x4D_3C_B2_A1;

    const LINK_LAYER_TYPE_MASK: u32 = 0x0F_FF_FF_FF;
    const FRAME_CYCLIC_SEQUENCE_FLAG_MASK: u32 = 0x10_00_00_00;
    const FRAME_CYCLIC_SEQUENCE_MASK: u32 = 0xE0_00_00_00;

    pub fn parse(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = ReadOnlyEndiannessAwareCursor::new(buffer, Endianness::Identical);

        let (endianness, timestamp_precision) = match cursor.get_u32() {
            Self::MAGIC_NUMBER_IDENTICAL_MICRO => {
                (Endianness::Identical, TimestampPrecision::Micro)
            }
            Self::MAGIC_NUMBER_SWAPPED_MICRO => {
                (Endianness::Swapped, TimestampPrecision::Micro)
            }
            Self::MAGIC_NUMBER_IDENTICAL_NANO => {
                (Endianness::Identical, TimestampPrecision::Nano)
            }
            Self::MAGIC_NUMBER_SWAPPED_NANO => {
                (Endianness::Swapped, TimestampPrecision::Nano)
            }
            magic_number => return Err(Error::UnknownMagicNumber(magic_number)),
        };

        cursor.set_endianness(endianness);

        let version = Version(cursor.get_u16(), cursor.get_u16());
        cursor.advance(4 + 4);
        let maximum_packet_length = MaximumPacketLength(cursor.get_u32());

        let fcs_link_layer_type = cursor.get_u32();
        let (frame_cyclic_sequence, link_layer_type) = {
            let link_layer_type = match fcs_link_layer_type & Self::LINK_LAYER_TYPE_MASK {
                1 => LinkLayerType::En10Mb,
                link_layer_type => return Err(Error::UnknownLinkLayerType(link_layer_type)),
            };

            let frame_cyclic_sequence =
                if (fcs_link_layer_type & Self::FRAME_CYCLIC_SEQUENCE_FLAG_MASK) != 0 {
                    let frame_cyclic_sequence: FrameCyclicSequence = FrameCyclicSequence(
                        fcs_link_layer_type
                            .bitand(Self::FRAME_CYCLIC_SEQUENCE_MASK)
                            .overflowing_shr(29)
                            .0 as u8,
                    );

                    Some(frame_cyclic_sequence)
                } else {
                    None
                };

            (frame_cyclic_sequence, link_layer_type)
        };

        let capture_header = Self {
            endianness,
            timestamp_precision,
            version,
            maximum_packet_length,
            frame_cyclic_sequence,
            link_layer_type,
        };

        Ok(capture_header)
    }

    pub fn compose(&self) -> Vec<u8> {
        let mut cursor = WriteOnlyEndiannessAwareCursor::new(self.endianness);

        match self.timestamp_precision {
            TimestampPrecision::Micro => {
                cursor.put_u32(Self::MAGIC_NUMBER_IDENTICAL_MICRO);
            }
            TimestampPrecision::Nano => cursor.put_u32(Self::MAGIC_NUMBER_IDENTICAL_NANO),
        }

        cursor.put_u16(self.version.0);
        cursor.put_u16(self.version.1);

        cursor.advance(4 + 4);

        cursor.put_u32(self.maximum_packet_length.0);

        let link_layer_type = match self.link_layer_type {
            LinkLayerType::En10Mb => 1,
        };
        let frame_cyclic_sequence = self.frame_cyclic_sequence
            .map_or(
                0,
                |sequence| (sequence.0 as u32).shl(29) | 1u32.shl(28),
            );

        cursor.put_u32(link_layer_type | frame_cyclic_sequence);

        cursor.into_vec()
    }
}

impl Display for CaptureHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CaptureHeader [ Endianness: {}, Timestamp Precision: {} ]",
            self.endianness, self.timestamp_precision
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TimestampPrecision {
    Micro,
    Nano,
}

impl Display for TimestampPrecision {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Micro => "microsecond",
                Self::Nano => "nanosecond",
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Version(pub u16, pub u16);

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MaximumPacketLength(pub u32);

impl Display for MaximumPacketLength {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FrameCyclicSequence(pub u8);

impl Display for FrameCyclicSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use claim::assert_err;

    use crate::capture_header::{
        CaptureHeader, Endianness, FrameCyclicSequence, LinkLayerType, MaximumPacketLength,
        TimestampPrecision, Version,
    };

    #[test]
    fn parsing_capture_header_with_swapped_endianness_microsecond_timestamp_magic_number_succeeds()
    {
        const PCAP_BYTE_ARRAY: [u8; CaptureHeader::LENGTH] = [
            0xd4, 0xc3, 0xb2, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        ];

        let header = CaptureHeader::parse(&PCAP_BYTE_ARRAY).unwrap();

        assert_eq!(header.endianness, Endianness::Swapped);
        assert_eq!(header.timestamp_precision, TimestampPrecision::Micro);
    }

    #[test]
    fn parsing_capture_header_with_identical_endianness_microsecond_timestamp_magic_number_succeeds(
    ) {
        const PCAP_BYTE_ARRAY: [u8; CaptureHeader::LENGTH] = [
            0xa1, 0xb2, 0xc3, 0xd4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        ];

        let header = CaptureHeader::parse(&PCAP_BYTE_ARRAY).unwrap();

        assert_eq!(header.endianness, Endianness::Identical);
        assert_eq!(header.timestamp_precision, TimestampPrecision::Micro);
    }

    #[test]
    fn parsing_capture_header_with_swapped_endianness_nanosecond_timestamp_magic_number_succeeds() {
        const PCAP_BYTE_ARRAY: [u8; CaptureHeader::LENGTH] = [
            0x4d, 0x3c, 0xb2, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        ];

        let header = CaptureHeader::parse(&PCAP_BYTE_ARRAY).unwrap();

        assert_eq!(header.endianness, Endianness::Swapped);
        assert_eq!(header.timestamp_precision, TimestampPrecision::Nano);
    }

    #[test]
    fn parsing_capture_header_with_identical_endianness_nanosecond_timestamp_magic_number_succeeds()
    {
        const PCAP_BYTE_ARRAY: [u8; CaptureHeader::LENGTH] = [
            0xa1, 0xb2, 0x3c, 0x4d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        ];

        let header = CaptureHeader::parse(&PCAP_BYTE_ARRAY).unwrap();

        assert_eq!(header.endianness, Endianness::Identical);
        assert_eq!(header.timestamp_precision, TimestampPrecision::Nano);
    }

    #[test]
    fn parsing_capture_header_with_garbage_magic_number_fails() {
        const PCAP_BYTE_ARRAY: [u8; CaptureHeader::LENGTH] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        ];

        let result = CaptureHeader::parse(&PCAP_BYTE_ARRAY);

        assert_err!(result);
    }

    #[test]
    fn parsing_version_succeeds() {
        const PCAP_BYTE_ARRAY: [u8; CaptureHeader::LENGTH] = [
            0xd4, 0xc3, 0xb2, 0xa1, 0x02, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        ];

        let header = CaptureHeader::parse(&PCAP_BYTE_ARRAY).unwrap();

        assert_eq!(header.version, Version(2, 4));
    }

    #[test]
    fn parsing_maximum_packet_length_succeeds() {
        const PCAP_BYTE_ARRAY: [u8; CaptureHeader::LENGTH] = [
            0xd4, 0xc3, 0xb2, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00,
        ];

        let header = CaptureHeader::parse(&PCAP_BYTE_ARRAY).unwrap();

        assert_eq!(header.maximum_packet_length, MaximumPacketLength(262_144));
    }

    #[test]
    fn parsing_frame_cyclic_sequence_succeeds() {
        const PCAP_BYTE_ARRAY: [u8; CaptureHeader::LENGTH] = [
            0xd4, 0xc3, 0xb2, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0xF0,
        ];

        let header = CaptureHeader::parse(&PCAP_BYTE_ARRAY).unwrap();

        assert_eq!(header.frame_cyclic_sequence, Some(FrameCyclicSequence(7)));
    }

    #[test]
    fn parsing_link_layer_type_succeeds() {
        const PCAP_BYTE_ARRAY: [u8; CaptureHeader::LENGTH] = [
            0xd4, 0xc3, 0xb2, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0xF0,
        ];

        let header = CaptureHeader::parse(&PCAP_BYTE_ARRAY).unwrap();

        assert_eq!(header.link_layer_type, LinkLayerType::En10Mb);
    }

    #[test]
    fn parsing_capture_header_with_garbage_link_layer_type_fails() {
        const PCAP_BYTE_ARRAY: [u8; CaptureHeader::LENGTH] = [
            0xd4, 0xc3, 0xb2, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0,
        ];

        let result = CaptureHeader::parse(&PCAP_BYTE_ARRAY);

        assert_err!(result);
    }

    #[test]
    fn composing_capture_header_with_identical_endianness_and_microsecond_timestamp_precision_succeeds(
    ) {
        let header = CaptureHeader {
            endianness: Endianness::Identical,
            timestamp_precision: TimestampPrecision::Micro,
            version: Version(0, 0),
            maximum_packet_length: MaximumPacketLength(0),
            frame_cyclic_sequence: None,
            link_layer_type: LinkLayerType::En10Mb,
        };

        let buffer = header.compose();

        assert_eq!(
            buffer.as_slice().split_at(4).0,
            CaptureHeader::MAGIC_NUMBER_IDENTICAL_MICRO.to_le_bytes(),
        );
    }

    #[test]
    fn composing_capture_header_with_swapped_endianness_and_microsecond_timestamp_precision_succeeds(
    ) {
        let header = CaptureHeader {
            endianness: Endianness::Swapped,
            timestamp_precision: TimestampPrecision::Micro,
            version: Version(0, 0),
            maximum_packet_length: MaximumPacketLength(0),
            frame_cyclic_sequence: None,
            link_layer_type: LinkLayerType::En10Mb,
        };

        let buffer = header.compose();

        assert_eq!(
            buffer.as_slice().split_at(4).0,
            CaptureHeader::MAGIC_NUMBER_SWAPPED_MICRO.to_le_bytes(),
        );
    }

    #[test]
    fn composing_capture_header_with_identical_endianness_and_nanosecond_timestamp_precision_succeeds(
    ) {
        let header = CaptureHeader {
            endianness: Endianness::Identical,
            timestamp_precision: TimestampPrecision::Nano,
            version: Version(0, 0),
            maximum_packet_length: MaximumPacketLength(0),
            frame_cyclic_sequence: None,
            link_layer_type: LinkLayerType::En10Mb,
        };

        let buffer = header.compose();

        assert_eq!(
            buffer.as_slice().split_at(4).0,
            CaptureHeader::MAGIC_NUMBER_IDENTICAL_NANO.to_le_bytes(),
        );
    }

    #[test]
    fn composing_capture_header_with_swapped_endianness_and_nanosecond_timestamp_precision_succeeds(
    ) {
        let header = CaptureHeader {
            endianness: Endianness::Swapped,
            timestamp_precision: TimestampPrecision::Nano,
            version: Version(0, 0),
            maximum_packet_length: MaximumPacketLength(0),
            frame_cyclic_sequence: None,
            link_layer_type: LinkLayerType::En10Mb,
        };

        let buffer = header.compose();

        assert_eq!(
            buffer.as_slice().split_at(4).0,
            CaptureHeader::MAGIC_NUMBER_SWAPPED_NANO.to_le_bytes(),
        );
    }

    #[test]
    fn composing_version_succeeds() {
        let header = CaptureHeader {
            endianness: Endianness::Swapped,
            timestamp_precision: TimestampPrecision::Nano,
            version: Version(2, 6),
            maximum_packet_length: MaximumPacketLength(0),
            frame_cyclic_sequence: None,
            link_layer_type: LinkLayerType::En10Mb,
        };

        let buffer = header.compose();

        assert_eq!(
            buffer.as_slice().split_at(4).1.split_at(4).0,
            [0x00, 0x02, 0x00, 0x06]
        );
    }

    #[test]
    fn composing_maximum_packet_length_succeeds() {
        let header = CaptureHeader {
            endianness: Endianness::Swapped,
            timestamp_precision: TimestampPrecision::Nano,
            version: Version(0, 0),
            maximum_packet_length: MaximumPacketLength(262_144),
            frame_cyclic_sequence: None,
            link_layer_type: LinkLayerType::En10Mb,
        };

        let buffer = header.compose();

        assert_eq!(
            buffer.as_slice().split_at(16).1.split_at(4).0,
            [0x00, 0x04, 0x00, 0x00]
        );
    }

    #[test]
    fn composing_capture_header_with_frame_cyclic_sequence_succeeds() {
        let header = CaptureHeader {
            endianness: Endianness::Swapped,
            timestamp_precision: TimestampPrecision::Nano,
            version: Version(0, 0),
            maximum_packet_length: MaximumPacketLength(0),
            frame_cyclic_sequence: Some(FrameCyclicSequence(7)),
            link_layer_type: LinkLayerType::En10Mb,
        };

        let buffer = header.compose();

        assert_eq!(buffer.as_slice().split_at(20).1, [0xF0, 0x00, 0x00, 0x01]);
    }

    #[test]
    fn composing_capture_header_without_frame_cyclic_sequence_succeeds() {
        let header = CaptureHeader {
            endianness: Endianness::Swapped,
            timestamp_precision: TimestampPrecision::Nano,
            version: Version(0, 0),
            maximum_packet_length: MaximumPacketLength(0),
            frame_cyclic_sequence: None,
            link_layer_type: LinkLayerType::En10Mb,
        };

        let buffer = header.compose();

        assert_eq!(buffer.as_slice().split_at(20).1, [0x00, 0x00, 0x00, 0x01]);
    }
}
