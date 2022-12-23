// SPDX-License-Identifier: MIT

pragma solidity >=0.7.0 <0.9.0;

contract Escrow {

    // Wallet address of the sender
    address payable sender;  

    //Map of deposits to recipients
    mapping(address => uint256) public deposits;

    // The amount of funds to be held in escrow
    uint256 amount;

    constructor () {
        sender = payable(msg.sender);
    }

    modifier onlySender() {
        require(msg.sender == sender);
        _;
    }
    // The release condition for the funds
    function releaseCondition() public view returns (bool) {
    uint256 releaseTimestamp = 1599368400; // September 10, 2021
    return block.timestamp > releaseTimestamp;
}

    // The send can send funds to the contract
    function deposit(address recipient)  public payable {
        require(msg.value > 0, "Deposit Amount must be greater than zero");
        deposits[recipient] += msg.value;
    }

    // The recipient can request the release of the funds
    // if the release condition is met
    function requestRelease(address payable recipient) public payable {
        require(releaseCondition(), "Release condition not met");
        recipient.transfer(deposits[recipient]);
        deposits[recipient] = 0;
    }

    // The payer can cancel the escrow and retrieve their funds
    // if the release condition is not met
    function cancel(address payable recipient) public onlySender {
        require(!releaseCondition(), "Release condition already met");
        payable(sender).transfer(deposits[recipient]);
        deposits[recipient] = 0;
    }

    function getBalance(address recipient) public view returns (uint256) {
        return deposits[recipient];
    }
}
