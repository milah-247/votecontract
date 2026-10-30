#![cfg(test)]
extern crate std;

use soroban_sdk::{
    testutils::{
        storage::{Instance as _, Persistent as _},
        Address as _, AuthorizedFunction, AuthorizedInvocation, Events as _, Ledger as _, MockAuth,
        MockAuthInvoke,
    },
    vec, Address, Env, Event, IntoVal, String, Symbol, Vec,
};

use crate::{
    events, storage, Ballot, BallotClient, DataKey, ElectionState, Error, Proposal, Voter,
    CHAIRPERSON_AGE, MAX_PROPOSALS, MAX_PROPOSAL_NAME_LEN, MAX_VOTER_AGE, MIN_VOTER_AGE,
};

// ----------------------------------------------------------------------
// Helpers
// ----------------------------------------------------------------------

fn names(env: &Env, list: &[&str]) -> Vec<String> {
    let mut out = Vec::new(env);
    for n in list {
        out.push_back(String::from_str(env, n));
    }
    out
}

/// Deploys a ballot with proposals "Alice" and "Bob" and all auths mocked.
fn setup(env: &Env) -> (BallotClient<'_>, Address) {
    env.mock_all_auths();
    let chair = Address::generate(env);
    let id = env.register(Ballot, (chair.clone(), names(env, &["Alice", "Bob"])));
    (BallotClient::new(env, &id), chair)
}

fn deploy_with(env: &Env, list: &[&str]) -> BallotClient<'static> {
    let chair = Address::generate(env);
    let id = env.register(Ballot, (chair, names(env, list)));
    BallotClient::new(env, &id)
}

fn registered_voter(env: &Env, client: &BallotClient, age: u32) -> Address {
    let voter = Address::generate(env);
    client.give_right_to_vote(&voter, &age);
    voter
}

/// Asserts the last invocation required exactly `signer`'s auth for `fn_name(args)`.
fn assert_auth(
    env: &Env,
    client: &BallotClient,
    signer: &Address,
    fn_name: &str,
    args: Vec<soroban_sdk::Val>,
) {
    assert_eq!(
        env.auths(),
        std::vec![(
            signer.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    client.address.clone(),
                    Symbol::new(env, fn_name),
                    args,
                )),
                sub_invocations: std::vec![],
            }
        )]
    );
}

/// Replaces the blanket auth mock with a single signature from `signer`
/// for `fn_name(args)`.
fn sign_as(
    env: &Env,
    client: &BallotClient,
    signer: &Address,
    fn_name: &str,
    args: Vec<soroban_sdk::Val>,
) {
    env.mock_auths(&[MockAuth {
        address: signer,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name,
            args,
            sub_invokes: &[],
        },
    }]);
}

// ----------------------------------------------------------------------
// Construction
// ----------------------------------------------------------------------

#[test]
fn constructor_sets_initial_state() {
    let env = Env::default();
    let (client, chair) = setup(&env);

    assert_eq!(client.chairperson(), chair);
    assert_eq!(client.state(), ElectionState::NotStarted);
    assert!(!client.voting_started());
    assert_eq!(client.voting_start_time(), 0);
    assert_eq!(client.voting_end_time(), 0);
    assert_eq!(client.proposal_count(), 2);
    assert_eq!(
        client.proposals(),
        vec![
            &env,
            Proposal {
                name: String::from_str(&env, "Alice"),
                vote_count: 0
            },
            Proposal {
                name: String::from_str(&env, "Bob"),
                vote_count: 0
            },
        ]
    );
}

#[test]
fn constructor_registers_chairperson_as_voter() {
    let env = Env::default();
    let (client, chair) = setup(&env);

    assert_eq!(
        client.get_voter(&chair),
        Some(Voter {
            weight: 1,
            voted: false,
            vote: 0,
            is_valid: true,
            age: CHAIRPERSON_AGE,
        })
    );
}

#[test]
fn constructor_accepts_max_proposals_and_max_name_length() {
    let env = Env::default();
    let mut list = Vec::new(&env);
    for i in 0..MAX_PROPOSALS {
        list.push_back(String::from_str(&env, &std::format!("proposal-{i}")));
    }
    let id = env.register(Ballot, (Address::generate(&env), list));
    assert_eq!(BallotClient::new(&env, &id).proposal_count(), MAX_PROPOSALS);

    let long = "x".repeat(MAX_PROPOSAL_NAME_LEN as usize);
    let client = deploy_with(&env, &[long.as_str()]);
    assert_eq!(client.proposal_count(), 1);
}

