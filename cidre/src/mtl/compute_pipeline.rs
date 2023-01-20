use crate::{
    arc, define_mtl, define_obj_type, mtl, ns,
    objc::{self, Class},
};

define_obj_type!(Reflection(ns::Id));

define_obj_type!(Descriptor(ns::Id));

impl Descriptor {
    define_mtl!(label, set_label);

    pub fn new() -> arc::R<Self> {
        unsafe { Self::cls().alloc().init() }
    }
    #[objc::msg_send2(init)]
    fn init(&self) -> arc::R<Self>;

    pub fn cls() -> &'static Class<Self> {
        unsafe { MTL_COMPUTE_PIPELINE_DESCRIPTOR }
    }

    #[objc::msg_send2(threadGroupSizeIsMultipleOfThreadExecutionWidth)]
    pub fn thread_group_size_is_multiple_of_thread_execution_width(&self) -> bool;

    #[objc::msg_send2(setThreadGroupSizeIsMultipleOfThreadExecutionWidth:)]
    pub fn set_thread_group_size_is_multiple_of_thread_execution_width(&mut self, value: bool);

    #[objc::msg_send2(computeFunction)]
    pub fn compute_function(&self) -> Option<&mtl::Function>;

    #[objc::msg_send2(setComputeFunction:)]
    pub fn set_compute_function(&mut self, value: Option<&mtl::Function>);

    #[objc::msg_send2(maxTotalThreadsPerThreadgroup)]
    pub fn max_total_threads_per_threadgroup(&self) -> usize;

    #[objc::msg_send2(setMaxTotalThreadsPerThreadgroup:)]
    pub fn set_max_total_threads_per_threadgroup(&mut self, value: usize);
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_COMPUTE_PIPELINE_DESCRIPTOR: &'static Class<Descriptor>;
}

define_obj_type!(State(ns::Id));

impl State {
    define_mtl!(device, label, gpu_resouce_id);

    #[objc::msg_send2(maxTotalThreadsPerThreadgroup)]
    pub fn max_total_threads_per_threadgroup(&self) -> usize;

    #[objc::msg_send2(threadExecutionWidth)]
    pub fn thread_execution_width(&self) -> usize;

    #[objc::msg_send2(staticThreadgroupMemoryLength)]
    pub fn static_threadgroup_memory_length(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use crate::{mtl, ns};

    #[test]
    fn basics() {
        let mut desc = mtl::ComputePipelineDescriptor::new();

        assert!(desc.label().is_none());

        let label = ns::String::with_str("label");

        desc.set_label(Some(&label));

        let lb = desc.label().unwrap();

        assert!(lb.is_equal(&label));

        assert_eq!(
            false,
            desc.thread_group_size_is_multiple_of_thread_execution_width()
        );
        desc.set_thread_group_size_is_multiple_of_thread_execution_width(true);
        assert_eq!(
            true,
            desc.thread_group_size_is_multiple_of_thread_execution_width()
        );
    }
}
