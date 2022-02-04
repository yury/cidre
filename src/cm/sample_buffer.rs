use crate::{
    cf::{Retained, Type},
    define_cf_type,
};

define_cf_type!(SampleBuffer(Type));

impl SampleBuffer {}

extern "C" {}
