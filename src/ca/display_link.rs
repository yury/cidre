use std::ffi::c_void;

use crate::{define_obj_type, ns, cf, objc::Delegate};

define_obj_type!(DisplayLink(ns::Id));

pub trait DisplayLinkDelegate {
    extern "C" fn on_display_link(
        &mut self,
        link: &DisplayLink,
    );

    fn delegate(self) -> Delegate<Self>
    where
        Self: Sized,
    {
        let b = Box::new(self);
        let table: [*const c_void; 2] = [
            b.as_ref() as *const _ as _,
            // &self as *const _ as *const _,
            Self::on_display_link as _,
        ];

        let ptr = table.as_ptr();
        let obj = unsafe { make_display_link_delegate(ptr as _) };

        Delegate { delegate: b, obj }
    }
}

#[link(name = "ca", kind = "static")]
extern "C" {
    fn make_display_link_delegate<'a>(vtable: *const *const c_void) -> cf::Retained<'a, ns::Id>;
}