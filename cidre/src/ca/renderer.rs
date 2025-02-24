#[cfg(feature = "cv")]
use crate::cv;

#[cfg(feature = "mtl")]
use crate::mtl;

use crate::{arc, ca, cf, cg, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    pub OptKey(ns::String)
);

impl OptKey {
    /// The [`&cg::ColorSpace`] object defining the output color space.
    #[doc(alias = "kCARendererColorSpace")]
    #[inline]
    pub fn color_space() -> &'static Self {
        unsafe { kCARendererColorSpace }
    }

    /// The Metal Command Queue object against which to submit work.
    ///
    /// If the client provides a queue, then we will only commit our
    /// command buffer and let the client handle it's own synchronization
    /// and/or resource synchronization blits.
    ///
    /// If none is provided, then we will use an internal queue which
    /// automatically commits and waitUntilScheduled.
    #[doc(alias = "kCARendererMetalCommandQueue")]
    #[inline]
    pub fn metal_cmd_queue() -> &'static Self {
        unsafe { kCARendererMetalCommandQueue }
    }
}

define_obj_type!(
    #[doc(alias = "CARenderer")]
    pub Renderer(ns::Id)
);

impl Renderer {
    define_cls!(CA_RENDERER);

    #[cfg(feature = "mtl")]
    #[objc::msg_send(rendererWithMTLTexture:options:)]
    pub fn with_mtl_texture(
        texture: &mtl::Texture,
        options: Option<&ns::Dictionary<OptKey, ns::Id>>,
    ) -> arc::R<Self>;

    /// The root layer of the layer-tree the receiver should render.
    #[objc::msg_send(layer)]
    pub fn layer(&self) -> Option<arc::R<ca::Layer>>;

    #[objc::msg_send(setLayer:)]
    pub fn set_layer(&mut self, val: Option<&ca::Layer>);

    /// The bounds rect of the render target.
    #[objc::msg_send(bounds)]
    pub fn bounds(&self) -> cg::Rect;

    /// The bounds rect of the render target.
    #[objc::msg_send(setBounds:)]
    pub fn set_bounds(&mut self, val: cg::Rect);

    /// Begin rendering a frame at time 't'. If 'ts' is non-null it defines
    /// the host time and update frequency of the target device.
    #[cfg(feature = "cv")]
    #[objc::msg_send(beginFrameAtTime:timeStamp:)]
    pub fn begin_frame_at(&mut self, time_stamp: cf::TimeInterval, ts: Option<&cv::TimeStamp>);

    /// Returns the bounds of the update region - the area that contains all
    /// pixels that will be rendered by the current frame. Initially this
    /// will include all differences between the current frame and the
    /// previously rendered frame.
    #[objc::msg_send(updateBounds:)]
    pub fn update_bounds(&mut self, bounds: cg::Rect);

    /// Add a rectangle to the update region of the current frame.
    #[objc::msg_send(addUpdateRect:)]
    pub fn add_update_rect(&mut self, rect: cg::Rect);

    /// Render the update region of the current frame to the target context.
    #[objc::msg_send(render)]
    pub fn render(&mut self);

    /// Returns the time at which the next update should happen. If infinite
    /// no update needs to be scheduled yet. If the current frame time, a
    /// continuous animation is running and an update should be scheduled
    /// after a "natural" delay.
    #[objc::msg_send(nextFrameTime)]
    pub fn next_frame_time(&self) -> cf::TimeInterval;

    /// Release any data associated with the current frame.
    #[objc::msg_send(endFrame)]
    pub fn end_frame(&mut self);

    /// Change the renderer's destination Metal texture.
    #[cfg(feature = "mtl")]
    #[objc::msg_send(setDestination:)]
    pub fn set_dst(&mut self, val: &mtl::Texture);
}

#[link(name = "ca", kind = "static")]
unsafe extern "C" {
    static CA_RENDERER: &'static objc::Class<Renderer>;
}

unsafe extern "C" {
    static kCARendererColorSpace: &'static OptKey;
    static kCARendererMetalCommandQueue: &'static OptKey;
}
