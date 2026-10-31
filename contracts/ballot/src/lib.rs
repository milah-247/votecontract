//! # Ballot
//!
//! A chairperson-managed election on Stellar Soroban. It is a port of the
//! original Solidity `Ballot` contract in this repository:
//!
//! * The deployer names a chairperson and a fixed list of proposals.
//! * The chairperson registers voters (age 18+), can invalidate voter
//!   cards, and opens and closes voting.
//! * Each registered, valid voter casts exactly one vote while voting is open.
//! * Anyone can read the proposals, voter cards and the current leader.
//!
//! Authorization uses Soroban's `require_auth`: chairperson-only functions
//! require the stored chairperson's signature, and `vote` requires the
//! voter's signature. There is no `msg.sender`.
#![no_std]

mod events;
mod storage;
mod types;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Env, String, Vec};

pub use types::{
    DataKey, ElectionState, Error, Proposal, Voter, CHAIRPERSON_AGE, MAX_PROPOSALS,
    MAX_PROPOSAL_NAME_LEN, MAX_VOTER_AGE, MIN_VOTER_AGE,
};

#[contract]
pub struct Ballot;

#[contractimpl]
impl Ballot {
    /// Sets up the election. Runs exactly once, atomically with deployment,
    /// so the contract can never be re-initialised or front-run.
    ///
    /// `chairperson` is registered as a voter (weight 1, age 18), as in the
    /// original contract.
    ///
    /// # Panics
    /// * [`Error::NoProposals`] if `proposal_names` is empty.
    /// * [`Error::TooManyProposals`] if there are more than [`MAX_PROPOSALS`].
    /// * [`Error::InvalidProposalName`] if a name is empty, longer than
    ///   [`MAX_PROPOSAL_NAME_LEN`] bytes, or duplicated.
    pub fn __constructor(env: Env, chairperson: Address, proposal_names: Vec<String>) {
        let count = proposal_names.len();
        if count == 0 {
            panic_with_error!(&env, Error::NoProposals);
        }
        if count > MAX_PROPOSALS {
            panic_with_error!(&env, Error::TooManyProposals);
        }

        let mut proposals: Vec<Proposal> = Vec::new(&env);
        for (i, name) in proposal_names.iter().enumerate() {
            if name.is_empty() || name.len() > MAX_PROPOSAL_NAME_LEN {
                panic_with_error!(&env, Error::InvalidProposalName);
            }
            // n <= MAX_PROPOSALS, so the quadratic duplicate check is cheap.
            for earlier in proposal_names.iter().take(i) {
                if earlier == name {
                    panic_with_error!(&env, Error::InvalidProposalName);
                }
            }
            proposals.push_back(Proposal {
                name,
                vote_count: 0,
            });
        }

        storage::set_chairperson(&env, &chairperson);
        storage::set_state(&env, ElectionState::NotStarted);
        storage::set_proposals(&env, &proposals);
        storage::set_voter(
            &env,
            &chairperson,
            &Voter {
                weight: 1,
                voted: false,
                vote: 0,
                is_valid: true,
                age: CHAIRPERSON_AGE,
            },
        );
        storage::extend_instance(&env);
    }

    // ------------------------------------------------------------------
    // Chairperson actions
    // ------------------------------------------------------------------

    /// Opens voting. Chairperson only. Records the ledger timestamp.
    ///
    /// # Errors
    /// * [`Error::VotingAlreadyStarted`] if voting is open.
    /// * [`Error::VotingEnded`] if the election has ended.
    pub fn start_voting(env: Env) -> Result<(), Error> {
        require_chairperson(&env);
        match storage::get_state(&env) {
            ElectionState::NotStarted => {}
            ElectionState::Active => return Err(Error::VotingAlreadyStarted),
            ElectionState::Ended => return Err(Error::VotingEnded),
        }

        let now = env.ledger().timestamp();
        storage::set_state(&env, ElectionState::Active);
        storage::set_start_time(&env, now);
        storage::extend_instance(&env);
        events::VotingStarted { timestamp: now }.publish(&env);
        Ok(())
    }

