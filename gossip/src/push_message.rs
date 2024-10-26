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

#[cfg(test)]
mod tests {
    use solana_sdk::{
        signature::Keypair,
        signer::{keypair, Signer},
    };

    use crate::{connection::Connection, packets::create_packet};

    use super::*;

    #[tokio::test]
    async fn test_send_push_message() {
        async fn send_push_message() -> Result<(), Error> {
            let keypair = Keypair::new();

            let remote_addr: SocketAddr = "34.83.231.102:8001"
                .parse()
                .expect("Invalid remote address");

            let mut connection = Connection::connect(remote_addr).await?;

            let message = create_push_message(&keypair.pubkey(), 0, connection.local_addr())?;

            let serialized_message = bincode::serialize(&message)?;

            let packet = create_packet(serialized_message, &remote_addr)?;

            let serialized_packet = bincode::serialize(&packet)?;

            let result = connection.send(serialized_packet).await;

            println!("result {:?}", result);

            let recive_result = connection.receive().await;

            println!("result {:?}", recive_result);

            Ok(())
        }

        send_push_message().await;
    }
}
