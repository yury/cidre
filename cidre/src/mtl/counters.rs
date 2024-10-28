use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(
    /// A specialized memory buffer that stores a GPU’s counter set data.
    #[doc(alias = "MTLCounterSampleBuffer")]
    pub CounterSampleBuf(ns::Id)
);

impl CounterSampleBuf {
    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;
}

#[derive(Debug)]
#[repr(C)]
pub struct CounterResultStatistic {
    pub tessellation_input_patches: u64,
    pub vertex_invocations: u64,
    pub post_tessellation_vertex_invocations: u64,
    pub clipper_invocations: u64,
    pub clipper_primitives_out: u64,
    pub fragment_invocations: u64,
    pub fragments_passed: u64,
    pub compute_kernel_invocations: u64,
}

define_obj_type!(pub Counter(ns::Id));

impl Counter {
    /// The name of a GPU’s counter instance.
    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::String;
}

define_obj_type!(pub CounterSet(ns::Id));

impl CounterSet {
    /// The name of the GPU’s counter set instance.
    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::String;

    #[objc::msg_send(counters)]
    pub fn counters(&self) -> &ns::Array<Counter>;
}

define_obj_type!(
    /// A group of properties that configures the counter sample buffers you create with it.
    #[doc(alias = "MTLCounterSampleBufferDescriptor")]
    pub Desc(ns::Id), MTL_COUNTER_SAMPLE_BUFFER_DESCRIPTOR
);

impl Desc {
    define_mtl!(set_label, storage_mode);

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    /// The number of instances of a counter set’s data that a counter sample buffer can store.
    #[objc::msg_send(sampleCount)]
    pub fn sample_count(&self) -> usize;

    #[objc::msg_send(setSampleCount:)]
    pub fn set_sample_count(&mut self, value: usize);
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_COUNTER_SAMPLE_BUFFER_DESCRIPTOR: &'static objc::Class<Desc>;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let mut desc = mtl::CounterSampleBufDesc::new();
        assert_eq!(desc.sample_count(), 0);
        desc.set_sample_count(10);
        assert_eq!(desc.sample_count(), 10);

        //let device = mtl::Device::default().unwrap();
    }
}