    /// Closes voting permanently. Chairperson only.
    ///
    /// # Errors
    /// * [`Error::VotingNotStarted`] if voting was never opened.
    /// * [`Error::VotingEnded`] if the election has already ended.
    pub fn end_voting(env: Env) -> Result<(), Error> {
        require_chairperson(&env);
        match storage::get_state(&env) {
            ElectionState::Active => {}
            ElectionState::NotStarted => return Err(Error::VotingNotStarted),
            ElectionState::Ended => return Err(Error::VotingEnded),
        }

        let now = env.ledger().timestamp();
        storage::set_state(&env, ElectionState::Ended);
        storage::set_end_time(&env, now);
        storage::extend_instance(&env);
        events::VotingEnded { timestamp: now }.publish(&env);
        Ok(())
    }

    /// Registers `voter` with weight 1. Chairperson only. Allowed before
    /// and during voting, but not after it ends.
    ///
    /// # Errors
    /// * [`Error::VotingEnded`] if the election has ended.
    /// * [`Error::AlreadyVoted`] if `voter` is registered and has voted.
    /// * [`Error::VoterAlreadyRegistered`] if `voter` is already registered.
    /// * [`Error::VoterUnderage`] if `age` < [`MIN_VOTER_AGE`].
    /// * [`Error::InvalidAge`] if `age` > [`MAX_VOTER_AGE`].
    pub fn give_right_to_vote(env: Env, voter: Address, age: u32) -> Result<(), Error> {
        require_chairperson(&env);
        if storage::get_state(&env) == ElectionState::Ended {
            return Err(Error::VotingEnded);
        }
        if let Some(existing) = storage::get_voter(&env, &voter) {
            if existing.voted {
                return Err(Error::AlreadyVoted);
            }
            return Err(Error::VoterAlreadyRegistered);
        }
        if age < MIN_VOTER_AGE {
            return Err(Error::VoterUnderage);
        }
        if age > MAX_VOTER_AGE {
            return Err(Error::InvalidAge);
        }

        storage::set_voter(
            &env,
            &voter,
            &Voter {
                weight: 1,
                voted: false,
                vote: 0,
                is_valid: true,
                age,
            },
        );
        storage::extend_instance(&env);
        events::VoterRegistered { voter, age }.publish(&env);
        Ok(())
    }

    /// Invalidates a voter card so it can no longer vote. Chairperson only.
    /// Invalidating an already-invalid card is a no-op.
    ///
    /// A vote that has already been counted cannot be revoked, so
    /// invalidating a voter who has voted is rejected rather than silently
    /// leaving their vote in the tally.
    ///
    /// # Errors
    /// * [`Error::VotingEnded`] if the election has ended.
    /// * [`Error::VoterNotRegistered`] if `voter` was never registered.
    /// * [`Error::AlreadyVoted`] if `voter` has already voted.
    pub fn invalidate_voter(env: Env, voter: Address) -> Result<(), Error> {
        require_chairperson(&env);
        if storage::get_state(&env) == ElectionState::Ended {
            return Err(Error::VotingEnded);
        }
        let mut card = storage::get_voter(&env, &voter).ok_or(Error::VoterNotRegistered)?;
        if card.voted {
            return Err(Error::AlreadyVoted);
        }
        if !card.is_valid {
            return Ok(());
        }

        card.is_valid = false;
        storage::set_voter(&env, &voter, &card);
        storage::extend_instance(&env);
        events::VoterInvalidated { voter }.publish(&env);
        Ok(())
    }

    // ------------------------------------------------------------------
    // Voting
    // ------------------------------------------------------------------

