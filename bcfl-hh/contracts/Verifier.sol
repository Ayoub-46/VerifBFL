// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity 0.8.20;

import "@chainlink/contracts/src/v0.8/ChainlinkClient.sol";
import "@chainlink/contracts/src/v0.8/shared/access/ConfirmedOwner.sol";

contract Verifier is ChainlinkClient, ConfirmedOwner {
    using Chainlink for Chainlink.Request;
    
    bool public verificationResult;
    bytes32 private jobId;
    uint256 private fee;

    event RequestVerification(bytes32 indexed requestId, bool res);

    constructor() ConfirmedOwner(msg.sender) {
        _setChainlinkToken(0xb9A219631Aed55eBC3D998f17C3840B7eC39C0cc);
        _setChainlinkOracle(0x7B8574CF3d0dAb8f88d5cDE6726E00D1f8a7bc46);
        jobId = "d4816b1f5fc64bfda9357ba8b8a2eec9";
        fee = (1 * LINK_DIVISIBILITY) / 10; // 0,1 * 10**18 (Varies by network and job)
    }

    function requestVerification(string memory proof) public returns (bytes32 requestId) {
        Chainlink.Request memory req = _buildChainlinkRequest(
            jobId,
            address(this),
            this.fulfill.selector
        );
        req._add("proof", proof);
        return _sendChainlinkRequest(req, fee);
    }

    /**
     * Receive the response in the form of bool
     */
    function fulfill(
        bytes32 _requestId,
        bool  _res
    ) public recordChainlinkFulfillment(_requestId) {
        emit RequestVerification(_requestId, _res);
        verificationResult = _res;
    }


    /**
     * Allow withdraw of Link tokens from the contract
     */
    function withdrawLink() public onlyOwner {
        LinkTokenInterface link = LinkTokenInterface(_chainlinkTokenAddress());
        require(
            link.transfer(msg.sender, link.balanceOf(address(this))),
            "Unable to transfer"
        );
    }
}