# pcap

Provides a simple interface to read and write packet capture (pcap) files packet-by-packet

Crate's main interfaces, `ReadOnlyCapture` and `WriteOnlyCapture` provides simple abstractions to
read from and write to captures.

## Example

Below is an example application that reads all packets from a capture, increases their timestamps by 1 second and writes them to another capture

 ```rust
use pcap::{ReadOnlyCapture, WriteOnlyCapture};

let read_capture = ReadOnlyCapture::open("example.pcap").unwrap();
let mut write_capture = WriteOnlyCapture::create("example_modified.pcap", read_capture.header).unwrap();

for (mut packet_header, packet) in read_capture {
    packet_header.timestamp.1 += 1;

    write_capture.put(packet_header, &packet).unwrap();
}
 ```
