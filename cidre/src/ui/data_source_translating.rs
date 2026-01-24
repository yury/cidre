use crate::{define_obj_type, ns, objc};

#[objc::protocol(UIDataSourceTranslating)]
pub trait DataSourceTranslating: objc::Obj {}

define_obj_type!(
    pub AnyDataSourceTranslating(ns::Id)
);

impl DataSourceTranslating for AnyDataSourceTranslating {}
