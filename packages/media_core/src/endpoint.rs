use std::{marker::PhantomData, time::Instant};

use media_server_protocol::{
    endpoint::{PeerId, RoomId, TrackMeta, TrackName},
    media::MediaPacket,
    transport::RpcResult,
};
use sans_io_runtime::{
    backend::{BackendIncoming, BackendOutgoing},
    TaskSwitcher,
};

use crate::{
    cluster::{ClusterEndpointControl, ClusterEndpointEvent},
    transport::{LocalTrackId, RemoteTrackId, Transport, TransportInput, TransportOutput},
};

use internal::EndpointInternal;

mod internal;
mod middleware;

pub struct EndpointSession(pub u64);

pub enum EndpointRemoteTrackReq {}

pub enum EndpointRemoteTrackRes {}

pub enum EndpointLocalTrackReq {
    Switch(Option<(PeerId, TrackName)>),
}

pub enum EndpointLocalTrackRes {
    Switch(RpcResult<()>),
}

pub struct EndpointReqId(pub u64);

/// This is control APIs, which is used to control server from Endpoint SDK
pub enum EndpointReq {
    JoinRoom(RoomId, PeerId),
    LeaveRoom,
    RemoteTrack(RemoteTrackId, EndpointRemoteTrackReq),
    LocalTrack(LocalTrackId, EndpointLocalTrackReq),
}

/// This is response, which is used to send response back to Endpoint SDK
pub enum EndpointRes {
    JoinRoom(RpcResult<()>),
    LeaveRoom(RpcResult<()>),
    RemoteTrack(RemoteTrackId, EndpointRemoteTrackRes),
    LocalTrack(LocalTrackId, EndpointLocalTrackRes),
}

/// This is used for controlling the local track, which is sent from endpoint
pub enum EndpointLocalTrackEvent {
    Media(MediaPacket),
}

/// This is used for controlling the remote track, which is sent from endpoint
pub enum EndpointRemoteTrackEvent {
    RequestKeyFrame,
    LimitBitrateBps(u64),
}

pub enum EndpointEvent {
    PeerJoined(PeerId),
    PeerLeaved(PeerId),
    PeerTrackStarted(PeerId, TrackName, TrackMeta),
    PeerTrackStopped(PeerId, TrackName),
    RemoteMediaTrack(RemoteTrackId, EndpointRemoteTrackEvent),
    LocalMediaTrack(LocalTrackId, EndpointLocalTrackEvent),
}

pub enum EndpointInput<'a, Ext> {
    Net(BackendIncoming<'a>),
    Cluster(ClusterEndpointEvent),
    Ext(Ext),
    Close,
}

pub enum EndpointOutput<'a, Ext> {
    Net(BackendOutgoing<'a>),
    Cluster(ClusterEndpointControl),
    Ext(Ext),
    Shutdown,
}

#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[repr(usize)]
enum TaskType {
    Transport = 0,
    Internal = 1,
}

pub struct Endpoint<T: Transport<ExtIn, ExtOut>, ExtIn, ExtOut> {
    transport: T,
    internal: EndpointInternal,
    switcher: TaskSwitcher,
    _tmp: PhantomData<(ExtIn, ExtOut)>,
}

impl<T: Transport<ExtIn, ExtOut>, ExtIn, ExtOut> Endpoint<T, ExtIn, ExtOut> {
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            internal: EndpointInternal::new(),
            switcher: TaskSwitcher::new(2),
            _tmp: PhantomData::default(),
        }
    }

    pub fn on_tick<'a>(&mut self, now: Instant) -> Option<EndpointOutput<'a, ExtOut>> {
        let s = &mut self.switcher;
        loop {
            match s.looper_current(now)?.try_into().ok()? {
                TaskType::Internal => {
                    if let Some(out) = s.looper_process(self.internal.on_tick(now)) {
                        return self.process_internal_output(now, out);
                    }
                }
                TaskType::Transport => {
                    if let Some(out) = s.looper_process(self.transport.on_tick(now)) {
                        return self.process_transport_output(now, out);
                    }
                }
            }
        }
    }

    pub fn on_event<'a>(&mut self, now: Instant, input: EndpointInput<'a, ExtIn>) -> Option<EndpointOutput<'a, ExtOut>> {
        match input {
            EndpointInput::Net(net) => {
                let out = self.transport.on_control(now, TransportInput::Net(net))?;
                self.process_transport_output(now, out)
            }
            EndpointInput::Ext(ext) => {
                let out = self.transport.on_control(now, TransportInput::Ext(ext))?;
                self.process_transport_output(now, out)
            }
            EndpointInput::Cluster(event) => {
                let out = self.internal.on_cluster_event(now, event)?;
                self.process_internal_output(now, out)
            }
            EndpointInput::Close => todo!(),
        }
    }

    pub fn pop_output<'a>(&mut self, now: Instant) -> Option<EndpointOutput<'a, ExtOut>> {
        let out = self.transport.pop_event(now)?;
        self.process_transport_output(now, out)
    }

    pub fn shutdown<'a>(&mut self, now: Instant) -> Option<EndpointOutput<'a, ExtOut>> {
        let out = self.internal.shutdown(now)?;
        self.process_internal_output(now, out)
    }
}

impl<T: Transport<ExtIn, ExtOut>, ExtIn, ExtOut> Endpoint<T, ExtIn, ExtOut> {
    fn process_transport_output<'a>(&mut self, now: Instant, out: TransportOutput<'a, ExtOut>) -> Option<EndpointOutput<'a, ExtOut>> {
        self.switcher.queue_flag_task(TaskType::Transport.into());
        match out {
            TransportOutput::Event(event) => {
                let out = self.internal.on_transport_event(now, event)?;
                self.process_internal_output(now, out)
            }
            TransportOutput::Ext(ext) => Some(EndpointOutput::Ext(ext)),
            TransportOutput::Net(net) => Some(EndpointOutput::Net(net)),
            TransportOutput::RpcReq(req_id, req) => {
                let out = self.internal.on_transport_rpc(now, req_id, req)?;
                self.process_internal_output(now, out)
            }
        }
    }

    fn process_internal_output<'a>(&mut self, now: Instant, out: internal::InternalOutput<'a>) -> Option<EndpointOutput<'a, ExtOut>> {
        self.switcher.queue_flag_task(TaskType::Internal.into());
        todo!()
    }
}
