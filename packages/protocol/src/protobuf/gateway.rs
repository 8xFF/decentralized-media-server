// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectRequest {
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub join: ::core::option::Option<super::shared::RoomJoin>,
    #[prost(message, optional, tag = "4")]
    pub features: ::core::option::Option<super::features::Config>,
    #[prost(message, optional, tag = "5")]
    pub tracks: ::core::option::Option<super::shared::Tracks>,
    #[prost(string, tag = "6")]
    pub sdp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectResponse {
    #[prost(string, tag = "1")]
    pub conn_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sdp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteIceRequest {
    #[prost(string, tag = "1")]
    pub conn_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub candidate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteIceResponse {}
