use crate::{cf, define_mtl, define_obj_type, msg_send, mtl, ns};

define_obj_type!(Reflection(ns::Id));

define_obj_type!(Descriptor(ns::Id));

impl Descriptor {
    define_mtl!(label, set_label);

    /// ```
    /// use cidre::{cf, mtl};
    ///
    /// let mut desc = mtl::ComputePipelineDescriptor::new();
    ///
    /// assert!(desc.label().is_none());
    ///
    /// let label = cf::String::from_str("label");
    ///
    /// desc.set_label(Some(&label));
    ///
    /// let lb = desc.label().unwrap();
    ///
    /// assert!(lb.equal(&label));
    ///
    /// assert_eq!(false, desc.thread_group_size_is_multiple_of_thread_execution_width());
    /// desc.set_thread_group_size_is_multiple_of_thread_execution_width(true);
    /// assert_eq!(true, desc.thread_group_size_is_multiple_of_thread_execution_width());
    /// ```
    pub fn new() -> cf::Retained<Self> {
        unsafe { MTLComputePipelineDescriptor_new() }
    }

    pub fn thread_group_size_is_multiple_of_thread_execution_width(&self) -> bool {
        unsafe { rsel_threadGroupSizeIsMultipleOfThreadExecutionWidth(self) }
    }

    pub fn set_thread_group_size_is_multiple_of_thread_execution_width(&mut self, value: bool) {
        unsafe { wsel_setThreadGroupSizeIsMultipleOfThreadExecutionWidth(self, value) }
    }

    pub fn compute_function(&self) -> Option<&mtl::Function> {
        unsafe { rsel_computeFunction(self) }
    }

    pub fn set_compute_function(&mut self, value: Option<&mtl::Function>) {
        unsafe { wsel_setComputeFunction(self, value) }
    }

    pub fn max_total_threads_per_threadgroup(&self) -> usize {
        unsafe { rsel_maxTotalThreadsPerThreadgroup(self) }
    }

    pub fn set_max_total_threads_per_threadgroup(&mut self, value: usize) {
        unsafe { wsel_setMaxTotalThreadsPerThreadgroup(self, value) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLComputePipelineDescriptor_new() -> cf::Retained<Descriptor>;

    fn rsel_threadGroupSizeIsMultipleOfThreadExecutionWidth(id: &ns::Id) -> bool;
    fn wsel_setThreadGroupSizeIsMultipleOfThreadExecutionWidth(id: &mut ns::Id, value: bool);

    fn rsel_computeFunction(id: &ns::Id) -> Option<&mtl::Function>;
    fn wsel_setComputeFunction(id: &mut ns::Id, value: Option<&mtl::Function>);
}

define_obj_type!(State(ns::Id));

impl State {
    define_mtl!(device, label, gpu_resouce_id);

    #[inline]
    pub fn max_total_threads_per_threadgroup(&self) -> usize {
        unsafe { rsel_maxTotalThreadsPerThreadgroup(self) }
    }

    #[inline]
    pub fn thread_execution_width(&self) -> usize {
        unsafe { rsel_threadExecutionWidth(self) }
    }

    #[inline]
    pub fn static_threadgroup_memory_length(&self) -> usize {
        unsafe { rsel_staticThreadgroupMemoryLength(self) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_maxTotalThreadsPerThreadgroup(id: &ns::Id) -> usize;
    fn wsel_setMaxTotalThreadsPerThreadgroup(id: &mut ns::Id, value: usize);
    // rwsel(, id, maxTotalThreadsPerThreadgroup, setMaxTotalThreadsPerThreadgroup, NSUInteger)
    fn rsel_threadExecutionWidth(id: &ns::Id) -> usize;
    fn rsel_staticThreadgroupMemoryLength(id: &ns::Id) -> usize;
}
