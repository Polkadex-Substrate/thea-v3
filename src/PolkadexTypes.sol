// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "./Ds.sol";

library PolkadexTypes {

    struct TheaMessage {
        uint64 blockNumber;
        uint64 nonce;
        uint8 networkId;
        uint64 validatorSetId;
        PayloadType payloadType;
        Withdrawal[] withdrawals;
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

    event DebuggerId(uint32 id);
    event DebuggerIdu64(string str,uint64 id);
    event Success(uint id);

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

    function decodeWithdrawal(bytes calldata data) public returns (Withdrawal memory) {
        Withdrawal memory withdrawal;
        uint8 idPrefix = uint8(bytesToUint(data[0:1]));
        require(idPrefix % 4 == 0, "Invalid prefix");
        uint32 idSize = uint32(idPrefix) >> 2;
        emit DebuggerId(idSize);
        require(idSize == 10, "ID size is not right");
        withdrawal.id = uint64(bytesToUint(data[1:11]));
        withdrawal.assetId = uint128(bytesToUint(data[11:27]));
        withdrawal.amount = uint128(bytesToUint(data[27:43]));
        uint8 recipientPrefix = uint8(bytesToUint(data[43:44]));
        require(recipientPrefix % 4 == 0, "Invalid prefix");
        uint32 recipientSize = uint32(recipientPrefix) >> 2;
        emit DebuggerId(recipientSize);
        require(recipientSize == 20, "Recipient size is not right");
        withdrawal.recipient = address(bytes20(reverse(data[44:64]))); //TODO: Check if this is right
        withdrawal.isBlocked = data[64] == hex"01" ? true : false;
        emit DebuggerId(99);
        return withdrawal;
    }

    function decodeRawWithdrawals(bytes calldata data) public returns (Withdrawal[] memory) {
        uint8 idPrefix = uint8(bytesToUint(data[0:1]));
        emit DebuggerId(20);
        if (idPrefix % 4 == 0) {
            emit DebuggerId(21);
            return decodeWithdrawals(data[1:]);
        } else if (idPrefix % 4 == 1) {
            emit DebuggerId(22);
            return decodeWithdrawals(data[2:]);
        } else if (idPrefix % 4 == 2) {
            emit DebuggerId(23);
            return decodeWithdrawals(data[4:]);
        }else {
            emit DebuggerId(24);
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

    function decodePayload(bytes calldata data) public returns (TheaMessage memory) {
        TheaMessage memory message;
        //TODO: Verify the length of payload
        message.blockNumber = uint64(bytesToUint(data[0:8]));
        message.nonce = uint64(bytesToUint(data[8:16]));
        message.networkId = uint8(bytesToUint(data[16:17]));
        emit DebuggerId(message.networkId);
        emit DebuggerId(11);
        message.validatorSetId = uint64(bytesToUint(data[17:25]));
        message.payloadType = decodePayloadType(bytes8(data[25:26]));
        if (message.payloadType == PayloadType.L1Deposit) {
            emit DebuggerId(12);
            message.withdrawals = decodeRawWithdrawals(data[26:]);
            emit DebuggerId(13);
        } else {
            revert("Invalid Payload Type");
        }
        return message;
    }
}
