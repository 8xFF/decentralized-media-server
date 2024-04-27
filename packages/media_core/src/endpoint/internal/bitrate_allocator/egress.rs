use derivative::Derivative;
use std::collections::VecDeque;

use media_server_protocol::endpoint::TrackPriority;

use crate::transport::LocalTrackId;

const DEFAULT_BITRATE_BPS: u64 = 800_000;
const NO_TRACK_BWE_CURRENT: u64 = 100_000;
const NO_TRACK_BWE_DESIREND: u64 = 300_000;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    SetBitrate(u64),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    Track(LocalTrackId, Action),
    BweConfig(u64, u64),
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct EgressBitrateAllocator {
    changed: bool,
    #[derivative(Default(value = "DEFAULT_BITRATE_BPS"))]
    egress_bitrate: u64,
    tracks: smallmap::Map<LocalTrackId, TrackPriority>,
    queue: VecDeque<Output>,
}

impl EgressBitrateAllocator {
    pub fn on_tick(&mut self) {
        self.process();
    }

    pub fn set_egress_estimate(&mut self, bitrate: u64) {
        self.egress_bitrate = bitrate;
        self.changed = true;
    }

    pub fn set_video_track(&mut self, track: LocalTrackId, priority: TrackPriority) {
        self.tracks.insert(track, priority);
        self.changed = true;
    }

    pub fn del_video_track(&mut self, track: LocalTrackId) {
        self.tracks.remove(&track);
        self.changed = true;
    }

    pub fn pop_output(&mut self) -> Option<Output> {
        self.queue.pop_front()
    }

    fn process(&mut self) {
        if !self.changed {
            return;
        }
        self.changed = false;
        let mut sum = TrackPriority(0);
        for (_track, priority) in self.tracks.iter() {
            sum = sum + *priority;
        }

        if *(sum.as_ref()) != 0 {
            for (track, priority) in self.tracks.iter() {
                self.queue
                    .push_back(Output::Track(*track, Action::SetBitrate((self.egress_bitrate * priority.0 as u64) / sum.0 as u64)));
            }
        }

        if self.tracks.len() > 0 {
            self.queue.push_back(Output::BweConfig(self.egress_bitrate, self.egress_bitrate * 6 / 5));
        } else {
            self.queue.push_back(Output::BweConfig(NO_TRACK_BWE_CURRENT, NO_TRACK_BWE_DESIREND));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::endpoint::internal::bitrate_allocator::egress::{EgressBitrateAllocator, NO_TRACK_BWE_CURRENT, NO_TRACK_BWE_DESIREND};

    use super::{Action, Output, DEFAULT_BITRATE_BPS};

    #[test]
    fn no_source() {
        let mut allocator = EgressBitrateAllocator::default();
        allocator.set_egress_estimate(200_000);
        allocator.on_tick();

        assert_eq!(allocator.pop_output(), Some(Output::BweConfig(NO_TRACK_BWE_CURRENT, NO_TRACK_BWE_DESIREND)));
        assert_eq!(allocator.pop_output(), None);
    }

    #[test]
    fn single_source() {
        let mut allocator = EgressBitrateAllocator::default();
        allocator.set_video_track(0.into(), 1.into());

        allocator.on_tick();
        assert_eq!(allocator.pop_output(), Some(Output::Track(0.into(), Action::SetBitrate(DEFAULT_BITRATE_BPS))));
        assert_eq!(allocator.pop_output(), Some(Output::BweConfig(DEFAULT_BITRATE_BPS, DEFAULT_BITRATE_BPS * 6 / 5)));
        assert_eq!(allocator.pop_output(), None);
    }

    #[test]
    fn multi_source() {
        let mut allocator = EgressBitrateAllocator::default();
        allocator.set_video_track(0.into(), 1.into());
        allocator.set_video_track(1.into(), 3.into());

        allocator.on_tick();
        assert_eq!(allocator.pop_output(), Some(Output::Track(0.into(), Action::SetBitrate(DEFAULT_BITRATE_BPS * 1 / 4))));
        assert_eq!(allocator.pop_output(), Some(Output::Track(1.into(), Action::SetBitrate(DEFAULT_BITRATE_BPS * 3 / 4))));
        assert_eq!(allocator.pop_output(), Some(Output::BweConfig(DEFAULT_BITRATE_BPS, DEFAULT_BITRATE_BPS * 6 / 5)));
        assert_eq!(allocator.pop_output(), None);
    }
}
