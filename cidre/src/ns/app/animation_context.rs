use crate::{arc, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

#[cfg(feature = "ca")]
use crate::ca;

define_obj_type!(
    #[doc(alias = "NSAnimationContext")]
    pub AnimationCtx(ns::Id),
    NS_ANIMATION_CONTEXT
);

impl AnimationCtx {
    #[objc::msg_send(beginGrouping)]
    pub fn begin_grouping();

    #[objc::msg_send(endGrouping)]
    pub fn end_grouping();

    #[objc::msg_send(currentContext)]
    pub fn current() -> arc::R<Self>;

    #[objc::msg_send(duration)]
    pub fn duration(&self) -> ns::TimeInterval;

    #[objc::msg_send(setDuration:)]
    pub fn set_duration(&mut self, val: ns::TimeInterval);

    #[cfg(feature = "ca")]
    #[objc::msg_send(timingFunction)]
    #[objc::available(macos = 10.7)]
    pub fn timing_fn(&self) -> Option<arc::R<ca::MediaTimingFn>>;

    #[cfg(feature = "ca")]
    #[objc::msg_send(setTimingFunction:)]
    #[objc::available(macos = 10.7)]
    pub fn set_timing_fn(&mut self, val: Option<&ca::MediaTimingFn>);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(completionHandler)]
    #[objc::available(macos = 10.7)]
    pub fn completion_handler(&mut self) -> Option<&mut blocks::EscBlock<fn()>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setCompletionHandler:)]
    #[objc::available(macos = 10.7)]
    pub fn set_completion_handler(&mut self, val: Option<&mut blocks::EscBlock<fn()>>);

    #[objc::msg_send(allowsImplicitAnimation)]
    #[objc::available(macos = 10.8)]
    pub fn allows_implicit_animation(&self) -> bool;

    #[objc::msg_send(setAllowsImplicitAnimation:)]
    #[objc::available(macos = 10.8)]
    pub fn set_allows_implicit_animation(&mut self, val: bool);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(runAnimationGroup:completionHandler:)]
    #[objc::available(macos = 10.7)]
    pub fn run_animation_group_ch_block(
        changes: &mut blocks::NoEscBlock<fn(&mut ns::AnimationCtx)>,
        completion_handler: Option<&mut blocks::EscBlock<fn()>>,
    );

    #[cfg(feature = "blocks")]
    #[objc::available(macos = 10.7)]
    #[allow(unused_unsafe)]
    pub fn run_animation_group_ch(
        mut changes: impl FnMut(&mut ns::AnimationCtx),
        completion_handler: impl FnMut() + 'static,
    ) {
        let mut changes = unsafe { blocks::NoEscBlock::stack1(&mut changes) };
        let mut completion_handler = blocks::EscBlock::new0(completion_handler);
        unsafe {
            Self::run_animation_group_ch_block(&mut changes, Some(&mut completion_handler));
        }
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(runAnimationGroup:)]
    #[objc::available(macos = 10.12)]
    pub fn run_animation_group_block(changes: &mut blocks::NoEscBlock<fn(&mut ns::AnimationCtx)>);

    #[cfg(feature = "blocks")]
    #[objc::available(macos = 10.12)]
    #[allow(unused_unsafe)]
    pub fn run_animation_group(mut changes: impl FnMut(&mut ns::AnimationCtx)) {
        let mut changes = unsafe { blocks::NoEscBlock::stack1(&mut changes) };
        unsafe {
            Self::run_animation_group_block(&mut changes);
        }
    }
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_ANIMATION_CONTEXT: &'static objc::Class<AnimationCtx>;
}
