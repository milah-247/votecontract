// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Ballot {
    error AlreadyVoted();
    error NoRightToVote();
    error VoterUnderage();
    error InvalidVoterCard();
    error NotChairperson();
    error VotingNotStarted();
    error VotingAlreadyStarted();

    struct Voter {
        uint weight;
        bool voted;
        address delegate;
        uint vote;
        bool isValid;
        uint age; // Age of the voter
    }

    struct Proposal {
        string name;
        uint voteCount;
    }

    address public chairperson;
    uint public votingStartTime;
    bool public votingStarted;

    mapping(address => Voter) public voters;

    Proposal[] public proposals;

    event VoteCast(address indexed voter, uint proposal);

    constructor(string[] memory proposalNames) {
        chairperson = msg.sender;
        voters[chairperson] = Voter({weight: 1, voted: false, delegate: address(0), vote: 0, isValid: true, age: 18});

        for (uint i = 0; i < proposalNames.length; i++) {
            proposals.push(Proposal({name: proposalNames[i], voteCount: 0}));
        }
    }

    function startVoting() external {
        if (msg.sender != chairperson) revert NotChairperson();
        if (votingStarted) revert VotingAlreadyStarted();

        votingStartTime = block.timestamp;
        votingStarted = true;
    }

    function giveRightToVote(address voter, uint age) external {
        if (msg.sender != chairperson) revert NotChairperson();
        if (voters[voter].voted) revert AlreadyVoted();
        if (voters[voter].weight != 0) revert NoRightToVote();
        if (age < 18) revert VoterUnderage();

        voters[voter] = Voter({weight: 1, voted: false, delegate: address(0), vote: 0, isValid: true, age: age});
    }

    function invalidateVoter(address voter) external {
        if (msg.sender != chairperson) revert NotChairperson();
        voters[voter].isValid = false;
    }

    function vote(uint proposal) external {
        if (!votingStarted) revert VotingNotStarted();

        // The voter's card is identified by msg.sender — the user voting
        Voter storage sender = voters[msg.sender];
        if (sender.weight == 0) revert NoRightToVote();
        if (sender.voted) revert AlreadyVoted();
        if (!sender.isValid) revert InvalidVoterCard();
        if (sender.age < 18) revert VoterUnderage();

        sender.voted = true;
        sender.vote = proposal;

        proposals[proposal].voteCount += sender.weight;

        emit VoteCast(msg.sender, proposal);
    }

    function getMyVoterCard() external view returns (Voter memory) {
        return voters[msg.sender];
    }

    function winningProposal() public view returns (uint winningProposal_) {
        uint winningVoteCount = 0;
        for (uint p = 0; p < proposals.length; p++) {
            if (proposals[p].voteCount > winningVoteCount) {
                winningVoteCount = proposals[p].voteCount;
                winningProposal_ = p;
            }
        }
    }

    function winnerName() external view returns (string memory winnerName_) {
        winnerName_ = proposals[winningProposal()].name;
    }
}
