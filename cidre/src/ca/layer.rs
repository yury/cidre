use crate::{api, arc, ca, cf, cg, define_obj_type, define_opts, ns, objc};

define_obj_type!(
    #[doc(alias = "CALayerContentsGravity")]
    pub ContentsGravity(ns::String)
);

define_obj_type!(
    #[doc(alias = "CALayerContentsFormat")]
    pub ContentsFormat(ns::String)
);

define_obj_type!(
    #[doc(alias = "CALayerContentsFilter")]
    pub ContentsFilter(ns::String)
);

define_obj_type!(
    #[doc(alias = "CALayerCornerCurve")]
    pub CornerCurve(ns::String)
);

define_obj_type!(
    /// Options that control when to tone map ca::Layer contents and
    /// ca::MetalLayer drawables. Defaults to
    #[doc(alias = "CALayer.ToneMapMode")]
    #[doc(alias = "CAToneMapMode")]
    pub ToneMapMode(ns::String)
);

impl ToneMapMode {
    /// Let the OS decide whether to tone map.
    #[api::available(
        macos = 15.0,
        maccatalyst = 18.0,
        ios = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn automatic() -> &'static Self {
        unsafe { CAToneMapModeAutomatic }
    }

    /// Never tone map contents.
    #[api::available(
        macos = 15.0,
        maccatalyst = 18.0,
        ios = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn never() -> &'static Self {
        unsafe { CAToneMapModeNever }
    }

    /// Tone map whenever supported by the OS. This includes
    /// PQ, HLG and extended-range contents for ca::Layer
    /// and ca::MetalLayers.
    #[api::available(
        macos = 15.0,
        maccatalyst = 18.0,
        ios = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn if_supported() -> &'static Self {
        unsafe { CAToneMapModeIfSupported }
    }
}

define_opts!(
    #[doc(alias = "CAAutoresizingMask")]
    pub AutoresizingMask(u32)
);

impl AutoresizingMask {
    pub const NOT_SIZABLE: Self = Self(0);
    pub const MIN_X_MARGIN: Self = Self(1 << 0);
    pub const WIDTH_SIZABLE: Self = Self(1 << 1);
    pub const MAX_X_MARGIN: Self = Self(1 << 2);
    pub const MIN_Y_MARGIN: Self = Self(1 << 3);
    pub const HEIGHT_SIZABLE: Self = Self(1 << 4);
    pub const MAX_Y_MARGIN: Self = Self(1 << 5);
}

define_opts!(
    #[doc(alias = "CAEdgeAntialiasingMask")]
    pub EdgeAntialiasingMask(u32)
);

impl EdgeAntialiasingMask {
    /// Minimum X edge.
    pub const LEFT: Self = Self(1 << 0);

    /// Maximum X edge.
    pub const RIGHT: Self = Self(1 << 1);

    /// Minimum Y edge.
    pub const BOTTOM: Self = Self(1 << 2);

    /// Maximum Y edge.
    pub const TOP: Self = Self(1 << 3);
}

define_opts!(
    #[doc(alias = "CACornerMask")]
    pub CornerMask(usize)
);
impl CornerMask {
    pub const MIN_X_MIN_Y: Self = Self(1 << 0);
    pub const MAX_X_MIN_Y: Self = Self(1 << 1);
    pub const MIN_X_MAX_Y: Self = Self(1 << 2);
    pub const MAX_X_MAX_Y: Self = Self(1 << 3);
}

define_obj_type!(
    #[doc(alias = "CALayer")]
    pub Layer(ns::Id),
    CA_LAYER
);

impl Layer {
    #[objc::msg_send(bounds)]
    pub fn bounds(&self) -> cg::Rect;

    #[objc::msg_send(setBounds:)]
    pub fn set_bounds(&mut self, val: cg::Rect);

    #[objc::msg_send(position)]
    pub fn pos(&self) -> cg::Point;

    #[objc::msg_send(setPosition:)]
    pub fn set_pos(&mut self, val: cg::Point);

    #[objc::msg_send(zPosition)]
    pub fn z_pos(&self) -> cg::Float;

    #[objc::msg_send(setZPosition:)]
    pub fn set_z_pos(&mut self, val: cg::Float);

    #[objc::msg_send(anchorPoint)]
    pub fn anchor_point(&self) -> cg::Point;

    #[objc::msg_send(setAnchorPoint:)]
    pub fn set_anchor_point(&mut self, val: cg::Point);

    #[objc::msg_send(transform)]
    pub fn transform(&self) -> ca::Transform3d;

    #[objc::msg_send(setTransform:)]
    pub fn set_transform(&mut self, val: ca::Transform3d);

    /// Convenience methods for accessing the `transform' property as an affine transform.
    #[objc::msg_send(affineTransform)]
    pub fn affine_transform(&self) -> cg::AffineTransform;

    #[objc::msg_send(setAffineTransform:)]
    pub fn set_affine_transform(&self, val: cg::AffineTransform);

    #[objc::msg_send(frame)]
    pub fn frame(&self) -> cg::Rect;

    #[objc::msg_send(setFrame:)]
    pub fn set_frame(&mut self, val: cg::Rect);

    #[objc::msg_send(isHidden)]
    pub fn is_hidden(&self) -> bool;

    #[objc::msg_send(setHidden:)]
    pub fn set_hidden(&mut self, val: bool);

    #[objc::msg_send(addSublayer:)]
    pub fn add_sublayer(&mut self, layer: &Self);