#[test]
#[should_panic(expected = "Error(Contract, #10)")]
fn constructor_rejects_empty_proposal_list() {
    let env = Env::default();
    deploy_with(&env, &[]);
}

#[test]
#[should_panic(expected = "Error(Contract, #11)")]
fn constructor_rejects_too_many_proposals() {
    let env = Env::default();
    let mut list = Vec::new(&env);
    for i in 0..=MAX_PROPOSALS {
        list.push_back(String::from_str(&env, &std::format!("p{i}")));
    }
    env.register(Ballot, (Address::generate(&env), list));
}

#[test]
#[should_panic(expected = "Error(Contract, #12)")]
fn constructor_rejects_empty_proposal_name() {
    let env = Env::default();
    deploy_with(&env, &["Alice", ""]);
}

#[test]
#[should_panic(expected = "Error(Contract, #12)")]
fn constructor_rejects_overlong_proposal_name() {
    let env = Env::default();
    let long = "x".repeat(MAX_PROPOSAL_NAME_LEN as usize + 1);
    deploy_with(&env, &[long.as_str()]);
}

#[test]
#[should_panic(expected = "Error(Contract, #12)")]
fn constructor_rejects_duplicate_proposal_names() {
    let env = Env::default();
    deploy_with(&env, &["Alice", "Bob", "Alice"]);
}

// ----------------------------------------------------------------------
// Election lifecycle
// ----------------------------------------------------------------------

#[test]
fn start_voting_opens_election_and_records_time() {
    let env = Env::default();
    let (client, chair) = setup(&env);
    env.ledger().set_timestamp(1_700_000_000);

    client.start_voting();
    assert_auth(&env, &client, &chair, "start_voting", vec![&env]);
    assert_eq!(
        env.events().all(),
        [events::VotingStarted {
            timestamp: 1_700_000_000
        }
        .to_xdr(&env, &client.address)]
    );

    assert_eq!(client.state(), ElectionState::Active);
    assert!(client.voting_started());
    assert_eq!(client.voting_start_time(), 1_700_000_000);
}

#[test]
fn start_voting_twice_fails() {
    let env = Env::default();
    let (client, _) = setup(&env);
    client.start_voting();
    assert_eq!(
        client.try_start_voting(),
        Err(Ok(Error::VotingAlreadyStarted))
    );
}

#[test]
fn end_voting_closes_election_and_records_time() {
    let env = Env::default();
    let (client, chair) = setup(&env);
    env.ledger().set_timestamp(100);
    client.start_voting();
    env.ledger().set_timestamp(200);

    client.end_voting();
    assert_auth(&env, &client, &chair, "end_voting", vec![&env]);
    assert_eq!(
        env.events().all(),
        [events::VotingEnded { timestamp: 200 }.to_xdr(&env, &client.address)]
    );
    assert_eq!(client.state(), ElectionState::Ended);
    assert!(!client.voting_started());
    assert_eq!(client.voting_start_time(), 100);
    assert_eq!(client.voting_end_time(), 200);
}

#[test]
fn end_voting_before_start_fails() {
    let env = Env::default();
    let (client, _) = setup(&env);
    assert_eq!(client.try_end_voting(), Err(Ok(Error::VotingNotStarted)));
}

#[test]
fn ended_election_is_final() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 30);
    client.start_voting();
    client.end_voting();

    assert_eq!(client.try_end_voting(), Err(Ok(Error::VotingEnded)));
    assert_eq!(client.try_start_voting(), Err(Ok(Error::VotingEnded)));
    assert_eq!(client.try_vote(&voter, &0), Err(Ok(Error::VotingEnded)));
    assert_eq!(
        client.try_give_right_to_vote(&Address::generate(&env), &30),
        Err(Ok(Error::VotingEnded))
    );
    assert_eq!(
        client.try_invalidate_voter(&voter),
        Err(Ok(Error::VotingEnded))
    );
    assert_eq!(client.state(), ElectionState::Ended);
}

// ----------------------------------------------------------------------
// Authorization
// ----------------------------------------------------------------------

#[test]
fn non_chairperson_cannot_start_voting() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let attacker = Address::generate(&env);

    sign_as(&env, &client, &attacker, "start_voting", vec![&env]);
    assert!(client.try_start_voting().is_err());
    assert_eq!(client.state(), ElectionState::NotStarted);
}

