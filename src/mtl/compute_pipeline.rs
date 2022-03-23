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
    /// ```
    pub fn new<'create>() -> Retained<'create, Self> {
        unsafe { MTLComputePipelineDescriptor_new() }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLComputePipelineDescriptor_new<'create>() -> Retained<'create, Descriptor>;
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
