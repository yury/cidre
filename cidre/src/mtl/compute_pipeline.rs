use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(pub Reflection(ns::Id));

define_obj_type!(pub Desc(ns::Id), MTL_COMPUTE_PIPELINE_DESCRIPTOR);

impl Desc {
    define_mtl!(label, set_label);

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
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_COMPUTE_PIPELINE_DESCRIPTOR: &'static objc::Class<Desc>;
}

define_obj_type!(pub State(ns::Id));

impl State {
    define_mtl!(device, label, gpu_resource_id);

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

        let label = ns::String::with_str("label");

        desc.set_label(Some(&label));

        let lb = desc.label().unwrap();

        assert!(lb.eq(label.as_ref()));

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
