use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use media_server_core::{
    endpoint::{EndpointEvent, EndpointLocalTrackEvent},
    transport::{LocalTrackEvent, LocalTrackId, TransportError, TransportEvent, TransportOutput, TransportState},
};
use media_server_protocol::endpoint::{PeerId, TrackMeta, TrackName};
use str0m::{
    media::{Direction, MediaAdded, MediaKind, Mid},
    Event as Str0mEvent, IceConnectionState,
};

use super::{InternalOutput, TransportWebrtcInternal};

const TIMEOUT_SEC: u64 = 10;
const AUDIO_TRACK: LocalTrackId = LocalTrackId(0);
const VIDEO_TRACK: LocalTrackId = LocalTrackId(1);

#[derive(Debug)]
enum State {
    New,
    Connecting { at: Instant },
    ConnectError(TransportWebrtcError),
    Connected,
    Reconnecting { at: Instant },
    Disconnected(Option<TransportWebrtcError>),
}

#[derive(Debug)]
enum TransportWebrtcError {
    Timeout,
}

#[derive(Default)]
struct SubscribeStreams {
    peer: Option<PeerId>,
    audio: Option<TrackName>,
    video: Option<TrackName>,
}

pub struct TransportWebrtcWhep {
    state: State,
    audio_mid: Option<Mid>,
    video_mid: Option<Mid>,
    subscribed: SubscribeStreams,
    queue: VecDeque<InternalOutput<'static>>,
}

impl TransportWebrtcWhep {
    pub fn new() -> Self {
        Self {
            state: State::New,
            audio_mid: None,
            video_mid: None,
            subscribed: Default::default(),
            queue: VecDeque::new(),
        }
    }
}

impl TransportWebrtcInternal for TransportWebrtcWhep {
    fn on_tick<'a>(&mut self, now: Instant) -> Option<InternalOutput<'a>> {
        match &self.state {
            State::New => {
                self.state = State::Connecting { at: now };
                return Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::State(TransportState::Connecting))));
            }
            State::Connecting { at } => {
                if now - *at >= Duration::from_secs(TIMEOUT_SEC) {
                    log::info!("[TransportWebrtcWhep] connect timed out after {:?} => switched to ConnectError", now - *at);
                    self.state = State::ConnectError(TransportWebrtcError::Timeout);
                    return Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::State(TransportState::ConnectError(
                        TransportError::Timeout,
                    )))));
                }
            }
            State::Reconnecting { at } => {
                if now - *at >= Duration::from_secs(TIMEOUT_SEC) {
                    log::info!("[TransportWebrtcWhep] reconnect timed out after {:?} => switched to Disconnected", now - *at);
                    self.state = State::Disconnected(Some(TransportWebrtcError::Timeout));
                    return Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::State(TransportState::Disconnected(Some(
                        TransportError::Timeout,
                    ))))));
                }
            }
            _ => {}
        }
        None
    }

    fn on_endpoint_event<'a>(&mut self, now: Instant, event: EndpointEvent) -> Option<InternalOutput<'a>> {
        match event {
            EndpointEvent::PeerJoined(_) => None,
            EndpointEvent::PeerLeaved(_) => None,
            EndpointEvent::PeerTrackStarted(peer, track, meta) => self.try_subscribe(peer, track, meta),
            EndpointEvent::PeerTrackStopped(peer, track) => self.try_unsubscribe(peer, track),
            EndpointEvent::LocalMediaTrack(track, event) => match event {
                EndpointLocalTrackEvent::Media(pkt) => Some(InternalOutput::Str0mSendMedia(self.video_mid?, pkt)),
            },
            EndpointEvent::RemoteMediaTrack(track, event) => None,
        }
    }

    fn on_transport_rpc_res<'a>(&mut self, now: Instant, req_id: media_server_core::endpoint::EndpointReqId, res: media_server_core::endpoint::EndpointRes) -> Option<InternalOutput<'a>> {
        None
    }

    fn on_str0m_event<'a>(&mut self, now: Instant, event: str0m::Event) -> Option<InternalOutput<'a>> {
        match event {
            Str0mEvent::Connected => {
                log::info!("[TransportWebrtcWhep] connected");
                self.state = State::Connected;
                return Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::State(TransportState::Connected))));
            }
            Str0mEvent::IceConnectionStateChange(state) => self.on_str0m_state(now, state),
            Str0mEvent::MediaAdded(media) => self.on_str0m_media_added(now, media),
            Str0mEvent::KeyframeRequest(req) => {
                if self.video_mid == Some(req.mid) {
                    Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::LocalTrack(
                        VIDEO_TRACK,
                        LocalTrackEvent::RequestKeyFrame,
                    ))))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn close<'a>(&mut self, now: Instant) -> Option<InternalOutput<'a>> {
        self.queue.push_back(InternalOutput::Destroy);
        log::info!("[TransportWebrtcWhep] switched to disconnected with close action");
        self.state = State::Disconnected(None);
        Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::State(TransportState::Disconnected(None)))))
    }

    fn pop_output<'a>(&mut self, now: Instant) -> Option<InternalOutput<'a>> {
        self.queue.pop_front()
    }
}

