// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.4.22 <0.9.0;
pragma experimental ABIEncoderV2;

import "vrf-solidity/contracts/VRF.sol";
import "./MMR.sol";
import "./Ds.sol";

contract TheaV3 is Ds {

    // Variables
    uint256[2] public vrf_public_key; // VRF Public key
    address public relayer; // Relayer address
    Mode public mode;
    uint256 public blockDelay;
    mapping(uint256 => address[]) public validators; // Validator public keys as address grouped by epoch
    mapping(uint256 => bytes) public messages; // Approved messages indexed by message_id
    mapping(uint64 => NewPendingWithdrawals) public pendingWithdrawals; // Pending withdrawals

    // Events
    event MessageProcessed(uint64 message_id);

    // Public Functions

    function claimWithdraw(uint64 messageId, uint256 withdrawal_index) external {
        require(pendingWithdrawals[messageId].isBlocked == false, "Withdrawal is blocked");
        require(pendingWithdrawals[messageId].blockNumber <= block.number, "Withdrawal is not claimable yet");
        Withdrawal memory withdrawal = pendingWithdrawals[messageId].withdrawals[withdrawal_index];
        address recipient = withdrawal.user;
        uint amount = withdrawal.amount;

    }

    function sendMessage(Message memory message) external payable {
        if (mode == Mode.Relayer) {
            relayerValidation(message);
        } else {
            validatorValidation(message);
        }
    }

    function relayerValidation(Message memory message) internal {
        //TODO: What about nonce?
        bytes memory data = abi.encode(message.payload);
        bytes32 payload_hash = sha256(data);
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(message.signatures[0]);
        require(ecrecover(payload_hash, v, r, s) == relayer);
        processWithdrawals(message.message_id, message.payload.withdrawals);
    }

    function validatorValidation(Message memory message) internal {
        //TODO: What about nonce?
        bytes memory data = abi.encode(message.payload);
        bytes32 payload_hash = sha256(data);
        uint256[4] memory proof = VRF.decodeProof(message.vrf_proof);
        require(VRF.verify(vrf_public_key,proof,abi.encodePacked(payload_hash)), "VRF proof is not valid");
        //bytes32 random_seed = VRF.gammaToHash(proof[0],proof[1]); //TODO: Should iy be removed
        bytes32 random_seed = sha256(message.vrf_proof);
        address[] memory _validators = validators[message.epoch_id];
        uint256[] memory indices  = generate_random_indices(uint256(0), message.signatures.length,_validators.length);
        for (uint256 i=0; i < message.signatures.length; i++) {
            address validator = _validators[indices[i]];
            (uint8 v, bytes32 r, bytes32 s) = splitSignature(message.signatures[i]);
            require(ecrecover(payload_hash, v, r, s) == validator);
        }
        processWithdrawals(message.message_id, message.payload.withdrawals);
    }

    function processWithdrawals(uint64 messageId, Withdrawal[] memory withdrawals) internal {
        uint blockDelayForGiveBlockNo = this.getClaimableBlock();
        NewPendingWithdrawals memory pendingWithdrawal = NewPendingWithdrawals(withdrawals, false, blockDelayForGiveBlockNo);
        //pendingWithdrawals[messageId] = pendingWithdrawal; //FIXME: Not working
        emit MessageProcessed(messageId);
    }


    // Helper Functions

    function getClaimableBlock() public view returns(uint) {
        uint blockDelayForGiveBlockNo = block.number + blockDelay;
        return blockDelayForGiveBlockNo;
    }

    function generate_random_indices(uint256 seed, uint256 length, uint256 total_length) internal pure returns (uint256[] memory) {
        // Linear Congruential Generator (LCG) algorithm seed with 'seed'.
        uint256[] memory indices;
        indices = new uint256[](length);
        for(uint256 i; i<length;i++){
            seed = seed % total_length;
            indices[i] = seed;
        }

        return indices;
    }

    function splitSignature(bytes memory sig)
    internal
    pure
    returns (uint8, bytes32, bytes32)
    {
        require(sig.length == 65);

        bytes32 r;
        bytes32 s;
        uint8 v;

        assembly {
        // first 32 bytes, after the length prefix
            r := mload(add(sig, 32))
        // second 32 bytes
            s := mload(add(sig, 64))
        // final byte (first byte of the next 32 bytes)
            v := byte(0, mload(add(sig, 96)))
        }

        return (v, r, s);
    }
}