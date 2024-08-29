use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    OpenCaptureFile(std::io::Error),
    CaptureFileRead(std::io::Error),
    CaptureFileCreate(std::io::Error),
    CaptureFileWrite(std::io::Error),
    UnknownMagicNumber(u32),
    UnknownLinkLayerType(u32),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DUMMY ERROR - YOU SHOULDN'T BE SEEING THIS")
    }
}
