use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::pcap::error::DummyError;

#[derive(Debug)]
pub enum CaptureFile {
    ReadOnly(File),
    WriteOnly(File),
}

impl CaptureFile {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;

        let capture_file = CaptureFile::ReadOnly(file);

        Ok(capture_file)
    }

    pub fn read(&mut self, size: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        match self {
            CaptureFile::ReadOnly(file) => {
                let mut buffer: Vec<u8> = vec![0; size];
                file.read_exact(buffer.as_mut_slice())?;

                Ok(buffer)
            }
            CaptureFile::WriteOnly(_) => Err(DummyError::new()),
        }
    }

    pub fn create(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let file = File::create(path)?;

        let capture_file = CaptureFile::WriteOnly(file);

        Ok(capture_file)
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<(), Box<dyn Error>> {
        match self {
            CaptureFile::WriteOnly(file) => {
                file.write_all(buffer)?;

                Ok(())
            }
            CaptureFile::ReadOnly(_) => Err(DummyError::new()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{Read, Write};

    use rand::distributions::{Alphanumeric, DistString};

    use claim::assert_err;

    use crate::pcap::capture_file::CaptureFile;

    const PAYLOAD_LENGTH: usize = 32;

    #[test]
    fn open_succeeds() {
        let file_path = get_path_for_new_temp_file();

        File::create(&file_path).unwrap();
        CaptureFile::open(&file_path).unwrap();
    }

    #[test]
    fn read_succeeds() {
        let file_path = get_path_for_new_temp_file();
        let mut file = File::create(&file_path).unwrap();

        let payload: [u8; PAYLOAD_LENGTH] = rand::random();
        file.write_all(&payload).unwrap();

        drop(file);

        let mut capture_file = CaptureFile::open(&file_path).unwrap();
        let buffer = capture_file.read(PAYLOAD_LENGTH).unwrap();

        assert_eq!(buffer, payload);
    }

    #[test]
    fn calling_write_on_read_only_capture_file_fails() {
        let file_path = get_path_for_new_temp_file();
        File::create(&file_path).unwrap();

        let mut capture_file = CaptureFile::open(&file_path).unwrap();

        let payload: [u8; PAYLOAD_LENGTH] = rand::random();
        let result = capture_file.write(&payload);

        assert_err!(result);
    }

    #[test]
    fn create_succeeds() {
        let file_path = get_path_for_new_temp_file();

        CaptureFile::create(&file_path).unwrap();
        File::open(&file_path).unwrap();
    }

    #[test]
    fn write_succeeds() {
        const PAYLOAD_LENGTH: usize = 32;
        let payload: [u8; PAYLOAD_LENGTH] = rand::random();

        let file_path = get_path_for_new_temp_file();

        let mut capture_file = CaptureFile::create(&file_path).unwrap();
        capture_file.write(&payload).unwrap();

        drop(capture_file);

        let mut file = File::open(&file_path).unwrap();
        let mut buffer: Vec<u8> = vec![0; PAYLOAD_LENGTH];
        file.read_exact(&mut buffer).unwrap();

        assert_eq!(buffer, payload);
    }

    #[test]
    fn calling_read_on_write_only_capture_file_fails() {
        let file_path = get_path_for_new_temp_file();
        let mut capture_file = CaptureFile::create(&file_path).unwrap();

        let result = capture_file.read(20);

        assert_err!(result);
    }

    fn get_path_for_new_temp_file() -> String {
        format!(
            "/tmp/{}",
            Alphanumeric.sample_string(&mut rand::thread_rng(), 20)
        )
    }
}
