use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::error::Error;

#[derive(Debug)]
pub struct ReadFile {
    inner: File,
}

#[derive(Debug)]
pub struct WriteFile {
    inner: File,
}

impl ReadFile {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file = File::open(path).map_err(Error::OpenCaptureFile)?;

        let capture_file = Self { inner: file };

        Ok(capture_file)
    }

    pub fn read(&mut self, size: usize) -> Result<Vec<u8>, Error> {
        let mut buffer: Vec<u8> = vec![0; size];
        self.inner
            .read_exact(buffer.as_mut_slice())
            .map_err(Error::CaptureFileRead)?;

        Ok(buffer)
    }
}

impl WriteFile {
    pub fn create(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file = File::create(path).map_err(Error::CaptureFileCreate)?;

        let capture_file = Self { inner: file };

        Ok(capture_file)
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<(), Error> {
        self.inner
            .write_all(buffer)
            .map_err(Error::CaptureFileWrite)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{Read, Write};

    use rand::distr::{Alphanumeric, DistString};

    use crate::capture_file::{ReadFile, WriteFile};

    const PAYLOAD_LENGTH: usize = 32;

    #[test]
    fn open_succeeds() {
        let file_path = get_path_for_new_temp_file();

        File::create(&file_path).unwrap();
        ReadFile::open(&file_path).unwrap();
    }

    #[test]
    fn read_succeeds() {
        let file_path = get_path_for_new_temp_file();
        let mut file = File::create(&file_path).unwrap();

        let payload: [u8; PAYLOAD_LENGTH] = rand::random();
        file.write_all(&payload).unwrap();

        let mut capture_file = ReadFile::open(&file_path).unwrap();
        let buffer = capture_file.read(PAYLOAD_LENGTH).unwrap();

        assert_eq!(buffer, payload);
    }

    #[test]
    fn create_succeeds() {
        let file_path = get_path_for_new_temp_file();

        WriteFile::create(&file_path).unwrap();
        File::open(&file_path).unwrap();
    }

    #[test]
    fn write_succeeds() {
        const PAYLOAD_LENGTH: usize = 32;
        let payload: [u8; PAYLOAD_LENGTH] = rand::random();

        let file_path = get_path_for_new_temp_file();

        let mut capture_file = WriteFile::create(&file_path).unwrap();
        capture_file.write(&payload).unwrap();

        let mut file = File::open(&file_path).unwrap();
        let mut buffer: Vec<u8> = vec![0; PAYLOAD_LENGTH];
        file.read_exact(&mut buffer).unwrap();

        assert_eq!(buffer, payload);
    }

    fn get_path_for_new_temp_file() -> String {
        format!(
            "/tmp/{}",
            Alphanumeric.sample_string(&mut rand::thread_rng(), 20)
        )
    }
}
