use std::net::{SocketAddr, UdpSocket};

use bincode::{serialize, Error};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
};

use crate::{packets::create_packet, protocol::Protocol};

const GOSSIP_PING_TOKEN_SIZE: usize = 32;

fn create_ping_message(keypair: &Keypair) -> Result<Protocol, Error> {
    let token = create_token();
    let ping = Ping::new(token, keypair)?;
    let ping_message = Protocol::PingMessage(ping);
    Ok(ping_message)
}

fn create_token() -> [u8; GOSSIP_PING_TOKEN_SIZE] {
    let token = "token_ping".as_bytes();
    let mut token_array = [0u8; GOSSIP_PING_TOKEN_SIZE];

    token_array[..10].copy_from_slice(token);

    token_array
}

pub type PingMessage = Ping<[u8; GOSSIP_PING_TOKEN_SIZE]>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ping<T> {
    from: Pubkey,
    token: T,
    signature: Signature,
}

impl<T: Serialize> Ping<T> {
    pub fn new(token: T, keypair: &Keypair) -> Result<Self, Error> {
        let signature = keypair.sign_message(&serialize(&token)?);
        let ping = Ping {
            from: keypair.pubkey(),
            token,
            signature,
        };
        Ok(ping)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_token() {
        let token = create_token();
        assert!(token.len() == 32);
    }
}
