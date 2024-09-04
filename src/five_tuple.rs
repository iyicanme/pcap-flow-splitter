use base64::prelude::BASE64_STANDARD;
use base64::Engine;

use crate::packet_dissection::{NetworkLayer, PacketDissection, TransportLayer};

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct FiveTuple {
    inner: Vec<u8>,
}

impl FiveTuple {
    pub fn from_packet_dissection(dissection: &PacketDissection) -> Self {
        let (mut address, is_source_lower, mut is_v6) = match dissection.network_layer {
            NetworkLayer::IPv4(source, destination, _) => {
                let is_source_lower = source < destination;

                let mut source_buffer: Vec<u8> = source.to_ne_bytes().to_vec();
                source_buffer.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
                let mut destination_buffer: Vec<u8> = destination.to_ne_bytes().to_vec();
                destination_buffer.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

                let address = if is_source_lower {
                    source_buffer.append(&mut destination_buffer);
                    source_buffer
                } else {
                    destination_buffer.append(&mut source_buffer);
                    destination_buffer
                };

                (address, is_source_lower, vec![0])
            }
            NetworkLayer::IPv6(source, destination, _) => {
                let is_source_lower = source < destination;

                let mut source_buffer: Vec<u8> = source.to_ne_bytes().to_vec();
                let mut destination_buffer: Vec<u8> = destination.to_ne_bytes().to_vec();

                let address = if is_source_lower {
                    source_buffer.append(&mut destination_buffer);
                    source_buffer
                } else {
                    destination_buffer.append(&mut source_buffer);
                    destination_buffer
                };

                (address, is_source_lower, vec![1])
            }
        };

        let (mut port, mut is_tcp) = match dissection.transport_layer {
            TransportLayer::Udp(source, destination, _) => {
                let mut source_buffer: Vec<u8> = source.to_ne_bytes().to_vec();
                let mut destination_buffer: Vec<u8> = destination.to_ne_bytes().to_vec();

                let port = if is_source_lower {
                    source_buffer.append(&mut destination_buffer);
                    source_buffer
                } else {
                    destination_buffer.append(&mut source_buffer);
                    destination_buffer
                };

                (port, vec![0])
            }
            TransportLayer::Tcp(source, destination, _) => {
                let mut source_buffer: Vec<u8> = source.to_ne_bytes().to_vec();
                let mut destination_buffer: Vec<u8> = destination.to_ne_bytes().to_vec();

                let port = if is_source_lower {
                    source_buffer.append(&mut destination_buffer);
                    source_buffer
                } else {
                    destination_buffer.append(&mut source_buffer);
                    destination_buffer
                };

                (port, vec![1])
            }
        };

        let mut five_tuple: Vec<u8> = Vec::new();
        five_tuple.append(&mut address);
        five_tuple.append(&mut port);
        five_tuple.append(&mut is_v6);
        five_tuple.append(&mut is_tcp);

        Self { inner: five_tuple }
    }

    pub fn as_base64(&self) -> String {
        BASE64_STANDARD.encode(&self.inner)
    }
}
