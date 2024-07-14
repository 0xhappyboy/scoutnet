use std::collections::HashMap;

use crate::app;
use crate::app::app::App;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;

use super::device;

pub struct NetPack {}

impl NetPack {
    pub async fn listen(device_name: String, app: &mut App) {
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
                    Self::handle_packet(app, &packet);
                }
                Err(e) => {
                    panic!("An error occurred while reading: {}", e);
                }
            }
        }
    }

    fn handle_packet(app: &mut App, ethernet: &EthernetPacket) {
        // 对Ipv4的包按层解析
        match ethernet.get_ethertype() {
            EtherTypes::Ipv4 => {
                let header = Ipv4Packet::new(ethernet.payload());
                if let Some(header) = header {
                    match header.get_next_level_protocol() {
                        IpNextHeaderProtocols::Tcp => {
                            let tcp = TcpPacket::new(header.payload());
                            if let Some(tcp) = tcp {
                                let mut m: HashMap<String, String> = HashMap::new();
                                m.insert("k1".to_string(), header.get_source().to_string());
                                m.insert("k2".to_string(), tcp.get_source().to_string());
                                m.insert("k3".to_string(), header.get_destination().to_string());
                                m.insert("k4".to_string(), tcp.get_destination().to_string());
                                m.insert("k5".to_string(), tcp.get_destination().to_string());
                                m.insert("k6".to_string(), tcp.get_destination().to_string());
                                m.insert("k6".to_string(), tcp.get_destination().to_string());
                                app.monitor_page_real_time_net_pack_table_data.push(m);

                                // println!(
                                //     "获取到一个 tcp 数据包 {}:{} to {}:{}",
                                //     header.get_source(),
                                //     tcp.get_source(),
                                //     header.get_destination(),
                                //     tcp.get_destination()
                                // );
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