#[test]
fn non_chairperson_cannot_end_voting() {
    let env = Env::default();
    let (client, _) = setup(&env);
    client.start_voting();
    let attacker = Address::generate(&env);

    sign_as(&env, &client, &attacker, "end_voting", vec![&env]);
    assert!(client.try_end_voting().is_err());
    assert_eq!(client.state(), ElectionState::Active);
}

#[test]
fn non_chairperson_cannot_give_right_to_vote() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let attacker = Address::generate(&env);
    let voter = Address::generate(&env);

    sign_as(
        &env,
        &client,
        &attacker,
        "give_right_to_vote",
        (voter.clone(), 30_u32).into_val(&env),
    );
    assert!(client.try_give_right_to_vote(&voter, &30).is_err());
    assert_eq!(client.get_voter(&voter), None);
}

#[test]
fn non_chairperson_cannot_invalidate_voter() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 30);
    let attacker = Address::generate(&env);

    sign_as(
        &env,
        &client,
        &attacker,
        "invalidate_voter",
        (voter.clone(),).into_val(&env),
    );
    assert!(client.try_invalidate_voter(&voter).is_err());
    assert!(client.get_voter(&voter).unwrap().is_valid);
}

#[test]
fn chairperson_actions_fail_without_any_auth() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = Address::generate(&env);

    env.set_auths(&[]);
    assert!(client.try_start_voting().is_err());
    assert!(client.try_give_right_to_vote(&voter, &30).is_err());
}

#[test]
fn vote_requires_the_voters_own_auth() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 30);
    client.start_voting();
    let attacker = Address::generate(&env);

    // The attacker signs, but the vote is cast on behalf of `voter`.
    sign_as(
        &env,
        &client,
        &attacker,
        "vote",
        (voter.clone(), 0_u32).into_val(&env),
    );
    assert!(client.try_vote(&voter, &0).is_err());

    env.mock_all_auths();
    assert!(!client.get_voter(&voter).unwrap().voted);
    assert_eq!(client.proposal(&0).vote_count, 0);
}

#[test]
fn chairperson_signature_is_checked_with_real_mock_auth() {
    let env = Env::default();
    let (client, chair) = setup(&env);

    sign_as(&env, &client, &chair, "start_voting", vec![&env]);
    client.start_voting();
    assert_eq!(client.state(), ElectionState::Active);
}

// ----------------------------------------------------------------------
// Voter registration
// ----------------------------------------------------------------------

#[test]
fn give_right_to_vote_registers_voter() {
    let env = Env::default();
    let (client, chair) = setup(&env);
    let voter = Address::generate(&env);

    client.give_right_to_vote(&voter, &30);
    assert_auth(
        &env,
        &client,
        &chair,
        "give_right_to_vote",
        (voter.clone(), 30_u32).into_val(&env),
    );
    assert_eq!(
        env.events().all(),
        [events::VoterRegistered {
            voter: voter.clone(),
            age: 30
        }
        .to_xdr(&env, &client.address)]
    );
    assert_eq!(
        client.get_voter(&voter),
        Some(Voter {
            weight: 1,
            voted: false,
            vote: 0,
            is_valid: true,
            age: 30,
        })
    );
}

#[test]
fn give_right_to_vote_age_boundaries() {
    let env = Env::default();
    let (client, _) = setup(&env);

    assert_eq!(
        client.try_give_right_to_vote(&Address::generate(&env), &(MIN_VOTER_AGE - 1)),
        Err(Ok(Error::VoterUnderage))
    );
    assert_eq!(
        client.try_give_right_to_vote(&Address::generate(&env), &0),
        Err(Ok(Error::VoterUnderage))
    );
    assert_eq!(
        client.try_give_right_to_vote(&Address::generate(&env), &(MAX_VOTER_AGE + 1)),
        Err(Ok(Error::InvalidAge))
    );
    assert_eq!(
        client.try_give_right_to_vote(&Address::generate(&env), &u32::MAX),
        Err(Ok(Error::InvalidAge))
    );

    let youngest = registered_voter(&env, &client, MIN_VOTER_AGE);
    let oldest = registered_voter(&env, &client, MAX_VOTER_AGE);
    assert_eq!(client.get_voter(&youngest).unwrap().age, MIN_VOTER_AGE);
    assert_eq!(client.get_voter(&oldest).unwrap().age, MAX_VOTER_AGE);
}

