use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(pub Reflection(ns::Id));

define_obj_type!(
    #[doc(alias = "MTLComputePipelineDescriptor")]
    pub Desc(ns::Id),
    MTL_COMPUTE_PIPELINE_DESCRIPTOR
);

impl Desc {
    define_mtl!(set_label);

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(threadGroupSizeIsMultipleOfThreadExecutionWidth)]
    pub fn thread_group_size_is_multiple_of_thread_execution_width(&self) -> bool;

    #[objc::msg_send(setThreadGroupSizeIsMultipleOfThreadExecutionWidth:)]
    pub fn set_thread_group_size_is_multiple_of_thread_execution_width(&mut self, val: bool);

    #[objc::msg_send(computeFunction)]
    pub fn compute_fn(&self) -> Option<&mtl::Fn>;

    #[objc::msg_send(setComputeFunction:)]
    pub fn set_compute_fn(&mut self, val: Option<&mtl::Fn>);

    #[objc::msg_send(maxTotalThreadsPerThreadgroup)]
    pub fn max_total_threads_per_threadgroup(&self) -> usize;

    #[objc::msg_send(setMaxTotalThreadsPerThreadgroup:)]
    pub fn set_max_total_threads_per_threadgroup(&mut self, val: usize);

    #[objc::msg_send(supportIndirectCommandBuffers)]
    pub fn support_icbs(&self) -> bool;

    #[objc::msg_send(setSupportIndirectCommandBuffers:)]
    pub fn set_support_icbs(&mut self, val: bool);

    /// Restore all compute pipeline descriptor properties to their default values.
    #[objc::msg_send(reset)]
    pub fn reset(&mut self);
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_COMPUTE_PIPELINE_DESCRIPTOR: &'static objc::Class<Desc>;
}

define_obj_type!(
    #[doc(alias = "MTLComputePipelineState")]
    pub State(ns::Id)
);

impl State {
    define_mtl!(gpu_res_id);

    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(maxTotalThreadsPerThreadgroup)]
    pub fn max_total_threads_per_threadgroup(&self) -> usize;

    #[objc::msg_send(threadExecutionWidth)]
    pub fn thread_execution_width(&self) -> usize;

    #[objc::msg_send(staticThreadgroupMemoryLength)]
    pub fn static_threadgroup_memory_length(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use crate::{mtl, ns};

    #[test]
    fn basics() {
        let mut desc = mtl::ComputePipelineDesc::new();

        assert!(desc.label().is_none());

        let label = ns::str!(c"label");

        desc.set_label(Some(label));

        let lb = desc.label().unwrap();

        assert!(lb.eq(label));

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
