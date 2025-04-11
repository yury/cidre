use crate::{arc, blocks, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIZoomTransitionOptions")]
    pub ZoomTransitionOpts(ns::Id),
    UI_ZOOM_TRANSITION_OPTIONS
);

#[cfg(not(target_os = "watchos"))]
impl ZoomTransitionOpts {
    #[objc::msg_send(interactiveDismissShouldBegin)]
    pub fn interactive_dismiss_should_begin(
        &self,
    ) -> Option<arc::R<blocks::EscBlock<fn(ctx: &ui::ZoomTransitionInteractionCtx) -> bool>>>;

    /// Called when an interactive dismissal of this transition begins.
    /// Return value indicates whether the interaction should begin for the given context.
    #[objc::msg_send(setInteractiveDismissShouldBegin:)]
    pub fn set_interactive_dismiss_should_begin_block(
        &mut self,
        block: Option<&mut blocks::EscBlock<fn(ctx: &ui::ZoomTransitionInteractionCtx) -> bool>>,
    );
    pub fn set_interactive_dismiss_should_begin(
        &mut self,
        block: impl FnMut(&ui::ZoomTransitionInteractionCtx) -> bool + 'static,
    ) {
        let mut block = blocks::EscBlock::new1(block);
        self.set_interactive_dismiss_should_begin_block(Some(&mut block));
    }

    /// Return a frame in the zoomed view controller's view to which to align the source view.
    /// Return `cg::Rect::null()` to indicate no preference.
    #[objc::msg_send(alignmentRectProvider)]
    pub fn alignment_rect_provider(
        &self,
    ) -> Option<arc::R<blocks::EscBlock<fn(ctx: &ui::ZoomTransitionAlignmentRectCtx) -> cg::Rect>>>;

    #[objc::msg_send(setAlignmentRectProvider:)]
    pub fn set_alignment_rect_provider_block(
        &mut self,
        block: Option<
            &mut blocks::EscBlock<fn(ctx: &ui::ZoomTransitionAlignmentRectCtx) -> cg::Rect>,
        >,
    );

    pub fn set_alignment_rect_provider(
        &mut self,
        block: impl FnMut(&ui::ZoomTransitionAlignmentRectCtx) -> cg::Rect + 'static,
    ) {
        let mut block = blocks::EscBlock::new1(block);
        self.set_alignment_rect_provider_block(Some(&mut block));
    }

    #[objc::msg_send(dimmingColor)]
    pub fn dimming_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setDimmingColor:)]
    pub fn set_dimming_color(&mut self, val: Option<&ui::Color>);

    #[objc::msg_send(dimmingVisualEffect)]
    pub fn dimming_visual_effect(&self) -> Option<arc::R<ui::BlurEffect>>;

    #[objc::msg_send(setDimmingVisualEffect:)]
    pub fn set_dimming_visual_effect(&mut self, val: Option<&ui::BlurEffect>);
}

define_obj_type!(
    #[doc(alias = "UIZoomTransitionAlignmentRectContext")]
    pub ZoomTransitionInteractionCtx(ns::Id)
);

#[cfg(not(target_os = "watchos"))]
impl ZoomTransitionInteractionCtx {
    /// Location of the interaction in the displayed view controller's view's coordinate space.
    #[objc::msg_send(location)]
    pub fn location(&self) -> cg::Point;

    /// The interaction's velocity.
    #[objc::msg_send(velocity)]
    pub fn velocity(&self) -> cg::Vector;

    /// Whether the interaction would begin under the current conditions by default.
    #[objc::msg_send(willBegin)]
    pub fn will_begin(&self) -> bool;
}

define_obj_type!(
    #[doc(alias = "UIZoomTransitionAlignmentRectContext")]
    pub ZoomTransitionAlignmentRectCtx(ns::Id)
);

#[cfg(not(target_os = "watchos"))]
impl ZoomTransitionAlignmentRectCtx {
    #[objc::msg_send(sourceView)]
    pub fn src_view(&self) -> arc::R<ui::View>;

    #[objc::msg_send(zoomedViewController)]
    pub fn zoomed_vc(&self) -> arc::R<ui::ViewController>;
}

#[cfg(not(target_os = "watchos"))]
unsafe extern "C" {
    static UI_ZOOM_TRANSITION_OPTIONS: &'static objc::Class<ZoomTransitionOpts>;
}
