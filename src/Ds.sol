pragma solidity ^0.8.0;

contract Ds {
    struct Deposit {
        uint[] user; // beneficiary
        uint128 asset_id; // Asset id
        uint128 amount; // Amount in 10^12 UNITS
    }

    // Individual withdrawal
    struct Withdrawal{
        address user; // recipient address
        uint128 asset_id; // Asset id
        uint128 amount; // Amount
    }
    // Payload that is transmitted by Thea
    struct Payload {
        uint[] data; // arbitrary data
        uint64 last_processed_deposit_nonce; // Last processed deposit
        bytes32 deposit_root; // Merkle mountain range root of deposits
        Withdrawal[] withdrawals; // Withdrawals to process
    }

    // Message created during each
    struct Message {
        uint64 epoch_id; // Epoch number of validators
        uint64 message_id; // message_id number
        bytes vrf_proof; // VRF Proof, used to generate randomness for sampling
        bytes[] signatures; // Signatures from the random sample
        Payload payload; // Payload that is transmitted from Polkadex
    }
}
