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
#[proto_message(type_url = "/osmosis.superfluidmosmostaking.v1beta1.SetSuperfluidAssetsProposal")]
pub struct SetSuperfluidAssetsProposal {
}

// #[allow(clippy::derive_partial_eq_without_eq)]
// #[derive(
//     Clone,
//     PartialEq,
//     Eq,
//     ::prost::Message,
//     ::serde::Serialize,
//     ::serde::Deserialize,
//     ::schemars::JsonSchema,
//     CosmwasmExt,
// )]
// #[proto_message(type_url = "/osmosis.superfluid.v1beta1.SetSuperfluidAssetsProposal")]
// pub struct SetSuperfluidAssetsProposal {
//     #[prost(string, tag = "1")]
//     pub title: ::prost::alloc::string::String,
//     #[prost(string, tag = "2")]
//     pub description: ::prost::alloc::string::String,
//     #[prost(message, repeated, tag = "3")]
//     pub assets: ::prost::alloc::vec::Vec<super::SuperfluidAsset>,
// }
