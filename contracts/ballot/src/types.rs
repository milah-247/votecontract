use soroban_sdk::{contracterror, contracttype, Address, String};

/// Minimum age (in years) a voter must have to be registered or to vote.
pub const MIN_VOTER_AGE: u32 = 18;
/// Upper bound on a voter's age; rejects obviously invalid input.
pub const MAX_VOTER_AGE: u32 = 150;
/// Maximum number of proposals. Bounds the cost of `winning_proposal`
/// and the size of the instance-storage entry that holds the proposals.
pub const MAX_PROPOSALS: u32 = 32;
/// Maximum proposal name length in bytes.
pub const MAX_PROPOSAL_NAME_LEN: u32 = 64;
/// Age recorded for the chairperson, who is registered automatically
/// at deployment (kept from the original Solidity contract).
pub const CHAIRPERSON_AGE: u32 = MIN_VOTER_AGE;

/// Errors returned by the ballot contract.
///
/// Codes are part of the public interface: clients match on them, so
/// existing values must never be renumbered. Add new errors at the end.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// The voter has already cast their vote.
    AlreadyVoted = 1,
    /// The address is not a registered voter (or has zero weight).
    NoRightToVote = 2,
    /// The voter is younger than [`MIN_VOTER_AGE`].
    VoterUnderage = 3,
    /// The voter's card was invalidated by the chairperson.
    InvalidVoterCard = 4,
    /// The action requires voting to be open, but it has not started.
    VotingNotStarted = 5,
    /// `start_voting` was called after voting had already started.
    VotingAlreadyStarted = 6,
    /// The election has ended; no further state changes are allowed.
    VotingEnded = 7,
    /// `give_right_to_vote` was called for an already-registered voter.
    VoterAlreadyRegistered = 8,
    /// The proposal index is out of range.
    InvalidProposal = 9,
    /// The contract was deployed with no proposals.
    NoProposals = 10,
    /// More than [`MAX_PROPOSALS`] proposals were supplied.
    TooManyProposals = 11,
    /// A proposal name is empty, too long, or duplicated.
    InvalidProposalName = 12,
    /// Age is above [`MAX_VOTER_AGE`].
    InvalidAge = 13,
    /// No votes have been cast, so there is no winner.
    NoVotesCast = 14,
    /// The address has never been registered as a voter.
    VoterNotRegistered = 15,
    /// A vote count would overflow.
    Overflow = 16,
}

/// Lifecycle of the election. Transitions only move forward:
/// `NotStarted -> Active -> Ended`.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ElectionState {
    /// Voters can be registered; votes are rejected.
    NotStarted = 0,
    /// Votes are accepted; voters can still be registered or invalidated.
    Active = 1,
    /// Final. No registrations, invalidations or votes are accepted.
    Ended = 2,
}

/// A registered voter ("voter card").
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Voter {
    /// Number of votes this voter contributes. Always 1 today.
    pub weight: u32,
    /// Whether the voter has voted.
    pub voted: bool,
    /// Index of the chosen proposal. Only meaningful when `voted` is true.
    pub vote: u32,
    /// False once the chairperson invalidates the card.
    pub is_valid: bool,
    /// Age supplied by the chairperson at registration.
    pub age: u32,
}

/// A proposal and its running vote total.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub name: String,
    pub vote_count: u64,
}

/// Storage keys.
///
/// Instance storage: `Chairperson`, `State`, `StartTime`, `EndTime`, `Proposals`.
/// Persistent storage: `Voter(Address)`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Chairperson,
    State,
    StartTime,
    EndTime,
    Proposals,
    Voter(Address),
}
