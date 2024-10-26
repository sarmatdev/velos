use std::net::{SocketAddr, UdpSocket};
use std::str;
use std::time::Duration;

use bincode::Error;
use solana_sdk::packet::Packet;
use solana_sdk::{pubkey::Pubkey, timing::timestamp};

use crate::{contact_info::ContactInfo, data::GossipTableData, protocol::Protocol};

fn create_push_message(
    pubkey: &Pubkey,
    shred_version: u16,
    gossip: SocketAddr,
) -> Result<Vec<u8>, Error> {
    let contact_info = ContactInfo::new(pubkey.clone(), timestamp(), shred_version, gossip);

    let push_message = Protocol::PushMessage(
        pubkey.clone(),
        vec![GossipTableData::ContactInfo(contact_info)],
    );

    let serialized = bincode::serialize(&push_message)?;

    Ok(serialized)
}

fn listen_for_gossip_messages(socket: &UdpSocket) -> Option<Packet> {
    let mut buf = [0u8; 2000];
    match socket.recv_from(&mut buf) {
        Ok((size, _src)) => {
            let message: Packet =
                bincode::deserialize(&buf[..size]).expect("Failed to deserialize gossip message");
            Some(message)
        }
        Err(e) => {
            eprintln!("Failed to receive gossip message: {}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use solana_sdk::{
        signature::Keypair,
        signer::{keypair, Signer},
    };

    use crate::packtes::create_packet;

    use super::*;

    #[test]
    fn test_send_push_message() {
        fn send_push_message() -> Result<(), Error> {
            let keypair = Keypair::new();

            let local_socket =
                UdpSocket::bind("0.0.0.0:8001").expect("Failed to bind to local socket");
            let remote_addr: SocketAddr = "34.83.231.102:8001"
                .parse()
                .expect("Invalid remote address");

            let local_addr = local_socket
                .local_addr()
                .expect("Failed to get local address");

            let message = create_push_message(&keypair.pubkey(), 0, local_addr)?;

            let serialized_message = bincode::serialize(&message)?;

            let packet = create_packet(serialized_message, &remote_addr)?;

            let serialized_packet = bincode::serialize(&packet)?;

            let result = local_socket.send_to(&serialized_packet, remote_addr);

            println!("result {:?}", result);

            let listen_result = listen_for_gossip_messages(&local_socket);

            println!("result {:?}", listen_result);

            Ok(())
        }

        send_push_message();
    }
}
