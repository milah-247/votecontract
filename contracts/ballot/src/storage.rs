//! Storage helpers and TTL (time-to-live) management.
//!
//! Election-wide data lives in instance storage, which shares the contract
//! instance's TTL. Voter cards live in persistent storage, one entry per
//! address, and have their TTL extended whenever they are written.

use soroban_sdk::{Address, Env, Vec};

use crate::types::{DataKey, ElectionState, Proposal, Voter};

/// Ledgers per day, assuming ~5 second ledger close times.
const DAY_IN_LEDGERS: u32 = 17_280;
/// Extend entries to live for ~30 days...
pub const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
/// ...whenever their remaining TTL falls below ~29 days.
pub const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn get_chairperson(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&DataKey::Chairperson)
        .expect("chairperson is set in the constructor")
}

pub fn set_chairperson(env: &Env, chairperson: &Address) {
    env.storage()
        .instance()
        .set(&DataKey::Chairperson, chairperson);
}

pub fn get_state(env: &Env) -> ElectionState {
    env.storage()
        .instance()
        .get(&DataKey::State)
        .unwrap_or(ElectionState::NotStarted)
}

pub fn set_state(env: &Env, state: ElectionState) {
    env.storage().instance().set(&DataKey::State, &state);
}

pub fn get_start_time(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::StartTime)
        .unwrap_or(0)
}

pub fn set_start_time(env: &Env, timestamp: u64) {
    env.storage()
        .instance()
        .set(&DataKey::StartTime, &timestamp);
}

pub fn get_end_time(env: &Env) -> u64 {
    env.storage().instance().get(&DataKey::EndTime).unwrap_or(0)
}

pub fn set_end_time(env: &Env, timestamp: u64) {
    env.storage().instance().set(&DataKey::EndTime, &timestamp);
}

pub fn get_proposals(env: &Env) -> Vec<Proposal> {
    env.storage()
        .instance()
        .get(&DataKey::Proposals)
        .unwrap_or_else(|| Vec::new(env))
}

pub fn set_proposals(env: &Env, proposals: &Vec<Proposal>) {
    env.storage().instance().set(&DataKey::Proposals, proposals);
}

pub fn get_voter(env: &Env, voter: &Address) -> Option<Voter> {
    env.storage()
        .persistent()
        .get(&DataKey::Voter(voter.clone()))
}

pub fn set_voter(env: &Env, address: &Address, voter: &Voter) {
    let key = DataKey::Voter(address.clone());
    env.storage().persistent().set(&key, voter);
    env.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}
