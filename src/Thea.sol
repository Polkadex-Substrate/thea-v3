// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.4.22 <0.9.0;
pragma experimental ABIEncoderV2;

import "../lib/openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";
import "./PolkadexTypes.sol";
import "../lib/vrf-solidity/contracts/VRF.sol";
import "./MMR.sol";
import "./Ds.sol";

contract TheaV3 is Ds {
    using SafeERC20 for IERC20;

    // Variables
    uint256[2] public vrf_public_key; // VRF Public key
    address public relayer; // Relayer address
    address public etherAddress; // Ether address
    Mode public mode;
    uint256 public blockDelay;
    uint64 public outgoingNonce;
    uint64 public incomingNonce;
    address[] public councilMembers;
    mapping(uint256 => address[]) public validators; // Validator public keys as address grouped by epoch
    mapping(uint256 => bytes) public messages; // Approved messages indexed by message_id
    mapping(uint64 => PolkadexTypes.Withdrawal[]) public pendingWithdrawals; // Pending withdrawals
    mapping(uint128 => address) public assetBook;

    // Events //TODO: Discuss with emmanuel about indexing
    event MessageProcessed(uint64 message_id);
    event WithdrawalClaimed(uint64 messageId, uint64 withdrawal_index);
    event DepositEvent(bytes recipient, uint128 assetId, uint256 amount);
    event DepositEventOb(bytes mainAccount, bytes tradingAccount, uint128 assetId, uint256 amount);
    event ValidatorsRotated(address[] validators, uint256 epoch_id);
    event ModeSwitched(Mode mode);
    event CouncilRotated(address[] councilMembers);
    event TransactionBlocked(uint64 messageId, uint64 withdrawal_index);
    event TheaTransaction(TheaEvent theaEvent);
    event printRelayerAddress(address relayer);
    event Here(uint64 id);
    event HereAddress(address id);
    event Here32(bytes32 id);

    //Add constructor
    constructor(uint256[2] memory _vrf_public_key, address _relayer) public payable {
        vrf_public_key = _vrf_public_key;
        relayer = _relayer;
        etherAddress = address(0x1);
        mode = Mode.Relayer;
        blockDelay = 2;
        outgoingNonce = 0;
        incomingNonce = 0;
        councilMembers = new address[](1);
        councilMembers[0] = msg.sender;
    }

    function setIncomingNonce(uint64 nonce) external {
        //Check if sender is relayer
        require(msg.sender == relayer, "Not a relayer");
        incomingNonce = nonce;
    }

    function setOutgoingNonce(uint64 nonce) external {
        require(msg.sender == relayer, "Not a relayer");
        outgoingNonce = nonce;
    }

    function claimWithdrawal(uint64 nonce, uint64 withdrawalIndex) external payable {
        require(pendingWithdrawals[nonce][withdrawalIndex].isBlocked == false, "Withdrawal is blocked");
        address recipient = pendingWithdrawals[nonce][withdrawalIndex].recipient;
        uint256 amount = pendingWithdrawals[nonce][withdrawalIndex].amount;
        IERC20 token = IERC20(assetBook[pendingWithdrawals[nonce][withdrawalIndex].assetId]);
        if (address(token) == etherAddress) {
            (bool status, ) = recipient.call{value: amount}("");
            require(status, "Ether transfer was not successful");
        } else {
            token.safeTransfer(recipient, amount);
        }
        pendingWithdrawals[nonce][withdrawalIndex].isBlocked = true;
        emit WithdrawalClaimed(nonce, withdrawalIndex);
    }

    function addValidator(uint256 epoch_id, address validator) public {
        validators[epoch_id].push(validator);
    }

    function changeMode(Mode _mode) external {
        mode = _mode;
    }

    // Public Functions
    //TODO; Fix this
//    function claimWithdraw(uint64 messageId, uint64 withdrawal_index) external payable {
//        require(pendingWithdrawals[messageId].isBlocked[withdrawal_index] == false, "Withdrawal is blocked");
//        require(pendingWithdrawals[messageId].blockNumber <= block.number, "Withdrawal is not claimable yet");
//        address recipient = pendingWithdrawals[messageId].recipients[withdrawal_index];
//        uint256 amount = pendingWithdrawals[messageId].amount[withdrawal_index];
//        IERC20 token = pendingWithdrawals[messageId].tokens[withdrawal_index];
//        if (address(token) == etherAddress) {
//            (bool status, ) = recipient.call{value: amount}("");
//            require(status, "Ether transfer was not successful");
//        } else {
//            token.safeTransfer(recipient, amount);
//        }
//        pendingWithdrawals[messageId].isBlocked[withdrawal_index] = true;
//        emit WithdrawalClaimed(messageId, withdrawal_index);
//    }

    // Convert bytes32 to uint128
//    function bytes32ToUint128(bytes32 b) public pure returns (uint128) {
//        return uint128(uint256(b));
//    }

    function addressToUint128(address b) public pure returns (uint128) {
        bytes32 assetId = sha256(abi.encode(2, b)); //FIXME: NetworkId is hard coded here. Make it constant
        return uint128(bytes16(assetId));
    }


    // This is normal the deposit
    function deposit(IERC20 token, uint256 amount, bytes memory recipient) external payable {
        // Directly Deposit to Orderbook
        if (address(token) == etherAddress) {
            (bool status, ) = address(this).call{value: amount}("");
            require(status, "Ether transfer was not successful");
        } else {
            token.transferFrom(msg.sender, address(this), amount);
        }
        uint128 assetId = addressToUint128(address(token));
        assetBook[assetId] = address(token);
        //bytes memory data = abi.encode(token, amount, recipient);
        //TODO: Who will send palletId and ExtId?
        emit DepositEvent(recipient, assetId, amount);
    }

    function depositOb(IERC20 token, uint256 amount, bytes memory mainAccount, bytes memory tradingAccount) external payable {
        if (address(token) == etherAddress) {
            (bool status, ) = address(this).call{value: amount}("");
            require(status, "Ether transfer was not successful");
        } else {
            token.transferFrom(msg.sender,address(this), amount);
        }
        // abi encode token, amount, mainAccount, tradingAccount
        uint128 assetId = addressToUint128(address(token));
        assetBook[assetId] = address(token);
        bytes memory data = abi.encode(token, amount, mainAccount, tradingAccount);
        //TODO: Who will send palletId and ExtId?
        emit DepositEventOb(mainAccount, tradingAccount, assetId, amount);
    }

    function blockTransaction(uint64 nonce, uint64 index) external{
        require(isActiveCouncilMember(msg.sender) == true, "Not a part of Active Council Member");
        pendingWithdrawals[nonce][index].isBlocked = true;
        emit TransactionBlocked(nonce, index);
    }

    function sendMessage(bytes memory message, bytes memory signature) external payable {
        require(mode == Mode.Relayer, "Invalid Mode");
        relayerValidation(message, signature);
        PolkadexTypes.TheaMessage memory theaMessage = PolkadexTypes.decodePayload(message);
        if (theaMessage.payloadType == PolkadexTypes.PayloadType.L1Deposit) {
            processWithdrawals(theaMessage.nonce, theaMessage.withdrawals);
        } else {
            revert("Invalid payload type");
        }
    }

    function sendMessageWithVrfProof(bytes memory message, bytes[] memory signature, uint256 epochId) external payable {
        require(mode == Mode.Validators, "Invalid Mode");
        //validatorValidation(message, signature, epochId);
        PolkadexTypes.TheaMessage memory theaMessage = PolkadexTypes.decodePayload(message);
        if (theaMessage.payloadType == PolkadexTypes.PayloadType.L1Deposit) {
            processWithdrawals(theaMessage.nonce, theaMessage.withdrawals);
        } else {
            revert("Invalid payload type");
        }
    }

    function getPayloadForVrf(bytes memory message) external view returns(bytes memory) {
        bytes32 payload_hash = sha256(message);
        return abi.encodePacked(payload_hash);
    }

    function relayerValidation(bytes memory message, bytes memory signature) internal {
        //TODO: What about nonce?
        bytes32 payload_hash = keccak256(message);
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
        require(v == 27 || v == 28, "Invalid signature version");
        emit printRelayerAddress(relayer);
        require(ecrecover(payload_hash, v, r, s) == relayer, "Relayers signature is not valid");
    }

    function validatorValidation(bytes memory message, bytes[] memory signatures, uint256 epochId) external payable {
        bytes32 payload_hash = sha256(message);
        emit Here32(payload_hash);
        address[] memory _validators = validators[epochId]; //TODO: Test Validator Set Change also
        emit Here(1);
        //emit Here(uint64(signatures.length));
        for (uint256 i=0; i < signatures.length; i++) {
            emit Here(3);
            address validator = _validators[i];
            emit Here(4);
            (uint8 v, bytes32 r, bytes32 s) = splitSignature(signatures[i]);
            emit Here(10);
            // Check v and convert to 27 or 28
            if (v == 0) {
                v = 27;
            } else if (v == 1) {
                v = 28;
            } else {
                revert("Invalid signature version");
            }
            emit Here(10);
            emit HereAddress(validator);
            emit HereAddress(ecrecover(payload_hash, v, r, s));
            ecrecover(payload_hash, v, r, s);
        }
        emit Here(5);
    }

    // Rector is in better way
    function processWithdrawals(uint64 nonce, PolkadexTypes.Withdrawal[] memory withdrawals) internal {
        // TODO: Store claim block no also
        for (uint i = 0; i < withdrawals.length; i++) {
            pendingWithdrawals[nonce].push(withdrawals[i]);
        }
        emit MessageProcessed(nonce);
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

    function isActiveCouncilMember(address _address) public view returns(bool) {
        for (uint i = 0; i < councilMembers.length; i++) {
            if (councilMembers[i] == _address) {
                return true;
            }
        }
        return false;
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