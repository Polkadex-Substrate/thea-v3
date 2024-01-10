// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Thea.sol";
import "../src/MMR.sol";
import "../src/Ds.sol";
import "../src/PolkadexTypes.sol";

contract TheaV3Test is Test, Ds {
    TheaV3 thea;

    function setUp() public {
        uint256[2] memory vrf_public_key = [uint256(20149468923017862635785269351026469201343513335253737999994330121872194856517), uint256(45558802482409728232371975206855032011893935284936184167394243449917294149765)];
        address relayer = address(0x4B257d164c8F94c042AC3a64855618C863b8dBEe);
        thea = new TheaV3(vrf_public_key, relayer);
        console.log("Setup is successful");
        // Insert validator into storage
        for (uint i = 0; i < 200; i++) {
            // Convert ecdsa public to to AccountId
            bytes memory ecdsa_pub_key = hex"020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1";
            bytes memory hash = abi.encodePacked(keccak256(ecdsa_pub_key));
            // Generate address using last 20 bytes of hash
            address addr;
            assembly {
                addr := mload(add(hash, 20))
            }
            thea.addValidator(0, addr);
        }
    }

//    function testDecodeWithdrawal() public {
//        bytes memory data = hex"0c280202020202020202020202000000000000000000000000000000030000000000000000000000000000005004040404040404040404040404040404040404040004052802020202020202020202020000000000000000000000000000000300000000000000000000000000000050040404040404040404040404040404040404040400040528020202020202020202020200000000000000000000000000000003000000000000000000000000000000500404040404040404040404040404040404040404000405";
//        PolkadexTypes.decodeWithdrawals(data);
//    }
//
//    function testDecodeRawWithdrawal() public {
//        bytes memory data = hex"0428b36010eb285c154a8cd6af098e0cc9d1d058b55527316ba8788b0a000000000000000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000";
//        PolkadexTypes.decodeRawWithdrawals(data);
//    }

    function testDecodeMessage() public {
        bytes memory data = hex"44000000000000000100000000000000020000000000000000020d010428b36010eb285c154a8cd6af098e0cc9d1d058b55527316ba8788b0a000000000000000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000";
        PolkadexTypes.decodePayload(data);
    }

    //44000000000000000100000000000000020000000000000000020d010428b36010eb285c154a8cd6af098e0cc9d1d058b55527316ba8788b0a000000000000000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000

    function testSendMessage() public {
        bytes memory message = hex"44000000000000000100000000000000020000000000000000020d010428b36010eb285c154a8cd6af098e0cc9d1d058b55527316ba8788b0a000000000000000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000";
        bytes memory signature = hex"8591e3639711056d42221705753f52fbe65e7ab8d5bdcc8533fc74f74fc77edf5f49a760835af86adaa4c5143ca3ff20cc84cd8b3f26068f1caad63ecd065f621c";
        thea.sendMessage(message, signature);
    }

    function testSendMessageWithVrfProof() public {
        thea.changeMode(Mode.Validators);
        bytes memory message = hex"0b000000000000000100000000000000020000000000000000020d010428b36010eb285c154a8cd60c0000000000000000000000000000000a000000000000000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000";
        bytes memory signature = hex"0ec8b09f096d1d7a5e9efc57cd1d6db15ddfbb0025e4c43cb83873c79fabf37f4f46db269760211b14bbb707a153806d7d57162f5aba28578de1f7b85a9032b201";
        bytes memory vrf_proof = hex"0208330248d8de6f1b48cc0d7fa907e7c6507d313754706646da1af62b52524e44253bdc2dd2f32ff3635b2aa989355d10ba74126ebbdae84ede03272543c8df26b479cc476565f56ff1095b34e613a93c";
        bytes[] memory signatures = new bytes[](133);
        for (uint i = 0; i < 133; i++) {
            signatures[i] = signature;
        }
        thea.validatorValidation(message, signatures, 0);
    }
}
