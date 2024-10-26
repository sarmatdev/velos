use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::{data::GossipTableData, ping_message::PingMessage};

#[derive(Debug, Serialize, Deserialize)]
pub enum Protocol {
    PingMessage(PingMessage),
    PushMessage(Pubkey, Vec<GossipTableData>),
}
