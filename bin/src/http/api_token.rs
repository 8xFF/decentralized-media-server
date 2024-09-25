use std::{marker::PhantomData, sync::Arc};

use super::{utils::TokenAuthorization, Response};
use media_server_protocol::tokens::{RtpEngineToken, WebrtcToken, WhepToken, WhipToken, RTPENGINE_TOKEN, WEBRTC_TOKEN, WHEP_TOKEN, WHIP_TOKEN};
use media_server_secure::MediaGatewaySecure;
use poem::{web::Data, Result};
use poem_openapi::{payload::Json, OpenApi};

pub struct TokenServerCtx<S>
where
    S: MediaGatewaySecure + Send + Sync,
{
    pub(crate) secure: Arc<S>,
}

impl<S: MediaGatewaySecure + Send + Sync> Clone for TokenServerCtx<S> {
    fn clone(&self) -> Self {
        Self { secure: self.secure.clone() }
    }
}

#[derive(poem_openapi::Object)]
struct WhipTokenReq {
    app: Option<String>,
    room: String,
    peer: String,
    ttl: u64,
    record: Option<bool>,
    extra_data: Option<String>,
}

#[derive(poem_openapi::Object)]
struct WhipTokenRes {
    token: String,
}

#[derive(poem_openapi::Object)]
struct WhepTokenReq {
    app: Option<String>,
    room: String,
    peer: Option<String>,
    ttl: u64,
    extra_data: Option<String>,
}

#[derive(poem_openapi::Object)]
struct WhepTokenRes {
    token: String,
}

#[derive(poem_openapi::Object)]
struct WebrtcTokenReq {
    app: Option<String>,
    room: Option<String>,
    peer: Option<String>,
    ttl: u64,
    record: Option<bool>,
    extra_data: Option<String>,
}

#[derive(poem_openapi::Object)]
struct WebrtcTokenRes {
    token: String,
}

#[derive(poem_openapi::Object)]
struct RtpEngineTokenReq {
    app: Option<String>,
    room: String,
    peer: String,
    ttl: u64,
    record: Option<bool>,
    extra_data: Option<String>,
}

#[derive(poem_openapi::Object)]
struct RtpEngineTokenRes {
    token: String,
}

pub struct TokenApis<S: MediaGatewaySecure + Send + Sync>(PhantomData<S>);

impl<S: MediaGatewaySecure + Send + Sync> TokenApis<S> {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

#[OpenApi]
impl<S: 'static + MediaGatewaySecure + Send + Sync> TokenApis<S> {
    /// create whip session token
    #[oai(path = "/whip", method = "post")]
    async fn whip_token(&self, Data(ctx): Data<&TokenServerCtx<S>>, body: Json<WhipTokenReq>, TokenAuthorization(token): TokenAuthorization) -> Result<Json<Response<WhipTokenRes>>> {
        if ctx.secure.validate_app(&token.token) {
            let body = body.0;
            Ok(Json(Response {
                status: true,
                data: Some(WhipTokenRes {
                    token: ctx.secure.encode_obj(
                        WHIP_TOKEN,
                        WhipToken {
                            app: body.app,
                            room: body.room,
                            peer: body.peer,
                            record: body.record.unwrap_or(false),
                            extra_data: body.extra_data,
                        },
                        body.ttl,
                    ),
                }),
                ..Default::default()
            }))
        } else {
            Ok(Json(Response {
                status: false,
                error: Some("APP_TOKEN_INVALID".to_string()),
                ..Default::default()
            }))
        }
    }

    /// create whep session token
    #[oai(path = "/whep", method = "post")]
    async fn whep_token(&self, Data(ctx): Data<&TokenServerCtx<S>>, body: Json<WhepTokenReq>, TokenAuthorization(token): TokenAuthorization) -> Json<Response<WhepTokenRes>> {
        if ctx.secure.validate_app(&token.token) {
            let body = body.0;
            Json(Response {
                status: true,
                data: Some(WhepTokenRes {
                    token: ctx.secure.encode_obj(
                        WHEP_TOKEN,
                        WhepToken {
                            app: body.app,
                            room: body.room,
                            peer: body.peer,
                            extra_data: body.extra_data,
                        },
                        body.ttl,
                    ),
                }),
                ..Default::default()
            })
        } else {
            Json(Response {
                status: false,
                error: Some("APP_TOKEN_INVALID".to_string()),
                ..Default::default()
            })
        }
    }

    #[oai(path = "/webrtc", method = "post")]
    async fn webrtc_token(&self, Data(ctx): Data<&TokenServerCtx<S>>, body: Json<WebrtcTokenReq>, TokenAuthorization(token): TokenAuthorization) -> Json<Response<WebrtcTokenRes>> {
        if ctx.secure.validate_app(&token.token) {
            let body = body.0;
            Json(Response {
                status: true,
                data: Some(WebrtcTokenRes {
                    token: ctx.secure.encode_obj(
                        WEBRTC_TOKEN,
                        WebrtcToken {
                            app: body.app,
                            room: body.room,
                            peer: body.peer,
                            record: body.record.unwrap_or(false),
                            extra_data: body.extra_data,
                        },
                        body.ttl,
                    ),
                }),
                ..Default::default()
            })
        } else {
            Json(Response {
                status: false,
                error: Some("APP_TOKEN_INVALID".to_string()),
                ..Default::default()
            })
        }
    }

    /// create rtpengine session token
    #[oai(path = "/rtpengine", method = "post")]
    async fn rtpengine_token(&self, Data(ctx): Data<&TokenServerCtx<S>>, body: Json<RtpEngineTokenReq>, TokenAuthorization(token): TokenAuthorization) -> Result<Json<Response<RtpEngineTokenRes>>> {
        if ctx.secure.validate_app(&token.token) {
            let body = body.0;
            Ok(Json(Response {
                status: true,
                data: Some(RtpEngineTokenRes {
                    token: ctx.secure.encode_obj(
                        RTPENGINE_TOKEN,
                        RtpEngineToken {
                            app: body.app,
                            room: body.room,
                            peer: body.peer,
                            record: body.record.unwrap_or(false),
                            extra_data: body.extra_data,
                        },
                        body.ttl,
                    ),
                }),
                ..Default::default()
            }))
        } else {
            Ok(Json(Response {
                status: false,
                error: Some("APP_TOKEN_INVALID".to_string()),
                ..Default::default()
            }))
        }
    }
}
