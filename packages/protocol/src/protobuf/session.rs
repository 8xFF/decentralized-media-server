// This file is @generated by prost-build.
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomJoin {
    #[prost(string, tag = "1")]
    pub room: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub peer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub publish: ::core::option::Option<super::shared::RoomInfoPublish>,
    #[prost(message, optional, tag = "4")]
    pub subscribe: ::core::option::Option<super::shared::RoomInfoSubscribe>,
    #[prost(message, optional, tag = "5")]
    pub features: ::core::option::Option<super::features::Config>,
    #[prost(string, optional, tag = "6")]
    pub metadata: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(uint32, tag = "1")]
    pub req_id: u32,
    #[prost(oneof = "request::Request", tags = "2, 3, 4, 5, 6, 7")]
    pub request: ::core::option::Option<request::Request>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Session {
        #[prost(oneof = "session::Request", tags = "1, 2, 3, 4")]
        pub request: ::core::option::Option<session::Request>,
    }
    /// Nested message and enum types in `Session`.
    pub mod session {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Join {
            #[prost(message, optional, tag = "1")]
            pub info: ::core::option::Option<super::super::RoomJoin>,
            #[prost(string, tag = "2")]
            pub token: ::prost::alloc::string::String,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Leave {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct UpdateSdp {
            #[prost(message, optional, tag = "1")]
            pub tracks: ::core::option::Option<super::super::super::shared::Tracks>,
            #[prost(string, tag = "2")]
            pub sdp: ::prost::alloc::string::String,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Disconnect {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Request {
            #[prost(message, tag = "1")]
            Join(Join),
            #[prost(message, tag = "2")]
            Leave(Leave),
            #[prost(message, tag = "3")]
            Sdp(UpdateSdp),
            #[prost(message, tag = "4")]
            Disconnect(Disconnect),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Room {
        #[prost(oneof = "room::Request", tags = "1, 2")]
        pub request: ::core::option::Option<room::Request>,
    }
    /// Nested message and enum types in `Room`.
    pub mod room {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SubscribePeer {
            #[prost(string, tag = "1")]
            pub peer: ::prost::alloc::string::String,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct UnsubscribePeer {
            #[prost(string, tag = "1")]
            pub peer: ::prost::alloc::string::String,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Request {
            #[prost(message, tag = "1")]
            Subscribe(SubscribePeer),
            #[prost(message, tag = "2")]
            Unsubscribe(UnsubscribePeer),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageChannel {
        #[prost(string, tag = "1")]
        pub label: ::prost::alloc::string::String,
        #[prost(oneof = "message_channel::Request", tags = "2, 3, 4, 5, 6")]
        pub request: ::core::option::Option<message_channel::Request>,
    }
    /// Nested message and enum types in `MessageChannel`.
    pub mod message_channel {
        /// TODO: Add Feedback
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct StartPublish {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct StopPublish {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Subscribe {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Unsubscribe {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Publish {
            #[prost(bytes = "vec", tag = "1")]
            pub data: ::prost::alloc::vec::Vec<u8>,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Request {
            #[prost(message, tag = "2")]
            StartPub(StartPublish),
            #[prost(message, tag = "3")]
            StopPub(StopPublish),
            #[prost(message, tag = "4")]
            Sub(Subscribe),
            #[prost(message, tag = "5")]
            Unsub(Unsubscribe),
            #[prost(message, tag = "6")]
            Pub(Publish),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Sender {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof = "sender::Request", tags = "2, 3, 4")]
        pub request: ::core::option::Option<sender::Request>,
    }
    /// Nested message and enum types in `Sender`.
    pub mod sender {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Attach {
            #[prost(message, optional, tag = "1")]
            pub source: ::core::option::Option<
                super::super::super::shared::sender::Source,
            >,
            #[prost(message, optional, tag = "2")]
            pub config: ::core::option::Option<
                super::super::super::shared::sender::Config,
            >,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Detach {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Request {
            #[prost(message, tag = "2")]
            Attach(Attach),
            #[prost(message, tag = "3")]
            Detach(Detach),
            #[prost(message, tag = "4")]
            Config(super::super::super::shared::sender::Config),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Receiver {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof = "receiver::Request", tags = "2, 3, 4")]
        pub request: ::core::option::Option<receiver::Request>,
    }
    /// Nested message and enum types in `Receiver`.
    pub mod receiver {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Attach {
            #[prost(message, optional, tag = "1")]
            pub source: ::core::option::Option<
                super::super::super::shared::receiver::Source,
            >,
            #[prost(message, optional, tag = "2")]
            pub config: ::core::option::Option<
                super::super::super::shared::receiver::Config,
            >,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Detach {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Request {
            #[prost(message, tag = "2")]
            Attach(Attach),
            #[prost(message, tag = "3")]
            Detach(Detach),
            #[prost(message, tag = "4")]
            Config(super::super::super::shared::receiver::Config),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "2")]
        Session(Session),
        #[prost(message, tag = "3")]
        Room(Room),
        #[prost(message, tag = "4")]
        Sender(Sender),
        #[prost(message, tag = "5")]
        Receiver(Receiver),
        #[prost(message, tag = "6")]
        Features(super::super::features::Request),
        #[prost(message, tag = "7")]
        MessageChannel(MessageChannel),
    }
}
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(uint32, tag = "1")]
    pub req_id: u32,
    #[prost(oneof = "response::Response", tags = "2, 3, 4, 5, 6, 7, 8")]
    pub response: ::core::option::Option<response::Response>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Session {
        #[prost(oneof = "session::Response", tags = "1, 2, 3, 4")]
        pub response: ::core::option::Option<session::Response>,
    }
    /// Nested message and enum types in `Session`.
    pub mod session {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Join {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Leave {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct UpdateSdp {
            #[prost(string, tag = "1")]
            pub sdp: ::prost::alloc::string::String,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Disconnect {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Response {
            #[prost(message, tag = "1")]
            Join(Join),
            #[prost(message, tag = "2")]
            Leave(Leave),
            #[prost(message, tag = "3")]
            Sdp(UpdateSdp),
            #[prost(message, tag = "4")]
            Disconnect(Disconnect),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Room {
        #[prost(oneof = "room::Response", tags = "1, 2")]
        pub response: ::core::option::Option<room::Response>,
    }
    /// Nested message and enum types in `Room`.
    pub mod room {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct SubscribePeer {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct UnsubscribePeer {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
        pub enum Response {
            #[prost(message, tag = "1")]
            Subscribe(SubscribePeer),
            #[prost(message, tag = "2")]
            Unsubscribe(UnsubscribePeer),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageChannel {
        #[prost(string, tag = "1")]
        pub label: ::prost::alloc::string::String,
        #[prost(oneof = "message_channel::Response", tags = "2, 3, 4, 5, 6")]
        pub response: ::core::option::Option<message_channel::Response>,
    }
    /// Nested message and enum types in `MessageChannel`.
    pub mod message_channel {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct StartPublish {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct StopPublish {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Subscribe {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Unsubscribe {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Publish {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
        pub enum Response {
            #[prost(message, tag = "2")]
            StartPub(StartPublish),
            #[prost(message, tag = "3")]
            StopPub(StopPublish),
            #[prost(message, tag = "4")]
            Sub(Subscribe),
            #[prost(message, tag = "5")]
            Unsub(Unsubscribe),
            #[prost(message, tag = "6")]
            Pub(Publish),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Sender {
        #[prost(oneof = "sender::Response", tags = "1, 2, 3")]
        pub response: ::core::option::Option<sender::Response>,
    }
    /// Nested message and enum types in `Sender`.
    pub mod sender {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Attach {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Detach {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Config {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
        pub enum Response {
            #[prost(message, tag = "1")]
            Attach(Attach),
            #[prost(message, tag = "2")]
            Detach(Detach),
            #[prost(message, tag = "3")]
            Config(Config),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Receiver {
        #[prost(oneof = "receiver::Response", tags = "1, 2, 3")]
        pub response: ::core::option::Option<receiver::Response>,
    }
    /// Nested message and enum types in `Receiver`.
    pub mod receiver {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Attach {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Detach {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Config {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
        pub enum Response {
            #[prost(message, tag = "1")]
            Attach(Attach),
            #[prost(message, tag = "2")]
            Detach(Detach),
            #[prost(message, tag = "3")]
            Config(Config),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "2")]
        Error(super::super::shared::Error),
        #[prost(message, tag = "3")]
        Session(Session),
        #[prost(message, tag = "4")]
        Room(Room),
        #[prost(message, tag = "5")]
        Sender(Sender),
        #[prost(message, tag = "6")]
        Receiver(Receiver),
        #[prost(message, tag = "7")]
        Features(super::super::features::Response),
        #[prost(message, tag = "8")]
        MessageChannel(MessageChannel),
    }
}
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerEvent {
    #[prost(uint32, tag = "1")]
    pub seq: u32,
    #[prost(oneof = "server_event::Event", tags = "2, 3, 4, 5, 6, 7, 8")]
    pub event: ::core::option::Option<server_event::Event>,
}
/// Nested message and enum types in `ServerEvent`.
pub mod server_event {
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Session {
        #[prost(oneof = "session::Event", tags = "1, 2, 3, 4, 5")]
        pub event: ::core::option::Option<session::Event>,
    }
    /// Nested message and enum types in `Session`.
    pub mod session {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Connected {}
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct JoinedRoom {
            #[prost(string, tag = "1")]
            pub room: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub peer: ::prost::alloc::string::String,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LeavedRoom {
            #[prost(string, tag = "1")]
            pub room: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub peer: ::prost::alloc::string::String,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Disconnected {
            #[prost(string, tag = "1")]
            pub reason: ::prost::alloc::string::String,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GoAway {
            #[prost(string, tag = "1")]
            pub reason: ::prost::alloc::string::String,
            #[prost(uint32, tag = "2")]
            pub remain_seconds: u32,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Event {
            #[prost(message, tag = "1")]
            Connected(Connected),
            #[prost(message, tag = "2")]
            Joined(JoinedRoom),
            #[prost(message, tag = "3")]
            Leaved(LeavedRoom),
            #[prost(message, tag = "4")]
            Disconnected(Disconnected),
            #[prost(message, tag = "5")]
            Goway(GoAway),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Room {
        #[prost(oneof = "room::Event", tags = "1, 2, 3, 4, 5, 6")]
        pub event: ::core::option::Option<room::Event>,
    }
    /// Nested message and enum types in `Room`.
    pub mod room {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PeerJoined {
            #[prost(string, tag = "1")]
            pub peer: ::prost::alloc::string::String,
            #[prost(string, optional, tag = "2")]
            pub metadata: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "3")]
            pub extra_data: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PeerUpdated {
            #[prost(string, tag = "1")]
            pub peer: ::prost::alloc::string::String,
            #[prost(string, optional, tag = "2")]
            pub metadata: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PeerLeaved {
            #[prost(string, tag = "1")]
            pub peer: ::prost::alloc::string::String,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TrackStarted {
            #[prost(string, tag = "1")]
            pub peer: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub track: ::prost::alloc::string::String,
            #[prost(enumeration = "super::super::super::shared::Kind", tag = "3")]
            pub kind: i32,
            #[prost(string, optional, tag = "4")]
            pub metadata: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TrackUpdated {
            #[prost(string, tag = "1")]
            pub peer: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub track: ::prost::alloc::string::String,
            #[prost(enumeration = "super::super::super::shared::Kind", tag = "3")]
            pub kind: i32,
            #[prost(string, optional, tag = "4")]
            pub metadata: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TrackStopped {
            #[prost(string, tag = "1")]
            pub peer: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub track: ::prost::alloc::string::String,
            #[prost(enumeration = "super::super::super::shared::Kind", tag = "3")]
            pub kind: i32,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Event {
            #[prost(message, tag = "1")]
            PeerJoined(PeerJoined),
            #[prost(message, tag = "2")]
            PeerUpdated(PeerUpdated),
            #[prost(message, tag = "3")]
            PeerLeaved(PeerLeaved),
            #[prost(message, tag = "4")]
            TrackStarted(TrackStarted),
            #[prost(message, tag = "5")]
            TrackUpdated(TrackUpdated),
            #[prost(message, tag = "6")]
            TrackStopped(TrackStopped),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageChannel {
        #[prost(string, tag = "1")]
        pub label: ::prost::alloc::string::String,
        #[prost(oneof = "message_channel::Event", tags = "2")]
        pub event: ::core::option::Option<message_channel::Event>,
    }
    /// Nested message and enum types in `MessageChannel`.
    pub mod message_channel {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Message {
            #[prost(string, tag = "1")]
            pub peer: ::prost::alloc::string::String,
            #[prost(bytes = "vec", tag = "2")]
            pub message: ::prost::alloc::vec::Vec<u8>,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Event {
            #[prost(message, tag = "2")]
            Message(Message),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Sender {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof = "sender::Event", tags = "2")]
        pub event: ::core::option::Option<sender::Event>,
    }
    /// Nested message and enum types in `Sender`.
    pub mod sender {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct State {
            #[prost(
                enumeration = "super::super::super::shared::sender::Status",
                tag = "1"
            )]
            pub status: i32,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
        pub enum Event {
            #[prost(message, tag = "2")]
            State(State),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Receiver {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof = "receiver::Event", tags = "2, 3, 4")]
        pub event: ::core::option::Option<receiver::Event>,
    }
    /// Nested message and enum types in `Receiver`.
    pub mod receiver {
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct State {
            #[prost(
                enumeration = "super::super::super::shared::receiver::Status",
                tag = "1"
            )]
            pub status: i32,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Stats {
            #[prost(message, optional, tag = "1")]
            pub source: ::core::option::Option<stats::Source>,
            #[prost(message, optional, tag = "2")]
            pub transmit: ::core::option::Option<stats::Transmit>,
        }
        /// Nested message and enum types in `Stats`.
        pub mod stats {
            #[derive(serde::Serialize)]
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, Copy, PartialEq, ::prost::Message)]
            pub struct Source {
                #[prost(uint32, tag = "1")]
                pub bitrate_kbps: u32,
                #[prost(float, tag = "2")]
                pub rtt: f32,
                #[prost(float, tag = "3")]
                pub lost: f32,
                #[prost(float, tag = "4")]
                pub jitter: f32,
            }
            #[derive(serde::Serialize)]
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, Copy, PartialEq, ::prost::Message)]
            pub struct Transmit {
                #[prost(uint32, tag = "1")]
                pub spatial: u32,
                #[prost(uint32, tag = "2")]
                pub temporal: u32,
                #[prost(uint32, tag = "3")]
                pub bitrate_kbps: u32,
            }
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct VoiceActivity {
            #[prost(int32, tag = "1")]
            pub audio_level: i32,
        }
        #[derive(serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
        pub enum Event {
            #[prost(message, tag = "2")]
            State(State),
            #[prost(message, tag = "3")]
            Stats(Stats),
            #[prost(message, tag = "4")]
            VoiceActivity(VoiceActivity),
        }
    }
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag = "2")]
        Session(Session),
        #[prost(message, tag = "3")]
        Room(Room),
        #[prost(message, tag = "4")]
        Sender(Sender),
        #[prost(message, tag = "5")]
        Receiver(Receiver),
        #[prost(message, tag = "6")]
        Response(super::Response),
        #[prost(message, tag = "7")]
        Features(super::super::features::ServerEvent),
        #[prost(message, tag = "8")]
        MessageChannel(MessageChannel),
    }
}
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientEvent {
    #[prost(uint32, tag = "1")]
    pub seq: u32,
    #[prost(oneof = "client_event::Event", tags = "2")]
    pub event: ::core::option::Option<client_event::Event>,
}
/// Nested message and enum types in `ClientEvent`.
pub mod client_event {
    #[derive(serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag = "2")]
        Request(super::Request),
    }
}
