use crate::{define_obj_type, ns};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum Status {
    /// Indicates that the status of the asset reader is not currently known.
    Unknown = 0,
    /// Indicates that the asset reader is successfully reading samples from its asset.
    Reading = 1,
    /// Indicates that the asset reader has successfully read all of the samples in its time range.
    Completed = 2,
    /// Indicates that the asset reader can no longer read samples from its asset because of an error. The error is described by the value of the asset reader's error property.
    Failed = 3,
    /// Indicates that the asset reader can no longer read samples because reading was canceled with the cancelReading method.
    Cancelled = 4,
}


define_obj_type!(Reader(ns::Id));