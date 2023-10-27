// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Counter.sol";
import "../src/MMR.sol";
import "../src/Ds.sol";

contract CounterTest is Test, Ds {
    Thea thea;
    function setUp() public {
        uint256[2] memory vrf_public_key = [uint256(20149468923017862635785269351026469201343513335253737999994330121872194856517), uint256(45558802482409728232371975206855032011893935284936184167394243449917294149765)];
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
        Withdrawal[] memory withdrawals = new Withdrawal[](40);
        for (uint i = 0; i < 30; i++) {
            withdrawals[i] = Withdrawal(address(0x0), 1, 1);
        }
        Payload memory payload = Payload(data, last_processed_deposit_nonce, deposit_root, withdrawals);
        // Create Message
        uint64 epoch_id = 1;
        uint64 message_id = 1;
        bytes memory vrf_proof = hex"0226fe5dff1eed7fbf7bd994fdd864687acc40e1132d8ddf4672d0553d6da715e4b95241baf0b8049049f3125da44bef9eb6f191e503cd66616857bddf3acc9a148ab40021f9a12faff82d404cc2745abb";
        bytes[] memory signatures = new bytes[](1);
        for (uint i = 0; i < 1; i++) {
            signatures[i] = hex"21fbf0696d5e0aa2ef41a2b4ffb623bcaf070461d61cf7251c74161f82fec3a4370854bc0a34b3ab487c1bc021cd318c734c51ae29374f2beb0e6f2dd49b4bf41c";
        }
        Message memory message = Message(epoch_id, message_id, vrf_proof, signatures, payload);
        bytes memory hash = thea.hashMessage(message);
        console.logBytes(hash);
        console.log("here");
        thea.submitMessage(message);
    }
}
