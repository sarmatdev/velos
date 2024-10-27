use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::{ping_message::PingMessage, table::GossipValue};

#[derive(Debug, Serialize, Deserialize)]
pub enum Protocol {
    PingMessage(PingMessage),
    PushMessage(Pubkey, Vec<GossipValue>),
}
