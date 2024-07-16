use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::data::monitordata::real_time_net_pack_table_data;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;

use super::device;

#[derive(Debug, Clone)]
pub struct NetPack {
    pub pack_time: String,
    pub pack_source: String,
    pub pack_destination: String,
    pub pack_protocol: String,
    pub pack_length: String,
    pub pack_info: String,
    pub pack_version: String,
}

impl Default for NetPack {
    fn default() -> Self {
        let time: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Self {
            pack_time: time.to_string(),
            pack_source: "".to_string(),
            pack_destination: "".to_string(),
            pack_protocol: "".to_string(),
            pack_length: "".to_string(),
            pack_info: "".to_string(),
            pack_version: "".to_string(),
        }
    }
}

impl NetPack {
    pub fn new(
        p_time: String,
        p_source: String,
        p_destination: String,
        p_protocol: String,
        p_length: String,
        p_info: String,
        p_version: String,
    ) -> Self {
        Self {
            pack_time: p_time,
            pack_source: p_source,
            pack_destination: p_destination,
            pack_protocol: p_protocol,
            pack_length: p_length,
            pack_info: p_info,
            pack_version: p_version,
        }
    }

    pub async fn listen(device_name: String) {
        // get net device list
        let interfaces = datalink::interfaces();
        let interface = interfaces
            .into_iter()
            .filter(|iface: &NetworkInterface| iface.name == device_name)
            .next()
            .expect("Error getting interface");

        let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!(
                "An error occurred when creating the datalink channel: {}",
                e
            ),
        };
        loop {
            // get receive net pack
            match rx.next() {
                Ok(packet) => {
                    let packet = EthernetPacket::new(packet).unwrap();
                    let mut pack = Self::handle_packet(&packet);
                    if !pack.is_empty() {
                        real_time_net_pack_table_data.lock().unwrap().push(pack);
                    }
                }
                Err(e) => {
                    panic!("An error occurred while reading: {}", e);
                }
            }
        }
    }

    fn handle_packet(ethernet: &EthernetPacket) -> NetPack {
        let mut pack = NetPack::default();
        // analyze IPv4 packets by layer
        match ethernet.get_ethertype() {
            EtherTypes::Ipv4 => {
                let header = Ipv4Packet::new(ethernet.payload());
                if let Some(header) = header {
                    match header.get_next_level_protocol() {
                        IpNextHeaderProtocols::Tcp => {
                            let tcp = TcpPacket::new(header.payload());
                            if let Some(tcp) = tcp {
                                let time: u128 = SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis();
                                pack = NetPack::new(
                                    time.to_string(),
                                    header.get_source().to_string(),
                                    header.get_destination().to_string(),
                                    "TCP".to_string(),
                                    header.get_total_length().to_string(),
                                    tcp.get_window().to_string(),
                                    header.get_version().to_string(),
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        return pack;
    }

    pub fn is_empty(&mut self) -> bool {
        let mut flag: bool = false;
        if self.pack_time.is_empty() {
            flag = true;
        }
        if self.pack_source.is_empty() {
            flag = true;
        }
        if self.pack_destination.is_empty() {
            flag = true;
        }
        if self.pack_protocol.is_empty() {
            flag = true;
        }
        if self.pack_length.is_empty() {
            flag = true;
        }
        if self.pack_info.is_empty() {
            flag = true;
        }
        flag
    }
}
