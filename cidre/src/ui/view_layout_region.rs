use crate::{api, arc, cg, define_obj_type, ns, objc, ui};

/// The axis along which a layout region adapts to the corners of its container.
#[doc(alias = "UIViewLayoutRegionAdaptivityAxis")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(usize)]
pub enum ViewLayoutRegionAdaptivityAxis {
    #[doc(alias = "UIViewLayoutRegionAdaptivityAxisNone")]
    None,
    #[doc(alias = "UIViewLayoutRegionAdaptivityAxisHorizontal")]
    Horizontal,
    #[doc(alias = "UIViewLayoutRegionAdaptivityAxisVertical")]
    Vertical,
}

define_obj_type!(
    /// A region of a view to lay content in: resolve it with
    /// [`ui::View::layout_guide_for_layout_region`] or the insets variants.
    #[doc(alias = "UIViewLayoutRegion")]
    pub ViewLayoutRegion(ns::Id)
);

impl ViewLayoutRegion {
    #[api::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    crate::define_cls!(UI_VIEW_LAYOUT_REGION);

    #[objc::msg_send(safeAreaLayoutRegionWithCornerAdaptation:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn safe_area(corner_adaptation: ViewLayoutRegionAdaptivityAxis) -> arc::R<Self>;

    #[objc::msg_send(marginsLayoutRegionWithCornerAdaptation:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn margins(corner_adaptation: ViewLayoutRegionAdaptivityAxis) -> arc::R<Self>;

    #[objc::msg_send(readableContentLayoutRegionWithCornerAdaptation:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn readable_content(corner_adaptation: ViewLayoutRegionAdaptivityAxis) -> arc::R<Self>;

    /// A bar layout region of a given extent on a given edge.
    ///
    /// Only a single edge is allowed per region.
    #[objc::msg_send(layoutRegionForBarOnEdge:extent:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn for_bar_on_edge(edge: ui::RectEdge, extent: cg::Float) -> arc::R<Self>;

    /// A bar layout region of a given extent on a given directional edge.
    ///
    /// Only a single edge is allowed per region.
    #[objc::msg_send(layoutRegionForBarOnDirectionalEdge:extent:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn for_bar_on_directional_edge(
        edge: ui::DirectionalRectEdge,
        extent: cg::Float,
    ) -> arc::R<Self>;
}

unsafe extern "C" {
    static UI_VIEW_LAYOUT_REGION: &'static objc::Class<ViewLayoutRegion>;
}

#[cfg(test)]
mod tests {
    use crate::ui;

    #[test]
    fn basics() {
        // needs ios 26.0
        unsafe {
            let a = ui::ViewLayoutRegion::safe_area(ui::ViewLayoutRegionAdaptivityAxis::Vertical);
            let b = ui::ViewLayoutRegion::margins(ui::ViewLayoutRegionAdaptivityAxis::None);
            assert!(!a.is_equal(&b));

            let view = ui::View::new();
            let _guide = view.layout_guide_for_layout_region(&a);
            let _insets = view.edge_insets_for_layout_region(&b);
        }
    }
}
