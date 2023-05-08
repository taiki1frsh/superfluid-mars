use osmosis_std_derive::CosmwasmExt;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.superfluidmosmostaking.v1beta1.SuperfluidmOsmoDelegate")]
pub struct SuperfluidmOsmoDelegate {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.superfluidmosmostaking.v1beta1.SuperfluidmOsmoUndelegate")]
pub struct SuperfluidmOsmoUndelegate {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.superfluidmosmostaking.v1beta1.Rebalance")]
pub struct Rebalance {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    // rebalance_rate is intended to be sdk.Dec type
    #[prost(string, tag = "2")]
    pub rebalance_rate: ::prost::alloc::string::String,
}