    #[objc::msg_send(insertSublayer:atIndex:)]
    pub fn insert_sublayer_at(&mut self, layer: &Self, index: u32);

    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyLayerDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: LayerDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(needsLayout)]
    pub fn needs_layout(&self) -> bool;

    #[objc::msg_send(setNeedsLayout)]
    pub fn set_needs_layout(&mut self);

    #[objc::msg_send(layoutIfNeeded)]
    pub fn layout_if_needed(&mut self);

    #[objc::msg_send(layoutSublayers)]
    pub fn layout_sublayers(&self);

    #[objc::msg_send(resizeSublayersWithOldSize:)]
    pub fn resize_sublayers_with_old_size(&self, size: cg::Size);

    #[objc::msg_send(resizeWithOldSuperlayerSize:)]
    pub fn resize_with_old_superlayer_size(&self, size: cg::Size);

    #[objc::msg_send(removeAllAnimations)]
    pub fn remove_all_animations(&mut self);

    #[objc::msg_send(removeAnimationForKey:)]
    pub fn remove_animation_for_key(&mut self, key: &ns::String);

    #[objc::msg_send(animationKeys)]
    pub fn animation_keys(&self) -> Option<arc::R<ns::Array<ns::String>>>;

    #[objc::msg_send(contents)]
    pub fn contents(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setContents:)]
    pub fn set_ns_contents(&mut self, contents: Option<&ns::Id>);

    #[objc::msg_send(setContents:)]
    pub fn set_cf_contents(&mut self, contents: Option<&cf::Type>);

    #[objc::msg_send(contentsRect)]
    pub fn contents_rect(&self) -> cg::Rect;

    #[objc::msg_send(setContentsRect:)]
    pub fn set_contents_rect(&mut self, val: cg::Rect);

    #[objc::msg_send(contentsGravity)]
    pub fn contents_gravity(&self) -> arc::R<ContentsGravity>;

    #[objc::msg_send(setContentsGravity:)]
    pub fn set_contents_gravity(&mut self, val: &ContentsGravity);

    #[objc::msg_send(contentsScale)]
    pub fn contents_scale(&self) -> cg::Float;

    #[objc::msg_send(setContentsScale:)]
    pub fn set_contents_scale(&mut self, val: cg::Float);

    #[objc::msg_send(contentsCenter)]
    pub fn contents_center(&self) -> cg::Rect;

    #[objc::msg_send(setContentsCenter:)]
    pub fn set_contents_center(&mut self, val: cg::Rect);

    #[objc::msg_send(contentsFormat)]
    pub fn contents_format(&self) -> arc::R<ContentsFormat>;

    #[objc::msg_send(setContentsFormat:)]
    pub fn set_contents_format(&mut self, val: &ContentsFormat);

    #[objc::msg_send(toneMapMode)]
    #[api::available(
        macos = 15.0,
        maccatalyst = 18.0,
        ios = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn tone_map_mode(&self) -> arc::R<ToneMapMode>;

    #[objc::msg_send(setToneMapMode:)]
    #[api::available(
        macos = 15.0,
        maccatalyst = 18.0,
        ios = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn set_tone_map_mode(&mut self, val: &ToneMapMode);

    #[objc::msg_send(wantsExtendedDynamicRangeContent)]
    pub fn wants_extended_dynamic_range_content(&self) -> bool;

    #[objc::msg_send(setWantsExtendedDynamicRangeContent:)]
    pub fn set_wants_extended_dynamic_range_content(&mut self, val: bool);
}

#[objc::protocol(CAAction)]
pub trait Action: objc::Obj {
    #[objc::msg_send(runActionForKey:object:arguments:)]
    fn run_action_for_key(
        &mut self,
        event: &ns::String,
        obj: &ns::Id,
        args: Option<&ns::Dictionary<ns::String, ns::Id>>,
    );
}

impl Action for ns::Null {}

define_obj_type!(
    pub AnyAction(ns::Id)
);

impl Action for AnyAction {}

#[objc::protocol(CALayerDelegate)]
pub trait LayerDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(displayLayer:)]
    fn display_layer(&mut self, layer: &mut Layer);

    #[objc::optional]
    #[objc::msg_send(drawLayer:inContext:)]
    fn draw_layer(&mut self, layer: &mut Layer, context: &mut cg::Context);

    #[objc::optional]
    #[objc::msg_send(layerWillDraw:)]
    fn layer_will_draw(&mut self, layer: &mut Layer);

    #[objc::optional]
    #[objc::msg_send(layoutSublayersOfLayer:)]
    fn layout_sublayers_of_layer(&mut self, layer: &mut Layer);

    #[objc::optional]
    #[objc::msg_send(actionForLayer:forKey:)]
    fn action_for_layer_for_key(
        &mut self,
        layer: &mut Layer,
        key: &ns::String,
    ) -> Option<arc::R<AnyAction>>;
}

define_obj_type!(
    pub AnyLayerDelegate(ns::Id)
);

impl LayerDelegate for AnyLayerDelegate {}

#[link(name = "ca", kind = "static")]
unsafe extern "C" {
    static CA_LAYER: &'static objc::Class<Layer>;
}

#[link(name = "QuartzCore", kind = "framework")]
#[api::weak]
unsafe extern "C" {
    #[api::available(
        macos = 15.0,
        maccatalyst = 18.0,
        ios = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    static CAToneMapModeAutomatic: &'static ToneMapMode;

    #[api::available(
        macos = 15.0,
        maccatalyst = 18.0,
        ios = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    static CAToneMapModeNever: &'static ToneMapMode;

    #[api::available(
        macos = 15.0,
        maccatalyst = 18.0,
        ios = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    static CAToneMapModeIfSupported: &'static ToneMapMode;
}
