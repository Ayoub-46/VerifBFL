// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.20;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TokenContract is ERC20 {
    constructor(uint256 initialSupply) ERC20("fltoken", "FLT") {
        _mint(msg.sender, initialSupply);
    }
}
