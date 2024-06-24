// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectorRequest {
    #[prost(uint64, tag = "1")]
    pub req_id: u64,
    #[prost(uint64, tag = "2")]
    pub ts: u64,
    #[prost(oneof = "connector_request::Event", tags = "3")]
    pub event: ::core::option::Option<connector_request::Event>,
}
/// Nested message and enum types in `ConnectorRequest`.
pub mod connector_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag = "3")]
        Peer(super::PeerEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectorResponse {
    #[prost(uint64, tag = "1")]
    pub req_id: u64,
    #[prost(oneof = "connector_response::Response", tags = "2, 3")]
    pub response: ::core::option::Option<connector_response::Response>,
}
/// Nested message and enum types in `ConnectorResponse`.
pub mod connector_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Success {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Error {
        #[prost(uint32, tag = "1")]
        pub code: u32,
        #[prost(string, tag = "2")]
        pub message: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "2")]
        Success(Success),
        #[prost(message, tag = "3")]
        Error(Error),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerEvent {
    #[prost(uint64, tag = "1")]
    pub session_id: u64,
    #[prost(
        oneof = "peer_event::Event",
        tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19"
    )]
    pub event: ::core::option::Option<peer_event::Event>,
}
/// Nested message and enum types in `PeerEvent`.
pub mod peer_event {
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteBegin {
        #[prost(string, tag = "1")]
        pub remote_ip: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteSuccess {
        #[prost(uint32, tag = "1")]
        pub after_ms: u32,
        #[prost(uint32, tag = "2")]
        pub dest_node: u32,
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteError {
        #[prost(uint32, tag = "1")]
        pub after_ms: u32,
        #[prost(uint32, optional, tag = "2")]
        pub dest_node: ::core::option::Option<u32>,
        #[prost(enumeration = "route_error::ErrorType", tag = "3")]
        pub error: i32,
    }
    /// Nested message and enum types in `RouteError`.
    pub mod route_error {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum ErrorType {
            PoolEmpty = 0,
            Timeout = 1,
            GatewayError = 2,
            MediaError = 3,
        }
        impl ErrorType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ErrorType::PoolEmpty => "PoolEmpty",
                    ErrorType::Timeout => "Timeout",
                    ErrorType::GatewayError => "GatewayError",
                    ErrorType::MediaError => "MediaError",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "PoolEmpty" => Some(Self::PoolEmpty),
                    "Timeout" => Some(Self::Timeout),
                    "GatewayError" => Some(Self::GatewayError),
                    "MediaError" => Some(Self::MediaError),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Connecting {
        #[prost(string, tag = "1")]
        pub remote_ip: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConnectError {
        #[prost(uint32, tag = "1")]
        pub after_ms: u32,
        #[prost(enumeration = "connect_error::ErrorType", tag = "2")]
        pub error: i32,
    }
    /// Nested message and enum types in `ConnectError`.
    pub mod connect_error {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum ErrorType {
            InvalidSdp = 0,
            Timeout = 1,
        }
        impl ErrorType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ErrorType::InvalidSdp => "InvalidSdp",
                    ErrorType::Timeout => "Timeout",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "InvalidSdp" => Some(Self::InvalidSdp),
                    "Timeout" => Some(Self::Timeout),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Join {
        #[prost(string, tag = "1")]
        pub room: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub peer: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Leave {
        #[prost(string, tag = "1")]
        pub room: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub peer: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Connected {
        #[prost(uint32, tag = "1")]
        pub after_ms: u32,
        #[prost(string, tag = "2")]
        pub remote_ip: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Stats {
        #[prost(uint64, tag = "1")]
        pub sent_bytes: u64,
        #[prost(uint64, tag = "2")]
        pub received_bytes: u64,
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reconnecting {
        #[prost(string, tag = "1")]
        pub remote_ip: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reconnected {
        #[prost(uint32, tag = "1")]
        pub after_ms: u32,
        #[prost(string, tag = "2")]
        pub remote_ip: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Disconnected {
        #[prost(uint32, tag = "1")]
        pub duration_ms: u32,
        #[prost(enumeration = "disconnected::Reason", tag = "2")]
        pub reason: i32,
    }
    /// Nested message and enum types in `Disconnected`.
    pub mod disconnected {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Reason {
            UserAction = 0,
            Timeout = 1,
            NodeShutdown = 2,
            KickByApi = 3,
        }
        impl Reason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Reason::UserAction => "UserAction",
                    Reason::Timeout => "Timeout",
                    Reason::NodeShutdown => "NodeShutdown",
                    Reason::KickByApi => "KickByAPI",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UserAction" => Some(Self::UserAction),
                    "Timeout" => Some(Self::Timeout),
                    "NodeShutdown" => Some(Self::NodeShutdown),
                    "KickByAPI" => Some(Self::KickByApi),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RemoteTrackStarted {
        #[prost(string, tag = "1")]
        pub track: ::prost::alloc::string::String,
        #[prost(enumeration = "super::super::shared::Kind", tag = "2")]
        pub kind: i32,
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RemoteTrackEnded {
        #[prost(string, tag = "1")]
        pub track: ::prost::alloc::string::String,
        #[prost(enumeration = "super::super::shared::Kind", tag = "2")]
        pub kind: i32,
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocalTrackStarted {
        #[prost(string, tag = "1")]
        pub track: ::prost::alloc::string::String,
        #[prost(enumeration = "super::super::shared::Kind", tag = "2")]
        pub kind: i32,
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocalTrackAttach {
        #[prost(string, tag = "1")]
        pub track: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub remote_peer: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub remote_track: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocalTrackDetach {
        #[prost(string, tag = "1")]
        pub track: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub remote_peer: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub remote_track: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocalTrackEnded {
        #[prost(string, tag = "1")]
        pub track: ::prost::alloc::string::String,
        #[prost(enumeration = "super::super::shared::Kind", tag = "2")]
        pub kind: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag = "2")]
        RouteBegin(RouteBegin),
        #[prost(message, tag = "3")]
        RouteSuccess(RouteSuccess),
        #[prost(message, tag = "4")]
        RouteError(RouteError),
        #[prost(message, tag = "5")]
        Connecting(Connecting),
        #[prost(message, tag = "6")]
        Connected(Connected),
        #[prost(message, tag = "7")]
        ConnectError(ConnectError),
        #[prost(message, tag = "8")]
        Stats(Stats),
        #[prost(message, tag = "9")]
        Reconnect(Reconnecting),
        #[prost(message, tag = "10")]
        Reconnected(Reconnected),
        #[prost(message, tag = "11")]
        Disconnected(Disconnected),
        #[prost(message, tag = "12")]
        Join(Join),
        #[prost(message, tag = "13")]
        Leave(Leave),
        #[prost(message, tag = "14")]
        RemoteTrackStarted(RemoteTrackStarted),
        #[prost(message, tag = "15")]
        RemoteTrackEnded(RemoteTrackEnded),
        #[prost(message, tag = "16")]
        LocalTrackStarted(LocalTrackStarted),
        #[prost(message, tag = "17")]
        LocalTrackAttach(LocalTrackAttach),
        #[prost(message, tag = "18")]
        LocalTrackDetach(LocalTrackDetach),
        #[prost(message, tag = "19")]
        LocalTrackEnded(LocalTrackEnded),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParams {
    #[prost(uint32, tag = "1")]
    pub page: u32,
    #[prost(uint32, tag = "2")]
    pub limit: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRooms {
    #[prost(message, repeated, tag = "1")]
    pub rooms: ::prost::alloc::vec::Vec<get_rooms::RoomInfo>,
}
/// Nested message and enum types in `GetRooms`.
pub mod get_rooms {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoomInfo {
        #[prost(int32, tag = "1")]
        pub id: i32,
        #[prost(string, tag = "2")]
        pub room: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPeerParams {
    #[prost(int32, optional, tag = "1")]
    pub room: ::core::option::Option<i32>,
    #[prost(uint32, tag = "2")]
    pub page: u32,
    #[prost(uint32, tag = "3")]
    pub limit: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerSession {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int32, tag = "2")]
    pub peer_id: i32,
    #[prost(string, tag = "3")]
    pub peer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub session: u64,
    #[prost(uint64, tag = "5")]
    pub created_at: u64,
    #[prost(uint64, tag = "6")]
    pub joined_at: u64,
    #[prost(uint64, optional, tag = "7")]
    pub leaved_at: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPeers {
    #[prost(message, repeated, tag = "1")]
    pub peers: ::prost::alloc::vec::Vec<get_peers::PeerInfo>,
}
/// Nested message and enum types in `GetPeers`.
pub mod get_peers {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PeerInfo {
        #[prost(int32, tag = "1")]
        pub id: i32,
        #[prost(int32, tag = "2")]
        pub room_id: i32,
        #[prost(string, tag = "3")]
        pub room: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub peer: ::prost::alloc::string::String,
        #[prost(uint64, tag = "5")]
        pub created_at: u64,
        #[prost(message, repeated, tag = "6")]
        pub sessions: ::prost::alloc::vec::Vec<super::PeerSession>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSessions {
    #[prost(message, repeated, tag = "1")]
    pub sessions: ::prost::alloc::vec::Vec<get_sessions::SessionInfo>,
}
/// Nested message and enum types in `GetSessions`.
pub mod get_sessions {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SessionInfo {
        #[prost(uint64, tag = "1")]
        pub id: u64,
        #[prost(string, optional, tag = "2")]
        pub ip: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "3")]
        pub user_agent: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "4")]
        pub sdk: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint64, tag = "5")]
        pub created_at: u64,
        #[prost(message, repeated, tag = "6")]
        pub peers: ::prost::alloc::vec::Vec<super::PeerSession>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventParams {
    #[prost(uint64, optional, tag = "1")]
    pub session: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub start_ts: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub end_ts: ::core::option::Option<u64>,
    #[prost(uint32, tag = "4")]
    pub page: u32,
    #[prost(uint32, tag = "5")]
    pub limit: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<get_events::EventInfo>,
}
/// Nested message and enum types in `GetEvents`.
pub mod get_events {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EventInfo {
        #[prost(int32, tag = "1")]
        pub id: i32,
        #[prost(uint32, tag = "2")]
        pub node: u32,
        #[prost(uint64, tag = "3")]
        pub node_ts: u64,
        #[prost(uint64, tag = "4")]
        pub session: u64,
        #[prost(uint64, tag = "5")]
        pub created_at: u64,
        #[prost(string, tag = "6")]
        pub event: ::prost::alloc::string::String,
        #[prost(string, optional, tag = "7")]
        pub meta: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[allow(async_fn_in_trait)]
pub trait MediaConnectorServiceHandler<CTX> {
    async fn rooms(&self, ctx: &CTX, req: GetParams) -> Option<GetRooms>;
    async fn peers(&self, ctx: &CTX, req: GetPeerParams) -> Option<GetPeers>;
    async fn sessions(&self, ctx: &CTX, req: GetParams) -> Option<GetSessions>;
    async fn events(&self, ctx: &CTX, req: GetEventParams) -> Option<GetEvents>;
}
pub struct MediaConnectorServiceClient<
    D,
    C: crate::rpc::RpcClient<D, S>,
    S: crate::rpc::RpcStream,
> {
    client: C,
    _tmp: std::marker::PhantomData<(D, S)>,
}
impl<D, C: crate::rpc::RpcClient<D, S>, S: crate::rpc::RpcStream> Clone
for MediaConnectorServiceClient<D, C, S> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            _tmp: Default::default(),
        }
    }
}
impl<
    D,
    C: crate::rpc::RpcClient<D, S>,
    S: crate::rpc::RpcStream,
> MediaConnectorServiceClient<D, C, S> {
    pub fn new(client: C) -> Self {
        Self {
            client,
            _tmp: Default::default(),
        }
    }
    pub async fn rooms(&self, dest: D, req: GetParams) -> Option<GetRooms> {
        use prost::Message;
        let mut stream = self.client.connect(dest, "rooms.service").await?;
        let out_buf = req.encode_to_vec();
        stream.write(&out_buf).await?;
        let in_buf = stream.read().await?;
        GetRooms::decode(in_buf.as_slice()).ok()
    }
    pub async fn peers(&self, dest: D, req: GetPeerParams) -> Option<GetPeers> {
        use prost::Message;
        let mut stream = self.client.connect(dest, "peers.service").await?;
        let out_buf = req.encode_to_vec();
        stream.write(&out_buf).await?;
        let in_buf = stream.read().await?;
        GetPeers::decode(in_buf.as_slice()).ok()
    }
    pub async fn sessions(&self, dest: D, req: GetParams) -> Option<GetSessions> {
        use prost::Message;
        let mut stream = self.client.connect(dest, "sessions.service").await?;
        let out_buf = req.encode_to_vec();
        stream.write(&out_buf).await?;
        let in_buf = stream.read().await?;
        GetSessions::decode(in_buf.as_slice()).ok()
    }
    pub async fn events(&self, dest: D, req: GetEventParams) -> Option<GetEvents> {
        use prost::Message;
        let mut stream = self.client.connect(dest, "events.service").await?;
        let out_buf = req.encode_to_vec();
        stream.write(&out_buf).await?;
        let in_buf = stream.read().await?;
        GetEvents::decode(in_buf.as_slice()).ok()
    }
}
pub struct MediaConnectorServiceServer<
    CTX,
    H: MediaConnectorServiceHandler<CTX>,
    Sr: crate::rpc::RpcServer<S>,
    S: crate::rpc::RpcStream,
> {
    ctx: std::sync::Arc<CTX>,
    handler: std::sync::Arc<H>,
    server: Sr,
    _tmp: std::marker::PhantomData<S>,
}
impl<
    CTX: 'static + Clone,
    H: 'static + MediaConnectorServiceHandler<CTX>,
    Sr: crate::rpc::RpcServer<S>,
    S: 'static + crate::rpc::RpcStream,
> MediaConnectorServiceServer<CTX, H, Sr, S> {
    pub fn new(server: Sr, ctx: CTX, handler: H) -> Self {
        Self {
            ctx: std::sync::Arc::new(ctx),
            handler: std::sync::Arc::new(handler),
            server,
            _tmp: Default::default(),
        }
    }
    pub async fn run(&mut self) {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async move {
                self.run_local().await;
            })
            .await;
    }
    async fn run_local(&mut self) {
        use prost::Message;
        while let Some((domain, mut stream)) = self.server.accept().await {
            let ctx = self.ctx.clone();
            let handler = self.handler.clone();
            match domain.as_str() {
                "rooms.service" => {
                    tokio::task::spawn_local(async move {
                        if let Some(in_buf) = stream.read().await {
                            if let Ok(req) = GetParams::decode(in_buf.as_slice()) {
                                if let Some(res) = handler.rooms(&ctx, req).await {
                                    let out_buf = res.encode_to_vec();
                                    stream.write(&out_buf).await;
                                    stream.close().await;
                                }
                            }
                        }
                    });
                }
                "peers.service" => {
                    tokio::task::spawn_local(async move {
                        if let Some(in_buf) = stream.read().await {
                            if let Ok(req) = GetPeerParams::decode(in_buf.as_slice()) {
                                if let Some(res) = handler.peers(&ctx, req).await {
                                    let out_buf = res.encode_to_vec();
                                    stream.write(&out_buf).await;
                                    stream.close().await;
                                }
                            }
                        }
                    });
                }
                "sessions.service" => {
                    tokio::task::spawn_local(async move {
                        if let Some(in_buf) = stream.read().await {
                            if let Ok(req) = GetParams::decode(in_buf.as_slice()) {
                                if let Some(res) = handler.sessions(&ctx, req).await {
                                    let out_buf = res.encode_to_vec();
                                    stream.write(&out_buf).await;
                                    stream.close().await;
                                }
                            }
                        }
                    });
                }
                "events.service" => {
                    tokio::task::spawn_local(async move {
                        if let Some(in_buf) = stream.read().await {
                            if let Ok(req) = GetEventParams::decode(in_buf.as_slice()) {
                                if let Some(res) = handler.events(&ctx, req).await {
                                    let out_buf = res.encode_to_vec();
                                    stream.write(&out_buf).await;
                                    stream.close().await;
                                }
                            }
                        }
                    });
                }
                _ => {}
            }
        }
    }
}