impl TransportWebrtcWhep {
    fn on_str0m_state<'a>(&mut self, now: Instant, state: IceConnectionState) -> Option<InternalOutput<'a>> {
        log::info!("[TransportWebrtcWhep] str0m state changed {:?}", state);

        match state {
            IceConnectionState::New => None,
            IceConnectionState::Checking => None,
            IceConnectionState::Connected | IceConnectionState::Completed => match &self.state {
                State::Connecting { at } => {
                    log::info!("[TransportWebrtcWhep] switched to connected after {:?}", now - *at);
                    self.state = State::Connected;
                    Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::State(TransportState::Connected))))
                }
                State::Reconnecting { at } => {
                    log::info!("[TransportWebrtcWhep] switched to reconnected after {:?}", now - *at);
                    self.state = State::Connected;
                    Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::State(TransportState::Connected))))
                }
                _ => {
                    log::warn!("[TransportWebrtcWhep] wrong internal state {:?}", self.state);
                    None
                }
            },
            IceConnectionState::Disconnected => {
                if matches!(self.state, State::Connected) {
                    self.state = State::Reconnecting { at: now };
                    log::info!("[TransportWebrtcWhep] switched to reconnecting");
                    return Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::State(TransportState::Reconnecting))));
                } else {
                    return None;
                }
            }
        }
    }

    fn on_str0m_media_added<'a>(&mut self, now: Instant, media: MediaAdded) -> Option<InternalOutput<'a>> {
        if matches!(media.direction, Direction::RecvOnly | Direction::Inactive) {
            return None;
        }
        if media.kind == MediaKind::Audio {
            if self.audio_mid.is_some() {
                return None;
            }
            self.audio_mid = Some(media.mid);
            Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::LocalTrack(
                AUDIO_TRACK,
                LocalTrackEvent::Started,
            ))))
        } else {
            if self.video_mid.is_some() {
                return None;
            }
            self.video_mid = Some(media.mid);
            Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::LocalTrack(
                VIDEO_TRACK,
                LocalTrackEvent::Started,
            ))))
        }
    }
}

impl TransportWebrtcWhep {
    fn try_subscribe<'a>(&mut self, peer: PeerId, track: TrackName, meta: TrackMeta) -> Option<InternalOutput<'a>> {
        if self.subscribed.peer.is_none() || self.subscribed.peer.eq(&Some(peer.clone())) {
            if self.subscribed.audio.is_none() && meta.kind.is_audio() {
                self.subscribed.audio = Some(track.clone());
                return Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::LocalTrack(
                    AUDIO_TRACK,
                    LocalTrackEvent::Switch(Some((peer, track))),
                ))));
            }

            if self.subscribed.video.is_none() && meta.kind.is_audio() {
                self.subscribed.video = Some(track.clone());
                return Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::LocalTrack(
                    VIDEO_TRACK,
                    LocalTrackEvent::Switch(Some((peer, track))),
                ))));
            }
        }

        None
    }

    fn try_unsubscribe<'a>(&mut self, peer: PeerId, track: TrackName) -> Option<InternalOutput<'a>> {
        if self.subscribed.peer.eq(&Some(peer.clone())) {
            if self.subscribed.audio.eq(&Some(track.clone())) {
                self.subscribed.audio = None;
                return Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::LocalTrack(
                    AUDIO_TRACK,
                    LocalTrackEvent::Switch(None),
                ))));
            }

            if self.subscribed.video.eq(&Some(track)) {
                self.subscribed.video = None;
                return Some(InternalOutput::TransportOutput(TransportOutput::Event(TransportEvent::LocalTrack(
                    VIDEO_TRACK,
                    LocalTrackEvent::Switch(None),
                ))));
            }

            if self.subscribed.audio.is_none() && self.subscribed.video.is_none() {
                self.subscribed.peer = None;
            }
        }

        None
    }
}
