//! Bounded authored-test observations around the unchanged entropy writer.
//! This entire module, including its storage, is absent from ordinary builds.

use super::{Vec, mq};

// Three decisions per coefficient per plane, plus pass/segment boundaries.
// Low-level encoding also accepts the 32-plane i32::MIN magnitude.
const MAX_EVENTS: usize = 3 * 4096 * 32 + 4 * 94;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Event {
    Pass(u16, u8),
    Mq {
        position: usize,
        label: u8,
        bit: u32,
    },
    Raw {
        position: usize,
        bit: u32,
    },
    Finish {
        raw: bool,
        predictable: bool,
        length: usize,
    },
    Restart {
        raw: bool,
    },
    Reset,
}

#[derive(Default)]
pub(super) struct Trace {
    pub(super) enabled: bool,
    pub(super) events: Vec<Event>,
}

pub(super) struct Encoder<'a> {
    inner: mq::Encoder<'a>,
    trace: Trace,
    position: usize,
}

impl<'a> Encoder<'a> {
    pub(super) fn new(bytes: &'a mut Vec<u8>) -> Self {
        Self {
            inner: mq::Encoder::new(bytes),
            trace: Trace::default(),
            position: 0,
        }
    }

    pub(super) fn enable_trace(&mut self, enabled: bool) {
        self.trace.enabled = enabled;
    }

    fn record(&mut self, event: Event) {
        if self.trace.enabled {
            assert!(
                self.trace.events.len() < MAX_EVENTS,
                "authored encoder trace exceeded its bound"
            );
            self.trace.events.push(event);
        }
    }

    pub(super) fn take_trace(&mut self, destination: &mut Trace) {
        destination.events = core::mem::take(&mut self.trace.events);
    }

    pub(super) fn position(&mut self, position: usize) {
        self.position = position;
    }

    pub(super) fn pass(&mut self, pass: u16, plane: u8) {
        self.record(Event::Pass(pass, plane));
    }

    pub(super) fn write_bit(&mut self, label: u8, bit: u32) {
        self.record(Event::Mq {
            position: self.position,
            label,
            bit,
        });
        self.inner.write_bit(label, bit);
    }

    pub(super) fn write_raw_bit(&mut self, bit: u32) {
        self.record(Event::Raw {
            position: self.position,
            bit,
        });
        self.inner.write_raw_bit(bit);
    }

    pub(super) fn len(&self) -> usize {
        self.inner.len()
    }

    pub(super) fn current_segment_len(&self) -> usize {
        self.inner.current_segment_len()
    }

    pub(super) fn finish(&mut self) {
        self.inner.finish();
        self.record(Event::Finish {
            raw: false,
            predictable: false,
            length: self.inner.current_segment_len(),
        });
    }

    pub(super) fn finish_predictable(&mut self) {
        self.inner.finish_predictable();
        self.record(Event::Finish {
            raw: false,
            predictable: true,
            length: self.inner.current_segment_len(),
        });
    }

    pub(super) fn finish_raw(&mut self, predictable: bool) {
        self.inner.finish_raw(predictable);
        self.record(Event::Finish {
            raw: true,
            predictable,
            length: self.inner.current_segment_len(),
        });
    }

    pub(super) fn restart_segment(&mut self) {
        self.record(Event::Restart { raw: false });
        self.inner.restart_segment();
    }

    pub(super) fn restart_raw_segment(&mut self) {
        self.record(Event::Restart { raw: true });
        self.inner.restart_raw_segment();
    }

    pub(super) fn reset_contexts(&mut self) {
        self.record(Event::Reset);
        self.inner.reset_contexts();
    }
}