#[test]
fn give_right_to_vote_rejects_already_registered_voter() {
    let env = Env::default();
    let (client, chair) = setup(&env);
    let voter = registered_voter(&env, &client, 30);

    assert_eq!(
        client.try_give_right_to_vote(&voter, &40),
        Err(Ok(Error::VoterAlreadyRegistered))
    );
    // The chairperson is registered at deployment.
    assert_eq!(
        client.try_give_right_to_vote(&chair, &40),
        Err(Ok(Error::VoterAlreadyRegistered))
    );
    // The original card is untouched.
    assert_eq!(client.get_voter(&voter).unwrap().age, 30);
}

#[test]
fn give_right_to_vote_rejects_voter_who_already_voted() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 30);
    client.start_voting();
    client.vote(&voter, &1);

    assert_eq!(
        client.try_give_right_to_vote(&voter, &30),
        Err(Ok(Error::AlreadyVoted))
    );
}

#[test]
fn voters_can_be_registered_while_voting_is_active() {
    let env = Env::default();
    let (client, _) = setup(&env);
    client.start_voting();

    let late = registered_voter(&env, &client, 25);
    client.vote(&late, &0);
    assert_eq!(client.proposal(&0).vote_count, 1);
}

// ----------------------------------------------------------------------
// Invalidation
// ----------------------------------------------------------------------

#[test]
fn invalidate_voter_marks_card_invalid() {
    let env = Env::default();
    let (client, chair) = setup(&env);
    let voter = registered_voter(&env, &client, 30);

    client.invalidate_voter(&voter);
    assert_auth(
        &env,
        &client,
        &chair,
        "invalidate_voter",
        (voter.clone(),).into_val(&env),
    );
    assert_eq!(
        env.events().all(),
        [events::VoterInvalidated {
            voter: voter.clone()
        }
        .to_xdr(&env, &client.address)]
    );
    assert!(!client.get_voter(&voter).unwrap().is_valid);
}

#[test]
fn invalidated_voter_cannot_vote() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 25);
    client.invalidate_voter(&voter);
    client.start_voting();

    assert_eq!(
        client.try_vote(&voter, &1),
        Err(Ok(Error::InvalidVoterCard))
    );
    assert_eq!(client.proposal(&1).vote_count, 0);
}

#[test]
fn invalidate_unregistered_voter_fails() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let stranger = Address::generate(&env);

    assert_eq!(
        client.try_invalidate_voter(&stranger),
        Err(Ok(Error::VoterNotRegistered))
    );
    // No empty record is created as a side effect.
    assert_eq!(client.get_voter(&stranger), None);
}

#[test]
fn invalidate_voter_after_they_voted_fails() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 30);
    client.start_voting();
    client.vote(&voter, &0);

    assert_eq!(
        client.try_invalidate_voter(&voter),
        Err(Ok(Error::AlreadyVoted))
    );
    assert!(client.get_voter(&voter).unwrap().is_valid);
    assert_eq!(client.proposal(&0).vote_count, 1);
}

#[test]
fn invalidate_voter_twice_is_a_noop() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 30);
    client.invalidate_voter(&voter);

    client.invalidate_voter(&voter);
    assert_eq!(env.events().all().events().len(), 0);
    assert!(!client.get_voter(&voter).unwrap().is_valid);
}

#[test]
fn chairperson_can_invalidate_own_card() {
    let env = Env::default();
    let (client, chair) = setup(&env);
    client.invalidate_voter(&chair);
    client.start_voting();
    assert_eq!(
        client.try_vote(&chair, &0),
        Err(Ok(Error::InvalidVoterCard))
    );
}

// ----------------------------------------------------------------------
// Voting
// ----------------------------------------------------------------------

#[test]
fn vote_records_choice_and_updates_tally() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 25);
    client.start_voting();

    client.vote(&voter, &0);
    assert_auth(
        &env,
        &client,
        &voter,
        "vote",
        (voter.clone(), 0_u32).into_val(&env),
    );
    assert_eq!(
        env.events().all(),
        [events::VoteCast {
            voter: voter.clone(),
            proposal: 0,
            weight: 1
        }
        .to_xdr(&env, &client.address)]
    );

    let card = client.get_voter(&voter).unwrap();
    assert!(card.voted);
    assert_eq!(card.vote, 0);
    assert_eq!(client.proposal(&0).vote_count, 1);
    assert_eq!(client.proposal(&1).vote_count, 0);
}

