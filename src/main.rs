use std::collections::HashMap;
use crate::packet_dissection::PacketDissection;
use pcap::{ReadOnlyCapture, WriteOnlyCapture};
use crate::five_tuple::FiveTuple;

mod endianness_aware_cursor;
mod packet;
mod packet_dissection;
mod packet_layer;
mod pcap;
mod five_tuple;

fn main() {
    let (capture_header, capture) = ReadOnlyCapture::open("./http.cap").unwrap();

    let mut out_files: HashMap<FiveTuple, WriteOnlyCapture> = HashMap::new();
    for (packet_header, packet) in capture {
        let packet_dissection = PacketDissection::from_packet(
            &packet,
            capture_header.endianness,
            capture_header.link_layer_type,
        ).unwrap();

        let five_tuple = FiveTuple::from_packet_dissection(packet_dissection);
        if let Some(out_capture) = out_files.get_mut(&five_tuple) {
            out_capture.put(packet_header, &packet).unwrap();
        } else {
            let mut out_capture = WriteOnlyCapture::create(five_tuple.as_base64() + ".pcap", capture_header).unwrap();
            out_capture.put(packet_header, &packet).unwrap();

            out_files.insert(five_tuple, out_capture);
        }
    }
}
