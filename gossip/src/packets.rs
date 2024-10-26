use std::net::SocketAddr;

use solana_sdk::packet::Packet;

pub fn create_packet(data: Vec<u8>, addr: &SocketAddr) -> Result<Packet, Box<bincode::ErrorKind>> {
    let packet = Packet::from_data(Some(addr), data);

    packet
}
