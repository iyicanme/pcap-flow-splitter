use std::fmt::{Display, Formatter};
use std::ops::{Div, Rem};

pub struct TimestampDisplay(pub u64);

impl TimestampDisplay {
    const NANOS_DIVISOR: u64 = 1_000_000_000;
}

impl Display for TimestampDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{:0>9}",
            self.0.div(Self::NANOS_DIVISOR),
            self.0.rem(Self::NANOS_DIVISOR)
        )
    }
}
