// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.4.22 <0.9.0;
pragma experimental ABIEncoderV2;

import "vrf-solidity/contracts/VRF.sol";
import "./MMR.sol";
import "./Ds.sol";

contract Thea is Ds {
    mapping(uint256 => address[]) public validators; // Validator public keys as address grouped by epoch
    mapping(uint256 => bytes) public messages; // Approved messages indexed by message_id
    uint256[2] public vrf_public_key; // VRF Public key

    uint256 public last_deposit_nonce; // Last processed deposit nonce, it is same as the index of Deposit[]
    mapping(uint256 => Deposit) public deposits; // Map of all deposits
    mapping(uint256 => Withdrawal[]) public claimed_withdrawals; // Claimed withdrawals

    MMR.Tree public deposits_trie;

    // Individual deposit


    event DepositEvent(bytes ecdsa_pubkey, uint128 asset_id, uint128 amount);
    event MessageProcessed(uint64 message_id);
    event WithdrawalProcessed(uint256 message_id, uint256 withdrawal_index, address user, uint128 asset_id, uint128 amount);

    constructor(uint256[2] memory _vrf_public_key) public payable {
        vrf_public_key = _vrf_public_key;
    }

    function update_vrf_key(uint256[2] memory new_vrf_key) public {
        vrf_public_key = new_vrf_key;
        // TODO: this should be called only by the owner of this contract
    }

    function addDeposit(uint[] memory user, uint128 asset_id, uint128 amount) public {
        Deposit memory deposit = Deposit(user, asset_id, amount);
        deposits[0] = deposit;
        emit DepositEvent(abi.encode(user), asset_id, amount);
    }

    function deposit() public {
        // TODO: Allows the user to deposit funds to orderbook
        // 1. Convert the Token contract address to AssetId (u128)
        // 2. Check if the token is allow-listed for exchange
        // 3. Move the token to smart contract account
        // 4. Insert Deposit into storage
        // 5. Emit the event Deposit event
    }

    function withdraw(uint256 message_id, uint256 withdrawal_index) public {
        // TODO: Allows anyone to claim the withdraw
        // 1. Get the withdrawal from storage
        // 2. Make the transfer based on withdrawal details from storage
        // 3. Emit the event WithdrawalProcessed event
    }

    function hashMessage(Message memory message) public pure returns (bytes memory) {
        return abi.encodePacked(sha256(abi.encode(message.payload)));
    }

    function addValidator(uint256 epoch_id, address validator) public {
        validators[epoch_id].push(validator);
    }



    function submitMessage(Message memory message) external {
        // Hash the message
        bytes memory data = abi.encode(message.payload);
        bytes32 payload_hash = sha256(data);

        // Compute the params
        uint256[4] memory proof = VRF.decodeProof(message.vrf_proof);
        (uint256[2] memory _uPoint, uint256[4] memory _vComponents) = VRF.computeFastVerifyParams(vrf_public_key,proof, abi.encodePacked(payload_hash));

        //VRF.fastVerify(vrf_public_key,proof,abi.encodePacked(payload_hash),_uPoint,_vComponents);
        // Verify VRF proof
        require(VRF.verify(vrf_public_key,proof,abi.encodePacked(payload_hash)), "VRF proof is not valid");
        // Compute randomness by hashing the proof
        //bytes32 random_seed = VRF.gammaToHash(proof[0],proof[1]);
        //bytes32 random_seed = sha256(message.vrf_proof);

        // Sample the public keys from stored validator public keys
        address[] memory _validators = validators[message.epoch_id];
        uint256[] memory indices  = generate_random_indices(uint256(0), message.signatures.length,_validators.length);

        // Loop and verify signatures
        //require(indices.length == message.signatures.length);
        for (uint256 i=0; i < message.signatures.length; i++) {
            address validator = _validators[indices[i]];
            (uint8 v, bytes32 r, bytes32 s) = splitSignature(message.signatures[i]);
            ecrecover(payload_hash, v, r, s);
            //require(ecrecover(payload_hash, v, r, s) == validator);
        }
        //Compute Merkle Mountain Range root of Deposits
        for(uint256 i = last_deposit_nonce; i < message.payload.last_processed_deposit_nonce; i++){
            Deposit memory deposit = deposits[i];
            MMR.append(deposits_trie, abi.encode(deposit));
        }
        // Verify root
        MMR.getRoot(deposits_trie);
        //require(MMR.getRoot(deposits_trie) == message.payload.deposit_root, "Here");
        // Store the message
        messages[message.message_id] = abi.encode(message);
        // Update the deposit nonce
        last_deposit_nonce = message.payload.last_processed_deposit_nonce;

        emit MessageProcessed(message.message_id);
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