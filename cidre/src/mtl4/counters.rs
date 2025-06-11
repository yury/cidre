use crate::{api, arc, define_obj_type, mtl4, ns, objc};

#[doc(alias = "MTL4TimestampHeapEntry")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct TimestampHeapEntry {
    pub timestamp: u64,
}

#[doc(alias = "MTL4CounterHeapType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum CounterHeapType {
    Invalid,
    Timestamp,
}

#[doc(alias = "MTL4TimestampGranularity")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum TimestampGranularity {
    /// A minimally-invasive timestamp which may be less precise.
    ///
    /// Using this granularity incurs in the lowest overhead, at the cost of precision. For example, it may sample at
    /// command encoder boundaries.
    Relaxed = 0,

    /// A timestamp as precise as possible.
    ///
    /// Using this granularity may incur in a performance penalty, for example, it may cause splitting of command encoders.
    Precise = 1,
}

define_obj_type!(
    #[doc(alias = "MTL4CounterHeapDescriptor")]
    pub CounterHeapDesc(ns::Id),
    MTL4_COUNTER_HEAP_DESCRIPTOR,
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)]
);

impl CounterHeapDesc {
    #[objc::msg_send(type)]
    pub fn type_(&self) -> mtl4::CounterHeapType;

    /// Assigns the type of data that the heap contains.
    #[objc::msg_send(setType:)]
    pub fn set_type(&mut self, val: mtl4::CounterHeapType);

    #[objc::msg_send(entryCount)]
    pub fn entry_count(&self) -> usize;

    #[objc::msg_send(setEntryCount:)]
    pub fn set_entry_count(&mut self, val: usize);
}

define_obj_type!(
    /// Represents an opaque, driver-controlled section of memory that can store GPU counter data.
    #[doc(alias = "MTL4CounterHeap")]
    pub CounterHeap(ns::Id)
);

impl CounterHeap {
    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(count)]
    pub fn count(&self) -> usize;

    #[objc::msg_send(type)]
    pub fn type_(&self) -> mtl4::CounterHeapType;

    /// Resolves heap data on the CPU timeline.
    #[objc::msg_send(resolveCounterRange:)]
    pub fn resolve_counter_range(&self, range: ns::Range) -> Option<arc::R<ns::Data>>;

    /// Invalidates a range of entries in this counter heap.
    #[objc::msg_send(invalidateCounterRange:)]
    pub fn invalidate_counter_range(&mut self, range: ns::Range);
}

#[link(name = "mtl", kind = "static")]
unsafe extern "C" {
    static MTL4_COUNTER_HEAP_DESCRIPTOR: &'static objc::Class<CounterHeapDesc>;
}
