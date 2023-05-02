use crate::{arc, cf, cv, define_cls, define_obj_type, ns, objc};

define_obj_type!(Renderer(ns::Id));

impl Renderer {
    define_cls!(CA_RENDERER);

    #[objc::cls_msg_send(rendererWithMTLTexture:options:)]
    pub fn with_mtl_texture_ar(
        texture: &mtl::Texture,
        options: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> &'ar Self;

    #[objc::rar_retain]
    pub fn with_mtl_texture(
        texture: &mtl::Texture,
        options: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<Self>;

    #[objc::msg_send(layer)]
    pub fn layer(&self) -> Option<&ca::Layer>;

    #[objc::msg_send(setLayer:)]
    pub fn set_layer(&mut self, value: Option<&ca::Layer>);

    /// The bounds rect of the render target.
    #[objc::msg_send(bounds)]
    pub fn bounds(&self) -> cg::Rect;

    #[objc::msg_send(setBounds:)]
    pub fn set_bounds(&mut self, value: cg::Rect);

    #[objc::msg_send(beginFrameAtTime:timeStamp:)]
    pub fn begin_frame_at(&mut self, time_stamp: cf::TimeInterval, ts: Option<&cv::TimeStamp>);

    #[objc::msg_send(updateBounds)]
    pub fn update_bounds(&mut self, bounds: cg::Rect);

    #[objc::msg_send(addUpdateRect:)]
    pub fn add_update_rect(&mut self, rect: cg::Rect);

    #[objc::msg_send(render)]
    pub fn render(&mut self);

    #[objc::msg_send(nextFrameTime)]
    pub fn next_frame_time(&self) -> cf::TimeInterval;

    #[objc::msg_send(endFrame)]
    pub fn end_frame(&mut self);

    #[objc::msg_send(setDestination:)]
    pub fn set_destination(&mut self, texture: &mtl::Texture);
}

#[link(name = "ca", kind = "static")]
extern "C" {
    static CA_RENDERER: &'static objc::Class<Renderer>;
}
