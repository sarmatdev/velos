//! Cluster Replicated Data Store
//! Stores gossip data
//! Work in progress...
//? References:
// 1. https://github.com/Syndica/sig/blob/main/src/gossip/table.zig
// 2. https://github.com/anza-xyz/agave/blob/master/gossip/src/crds.rs

use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    sync::{Mutex, RwLock},
};

use indexmap::{IndexMap, IndexSet};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use solana_sdk::{clock::Slot, hash::Hash, pubkey::Pubkey, signature::Signature};

use crate::data::GossipTableData;

pub struct GossipTable {
    table: IndexMap<GossipTableData, VersionedGossipValue>,
    cursor: Cursor,
    shards: Shards,
    nodes: IndexSet<usize>,
    votes: BTreeMap<u64, usize>,
    epoch_slots: BTreeMap<u64, usize>,
    duplicate_shreds: BTreeMap<u64, usize>,
    records: HashMap<Pubkey, IndexSet<usize>>,
    entries: BTreeMap<u64, usize>,
    purged: VecDeque<(Hash, u64)>,
    shred_version: HashMap<Pubkey, u16>,
    stats: Mutex<GossipStats>,
}

impl GossipTable {
    pub fn new() {}
}

struct Shards {
    // shards[k] includes crds values which the first shard_bits of their hash
    // value is equal to k. Each shard is a mapping from crds values indices to
    // their hash value.
    shards: Vec<IndexMap<usize, u64>>,
    shard_bits: u32,
}

pub struct VersionedGossipValue {
    /// Ordinal index indicating insert order.
    ordinal: u64,
    pub value: GossipValue,
    /// local time when updated
    pub local_timestamp: u64,
    /// value hash
    pub value_hash: Hash,
    /// None -> value upserted by GossipRoute::{LocalMessage,PullRequest}
    /// Some(0) -> value upserted by GossipRoute::PullResponse
    /// Some(k) if k > 0 -> value upserted by GossipRoute::PushMessage w/ k - 1 push duplicates
    num_push_recv: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GossipValue {
    pub signature: Signature,
    pub data: GossipTableData,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GossipTableError {
    DuplicatePush(/*num dups:*/ u8),
    InsertFailed,
    UnknownStakes,
}

#[derive(Clone, Copy)]
pub enum GossipRoute<'a> {
    LocalMessage,
    PullRequest,
    PullResponse,
    PushMessage(/*from:*/ &'a Pubkey),
}

/// A cursor is a mechanism used to track the ordinal position or the insert order of elements within the GossipTable
/// Specifically, it helps keep track of the next insertion point and updates accordingly as new elements are added.
#[derive(Clone, Copy, Default)]
pub struct Cursor(u64);

impl Cursor {
    // returns the current value of the cursor
    fn ordinal(&self) -> u64 {
        self.0
    }

    // Updates the cursor position given the ordinal index of value consumed.
    // method updates the cursor by advancing it when a new entry is consumed or inserted, ensuring that it tracks the maximum ordinal index it has seen.
    #[inline]
    fn consume(&mut self, ordinal: u64) {
        self.0 = self.0.max(ordinal + 1);
    }
}

pub struct GossipStats {
    pub pull: GossipDataStats,
    pub push: GossipDataStats,
    pub num_redundant_pull_responses: u64,
}

type GossipCountsArray = [usize; 14];

pub struct GossipDataStats {
    pub counts: GossipCountsArray,
    pub fails: GossipCountsArray,
    pub votes: LruCache<Slot, usize>,
}
