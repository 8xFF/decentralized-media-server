// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GatewayEvent {
    #[prost(oneof = "gateway_event::Event", tags = "1")]
    pub event: ::core::option::Option<gateway_event::Event>,
}
/// Nested message and enum types in `GatewayEvent`.
pub mod gateway_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag = "1")]
        Ping(super::PingEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PingEvent {
    #[prost(uint32, tag = "3")]
    pub cpu: u32,
    #[prost(uint32, tag = "4")]
    pub memory: u32,
    #[prost(uint32, tag = "5")]
    pub disk: u32,
    #[prost(message, optional, tag = "6")]
    pub webrtc: ::core::option::Option<ping_event::ServiceStats>,
    #[prost(oneof = "ping_event::Origin", tags = "1, 2")]
    pub origin: ::core::option::Option<ping_event::Origin>,
}
/// Nested message and enum types in `PingEvent`.
pub mod ping_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct MediaOrigin {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct GatewayOrigin {
        #[prost(uint32, tag = "1")]
        pub zone: u32,
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<gateway_origin::Location>,
    }
    /// Nested message and enum types in `GatewayOrigin`.
    pub mod gateway_origin {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Location {
            #[prost(float, tag = "1")]
            pub lat: f32,
            #[prost(float, tag = "2")]
            pub lon: f32,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ServiceStats {
        #[prost(uint32, tag = "1")]
        pub live: u32,
        #[prost(uint32, tag = "2")]
        pub max: u32,
        #[prost(bool, tag = "3")]
        pub active: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum Origin {
        #[prost(message, tag = "1")]
        Media(MediaOrigin),
        #[prost(message, tag = "2")]
        Gateway(GatewayOrigin),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Empty {}
/// For whip
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhipConnectRequest {
    #[prost(string, tag = "1")]
    pub user_agent: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub ip: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sdp: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub room: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub peer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub session_id: u64,
    #[prost(bool, tag = "7")]
    pub record: bool,
    #[prost(string, optional, tag = "8")]
    pub extra_data: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhipConnectResponse {
    #[prost(string, tag = "1")]
    pub conn: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sdp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhipRemoteIceRequest {
    #[prost(string, tag = "1")]
    pub conn: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub ice: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhipRemoteIceResponse {
    #[prost(string, tag = "1")]
    pub conn: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhipCloseRequest {
    #[prost(string, tag = "1")]
    pub conn: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhipCloseResponse {
    #[prost(string, tag = "1")]
    pub conn: ::prost::alloc::string::String,
}
/// For whep
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhepConnectRequest {
    #[prost(string, tag = "1")]
    pub user_agent: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub ip: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sdp: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub room: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub peer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub session_id: u64,
    #[prost(string, optional, tag = "8")]
    pub extra_data: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhepConnectResponse {
    #[prost(string, tag = "1")]
    pub conn: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sdp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhepRemoteIceRequest {
    #[prost(string, tag = "1")]
    pub conn: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub ice: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhepRemoteIceResponse {
    #[prost(string, tag = "1")]
    pub conn: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhepCloseRequest {
    #[prost(string, tag = "1")]
    pub conn: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhepCloseResponse {
    #[prost(string, tag = "1")]
    pub conn: ::prost::alloc::string::String,
}
/// For SDK
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebrtcConnectRequest {
    #[prost(string, tag = "1")]
    pub user_agent: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub ip: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub req: ::core::option::Option<super::gateway::ConnectRequest>,
    #[prost(uint64, tag = "4")]
    pub session_id: u64,
    #[prost(bool, tag = "5")]
    pub record: bool,
    #[prost(string, optional, tag = "8")]
    pub extra_data: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebrtcConnectResponse {
    #[prost(message, optional, tag = "1")]
    pub res: ::core::option::Option<super::gateway::ConnectResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebrtcRemoteIceRequest {
    #[prost(string, tag = "1")]
    pub conn: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub candidates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct WebrtcRemoteIceResponse {
    #[prost(uint32, tag = "1")]
    pub added: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebrtcRestartIceRequest {
    #[prost(string, tag = "1")]
    pub conn: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_agent: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub ip: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub req: ::core::option::Option<super::gateway::ConnectRequest>,
    #[prost(bool, tag = "5")]
    pub record: bool,
    #[prost(string, optional, tag = "8")]
    pub extra_data: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebrtcRestartIceResponse {
    #[prost(message, optional, tag = "1")]
    pub res: ::core::option::Option<super::gateway::ConnectResponse>,
}
#[allow(async_fn_in_trait)]
pub trait MediaEdgeServiceHandler<CTX> {
    async fn whip_connect(&self, ctx: &CTX, req: WhipConnectRequest) -> Option<WhipConnectResponse>;
    async fn whip_remote_ice(&self, ctx: &CTX, req: WhipRemoteIceRequest) -> Option<WhipRemoteIceResponse>;
    async fn whip_close(&self, ctx: &CTX, req: WhipCloseRequest) -> Option<WhipCloseResponse>;
    async fn whep_connect(&self, ctx: &CTX, req: WhepConnectRequest) -> Option<WhepConnectResponse>;
    async fn whep_remote_ice(&self, ctx: &CTX, req: WhepRemoteIceRequest) -> Option<WhepRemoteIceResponse>;
    async fn whep_close(&self, ctx: &CTX, req: WhepCloseRequest) -> Option<WhepCloseResponse>;
    async fn webrtc_connect(&self, ctx: &CTX, req: WebrtcConnectRequest) -> Option<WebrtcConnectResponse>;
    async fn webrtc_remote_ice(&self, ctx: &CTX, req: WebrtcRemoteIceRequest) -> Option<WebrtcRemoteIceResponse>;
    async fn webrtc_restart_ice(&self, ctx: &CTX, req: WebrtcRestartIceRequest) -> Option<WebrtcRestartIceResponse>;
}
pub struct MediaEdgeServiceClient<D, C: crate::rpc::RpcClient<D, S>, S: crate::rpc::RpcStream> {
    client: C,
    _tmp: std::marker::PhantomData<(D, S)>,
}
impl<D, C: crate::rpc::RpcClient<D, S>, S: crate::rpc::RpcStream> Clone for MediaEdgeServiceClient<D, C, S> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            _tmp: Default::default(),
        }
    }
}
impl<D, C: crate::rpc::RpcClient<D, S>, S: crate::rpc::RpcStream> MediaEdgeServiceClient<D, C, S> {
    pub fn new(client: C) -> Self {
        Self { client, _tmp: Default::default() }
    }
    pub async fn whip_connect(&self, dest: D, req: WhipConnectRequest) -> Option<WhipConnectResponse> {
        use prost::Message;
        let mut stream = self.client.connect(dest, "whip_connect.service").await?;
        let out_buf = req.encode_to_vec();
        stream.write(&out_buf).await?;
        let in_buf = stream.read().await?;
        WhipConnectResponse::decode(in_buf.as_slice()).ok()
    }
    pub async fn whip_remote_ice(&self, dest: D, req: WhipRemoteIceRequest) -> Option<WhipRemoteIceResponse> {
        use prost::Message;
        let mut stream = self.client.connect(dest, "whip_remote_ice.service").await?;
        let out_buf = req.encode_to_vec();
        stream.write(&out_buf).await?;
        let in_buf = stream.read().await?;
        WhipRemoteIceResponse::decode(in_buf.as_slice()).ok()
    }
    pub async fn whip_close(&self, dest: D, req: WhipCloseRequest) -> Option<WhipCloseResponse> {
        use prost::Message;
        let mut stream = self.client.connect(dest, "whip_close.service").await?;
        let out_buf = req.encode_to_vec();
        stream.write(&out_buf).await?;
        let in_buf = stream.read().await?;
        WhipCloseResponse::decode(in_buf.as_slice()).ok()
    }
    pub async fn whep_connect(&self, dest: D, req: WhepConnectRequest) -> Option<WhepConnectResponse> {
        use prost::Message;
        let mut stream = self.client.connect(dest, "whep_connect.service").await?;
        let out_buf = req.encode_to_vec();
        stream.write(&out_buf).await?;
        let in_buf = stream.read().await?;
        WhepConnectResponse::decode(in_buf.as_slice()).ok()
    }
    pub async fn whep_remote_ice(&self, dest: D, req: WhepRemoteIceRequest) -> Option<WhepRemoteIceResponse> {
        use prost::Message;
        let mut stream = self.client.connect(dest, "whep_remote_ice.service").await?;
        let out_buf = req.encode_to_vec();
        stream.write(&out_buf).await?;
        let in_buf = stream.read().await?;
        WhepRemoteIceResponse::decode(in_buf.as_slice()).ok()
    }
    pub async fn whep_close(&self, dest: D, req: WhepCloseRequest) -> Option<WhepCloseResponse> {
        use prost::Message;
        let mut stream = self.client.connect(dest, "whep_close.service").await?;
        let out_buf = req.encode_to_vec();
        stream.write(&out_buf).await?;
        let in_buf = stream.read().await?;
        WhepCloseResponse::decode(in_buf.as_slice()).ok()
    }
    pub async fn webrtc_connect(&self, dest: D, req: WebrtcConnectRequest) -> Option<WebrtcConnectResponse> {
        use prost::Message;
        let mut stream = self.client.connect(dest, "webrtc_connect.service").await?;
        let out_buf = req.encode_to_vec();
        stream.write(&out_buf).await?;
        let in_buf = stream.read().await?;
        WebrtcConnectResponse::decode(in_buf.as_slice()).ok()
    }
    pub async fn webrtc_remote_ice(&self, dest: D, req: WebrtcRemoteIceRequest) -> Option<WebrtcRemoteIceResponse> {
        use prost::Message;
        let mut stream = self.client.connect(dest, "webrtc_remote_ice.service").await?;
        let out_buf = req.encode_to_vec();
        stream.write(&out_buf).await?;
        let in_buf = stream.read().await?;
        WebrtcRemoteIceResponse::decode(in_buf.as_slice()).ok()
    }
    pub async fn webrtc_restart_ice(&self, dest: D, req: WebrtcRestartIceRequest) -> Option<WebrtcRestartIceResponse> {
        use prost::Message;
        let mut stream = self.client.connect(dest, "webrtc_restart_ice.service").await?;
        let out_buf = req.encode_to_vec();
        stream.write(&out_buf).await?;
        let in_buf = stream.read().await?;
        WebrtcRestartIceResponse::decode(in_buf.as_slice()).ok()
    }
}
pub struct MediaEdgeServiceServer<CTX, H: MediaEdgeServiceHandler<CTX>, Sr: crate::rpc::RpcServer<S>, S: crate::rpc::RpcStream> {
    ctx: std::sync::Arc<CTX>,
    handler: std::sync::Arc<H>,
    server: Sr,
    _tmp: std::marker::PhantomData<S>,
}
impl<CTX: 'static + Clone, H: 'static + MediaEdgeServiceHandler<CTX>, Sr: crate::rpc::RpcServer<S>, S: 'static + crate::rpc::RpcStream> MediaEdgeServiceServer<CTX, H, Sr, S> {
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
                "whip_connect.service" => {
                    tokio::task::spawn_local(async move {
                        if let Some(in_buf) = stream.read().await {
                            if let Ok(req) = WhipConnectRequest::decode(in_buf.as_slice()) {
                                if let Some(res) = handler.whip_connect(&ctx, req).await {
                                    let out_buf = res.encode_to_vec();
                                    stream.write(&out_buf).await;
                                    stream.close().await;
                                }
                            }
                        }
                    });
                }
                "whip_remote_ice.service" => {
                    tokio::task::spawn_local(async move {
                        if let Some(in_buf) = stream.read().await {
                            if let Ok(req) = WhipRemoteIceRequest::decode(in_buf.as_slice()) {
                                if let Some(res) = handler.whip_remote_ice(&ctx, req).await {
                                    let out_buf = res.encode_to_vec();
                                    stream.write(&out_buf).await;
                                    stream.close().await;
                                }
                            }
                        }
                    });
                }
                "whip_close.service" => {
                    tokio::task::spawn_local(async move {
                        if let Some(in_buf) = stream.read().await {
                            if let Ok(req) = WhipCloseRequest::decode(in_buf.as_slice()) {
                                if let Some(res) = handler.whip_close(&ctx, req).await {
                                    let out_buf = res.encode_to_vec();
                                    stream.write(&out_buf).await;
                                    stream.close().await;
                                }
                            }
                        }
                    });
                }
                "whep_connect.service" => {
                    tokio::task::spawn_local(async move {
                        if let Some(in_buf) = stream.read().await {
                            if let Ok(req) = WhepConnectRequest::decode(in_buf.as_slice()) {
                                if let Some(res) = handler.whep_connect(&ctx, req).await {
                                    let out_buf = res.encode_to_vec();
                                    stream.write(&out_buf).await;
                                    stream.close().await;
                                }
                            }
                        }
                    });
                }
                "whep_remote_ice.service" => {
                    tokio::task::spawn_local(async move {
                        if let Some(in_buf) = stream.read().await {
                            if let Ok(req) = WhepRemoteIceRequest::decode(in_buf.as_slice()) {
                                if let Some(res) = handler.whep_remote_ice(&ctx, req).await {
                                    let out_buf = res.encode_to_vec();
                                    stream.write(&out_buf).await;
                                    stream.close().await;
                                }
                            }
                        }
                    });
                }
                "whep_close.service" => {
                    tokio::task::spawn_local(async move {
                        if let Some(in_buf) = stream.read().await {
                            if let Ok(req) = WhepCloseRequest::decode(in_buf.as_slice()) {
                                if let Some(res) = handler.whep_close(&ctx, req).await {
                                    let out_buf = res.encode_to_vec();
                                    stream.write(&out_buf).await;
                                    stream.close().await;
                                }
                            }
                        }
                    });
                }
                "webrtc_connect.service" => {
                    tokio::task::spawn_local(async move {
                        if let Some(in_buf) = stream.read().await {
                            if let Ok(req) = WebrtcConnectRequest::decode(in_buf.as_slice()) {
                                if let Some(res) = handler.webrtc_connect(&ctx, req).await {
                                    let out_buf = res.encode_to_vec();
                                    stream.write(&out_buf).await;
                                    stream.close().await;
                                }
                            }
                        }
                    });
                }
                "webrtc_remote_ice.service" => {
                    tokio::task::spawn_local(async move {
                        if let Some(in_buf) = stream.read().await {
                            if let Ok(req) = WebrtcRemoteIceRequest::decode(in_buf.as_slice()) {
                                if let Some(res) = handler.webrtc_remote_ice(&ctx, req).await {
                                    let out_buf = res.encode_to_vec();
                                    stream.write(&out_buf).await;
                                    stream.close().await;
                                }
                            }
                        }
                    });
                }
                "webrtc_restart_ice.service" => {
                    tokio::task::spawn_local(async move {
                        if let Some(in_buf) = stream.read().await {
                            if let Ok(req) = WebrtcRestartIceRequest::decode(in_buf.as_slice()) {
                                if let Some(res) = handler.webrtc_restart_ice(&ctx, req).await {
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