    /// Casts `voter`'s vote for proposal index `proposal`. Requires the
    /// voter's authorization.
    ///
    /// # Errors
    /// * [`Error::VotingNotStarted`] / [`Error::VotingEnded`] if voting is not open.
    /// * [`Error::NoRightToVote`] if `voter` is not registered.
    /// * [`Error::AlreadyVoted`] if `voter` has already voted.
    /// * [`Error::InvalidVoterCard`] if the card was invalidated.
    /// * [`Error::VoterUnderage`] if the recorded age is below the minimum.
    /// * [`Error::InvalidProposal`] if `proposal` is out of range.
    /// * [`Error::Overflow`] if the proposal's tally would overflow.
    pub fn vote(env: Env, voter: Address, proposal: u32) -> Result<(), Error> {
        voter.require_auth();
        match storage::get_state(&env) {
            ElectionState::Active => {}
            ElectionState::NotStarted => return Err(Error::VotingNotStarted),
            ElectionState::Ended => return Err(Error::VotingEnded),
        }

        let mut card = storage::get_voter(&env, &voter).ok_or(Error::NoRightToVote)?;
        if card.weight == 0 {
            return Err(Error::NoRightToVote);
        }
        if card.voted {
            return Err(Error::AlreadyVoted);
        }
        if !card.is_valid {
            return Err(Error::InvalidVoterCard);
        }
        if card.age < MIN_VOTER_AGE {
            return Err(Error::VoterUnderage);
        }

        let mut proposals = storage::get_proposals(&env);
        let mut chosen = proposals.get(proposal).ok_or(Error::InvalidProposal)?;
        chosen.vote_count = chosen
            .vote_count
            .checked_add(u64::from(card.weight))
            .ok_or(Error::Overflow)?;
        proposals.set(proposal, chosen);

        card.voted = true;
        card.vote = proposal;

        storage::set_proposals(&env, &proposals);
        storage::set_voter(&env, &voter, &card);
        storage::extend_instance(&env);
        events::VoteCast {
            voter,
            proposal,
            weight: card.weight,
        }
        .publish(&env);
        Ok(())
    }

    // ------------------------------------------------------------------
    // Read-only queries
    // ------------------------------------------------------------------

    /// The chairperson's address.
    pub fn chairperson(env: Env) -> Address {
        storage::get_chairperson(&env)
    }

    /// Current election state.
    pub fn state(env: Env) -> ElectionState {
        storage::get_state(&env)
    }

    /// Ledger timestamp when voting opened, or 0 if it has not.
    pub fn voting_start_time(env: Env) -> u64 {
        storage::get_start_time(&env)
    }

    /// Ledger timestamp when voting closed, or 0 if it has not.
    pub fn voting_end_time(env: Env) -> u64 {
        storage::get_end_time(&env)
    }

    /// Whether voting is currently open.
    pub fn voting_started(env: Env) -> bool {
        storage::get_state(&env) == ElectionState::Active
    }

    /// The voter card for `voter`, or `None` if never registered.
    /// Replaces Solidity's `getMyVoterCard` (Soroban has no `msg.sender`
    /// and contract storage is public anyway).
    pub fn get_voter(env: Env, voter: Address) -> Option<Voter> {
        storage::get_voter(&env, &voter)
    }

    /// Number of proposals.
    pub fn proposal_count(env: Env) -> u32 {
        storage::get_proposals(&env).len()
    }

    /// The proposal at `index`.
    ///
    /// # Errors
    /// * [`Error::InvalidProposal`] if `index` is out of range.
    pub fn proposal(env: Env, index: u32) -> Result<Proposal, Error> {
        storage::get_proposals(&env)
            .get(index)
            .ok_or(Error::InvalidProposal)
    }

    /// All proposals with their current tallies.
    pub fn proposals(env: Env) -> Vec<Proposal> {
        storage::get_proposals(&env)
    }

    /// Index of the proposal with the most votes. On a tie, the lowest
    /// index wins (same rule as the original contract). Can be called at
    /// any time; the result is only final once [`ElectionState::Ended`].
    ///
    /// # Errors
    /// * [`Error::NoVotesCast`] if no votes have been cast. (The Solidity
    ///   version returned proposal 0 in this case.)
    pub fn winning_proposal(env: Env) -> Result<u32, Error> {
        let mut best_index: u32 = 0;
        let mut best_count: u64 = 0;
        for (i, p) in storage::get_proposals(&env).iter().enumerate() {
            if p.vote_count > best_count {
                best_count = p.vote_count;
                // i < MAX_PROPOSALS, so this never truncates.
                best_index = i as u32;
            }
        }
        if best_count == 0 {
            return Err(Error::NoVotesCast);
        }
        Ok(best_index)
    }

    /// Name of the winning proposal. See [`Ballot::winning_proposal`].
    pub fn winner_name(env: Env) -> Result<String, Error> {
        let index = Self::winning_proposal(env.clone())?;
        Ok(storage::get_proposals(&env)
            .get(index)
            .ok_or(Error::InvalidProposal)?
            .name)
    }
}

fn require_chairperson(env: &Env) {
    storage::get_chairperson(env).require_auth();
}
