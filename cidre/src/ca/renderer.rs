#[cfg(feature = "cv")]
use crate::cv;

#[cfg(feature = "mtl")]
use crate::mtl;

use crate::{arc, ca, cf, cg, define_cls, define_obj_type, ns, objc};

define_obj_type!(pub OptionKey(ns::String));

impl OptionKey {
    #[inline]
    pub fn color_space() -> &'static Self {
        unsafe { kCARendererColorSpace }
    }

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
        options: Option<&ns::Dictionary<OptionKey, ns::Id>>,
    ) -> arc::R<Self>;

    /// The root layer of the layer-tree the receiver should render.
    #[objc::msg_send(layer)]
    pub fn layer(&self) -> Option<arc::R<ca::Layer>>;

    #[objc::msg_send(setLayer:)]
    pub fn set_layer(&mut self, val: Option<&ca::Layer>);

    /// The bounds rect of the render target.
    #[objc::msg_send(bounds)]
    pub fn bounds(&self) -> cg::Rect;

    #[objc::msg_send(setBounds:)]
    pub fn set_bounds(&mut self, val: cg::Rect);

    #[cfg(feature = "cv")]
    #[objc::msg_send(beginFrameAtTime:timeStamp:)]
    pub fn begin_frame_at(&mut self, time_stamp: cf::TimeInterval, ts: Option<&cv::TimeStamp>);

    #[objc::msg_send(updateBounds:)]
    pub fn update_bounds(&mut self, bounds: cg::Rect);

    #[objc::msg_send(addUpdateRect:)]
    pub fn add_update_rect(&mut self, rect: cg::Rect);

    #[objc::msg_send(render)]
    pub fn render(&mut self);

    #[objc::msg_send(nextFrameTime)]
    pub fn next_frame_time(&self) -> cf::TimeInterval;

    #[objc::msg_send(endFrame)]
    pub fn end_frame(&mut self);

    #[cfg(feature = "mtl")]
    #[objc::msg_send(setDestination:)]
    pub fn set_dst(&mut self, val: &mtl::Texture);
}

#[link(name = "ca", kind = "static")]
unsafe extern "C" {
    static CA_RENDERER: &'static objc::Class<Renderer>;
}

unsafe extern "C" {
    static kCARendererColorSpace: &'static OptionKey;
    static kCARendererMetalCommandQueue: &'static OptionKey;
}
