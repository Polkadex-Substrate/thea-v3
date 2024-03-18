// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "../lib/openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";
import "../lib/openzeppelin-contracts-upgradeable/contracts/proxy/utils/UUPSUpgradeable.sol";
import "../lib/openzeppelin-contracts-upgradeable/contracts/proxy/utils/Initializable.sol";
import "../lib/openzeppelin-contracts/contracts/utils/ReentrancyGuard.sol";
import "../lib/secp256k1-solidity/contracts/SECP256K1.sol";
import "./PolkadexTypes.sol";
import {Ds} from "./Ds.sol";

/**
    @title Manages EVM side of things for Thea Protocol.
    @author Polkadex OU.
    @notice This contract is intended to be used with Thea Protocol.
 */
contract TheaLatest is Ds, Initializable, UUPSUpgradeable, ReentrancyGuard {
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
    uint8 public networkId;
    mapping(uint128 => address) public assetBook;

    modifier onlyManager {
        require(msg.sender == manager, "Not a manager");
        _;
    }

    event MessageProcessed(uint64 nonce);
    event WithdrawalClaimed(uint64 messageId, uint64 withdrawal_index);
    event DepositEvent(bytes recipient, uint128 assetId, uint256 amount);
    event TransactionBlocked(uint64 messageId, uint64 withdrawal_index);
    event DepositEventOb(bytes mainAccount, bytes tradingAccount, uint128 assetId, uint256 amount);
    event debuggerBytes(bytes address1);

    /**
        @notice Initialize the contract with initial values.
        @param _manager Address of the manager.
        @param _validatorSetId Validator Set Id.
        @param _validators Array of validator addresses.
        @param _indexSize Size of the index.
        @param _validatorsIndexForVerification Array of validator indexes for verification.
     */
    function initialize(address _manager, uint _validatorSetId, address[] memory _validators, uint64 _indexSize, uint64[] memory _validatorsIndexForVerification, uint64 _outgoingNonce, uint64 _incomingNonce, uint8 _networkId) public initializer {
        manager = _manager;
        for (uint i = 0; i < _validators.length; i++) {
            validators[_validatorSetId].push(_validators[i]);
        }
        indexSize = _indexSize;
        validatorsIndexForVerification = _validatorsIndexForVerification;
        latestValidatorSetId = uint64(_validatorSetId);
        outgoingNonce = _outgoingNonce;
        incomingNonce = _incomingNonce;
        networkId = _networkId;
    }


    function _authorizeUpgrade(address) internal override onlyManager {}

    /**
        @notice Send message from Polkadex to Ethereum.
        @param message Message payload.
        @param signature Signature of the message.
    */
    function sendMessage(bytes memory message, bytes[] memory signature, uint64[] memory signaturesIndex) external payable {
        //TODO: Change signaturesIndex to uint256 and use bits to find the validator index-v3
        updateValidatorIndex(message, latestValidatorSetId, signaturesIndex);
        validatorValidation(message, signature, latestValidatorSetId);
        PolkadexTypes.TheaMessage memory theaMessage = PolkadexTypes.decodePayload(message);
        require(theaMessage.networkId == networkId, "Network Id is not valid");
        verifyAndUpdateNonce(theaMessage.nonce);
        if (theaMessage.payloadType == PolkadexTypes.PayloadType.L1Deposit) {
            PolkadexTypes.Withdrawal[] memory withdrawal = PolkadexTypes.processRawWithdrawalPayload(theaMessage.payload);
            processWithdrawals(theaMessage.nonce, withdrawal);
        } else if (theaMessage.payloadType == PolkadexTypes.PayloadType.ScheduledRotateValidators) {
            PolkadexTypes.ValidatorSet memory validatorSet = PolkadexTypes.decodeRawNewValidators(theaMessage.payload);
            validators[validatorSet.id] = validatorSet.validators;
            nextValidatorSetId = validatorSet.id;
        } else if (theaMessage.payloadType == PolkadexTypes.PayloadType.ValidatorsRotated) {
            latestValidatorSetId = nextValidatorSetId;
        }
        //updateValidatorIndex(message, latestValidatorSetId);
        emit MessageProcessed(theaMessage.nonce);
    }

    /**
        @notice Claim the withdrawal.
        @param nonce Nonce of the message.
        @param withdrawalIndex Index of the withdrawal.
    */
    function claimWithdrawal(uint64 nonce, uint64 withdrawalIndex) external payable {
        require(pendingWithdrawals[nonce][withdrawalIndex].isBlocked == false, "Withdrawal is blocked");
        address recipient = pendingWithdrawals[nonce][withdrawalIndex].recipient;
        uint256 amount = pendingWithdrawals[nonce][withdrawalIndex].amount;
        IERC20 token = IERC20(assetBook[pendingWithdrawals[nonce][withdrawalIndex].assetId]);
        if (address(token) == etherAddress) {
            (bool status,) = recipient.call{value: amount}("");
            require(status, "Ether transfer was not successful");
        } else {
            token.safeTransfer(recipient, amount);
        }
        pendingWithdrawals[nonce][withdrawalIndex].isBlocked = true;
        emit WithdrawalClaimed(nonce, withdrawalIndex);
    }

    /**
        @notice Deposit the funds to the contract.
        @param token Token address.
        @param amount Amount to deposit.
        @param recipient Recipient address.
    */
    function deposit(IERC20 token, uint256 amount, bytes memory recipient) external payable {
        if (address(token) == etherAddress) {
            (bool status,) = address(this).call{value: amount}("");
            require(status, "Ether transfer was not successful");
        } else {
            token.transferFrom(msg.sender, address(this), amount);
        }
        uint128 assetId = addressToUint128(address(token));
        assetBook[assetId] = address(token);
        //bytes memory data = abi.encode(token, amount, recipient);
        //TODO: Who will send palletId and ExtId?
        //TODO: Should have uniq id
        emit DepositEvent(recipient, assetId, amount);
    }

    /**
        @notice Deposit the funds to the contract.
        @param token Token address.
        @param amount Amount to deposit.
        @param mainAccount Main account address.
        @param tradingAccount Trading account address.
    */
    function depositOb(IERC20 token, uint256 amount, bytes memory mainAccount, bytes memory tradingAccount) external payable {
        if (address(token) == etherAddress) {
            (bool status,) = address(this).call{value: amount}("");
            require(status, "Ether transfer was not successful");
        } else {
            token.transferFrom(msg.sender, address(this), amount);
        }
        // abi encode token, amount, mainAccount, tradingAccount
        uint128 assetId = addressToUint128(address(token));
        assetBook[assetId] = address(token);
        bytes memory data = abi.encode(token, amount, mainAccount, tradingAccount);
        //TODO: Who will send palletId and ExtId?
        emit DepositEventOb(mainAccount, tradingAccount, assetId, amount);
    }

    /**
        @notice Set the incoming nonce.
        @param nonce Nonce to set.
    */
    function setIncomingNonce(uint64 nonce) external {
        //Check if sender is relayer
        require(msg.sender == manager, "Not a manager");
        incomingNonce = nonce;
    }

    /**
        @notice Set the outgoing nonce.
        @param nonce Nonce to set.
    */
    function setOutgoingNonce(uint64 nonce) external {
        require(msg.sender == manager, "Not a manager");
        outgoingNonce = nonce;
    }

    /**
        @notice Block transaction.
        @param nonce Nonce of the message.
        @param index Index of the transaction.
    */
    function blockTransaction(uint64 nonce, uint64 index) external {
        require(msg.sender == manager, "Not a manager");
        pendingWithdrawals[nonce][index].isBlocked = true;
        emit TransactionBlocked(nonce, index);
    }

    //TODO: Optimize this function
    function getValidatorIndex(bytes memory message, uint validatorSetId, uint64[] memory index) public returns (uint64[] memory) {
           uint randNonce = uint(keccak256(abi.encodePacked(message))) % 100;
            uint64[] memory validatorIndex = new uint64[](indexSize);
            for (uint i = 0; i < indexSize;) {
                randNonce++;
                uint64 randNo = uint64(uint(keccak256(abi.encodePacked(message, randNonce))));
                uint randomValidatorIndex = randNo % uint64(validators[validatorSetId].length);
                for (uint j = 0; j < index.length; j++) {
                    if (randomValidatorIndex == index[j]) {
                        validatorIndex[i] = uint64(randomValidatorIndex);
                        i++;
                    }
                }
            }
            return validatorIndex;
    }

    /**
        @notice Update the validator index.
        @param message Message payload.
        @param validatorSetId Epoch id of the validator set.
    */
    function updateValidatorIndex(bytes memory message, uint validatorSetId, uint64[] memory index) private {
        uint64[] memory validatorIndex = getValidatorIndex(message, validatorSetId, index);
        emit PolkadexTypes.DebuggerBytes(validatorIndex);
        //FIXME: Update the validator_index
        validatorsIndexForVerification = validatorIndex;
    }

    /**
        @notice Validate the message.
        @param message Message payload.
        @param signatures Array of signatures.
        @param epochId Epoch id of the validator set.
    */
    function validatorValidation(bytes memory message, bytes[] memory signatures, uint256 epochId) private {
        require(signatures.length == validatorsIndexForVerification.length, "Invalid signature length");
        for (uint i = 0; i < validatorsIndexForVerification.length; i++) {
            address validator = validators[epochId][validatorsIndexForVerification[i]];
            bytes32 hash = sha256(message);
            emit PolkadexTypes.DDebuggerHash(bytes32(hash));
            (uint8 v, bytes32 r, bytes32 s) = splitSignature(signatures[i]);
            if (v == 0) {
                v = 27;
            } else if (v == 1) {
                v = 28;
            } else {
                revert("Invalid Signature Version");
            }
            address actualAddress = ecrecover(hash, v, r, s);
            require(actualAddress == validator, "Invalid signature");
        }
    }

    function abcValidatorValidation(bytes memory message, bytes memory signature, address validator) public {
        bytes32 hash = sha256(message);
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
        if (v == 0) {
            v = 27;
        } else if (v == 1) {
            v = 28;
        } else {
            revert("Invalid signature version");
        }
        //(uint256 x, uint256 y) = SECP256K1.recover(uint256(hash), v - 27, uint256(r), uint256(s));
        address a = ecrecover(hash, v, r, s);
        //bytes memory signer =  abi.encodePacked(x, y);

        bytes memory validator = hex"1234";
        //emit debuggerBytes(bytes(a));
        //require(false);
        //require(signer == validator, "Invalid signature");
    }

    function verifyAndUpdateNonce(uint64 actualNonce) internal {
        require(actualNonce == incomingNonce, "Nonce is invalid");
        incomingNonce += 1;
    }

    function bytesToUint256(bytes calldata xCompressedKey) public returns (uint) {
        uint x = PolkadexTypes.bytesToUintWithoutReverse(xCompressedKey[1:33]);
        return x;
    }

    function decompressPublicKey(bytes memory compressedKey) public returns (bytes memory) {
        require(compressedKey.length == 33, "Invalid compressed key length");

        uint8 prefix = uint8(compressedKey[0]);
        bool isEven = (prefix % 2 == 0);
        uint8 yParity = isEven ? 0x02 : 0x03;
        uint x = this.bytesToUint256(compressedKey);

        //uint256 x = uint256(bytes32(compressedKeyE << 1));

        uint256 p = 2**256 - 2**32 - 977;
        uint256 a = 0;
        uint256 b = 7;
        uint256 ySquared = (x**3 + a*x + b) % p;
        uint256 y = modExp(ySquared, (p+1)/4, p);

        require(y**2 % p == ySquared, "Invalid y coordinate");

        bytes memory uncompressedKey = new bytes(65);
        uncompressedKey[0] = 0x04;
        assembly {
            mstore(add(uncompressedKey, 0x20), x)
            mstore(add(uncompressedKey, 0x40), y)
        }

        return uncompressedKey;
    }

    function modExp(uint256 base, uint256 exponent, uint256 modulus) internal pure returns (uint256) {
        if (modulus == 1) return 0;
        uint256 result = 1;
        base = base % modulus;
        while (exponent > 0) {
            if (exponent % 2 == 1) {
                result = (result * base) % modulus;
            }
            exponent = exponent >> 1;
            base = (base * base) % modulus;
        }
        return result;
    }

    /**
        @notice Process the withdrawals.
        @param nonce Nonce of the message.
        @param withdrawals Array of withdrawals.
    */
    function processWithdrawals(uint64 nonce, PolkadexTypes.Withdrawal[] memory withdrawals) internal {
        // TODO: Store claim block no also
        //TODO: This has to be optimised
        for (uint i = 0; i < withdrawals.length; i++) {
            pendingWithdrawals[nonce].push(withdrawals[i]);
        }
    }

    /**
        @notice Convert address to uint128.
        @param b Address to convert.
    */
    function addressToUint128(address b) public pure returns (uint128) {
        bytes32 assetId = sha256(abi.encode(networkId, b)); //FIXME: NetworkId is hard coded here. Make it constant
        return uint128(bytes16(assetId));
    }

    /**
        @notice Split the signature.
        @param sig Signature to split.
    */
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
