pragma solidity ^0.8.0;
import "openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";

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

    struct NewPendingWithdrawals {
        Withdrawal[] withdrawals; // Withdrawals to process
        bool isBlocked; // Is blocked
        uint256 blockNumber; // Claimed block number
    }


    // Create Enum called Mode which has Relayer or Validators
    enum Mode {Relayer, Validators}

}
