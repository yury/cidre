use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    /// A toolbar separator that lines up with a divider of a split view.
    #[doc(alias = "NSTrackingSeparatorToolbarItem")]
    pub TrackingSeparatorToolbarItem(ns::ToolbarItem),
    sym NSTrackingSeparatorToolbarItem
);

impl TrackingSeparatorToolbarItem {
    /// A separator that tracks the divider at `divider_index` of `split_view`.
    #[objc::msg_send(trackingSeparatorToolbarItemWithIdentifier:splitView:dividerIndex:)]
    pub fn with_id(
        id: &ns::ToolbarItemId,
        split_view: &ns::SplitView,
        divider_index: ns::Integer,
    ) -> arc::R<Self>;

    #[objc::msg_send(splitView)]
    pub fn split_view(&self) -> arc::R<ns::SplitView>;

    #[objc::msg_send(setSplitView:)]
    pub fn set_split_view(&mut self, val: &ns::SplitView);

    #[objc::msg_send(dividerIndex)]
    pub fn divider_index(&self) -> ns::Integer;

    #[objc::msg_send(setDividerIndex:)]
    pub fn set_divider_index(&mut self, val: ns::Integer);
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let split_view = ns::SplitView::new();
        let id = ns::ToolbarItemId::with_string(ns::str!(c"tracking"));
        let mut item = ns::TrackingSeparatorToolbarItem::with_id(id, &split_view, 0);
        assert_eq!(item.divider_index(), 0);
        item.set_divider_index(1);
        assert_eq!(item.divider_index(), 1);
        assert!(item.split_view().is_equal(&split_view));
    }
}
