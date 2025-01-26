// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.20;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

contract AccessManagement is AccessControl {
    bytes32 public constant TRAINER_ROLE = keccak256("TRAINER_ROLE");
    bytes32 public constant AGGREGATOR_ROLE = keccak256("AGGREGATOR_ROLE");
    bytes32 public constant VALIDATOR_ROLE = keccak256("VALIDATOR_ROLE");
    bytes32 public constant PUBLISHER_ROLE = keccak256("PUBLISHER_ROLE");

    constructor() {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    function electTrainer(address _a) public onlyRole(DEFAULT_ADMIN_ROLE) {
        _grantRole(TRAINER_ROLE, _a);
    }

    function electAggregator(address _a) public onlyRole(DEFAULT_ADMIN_ROLE) {
        _grantRole(AGGREGATOR_ROLE, _a);
    }

    function electValidator(address _a) public onlyRole(DEFAULT_ADMIN_ROLE) {
        _grantRole(VALIDATOR_ROLE, _a);
    }

    function removeTrainer(address _a) public onlyRole(DEFAULT_ADMIN_ROLE) {
        _revokeRole(TRAINER_ROLE, _a);
    }

    function removeAggregator(address _a) public onlyRole(DEFAULT_ADMIN_ROLE) {
        _revokeRole(AGGREGATOR_ROLE, _a);
    }

    function removeValidator(address _a) public onlyRole(DEFAULT_ADMIN_ROLE) {
        _revokeRole(VALIDATOR_ROLE, _a);
    }

    function relectPublisher(address _a) public onlyRole(DEFAULT_ADMIN_ROLE) {
        _revokeRole(PUBLISHER_ROLE, _a);
    }

    function removePublisher(address _a) public onlyRole(DEFAULT_ADMIN_ROLE) {
        _revokeRole(PUBLISHER_ROLE, _a);
    }
}
