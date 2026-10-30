//! Contract events. Off-chain indexers rely on these names and fields,
//! so treat changes as breaking.

use soroban_sdk::{contractevent, Address};

/// Emitted once when voting opens.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VotingStarted {
    pub timestamp: u64,
}

/// Emitted once when voting closes.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VotingEnded {
    pub timestamp: u64,
}

/// Emitted when the chairperson registers a voter.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoterRegistered {
    #[topic]
    pub voter: Address,
    pub age: u32,
}

/// Emitted when the chairperson invalidates a voter card.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoterInvalidated {
    #[topic]
    pub voter: Address,
}

/// Emitted for every accepted vote.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoteCast {
    #[topic]
    pub voter: Address,
    pub proposal: u32,
    pub weight: u32,
}
