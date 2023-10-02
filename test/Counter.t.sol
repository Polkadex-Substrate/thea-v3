// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Counter.sol";
import "../src/MMR.sol";
import "../src/Ds.sol";

contract CounterTest is Test, Ds {
    Thea thea;
    function setUp() public {
        uint256[2] memory vrf_public_key = [uint256(0x032c8c31fc9f990c6b55e3865a184a4ce50e09481f2eaeb3e60ec1cea13a6ae6), uint256(0x45)];
        thea = new Thea(vrf_public_key);
    }

    function testSendMessage() public {
        uint[] memory data = new uint[](1);
        data[0] = 1;
        uint64 last_processed_deposit_nonce = 0;
        bytes32 deposit_root = 0x0;
        Withdrawal[] memory withdrawals = new Withdrawal[](1);
        withdrawals[0] = Withdrawal(address(0x0), 1, 1);
        Payload memory payload = Payload(data, last_processed_deposit_nonce, deposit_root, withdrawals);
        // Create Message
        uint64 epoch_id = 1;
        uint64 message_id = 1;
        bytes memory vrf_proof = hex"03e355ca6313bea96a671d660fba40bccedc1b60fea6624f519f09c459e05c3f51c1847e0af767f842a2bc70efe55fce5d516e9a0e9ab6c5acf98ba35c6392a09d9a0ec1c6bdc7a83e5043dbb737bc1393";
        bytes[] memory signatures = new bytes[](1);
        signatures[0] = new bytes(1);
        Message memory message = Message(epoch_id, message_id, vrf_proof, signatures, payload);
        bytes memory hash = thea.hashMessage(message);
        console.logBytes(hash);
        thea.submitMessage(message);
    }
}
