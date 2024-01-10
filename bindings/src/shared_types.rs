///`Message(uint64,uint64,bytes,bytes[],(bytes,uint8))`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Message {
    pub epoch_id: u64,
    pub message_id: u64,
    pub vrf_proof: ::ethers::core::types::Bytes,
    pub signatures: ::std::vec::Vec<::ethers::core::types::Bytes>,
    pub payload: Payload,
}
///`Payload(bytes,uint8)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Payload {
    pub data: ::ethers::core::types::Bytes,
    pub payload_type: u8,
}