#[test]
fn vote_before_voting_starts_fails() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 25);

    assert_eq!(
        client.try_vote(&voter, &0),
        Err(Ok(Error::VotingNotStarted))
    );
}

#[test]
fn vote_twice_fails_and_does_not_double_count() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 25);
    client.start_voting();
    client.vote(&voter, &0);

    assert_eq!(client.try_vote(&voter, &1), Err(Ok(Error::AlreadyVoted)));
    assert_eq!(client.try_vote(&voter, &0), Err(Ok(Error::AlreadyVoted)));
    assert_eq!(client.proposal(&0).vote_count, 1);
    assert_eq!(client.proposal(&1).vote_count, 0);
    assert_eq!(client.get_voter(&voter).unwrap().vote, 0);
}

#[test]
fn unregistered_address_cannot_vote() {
    let env = Env::default();
    let (client, _) = setup(&env);
    client.start_voting();

    assert_eq!(
        client.try_vote(&Address::generate(&env), &0),
        Err(Ok(Error::NoRightToVote))
    );
}

#[test]
fn vote_for_out_of_range_proposal_fails_without_side_effects() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 25);
    client.start_voting();

    assert_eq!(client.try_vote(&voter, &2), Err(Ok(Error::InvalidProposal)));
    assert_eq!(
        client.try_vote(&voter, &u32::MAX),
        Err(Ok(Error::InvalidProposal))
    );
    // The voter can still vote for a valid proposal afterwards.
    assert!(!client.get_voter(&voter).unwrap().voted);
    client.vote(&voter, &1);
    assert_eq!(client.proposal(&1).vote_count, 1);
}

#[test]
fn chairperson_can_vote() {
    let env = Env::default();
    let (client, chair) = setup(&env);
    client.start_voting();
    client.vote(&chair, &1);
    assert_eq!(client.proposal(&1).vote_count, 1);
}

#[test]
fn vote_rejects_underage_card_defensively() {
    // Registration already rejects underage voters; this checks the
    // second guard in `vote` against a card written directly to storage.
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = Address::generate(&env);
    env.as_contract(&client.address, || {
        storage::set_voter(
            &env,
            &voter,
            &Voter {
                weight: 1,
                voted: false,
                vote: 0,
                is_valid: true,
                age: 10,
            },
        );
    });
    client.start_voting();
    assert_eq!(client.try_vote(&voter, &0), Err(Ok(Error::VoterUnderage)));
}

#[test]
fn vote_rejects_zero_weight_card() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = Address::generate(&env);
    env.as_contract(&client.address, || {
        storage::set_voter(
            &env,
            &voter,
            &Voter {
                weight: 0,
                voted: false,
                vote: 0,
                is_valid: true,
                age: 30,
            },
        );
    });
    client.start_voting();
    assert_eq!(client.try_vote(&voter, &0), Err(Ok(Error::NoRightToVote)));
}

#[test]
fn vote_tally_overflow_is_rejected() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 30);
    env.as_contract(&client.address, || {
        let mut proposals = storage::get_proposals(&env);
        let mut p = proposals.get(0).unwrap();
        p.vote_count = u64::MAX;
        proposals.set(0, p);
        storage::set_proposals(&env, &proposals);
    });
    client.start_voting();
    assert_eq!(client.try_vote(&voter, &0), Err(Ok(Error::Overflow)));
    assert!(!client.get_voter(&voter).unwrap().voted);
}

// ----------------------------------------------------------------------
// Counting and results
// ----------------------------------------------------------------------

#[test]
fn winner_with_no_votes_is_an_error() {
    let env = Env::default();
    let (client, _) = setup(&env);
    assert_eq!(client.try_winning_proposal(), Err(Ok(Error::NoVotesCast)));
    assert_eq!(client.try_winner_name(), Err(Ok(Error::NoVotesCast)));

    client.start_voting();
    client.end_voting();
    assert_eq!(client.try_winning_proposal(), Err(Ok(Error::NoVotesCast)));
}

