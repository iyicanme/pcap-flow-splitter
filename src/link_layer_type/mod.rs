use std::fmt::{Display, Formatter};

/// Represents link layer type of packets in the capture.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LinkLayerType {
    /// IEEE 802.3 Ethernet
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
