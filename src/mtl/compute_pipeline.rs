use crate::cf::Retained;
use crate::{define_mtl, define_obj_type};

use crate::ns::Id;

define_obj_type!(Reflection(Id));

define_obj_type!(Descriptor(Id));

impl Descriptor {
    define_mtl!(mut label);

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
    pub fn new<'create>() -> Retained<'create, Self> {
        unsafe { MTLComputePipelineDescriptor_new() }
    }

    pub fn thread_group_size_is_multiple_of_thread_execution_width(&self) -> bool {
        unsafe {
            rsel_threadGroupSizeIsMultipleOfThreadExecutionWidth(self)
        }
    }

    pub fn set_thread_group_size_is_multiple_of_thread_execution_width(&mut self, value: bool) {
        unsafe {
            wsel_setThreadGroupSizeIsMultipleOfThreadExecutionWidth(self, value)
        }
    }

}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLComputePipelineDescriptor_new<'create>() -> Retained<'create, Descriptor>;

    fn rsel_threadGroupSizeIsMultipleOfThreadExecutionWidth(id: &Id) -> bool;
    fn wsel_setThreadGroupSizeIsMultipleOfThreadExecutionWidth(id: &mut Id, value: bool);
}

define_obj_type!(State(Id));

impl State {
    define_mtl!(device, get label);

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
    fn rsel_maxTotalThreadsPerThreadgroup(id: &Id) -> usize;
    fn rsel_threadExecutionWidth(id: &Id) -> usize;
    fn rsel_staticThreadgroupMemoryLength(id: &Id) -> usize;
}
