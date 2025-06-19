use crate::{cf, define_cf_type};

define_cf_type!(
    #[doc(alias = "CGContext")]
    #[doc(alias = "CGContextRef")]
    Context(cf::Type)
);
