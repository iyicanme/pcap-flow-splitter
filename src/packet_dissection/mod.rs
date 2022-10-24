use crate::endianness_aware_cursor::{Endianness, ReadOnlyEndiannessAwareCursor};
use crate::link_layer_type::LinkLayerType;
use crate::packet::Packet;

pub struct PacketDissection {}

impl PacketDissection {
    pub fn from_packet(packet: &Packet, endianness: Endianness, link_layer_type: LinkLayerType) {
        let cursor = ReadOnlyEndiannessAwareCursor::new(packet.as_slice(), endianness);

        match LinkLayerType {}
    }
}
