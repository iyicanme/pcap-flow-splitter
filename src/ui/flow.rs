use std::collections::hash_map::{Entry, Keys};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::slice::Iter;

use ratatui::widgets::Row;

use crate::capture::ReadOnlyCapture;
use crate::error::Error;
use crate::five_tuple::FiveTuple;
use crate::packet_dissection::PacketDissection;
use crate::packet_header::PacketHeader;
use crate::packet_layer::TransportLayerType;

pub fn extract_flows(file_path: impl AsRef<Path>) -> Result<Flows, Error> {
    let mut packets: HashMap<FiveTuple, Vec<(PacketHeader, PacketDissection)>> = HashMap::new();

    let (capture_header, capture) = ReadOnlyCapture::open(file_path)?;

    for (packet_header, packet) in capture {
        let dissection = PacketDissection::from_packet(&packet, capture_header.endianness, capture_header.link_layer_type)?;

        let five_tuple = FiveTuple::from_packet_dissection(&dissection);
        match packets.entry(five_tuple) {
            Entry::Occupied(mut o) => { o.get_mut().push((packet_header, dissection)); }
            Entry::Vacant(v) => { v.insert(vec![(packet_header, dissection)]); }
        }
    }

    for (_, flow) in &mut packets {
        flow.sort_by(|(header, _), (other, _) | header.timestamp.cmp(&other.timestamp))
    }

    let mut flows: HashMap<FiveTuple, Flow> = HashMap::new();
    for (five_tuple, vec) in &packets {
        for (header, dissection) in vec {
            match flows.entry(five_tuple.to_owned()) {
                Entry::Occupied(mut o) => { o.get_mut().insert_packet(dissection, header); }
                Entry::Vacant(v) => { v.insert(Flow::new(dissection, header)); }
            }
        }

        for flow in flows.values_mut() {
            flow.average_size = flow.total_size / flow.packet_count;
            flow.flow_duration = flow.previous_timestamp;
            flow.average_interarrival_time = flow.flow_duration / (flow.packet_count as u64 - 1);
        }
    }


    Ok(Flows { inner: flows })
}

pub struct Flows {
    inner: HashMap<FiveTuple, Flow>,
}

impl<'a> Flows {
    pub fn iter(&self, index: usize) -> PacketIterator {
        PacketIterator { packets: self.inner.values().nth(index).expect("we ensure index is within 0..flows.len()").packets.iter(), index: 0 }
    }

    pub fn keys(&self) -> NameIterator {
        NameIterator { names: self.inner.keys() }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

pub struct Flow {
    initiator: SocketAddr,
    respondent: SocketAddr,
    protocol: TransportLayerType,
    packet_count: usize,
    total_size: usize,
    average_size: usize,
    minimum_size: usize,
    maximum_size: usize,
    flow_duration: u64,
    average_interarrival_time: u64,
    minimum_interarrival_time: u64,
    maximum_interarrival_time: u64,
    packets: Vec<FlowPacket>,
    flow_start: u64,
    previous_timestamp: u64,
}

impl Flow {
    pub fn new(dissection: &PacketDissection, header: &PacketHeader) -> Self {
        let (initiator, respondent) = dissection.socket_addrs().unwrap();
        let protocol = dissection.network_layer.get_transport_layer_type();

        let size = header.actual_length.as_usize();
        let timestamp = header.timestamp.nanos();

        let flow_packet = FlowPacket { from_initiator_to_respondent: true, timestamp: 0u64, size };

        Flow {
            initiator,
            respondent,
            protocol,
            packet_count: 1,
            total_size: size,
            average_size: 0,
            minimum_size: size,
            maximum_size: size,
            flow_duration: 0,
            average_interarrival_time: 0,
            minimum_interarrival_time: u64::MAX,
            maximum_interarrival_time: 0,
            packets: vec![flow_packet],
            flow_start: timestamp,
            previous_timestamp: 0,
        }
    }

    pub fn insert_packet(&mut self, dissection: &PacketDissection, header: &PacketHeader) {
        let (initiator, respondent) = dissection.socket_addrs().unwrap();
        let timestamp = header.timestamp.nanos() - self.flow_start;

        self.packet_count += 1;

        let size = header.actual_length.as_usize();
        self.total_size += size;
        self.maximum_size = self.maximum_size.max(size);
        self.minimum_size = self.minimum_size.min(size);

        let interarrival_time = timestamp - self.previous_timestamp;
        self.maximum_interarrival_time = self.maximum_interarrival_time.max(interarrival_time);
        self.minimum_interarrival_time = self.minimum_interarrival_time.min(interarrival_time);
        self.previous_timestamp = timestamp;

        let packet = FlowPacket {
            from_initiator_to_respondent: initiator == self.initiator && respondent == self.respondent,
            timestamp,
            size: header.actual_length.as_usize(),
        };

        self.packets.push(packet);
    }
}

#[derive(Copy, Clone)]
struct FlowPacket {
    from_initiator_to_respondent: bool,
    timestamp: u64,
    size: usize,
}

pub struct PacketIterator<'a> {
    packets: Iter<'a, FlowPacket>,
    index: usize,
}

impl<'a> Iterator for PacketIterator<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.packets.next().map(|p| {
            let direction = if p.from_initiator_to_respondent { "→" } else { "←" }.to_string();

            Row::new([self.index.to_string(), direction, p.timestamp.to_string(), p.size.to_string()])
        })
    }
}

pub struct NameIterator<'a> {
    names: Keys<'a, FiveTuple, Flow>,
}

impl<'a> Iterator for NameIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.names.next().map(|ft| ft.to_string())
    }
}