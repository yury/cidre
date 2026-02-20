use crate::{blocks, cf, define_obj_type, ns, objc, objc::Obj};

#[doc(alias = "MTLDrawablePresentedHandler")]
pub type DrawablePresentedHandler = blocks::EscBlock<fn(drawable: &AnyDrawable)>;

/// A displayable resource that can be rendered or written to.
///
/// Objects that implement this protocol are connected both to the
/// Metal framework and an underlying display system (such as Core Animation)
/// that’s capable of showing content onscreen. You use drawable objects
/// when you want to render images using Metal and present them onscreen.
///
/// Don’t implement this protocol yourself; instead, see ca::MetalLayer,
/// for a class that can create and manage drawable objects for you.
pub trait Drawable<T: Obj>: Obj {
    /// Presents the drawable onscreen as soon as possible.
    ///
    /// When a command queue schedules a command buffer for execution,
    /// it tracks whether any commands in that command buffer need to render
    /// or write to the drawable object. When you call this method, the drawable
    /// presents its contents as soon as possible after all scheduled render or write
    /// requests for that drawable are complete.
    ///
    /// # Note
    ///
    /// To avoid presenting a drawable before any work is scheduled, or to avoid holding
    /// on to a drawable longer than necessary, call a command buffer’s present_drawable
    /// method instead of this method. The present_drawable method is a convenience method
    /// that calls the drawable’s present method after the command queue schedules that
    /// command buffer for execution.
    #[objc::msg_send(present)]
    fn present(&self);

    /// Presents the drawable onscreen at a specific host time.
    ///
    /// When a command queue schedules a command buffer for execution, it tracks whether any commands
    /// in that command buffer need to render or write to the drawable object. When you
    /// call this method, the drawable waits until all render and write requests for that
    /// drawable are complete. If they complete prior to the specified time, the drawable
    /// presents the content at that time. If the commands complete after the presentation time,
    /// the drawable presents its contents as soon as possible.
    ///
    /// # Note
    ///
    /// To avoid presenting a drawable before any work is scheduled, or to avoid holding
    /// on to a drawable longer than necessary, call a command buffer’s present_drawable_at method
    /// instead of a drawable’s present_at method. The present_drawable_at method is a convenience
    /// method that calls the given drawable’s present_at method after the command queue schedules
    /// that command buffer for execution.
    ///
    /// # Arguments
    ///
    /// * `presentation_time` - The Mach absolute time at which the drawable should be presented, in seconds.
    #[objc::msg_send(presentAtTime:)]
    fn present_at(&self, presentation_time: cf::TimeInterval);

    /// Presents the drawable after a minimum amount of time has elapsed.
    ///
    /// # Arguments
    ///
    /// * `duration` - The minimum duration, in seconds, before this drawable appears onscreen.
    #[objc::msg_send(presentAfterMinimumDuration:)]
    fn present_after_minimum_duration(&self, duration: cf::TimeInterval);

    /// Adds a callback block that runs when this drawable appears onscreen.
    #[objc::msg_send(addPresentedHandler:)]
    fn add_presented_handler_block(&mut self, block: &mut DrawablePresentedHandler);

    /// Adds a callback that runs when this drawable appears onscreen.
    #[inline]
    fn add_presented_handler(&mut self, block: impl FnMut(&AnyDrawable) + Send + 'static) {
        let mut block = DrawablePresentedHandler::new1(block);
        self.add_presented_handler_block(&mut block);
    }

    /// The host time, in seconds, when this drawable appeared onscreen.
    ///
    /// Returns `0.0` if a frame hasn’t been presented or was skipped.
    #[objc::msg_send(presentedTime)]
    fn presented_time(&self) -> cf::TimeInterval;

    /// The monotonically increasing identifier for drawables from the same layer.
    #[objc::msg_send(drawableID)]
    fn drawable_id(&self) -> usize;
}

define_obj_type!(pub AnyDrawable(ns::Id));

impl Drawable<AnyDrawable> for AnyDrawable {}
