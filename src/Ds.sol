pragma solidity >=0.4.22 <0.9.0;
import "openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";
import "./PolkadexTypes.sol";

contract Ds {
    using SafeERC20 for IERC20;
    struct Deposit {
        uint[] user; // beneficiary
        uint128 asset_id; // Asset id
        uint128 amount; // Amount in 10^12 UNITS
    }

    // Individual withdrawal
//    struct Withdrawal{
//        address user; // recipient address
//        IERC20 asset_id; // Asset id
//        uint256 amount; // Amount
//        bool isBlocked; // Is blocked
//    }
    // Payload that is transmitted by Thea
    struct Payload {
        bytes data; // arbitrary data
        PolkadexTypes.PayloadType payload_type; // Payload type
        // ABI Encoded data type
    }

    // Message created during each
    struct Message {
        uint64 epoch_id; // Epoch number of validators
        uint64 message_id; // message_id number
        bytes vrf_proof; // VRF Proof, used to generate randomness for sampling
        bytes[] signatures; // Signatures from the random sample
        Payload payload; // Payload that is transmitted from Polkadex
    }

//    struct NewPendingWithdrawals {
//        Withdrawal[] withdrawals; // Withdrawals to process
//        uint256 blockNumber; // Claimed block number
//    }

//    struct WithdrawalPayload {
//        address[] recipients; // recipient address
//        IERC20[] tokens; // Asset id
//        uint256[] amount; // Amount
//        bool[] isBlocked; // Is blocked
//        uint256 blockNumber; // Claimed block number
//    }

    struct RotateValidatorsPayload {
        address[] validators; // Validator public keys as address
        uint256 epoch_id; // Epoch number of validators
    }

    struct RotateCouncilPayload {
        address[] councilMembers; // Council members
    }

    struct SwitchControlPayload {
        Mode mode; // Mode of the contract
    }

    struct TheaEvent {
        uint64 palletId;
        uint64 extId;
        uint64 nonceId;
        bytes data;
    }
    // Create Enum called Mode which has Relayer or Validators
    enum Mode {Relayer, Validators}

}