#[test]
fn counts_votes_and_picks_winner() {
    let env = Env::default();
    let chair = Address::generate(&env);
    env.mock_all_auths();
    let id = env.register(
        Ballot,
        (chair.clone(), names(&env, &["Alice", "Bob", "Carol"])),
    );
    let client = BallotClient::new(&env, &id);

    let voters: std::vec::Vec<Address> = (0..6)
        .map(|_| registered_voter(&env, &client, 30))
        .collect();
    client.start_voting();
    // Alice 1, Bob 2, Carol 3
    let choices = [0_u32, 1, 1, 2, 2, 2];
    for (v, c) in voters.iter().zip(choices) {
        client.vote(v, &c);
    }

    assert_eq!(client.proposal(&0).vote_count, 1);
    assert_eq!(client.proposal(&1).vote_count, 2);
    assert_eq!(client.proposal(&2).vote_count, 3);
    assert_eq!(client.winning_proposal(), 2);
    assert_eq!(client.winner_name(), String::from_str(&env, "Carol"));

    let total: u64 = client.proposals().iter().map(|p| p.vote_count).sum();
    assert_eq!(total, 6);

    // Results stay readable after the election ends.
    client.end_voting();
    assert_eq!(client.winning_proposal(), 2);
}

#[test]
fn tie_resolves_to_lowest_index() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let a = registered_voter(&env, &client, 30);
    let b = registered_voter(&env, &client, 30);
    client.start_voting();
    client.vote(&a, &1);
    client.vote(&b, &0);

    assert_eq!(client.winning_proposal(), 0);
    assert_eq!(client.winner_name(), String::from_str(&env, "Alice"));
}

#[test]
fn proposal_lookup_out_of_range_fails() {
    let env = Env::default();
    let (client, _) = setup(&env);
    assert_eq!(client.try_proposal(&2), Err(Ok(Error::InvalidProposal)));
}

#[test]
fn get_voter_for_unknown_address_is_none() {
    let env = Env::default();
    let (client, _) = setup(&env);
    assert_eq!(client.get_voter(&Address::generate(&env)), None);
}

// ----------------------------------------------------------------------
// Storage and TTL
// ----------------------------------------------------------------------

#[test]
fn writes_extend_voter_and_instance_ttl() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 30);

    env.as_contract(&client.address, || {
        let voter_ttl = env
            .storage()
            .persistent()
            .get_ttl(&DataKey::Voter(voter.clone()));
        assert!(voter_ttl >= storage::LIFETIME_THRESHOLD);
        assert!(env.storage().instance().get_ttl() >= storage::LIFETIME_THRESHOLD);
    });
}

#[test]
fn election_survives_ledger_advance_within_ttl() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter = registered_voter(&env, &client, 30);
    client.start_voting();

    // Advance ~20 days: within the TTL extended on the last write.
    env.ledger().with_mut(|l| l.sequence_number += 20 * 17_280);
    client.vote(&voter, &0);
    assert_eq!(client.proposal(&0).vote_count, 1);
}

// ----------------------------------------------------------------------
// Ports of the original Foundry tests (test/vote.t.sol)
// ----------------------------------------------------------------------

#[test]
fn foundry_only_chairperson_can_start_voting() {
    let env = Env::default();
    let (client, chair) = setup(&env);
    let voter1 = Address::generate(&env);

    sign_as(&env, &client, &voter1, "start_voting", vec![&env]);
    assert!(client.try_start_voting().is_err());

    sign_as(&env, &client, &chair, "start_voting", vec![&env]);
    client.start_voting();

    env.mock_all_auths();
    assert_eq!(
        client.try_start_voting(),
        Err(Ok(Error::VotingAlreadyStarted))
    );
}

#[test]
fn foundry_give_right_to_vote_and_vote() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter1 = registered_voter(&env, &client, 25);

    assert_eq!(
        client.try_vote(&voter1, &0),
        Err(Ok(Error::VotingNotStarted))
    );
    client.start_voting();
    client.vote(&voter1, &0);
    assert_eq!(client.proposal(&0).vote_count, 1);
    assert_eq!(client.try_vote(&voter1, &1), Err(Ok(Error::AlreadyVoted)));
}

#[test]
fn foundry_underage_voter_cannot_get_right() {
    let env = Env::default();
    let (client, _) = setup(&env);
    assert_eq!(
        client.try_give_right_to_vote(&Address::generate(&env), &17),
        Err(Ok(Error::VoterUnderage))
    );
}

#[test]
fn foundry_get_voter_card() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let voter1 = registered_voter(&env, &client, 30);
    let card = client.get_voter(&voter1).unwrap();
    assert_eq!(card.weight, 1);
    assert!(!card.voted);
    assert_eq!(card.age, 30);
    assert!(card.is_valid);
}
