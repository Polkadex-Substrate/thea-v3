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
        address[] memory validators = new address[](200);
        for (uint i = 0; i < 200; i++) {
            validators[i] = address(0x0);
            thea.addValidator(1, validators[i]);
        }
        for (uint i = 0; i < 20; i++) {
            uint[] memory user = new uint[](1);
            user[0] = 1;
            thea.addDeposit(user, 1, 1);
        }
    }

    function testSendMessage() public {
        uint[] memory data = new uint[](1);
        data[0] = 1;
        uint64 last_processed_deposit_nonce = 20;
        bytes32 deposit_root = 0x0;
        Withdrawal[] memory withdrawals = new Withdrawal[](10);
        for (uint i = 0; i < 10; i++) {
            withdrawals[i] = Withdrawal(address(0x0), 1, 1);
        }
        Payload memory payload = Payload(data, last_processed_deposit_nonce, deposit_root, withdrawals);
        // Create Message
        uint64 epoch_id = 1;
        uint64 message_id = 1;
        bytes memory vrf_proof = hex"02f5238dd597cd4fae46649556e417281537fbacc87d2016e0f2c583f96e8d9f490eb47853118d0645e2ca0c44920dbb4d5aded0a976667ee8eceedb77f6cf604ecf1baa008cd5fa51413ec56323591897";
        bytes[] memory signatures = new bytes[](100);
        for (uint i = 0; i < 100; i++) {
            signatures[i] = hex"21fbf0696d5e0aa2ef41a2b4ffb623bcaf070461d61cf7251c74161f82fec3a4370854bc0a34b3ab487c1bc021cd318c734c51ae29374f2beb0e6f2dd49b4bf41c";
        }
        Message memory message = Message(epoch_id, message_id, vrf_proof, signatures, payload);
        bytes memory hash = thea.hashMessage(message);
        console.logBytes(hash);
        thea.submitMessage(message);
    }
}
