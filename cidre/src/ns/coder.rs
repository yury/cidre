use crate::{arc, define_obj_type, ns, objc};

/// Describes the action an ns::Coder should take when it encounters decode failures (e.g. corrupt data)
/// for non-TopLevel decodes.
#[doc(alias = "NSDecodingFailurePolicy")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum DecodingFailurePolicy {
    /// On decode failure, the ns::Coder will raise an exception internally
    /// to propagate failure messages (and unwind the stack). This exception can be transformed
    /// into an ns::Error via any of the TopLevel decode APIs.
    RaiseException,

    /// On decode failure, the NSCoder will capture the failure as an ns::Error,
    /// and prevent further decodes (by returning 0 / None equivalent as appropriate).
    /// Clients should consider using this policy if they know that all encoded objects behave correctly
    /// in the presence of decode failures (e.g. they use fail_with_error to communicate decode failures
    /// and don't raise exceptions for error propagation)
    SetErrorAndReturn,
}

define_obj_type!(Coder(ns::Id), NS_CODER);
impl Coder {}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_CODER: &'static objc::Class<Coder>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let coder = ns::Coder::new();
        println!("{coder:?}");
    }
}
