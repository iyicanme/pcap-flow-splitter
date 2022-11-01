use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LinkLayerType {
    En10Mb,
}

impl Display for LinkLayerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LinkLayerType::En10Mb => "Ethernet",
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum NetworkLayerType {
    IPv4,
    IPv6,
}

impl Display for NetworkLayerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NetworkLayerType::IPv4 => "IPv4",
                NetworkLayerType::IPv6 => "IPv6",
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TransportLayerType {
    TCP,
    UDP,
}

impl Display for TransportLayerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TransportLayerType::UDP => "UDP",
                TransportLayerType::TCP => "TCP",
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ApplicationLayerType {
    OctetArray,
}

impl Display for ApplicationLayerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ApplicationLayerType::OctetArray => "Octet Stream",
            }
        )
    }
}
