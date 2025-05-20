use crate::{arc, define_cls, define_obj_type, ns, objc, wk};

#[doc(alias = "WKUserScriptInjectionTime")]
#[repr(isize)]
pub enum UserScriptInjectionTime {
    AtDocumentStart,
    AtDocumentEnd,
}

define_obj_type!(
    pub UserScript(ns::Id)
);

impl arc::A<UserScript> {
    #[objc::msg_send(initWithSource:injectionTime:forMainFrameOnly:)]
    pub fn init_with_src_injection_time_for_main_frame_only(
        self,
        src: &ns::String,
        injection_time: wk::UserScriptInjectionTime,
        for_main_frame_only: bool,
    ) -> arc::R<UserScript>;
}

impl UserScript {
    define_cls!(WK_USER_SCRIPT);

    pub fn with_src(
        src: &ns::String,
        injection_time: wk::UserScriptInjectionTime,
        for_main_frame_only: bool,
    ) -> arc::R<Self> {
        Self::alloc().init_with_src_injection_time_for_main_frame_only(
            src,
            injection_time,
            for_main_frame_only,
        )
    }

    #[objc::msg_send(source)]
    pub fn src(&self) -> arc::R<ns::String>;

    #[objc::msg_send(injectionTime)]
    pub fn injection_time(&self) -> wk::UserScriptInjectionTime;

    #[objc::msg_send(isForMainFrameOnly)]
    pub fn is_for_main_frame_only(&self) -> bool;
}

#[link(name = "wk", kind = "static")]
unsafe extern "C" {
    static WK_USER_SCRIPT: &'static objc::Class<UserScript>;
}
