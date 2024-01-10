// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "../lib/openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "./PolkadexTypes.sol";
import {Ds} from "./Ds.sol";

contract TheaLatest is Ds {
    using SafeERC20 for IERC20;

    uint64 public outgoingNonce;
    uint64 public incomingNonce;
    address manager;
    mapping(uint256 => address[]) public validators;
    mapping(uint64 => PolkadexTypes.Withdrawal[]) public pendingWithdrawals; // Pending withdrawals
    address public etherAddress;
    uint64[] public validatorsIndexForVerification;
    uint64 public indexSize;
    uint64 public latestValidatorSetId;
    uint64 public nextValidatorSetId;
    mapping(uint128 => address) public assetBook;

    event MessageProcessed(uint64 nonce);
    event WithdrawalClaimed(uint64 messageId, uint64 withdrawal_index);
    event DepositEvent(bytes recipient, uint128 assetId, uint256 amount);
    event TransactionBlocked(uint64 messageId, uint64 withdrawal_index);
    event DepositEventOb(bytes mainAccount, bytes tradingAccount, uint128 assetId, uint256 amount);

    constructor(address _manager, uint _epochId,address[] memory _validators, uint64 _indexSize, uint64[] memory _validatorsIndexForVerification) public payable {
        manager = _manager;
        validators[_epochId] = _validators;
        indexSize = _indexSize;
        validatorsIndexForVerification = _validatorsIndexForVerification;
    }

    function sendMessage(bytes memory message, bytes[] memory signature) external payable {
        PolkadexTypes.TheaMessage memory theaMessage = PolkadexTypes.decodePayload(message);
        validatorValidation(message, signature, theaMessage.validatorSetId);
        if (theaMessage.payloadType == PolkadexTypes.PayloadType.L1Deposit) {
            processWithdrawals(theaMessage.nonce, theaMessage.withdrawals);
        }
        updateValidatorIndex(message, theaMessage.validatorSetId);
        emit MessageProcessed(theaMessage.nonce);
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

    function deposit(IERC20 token, uint256 amount, bytes memory recipient) external payable {
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

    function setIncomingNonce(uint64 nonce) external {
        //Check if sender is relayer
        require(msg.sender == manager, "Not a manager");
        incomingNonce = nonce;
    }

    function setOutgoingNonce(uint64 nonce) external {
        require(msg.sender == manager, "Not a manager");
        outgoingNonce = nonce;
    }

    function blockTransaction(uint64 nonce, uint64 index) external{
        require(msg.sender == manager, "Not a manager");
        pendingWithdrawals[nonce][index].isBlocked = true;
        emit TransactionBlocked(nonce, index);
    }

    function updateValidatorIndex(bytes memory message, uint epochId) private {
        uint randNonce = uint(keccak256(abi.encodePacked(block.timestamp, message))) % 100;
        uint64[] memory validatorIndex = new uint64[](indexSize);
        for (uint i = 0; i < indexSize; i++) {
            randNonce++;
            uint64 randNo = uint64(uint(keccak256(abi.encodePacked(block.timestamp, message, randNonce))));
            validatorIndex[i] = randNo % uint64(validators[epochId].length);
        }
    }

    function validatorValidation(bytes memory message, bytes[] memory signatures, uint256 epochId) private {
        require(signatures.length == validatorsIndexForVerification.length, "Invalid signature length");
        for (uint i = 0; i < validatorsIndexForVerification.length; i++) {
            address validator = validators[epochId][validatorsIndexForVerification[i]];
            bytes32 hash = keccak256(abi.encodePacked(message));
            (uint8 v, bytes32 r, bytes32 s) = splitSignature(signatures[i]);
            address signer = ecrecover(hash, v, r, s);
            require(signer == validator, "Invalid signature");
        }
    }

    function processWithdrawals(uint64 nonce, PolkadexTypes.Withdrawal[] memory withdrawals) internal {
        // TODO: Store claim block no also
        for (uint i = 0; i < withdrawals.length; i++) {
            pendingWithdrawals[nonce].push(withdrawals[i]);
        }
    }

    function addressToUint128(address b) public pure returns (uint128) {
        bytes32 assetId = sha256(abi.encode(2, b)); //FIXME: NetworkId is hard coded here. Make it constant
        return uint128(bytes16(assetId));
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
