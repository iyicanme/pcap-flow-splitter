use pcap::{ReadOnlyCapture, WriteOnlyCapture};

pub mod endianness_aware_cursor;
pub mod link_layer_type;
pub mod packet;
pub mod packet_dissection;
pub mod pcap;

fn main() {
    let capture = ReadOnlyCapture::open("").unwrap();
    let capture_header = capture.header;

    for (packet_header, packet) in capture {
        let mut destination_capture = WriteOnlyCapture::create("", capture_header).unwrap();

        destination_capture.put(packet_header, &packet).unwrap();
    }
}
