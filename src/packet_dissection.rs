use std::net::{AddrParseError, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::ops::{BitAnd, Mul, Shr};

use crate::endianness_aware_cursor::{Endianness, ReadOnlyEndiannessAwareCursor};
use crate::error::Error;
use crate::packet::Packet;
use crate::packet_layer::{
    ApplicationLayerType, LinkLayerType, NetworkLayerType, TransportLayerType,
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PacketDissection {
    pub link_layer: LinkLayer,
    pub network_layer: NetworkLayer,
    pub transport_layer: TransportLayer,
}

impl PacketDissection {
    pub fn from_packet(
        packet: &Packet,
        endianness: Endianness,
        link_layer_type: LinkLayerType,
    ) -> Result<Self, Error> {
        let mut cursor = ReadOnlyEndiannessAwareCursor::new(packet.as_slice(), endianness);

        let link_layer = LinkLayer::parse(&mut cursor, link_layer_type)?;
        let network_layer = NetworkLayer::parse(&mut cursor, link_layer.get_network_layer_type())?;
        let transport_layer =
            TransportLayer::parse(&mut cursor, network_layer.get_transport_layer_type())?;

        let packet_dissection = Self {
            link_layer,
            network_layer,
            transport_layer,
        };

        Ok(packet_dissection)
    }

    pub fn socket_addrs(&self) -> Result<(SocketAddr, SocketAddr), AddrParseError> {
        let addrs = match (&self.network_layer, &self.transport_layer) {
            (
                NetworkLayer::IPv4(addr_a, addr_b, _),
                TransportLayer::Tcp(port_a, port_b, _) | TransportLayer::Udp(port_a, port_b, _),
            ) => (
                SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from(*addr_a), *port_a)),
                SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from(*addr_b), *port_b)),
            ),
            (
                NetworkLayer::IPv6(addr_a, addr_b, _),
                TransportLayer::Tcp(port_a, port_b, _) | TransportLayer::Udp(port_a, port_b, _),
            ) => (
                SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::from(*addr_a), *port_a, 0, 0)),
                SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::from(*addr_b), *port_b, 0, 0)),
            ),
        };

        Ok(addrs)
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LinkLayer {
    Ethernet(NetworkLayerType),
}

impl LinkLayer {
    pub fn parse(
        cursor: &mut ReadOnlyEndiannessAwareCursor,
        link_layer_type: LinkLayerType,
    ) -> Result<Self, Error> {
        let layer = match link_layer_type {
            LinkLayerType::En10Mb => {
                cursor.advance(12);

                let a = cursor.get_u16();
                let next_layer_type = match a {
                    0x0008 => NetworkLayerType::IPv4,
                    0xDD86 => NetworkLayerType::IPv6,
                    network_layer_type => {
                        return Err(Error::UnknownNetworkLayerType(network_layer_type))
                    }
                };

                Self::Ethernet(next_layer_type)
            }
        };

        Ok(layer)
    }

    pub const fn get_network_layer_type(&self) -> NetworkLayerType {
        match self {
            Self::Ethernet(next) => *next,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum NetworkLayer {
    IPv4(u32, u32, TransportLayerType),
    IPv6(u128, u128, TransportLayerType),
}

impl NetworkLayer {
    pub fn parse(
        cursor: &mut ReadOnlyEndiannessAwareCursor,
        network_layer_type: NetworkLayerType,
    ) -> Result<Self, Error> {
        let layer = match network_layer_type {
            NetworkLayerType::IPv4 => {
                let option_length: usize =
                    cursor.get_u8().bitand(0x0F).mul(4).wrapping_sub(20).into();

                cursor.advance(8);
                let protocol = match cursor.get_u8() {
                    6 => TransportLayerType::Tcp,
                    17 => TransportLayerType::Udp,
                    transport_layer_type => {
                        return Err(Error::UnknownTransportLayerType(transport_layer_type))
                    }
                };
                cursor.advance(2);

                let layer = Self::IPv4(cursor.get_u32(), cursor.get_u32(), protocol);
                cursor.advance(option_length);

                layer
            }
            NetworkLayerType::IPv6 => {
                let ipv6_option_header_ids = [0u8, 43u8, 44u8, 51u8, 60u8];

                cursor.advance(6);

                let mut next_header = cursor.get_u8();
                let mut check_next_header = ipv6_option_header_ids.contains(&next_header);

                cursor.advance(1);

                let source = cursor.get_u128();
                let destination = cursor.get_u128();

                while check_next_header {
                    let new_next_header = cursor.get_u8();
                    match next_header {
                        0 | 43 => cursor.advance(15),
                        44 => cursor.advance(7),
                        51 => {
                            let advancement = cursor.get_u8().wrapping_sub(2).into();
                            cursor.advance(advancement);
                        }
                        60 => {
                            let advancement = cursor.get_u8().wrapping_add(6).into();
                            cursor.advance(advancement);
                        }
                        next_header_length => {
                            return Err(Error::UnknownIPv6AdditionalHeaderLength(
                                next_header_length,
                            ))
                        }
                    }

                    next_header = new_next_header;
                    check_next_header = ipv6_option_header_ids.contains(&new_next_header);
                }

                let protocol = match next_header {
                    6 => TransportLayerType::Tcp,
                    17 => TransportLayerType::Udp,
                    transport_layer_type => {
                        return Err(Error::UnknownTransportLayerType(transport_layer_type))
                    }
                };

                Self::IPv6(source, destination, protocol)
            }
        };

        Ok(layer)
    }

    pub const fn get_transport_layer_type(&self) -> TransportLayerType {
        match self {
            Self::IPv4(_, _, next) | Self::IPv6(_, _, next) => *next,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TransportLayer {
    Udp(u16, u16, ApplicationLayerType),
    Tcp(u16, u16, ApplicationLayerType),
}

impl TransportLayer {
    pub fn parse(
        cursor: &mut ReadOnlyEndiannessAwareCursor,
        transport_layer_type: TransportLayerType,
    ) -> Result<Self, Error> {
        let layer = match transport_layer_type {
            TransportLayerType::Tcp => {
                let source = cursor.get_u16();
                let destination = cursor.get_u16();

                cursor.advance(8);
                let remaining_header_length: usize =
                    cursor.get_u8().shr(4u8).mul(4).wrapping_sub(13).into();
                cursor.advance(remaining_header_length);

                Self::Tcp(source, destination, ApplicationLayerType::OctetArray)
            }
            TransportLayerType::Udp => {
                let source = cursor.get_u16();
                let destination = cursor.get_u16();

                cursor.advance(4);

                Self::Udp(source, destination, ApplicationLayerType::OctetArray)
            }
        };

        Ok(layer)
    }
}
