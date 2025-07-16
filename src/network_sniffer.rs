use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::{Packet};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;

const SUSPICIOUS_PORTS: [u16; 5] = [23, 3389, 4444, 21, 22]; // Telnet, RDP, Metasploit, FTP, SSH

pub fn run_network_sniffer() {
    println!("🔍 Network Sniffer: Capturing packets... Press Ctrl+C to stop.\n");

    let interfaces = datalink::interfaces();

    // Print all interfaces for debugging
    println!("Available interfaces:");
    for iface in &interfaces {
        println!(
            " - {} (Up: {}, Loopback: {}, MAC: {:?})",
            iface.name,
            iface.is_up(),
            iface.is_loopback(),
            iface.mac
        );
    }

    // 👇 Manually specify the desired interface (replace this string if your adapter changes)
    let interface_name = r"\Device\NPF_{BA1636FC-91EF-4BA7-9868-DCD5D370E245}";
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("❌ Could not find specified interface");

    println!("✅ Using interface: {}", interface.name);

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, rx)) => ((), rx), // placeholder `()` for unused tx
        Ok(_) => panic!("❌ Unsupported channel type"),
        Err(e) => panic!("❌ Failed to create datalink channel: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(eth) = EthernetPacket::new(packet) {
                    if eth.get_ethertype() == EtherTypes::Ipv4 {
                        if let Some(ip_packet) = Ipv4Packet::new(eth.payload()) {
                            match ip_packet.get_next_level_protocol() {
                                IpNextHeaderProtocols::Tcp => {
                                    if let Some(tcp) = TcpPacket::new(ip_packet.payload()) {
                                        let dst_port = tcp.get_destination();
                                        if SUSPICIOUS_PORTS.contains(&dst_port) {
                                            println!(
                                                "🚨 Suspicious TCP Port Detected! {}:{} -> {}:{}",
                                                ip_packet.get_source(),
                                                tcp.get_source(),
                                                ip_packet.get_destination(),
                                                dst_port
                                            );
                                        }
                                    }
                                }
                                IpNextHeaderProtocols::Udp => {
                                    if let Some(udp) = UdpPacket::new(ip_packet.payload()) {
                                        let dst_port = udp.get_destination();
                                        if SUSPICIOUS_PORTS.contains(&dst_port) {
                                            println!(
                                                "🚨 Suspicious UDP Port Detected! {}:{} -> {}:{}",
                                                ip_packet.get_source(),
                                                udp.get_source(),
                                                ip_packet.get_destination(),
                                                dst_port
                                            );
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("⚠️ Failed to read packet: {}", e);
            }
        }
    }
}
