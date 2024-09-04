use std::collections::HashMap;
use std::ffi::OsString;
use std::path::Path;

use clap::Parser;

use crate::error::Error;
use capture::{ReadOnlyCapture, WriteOnlyCapture};

use crate::five_tuple::FiveTuple;
use crate::packet::Packet;
use crate::packet_dissection::PacketDissection;

mod capture;
mod capture_file;
mod capture_header;
mod endianness_aware_cursor;
mod error;
mod five_tuple;
mod packet;
mod packet_dissection;
mod packet_header;
mod packet_layer;
mod ui;

fn main() {
    let args = Args::parse();

    match args.file_path {
        Some(file_path) => oneshot(file_path).unwrap(),
        None => ui::run().unwrap(),
    }
}

fn oneshot(file_path: OsString) -> Result<(), Error> {
    let (capture_header, capture) = ReadOnlyCapture::open(file_path)?;

    let mut out_files: HashMap<FiveTuple, WriteOnlyCapture> = HashMap::new();
    for (packet_header, packet) in capture {
        let packet_dissection = PacketDissection::from_packet(
            &packet,
            capture_header.endianness,
            capture_header.link_layer_type,
        )?;

        let five_tuple = FiveTuple::from_packet_dissection(&packet_dissection);
        if let Some(out_capture) = out_files.get_mut(&five_tuple) {
            out_capture.put(packet_header, &packet)?;
        } else {
            let mut out_capture =
                WriteOnlyCapture::create(five_tuple.as_base64() + ".pcap", capture_header)?;
            out_capture.put(packet_header, &packet)?;

            out_files.insert(five_tuple, out_capture);
        }
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = None)]
    file_path: Option<OsString>,
}
