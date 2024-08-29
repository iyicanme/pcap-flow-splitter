use std::collections::HashMap;

use capture::{ReadOnlyCapture, WriteOnlyCapture};

use crate::five_tuple::FiveTuple;
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

fn main() {
    let (capture_header, capture) = ReadOnlyCapture::open("./http.cap").unwrap();

    let mut out_files: HashMap<FiveTuple, WriteOnlyCapture> = HashMap::new();
    for (packet_header, packet) in capture {
        let packet_dissection = PacketDissection::from_packet(
            &packet,
            capture_header.endianness,
            capture_header.link_layer_type,
        ).unwrap();

        let five_tuple = FiveTuple::from_packet_dissection(&packet_dissection);
        if let Some(out_capture) = out_files.get_mut(&five_tuple) {
            out_capture.put(packet_header, &packet).unwrap();
        } else {
            let mut out_capture = WriteOnlyCapture::create(five_tuple.as_base64() + ".pcap", capture_header).unwrap();
            out_capture.put(packet_header, &packet).unwrap();

            out_files.insert(five_tuple, out_capture);
        }
    }
}
