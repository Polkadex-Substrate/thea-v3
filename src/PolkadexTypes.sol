// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "./Ds.sol";

library PolkadexTypes {

    struct TheaMessage {
        uint64 blockNumber;
        uint64 nonce;
        uint8 networkId;
        PayloadType payloadType;
        bytes payload;
    }

    struct Withdrawal {
        uint64 id;
        uint128 assetId;
        uint128 amount;
        address recipient;
        bool isBlocked;
        uint64 extra;
    }

    struct PendingWithdrawal {
        uint128[] assetId;
        uint128[] amount;
        address[] recipient;
        bool[] isBlocked;
        uint256 blockNumber; // Claimed block number
    }

    enum PayloadType {ScheduledRotateValidators, ValidatorsRotated, L1Deposit}

    struct ValidatorSet {
        uint64 id;
        address[] validators;
    }

    event DebuggerId(uint32 id);
    event DebuggerIdu64(string str,uint64 id);
    event Success(uint id);
    event DebuggerValidatorCount(uint32 count);
    event debuggerAddress(address addr);
    event debugECDSA(bytes ecdsaPubKey);
    event DDebuggerHash(bytes32 hash);
    event DebuggerBytes(uint64[] b);

    function decodePayloadType(bytes8 payloadType) public pure returns (PayloadType) {
        if (payloadType == hex"00") {
            return PayloadType.ScheduledRotateValidators;
        } else if (payloadType == hex"01") {
            return PayloadType.ValidatorsRotated;
        } else if (payloadType == hex"02") {
            return PayloadType.L1Deposit;
        } else {
            revert("Invalid Payload Type");
        }
    }

    // reverse bytes[] and return reversed bytes
    function reverse(bytes memory input) public pure returns (bytes memory) {
        bytes memory output = new bytes(input.length);
        for (uint i = 0; i < input.length; i++) {
            output[i] = input[input.length - i - 1];
        }
        return output;
    }

    function bytesToUint(bytes memory b) public pure returns (uint256) {
        b = reverse(b);
        uint256 number;
        for (uint256 i = 0; i < b.length; i++) {
            number = number + uint256(uint8(b[i])) * (2 ** (8 * (b.length - (i + 1))));
        }
        return number;
    }

    function bytesToUintWithoutReverse(bytes memory b) public pure returns (uint256) {
        uint256 number;
        for (uint256 i = 0; i < b.length; i++) {
            number = number + uint256(uint8(b[i])) * (2 ** (8 * (b.length - (i + 1))));
        }
        return number;
    }

    function decodeWithdrawal(bytes calldata data) public returns (Withdrawal memory) {
        Withdrawal memory withdrawal;
        uint8 idPrefix = uint8(bytesToUint(data[0:1]));
        require(idPrefix % 4 == 0, "Invalid prefix");
        uint32 idSize = uint32(idPrefix) >> 2;
        require(idSize == 10, "ID size is not right");
        withdrawal.id = uint64(bytesToUint(data[1:11]));
        withdrawal.assetId = uint128(bytesToUint(data[11:27]));
        withdrawal.amount = uint128(bytesToUint(data[27:43]));
        uint8 recipientPrefix = uint8(bytesToUint(data[43:44]));
        require(recipientPrefix % 4 == 0, "Invalid prefix");
        uint32 recipientSize = uint32(recipientPrefix) >> 2;
        emit DebuggerId(recipientSize);
        require(recipientSize == 20, "Recipient size is not right");
        withdrawal.recipient = address(bytes20(data[44:64]));
        emit debuggerAddress(withdrawal.recipient);
        withdrawal.isBlocked = data[64] == hex"01" ? true : false;
        //emit DebuggerId(99);
        return withdrawal;
    }

    function decodeRawWithdrawals(bytes calldata data) public returns (Withdrawal[] memory) {
        uint8 idPrefix = uint8(bytesToUint(data[0:1]));
        //emit DebuggerId(20);
        if (idPrefix % 4 == 0) {
            //emit DebuggerId(21);
            return decodeWithdrawals(data[1:]);
        } else if (idPrefix % 4 == 1) {
            //emit DebuggerId(22);
            return decodeWithdrawals(data[2:]);
        } else if (idPrefix % 4 == 2) {
            //emit DebuggerId(23);
            return decodeWithdrawals(data[4:]);
        }else {
            //emit DebuggerId(24);
            revert("Invalid raw prefix");
        }
    }

    function decodeWithdrawals(bytes calldata data) public returns (Withdrawal[] memory) {
        uint8 prefix = uint8(bytesToUint(data[0:1]));
        require(prefix % 4 == 0, "Invalid withdrawal prefix");
        uint32 noOfWithdrawals = uint32(prefix) >> 2 ;
        emit DebuggerId(noOfWithdrawals);
        Withdrawal[] memory withdrawals = new Withdrawal[](noOfWithdrawals);
        uint offset = 1;
        for (uint i = 0; i < noOfWithdrawals; i++) {
            withdrawals[i] = decodeWithdrawal(data[offset:offset+66]);
            offset = offset + 66;
        }
        return withdrawals;
    }

    function decodePayload(bytes calldata data) public pure returns (TheaMessage memory) {
        TheaMessage memory message;
        message.blockNumber = uint64(bytesToUint(data[0:8]));
        message.nonce = uint64(bytesToUint(data[8:16]));
        message.networkId = uint8(bytesToUint(data[16:17]));
        message.payloadType = decodePayloadType(bytes8(data[17:18]));
        message.payload = data[18:];
        return message;
    }

    function processRawWithdrawalPayload(bytes calldata data) public returns (Withdrawal[] memory) {
        return decodeRawWithdrawals(data[0:]);
    }

    function decodeRawNewValidators(bytes calldata data) public returns (ValidatorSet memory) {
        uint8 idPrefix = uint8(bytesToUint(data[0:1]));
        if (idPrefix % 4 == 0) {
            return processNewValidators(data[1:]);
        } else if (idPrefix % 4 == 1) {
            return processNewValidators(data[2:]);
        } else if (idPrefix % 4 == 2) {
            return processNewValidators(data[4:]);
        }else {
            revert("Invalid raw prefix");
        }
    }

    function processNewValidators(bytes calldata data) public returns (ValidatorSet memory) {
        ValidatorSet memory validatorSet;
        validatorSet.id = uint64(bytesToUint(data[0:8]));
        uint8 prefix = uint8(bytesToUint(data[8:9]));
        uint32 noOfValidators;
        uint offset;
        if (prefix % 4 == 0) {
            noOfValidators = uint32(prefix) >> 2;
            offset = 10;
        } else if (prefix % 4 == 1) {
            offset = 10;
            noOfValidators = get_length(data[offset:]) / 21;
            offset = 11;
            emit DebuggerValidatorCount(noOfValidators);
        } else if (prefix % 4 == 2) {
            offset = 12;
            noOfValidators = get_length(data[offset:]) / 21;
            offset = 13;
            emit DebuggerValidatorCount(noOfValidators);
        } else {
            revert("Invalid validator prefix");
        }
        validatorSet.validators = new address[](noOfValidators);
        for (uint i = 0; i < noOfValidators; i++) {
            bytes memory ecdsaPubKey = data[offset:offset+20];
            require(ecdsaPubKey.length == 20, "Invalid ecdsa pub key");
            address validator = address(bytes20(ecdsaPubKey));
            validatorSet.validators[i] = validator;
            offset = offset + 21;
        }
        return validatorSet;
    }

    function get_length(bytes memory b) public pure returns (uint32) {
        return uint32(b.length);
    }
}
