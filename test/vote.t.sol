// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/vote.sol";

contract BallotTest is Test {
    Ballot ballot;
    address chairperson = address(0xABCD);
    address voter1 = address(0x1234);
    address voter2 = address(0x5678);
    string[] proposals;

    function setUp() public {
        // Setup proposal names
        proposals.push("Alice");
        proposals.push("Bob");

        // Deploy contract as chairperson
        vm.prank(chairperson);
        ballot = new Ballot(proposals);
    }

    function testOnlyChairpersonCanStartVoting() public {
        // Fail if non-chairperson tries to start voting
        vm.prank(voter1);
        vm.expectRevert(Ballot.NotChairperson.selector);
        ballot.startVoting();

        // Chairperson starts voting successfully
        vm.prank(chairperson);
        ballot.startVoting();

        // Starting voting again should fail
        vm.prank(chairperson);
        vm.expectRevert(Ballot.VotingAlreadyStarted.selector);
        ballot.startVoting();
    }

    function testGiveRightToVoteAndVote() public {
        vm.prank(chairperson);
        ballot.giveRightToVote(voter1, 25);

        // Before voting started, voting should fail
        vm.prank(voter1);
        vm.expectRevert(Ballot.VotingNotStarted.selector);
        ballot.vote(0);

        // Start voting
        vm.prank(chairperson);
        ballot.startVoting();

        // voter1 votes for proposal 0 ("Alice")
        vm.prank(voter1);
        ballot.vote(0);

        // Check vote counted
        (, uint voteCount) = ballot.proposals(0);
        assertEq(voteCount, 1);

        // voter1 cannot vote again
        vm.prank(voter1);
        vm.expectRevert(Ballot.AlreadyVoted.selector);
        ballot.vote(1);
    }

    function testUnderageVoterCannotGetRight() public {
        vm.prank(chairperson);
        vm.expectRevert(Ballot.VoterUnderage.selector);
        ballot.giveRightToVote(voter2, 17);
    }

    function testInvalidVoterCannotVote() public {
        vm.prank(chairperson);
        ballot.giveRightToVote(voter2, 25);

        vm.prank(chairperson);
        ballot.invalidateVoter(voter2);

        vm.prank(chairperson);
        ballot.startVoting();

        vm.prank(voter2);
        vm.expectRevert(Ballot.InvalidVoterCard.selector);
        ballot.vote(1);
    }

    function testGetMyVoterCard() public {
        vm.prank(chairperson);
        ballot.giveRightToVote(voter1, 30);

        vm.prank(voter1);
        Ballot.Voter memory voterCard = ballot.getMyVoterCard();
        assertEq(voterCard.weight, 1);
        assertFalse(voterCard.voted);
        assertEq(voterCard.age, 30);
        assertTrue(voterCard.isValid);
    }
}
