use std::fmt::{Display, Formatter};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::packet_dissection::{NetworkLayer, PacketDissection, TransportLayer};

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct FiveTuple {
    lower_addr: IpAddr,
    lower_port: u16,
    higher_addr: IpAddr,
    higher_port: u16,
    is_tcp: bool,
}

impl FiveTuple {
    pub fn from_packet_dissection(dissection: &PacketDissection) -> Self {
        let (is_source_lower, lower_addr, higher_addr) = match dissection.network_layer {
            NetworkLayer::IPv4(source, destination, _) => {
                let (lower_addr, higher_addr) = if source < destination {
                    (
                        IpAddr::V4(Ipv4Addr::from(source)),
                        IpAddr::V4(Ipv4Addr::from(destination)),
                    )
                } else {
                    (
                        IpAddr::V4(Ipv4Addr::from(destination)),
                        IpAddr::V4(Ipv4Addr::from(source)),
                    )
                };

                (source < destination, lower_addr, higher_addr)
            }
            NetworkLayer::IPv6(source, destination, _) => {
                let (lower_addr, higher_addr) = if source < destination {
                    (
                        IpAddr::V6(Ipv6Addr::from(source)),
                        IpAddr::V6(Ipv6Addr::from(destination)),
                    )
                } else {
                    (
                        IpAddr::V6(Ipv6Addr::from(destination)),
                        IpAddr::V6(Ipv6Addr::from(source)),
                    )
                };

                (source < destination, lower_addr, higher_addr)
            }
        };

        let (is_tcp, lower_port, higher_port) = match (is_source_lower, &dissection.transport_layer)
        {
            (true, TransportLayer::Udp(source, destination, _)) => (false, source, destination),
            (false, TransportLayer::Udp(source, destination, _)) => (false, destination, source),
            (true, TransportLayer::Tcp(source, destination, _)) => (true, source, destination),
            (false, TransportLayer::Tcp(source, destination, _)) => (true, destination, source),
        };

        FiveTuple {
            lower_addr,
            lower_port: *lower_port,
            higher_addr,
            higher_port: *higher_port,
            is_tcp,
        }
    }
}

impl Display for FiveTuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let protocol = if self.is_tcp { "TCP" } else { "UDP" };
        write!(
            f,
            "[{protocol}] {}:{} ↔ {}:{}",
            self.lower_addr, self.lower_port, self.higher_addr, self.higher_port
        )
    }
}
