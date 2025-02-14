// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct G1Commitment {
    /// The X coordinate of the KZG commitment. This is the raw byte representation of the field element.
    #[prost(bytes = "vec", tag = "1")]
    pub x: ::prost::alloc::vec::Vec<u8>,
    /// The Y coordinate of the KZG commitment. This is the raw byte representation of the field element.
    #[prost(bytes = "vec", tag = "2")]
    pub y: ::prost::alloc::vec::Vec<u8>,
}
/// BlobCommitment represents commitment of a specific blob, containing its
/// KZG commitment, degree proof, the actual degree, and data length in number of symbols.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobCommitment {
    #[prost(bytes = "vec", tag = "1")]
    pub commitment: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub length_commitment: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub length_proof: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "4")]
    pub length: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentHeader {
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub bin_index: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub cumulative_payment: ::prost::alloc::vec::Vec<u8>,
}
/// A chunk of a blob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkData {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
