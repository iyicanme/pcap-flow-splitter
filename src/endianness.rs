use std::fmt::{Display, Formatter};

/// Represents endianness relation between the capture file and the current machine
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Endianness {
    /// Capture has the same endianness with the current machine
    Identical,
    /// Capture has the inverse of the endianness of the current machine
    Swapped,
}

impl Display for Endianness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Endianness::Identical => "identical",
                Endianness::Swapped => "swapped",
            }
        )
    }
}
