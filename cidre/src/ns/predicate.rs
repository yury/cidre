#[cfg(feature = "blocks")]
use crate::blocks;
use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    pub Predicate(ns::Id),
    NS_PREDICATE
);

impl ns::Copying for Predicate {}

impl Predicate {
    #[objc::msg_send(predicateWithFormat:argumentArray:)]
    pub fn with_format(format: &ns::String, args: Option<&ns::Array<ns::Id>>) -> arc::R<Self>;

    #[cfg(target_os = "macos")]
    #[objc::msg_send(predicateFromMetadataQueryString:)]
    pub fn from_metadata_query_string(query: &ns::String) -> Option<arc::R<Self>>;

    #[objc::msg_send(predicateWithValue:)]
    pub fn with_value(value: bool) -> arc::R<Self>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(predicateWithBlock:)]
    pub fn with_block(
        block: &mut blocks::EscBlock<
            fn(
                evaluated_obj: Option<&ns::Id>,
                bindings: Option<&ns::Dictionary<ns::String, ns::Id>>,
            ) -> bool,
        >,
    ) -> arc::R<Self>;

    #[objc::msg_send(predicateFormat)]
    pub unsafe fn format_throws(&self) -> arc::R<ns::String>;

    pub fn format<'ear>(&self) -> ns::ExResult<'ear, arc::R<ns::String>> {
        ns::try_catch(|| unsafe { self.format_throws() })
    }

    #[objc::msg_send(evaluateWithObject:)]
    pub fn eval_with_obj(&self, obj: Option<&ns::Id>) -> bool;

    #[objc::msg_send(allowEvaluation)]
    pub fn allow_eval(&mut self);
}

/// NSPredicateSupport
impl<T: objc::Obj> ns::Array<T> {
    #[objc::msg_send(filteredArrayUsingPredicate:)]
    pub fn filtered_by_predicate(&self, predicate: &ns::Predicate) -> arc::R<Self>;
}

/// NSPredicateSupport
impl<T: objc::Obj> ns::ArrayMut<T> {
    #[objc::msg_send(filterUsingPredicate:)]
    pub fn filter_using_predicate(&mut self, predicate: &ns::Predicate);
}

/// NSPredicateSupport
impl<T: objc::Obj> ns::Set<T> {
    #[objc::msg_send(filteredSetUsingPredicate:)]
    pub fn filtered_by_predicate(&self, predicate: &ns::Predicate) -> arc::R<Self>;
}

/// NSPredicateSupport
impl<T: objc::Obj> ns::SetMut<T> {
    #[objc::msg_send(filterUsingPredicate:)]
    pub fn filter_using_predicate(&mut self, predicate: &ns::Predicate);
}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_PREDICATE: &'static objc::Class<Predicate>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let p = ns::Predicate::new();
        let format = p.format();
        assert!(format.is_err());

        let p = ns::Predicate::with_value(false);
        let format = p.format().unwrap();
        assert_eq!(format.as_ref(), "FALSEPREDICATE");
    }
}
