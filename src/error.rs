use std::fmt::{Display, Formatter};
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    OpenCaptureFile(IoError),
    CaptureFileRead(IoError),
    CaptureFileCreate(IoError),
    CaptureFileWrite(IoError),
    UnknownMagicNumber(u32),
    UnknownLinkLayerType(u32),
    TuiSetup(IoError),
    TuiDraw(IoError),
    TuiTeardown(IoError),
    UnknownNetworkLayerType(u16),
    UnknownTransportLayerType(u8),
    UnknownIPv6AdditionalHeaderLength(u8),
    ReadEnv(IoError),
    ReadDirContent(IoError),
    TuiReadInput(IoError),
    FileTypeConversion,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DUMMY ERROR - YOU SHOULDN'T BE SEEING THIS")
    }
}
