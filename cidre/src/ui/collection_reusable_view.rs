use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UICollectionReusableView")]
    pub CollectionReusableView(ui::View)
);

impl CollectionReusableView {
    #[objc::msg_send(reuseIdentifier)]
    pub fn reuse_id(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(prepareForReuse)]
    pub fn prepare_for_reuse(&self);

    #[objc::msg_send(applyLayoutAttributes:)]
    pub fn apply_layout_attributes(&self, layout_attributes: &ui::CollectionViewLayoutAttrs);

    #[objc::msg_send(willTransitionFromLayout:toLayout:)]
    pub fn will_transition_from_layout(
        &self,
        old_layout: &ui::CollectionViewLayout,
        new_layout: &ui::CollectionViewLayout,
    );

    #[objc::msg_send(didTransitionFromLayout:toLayout:)]
    pub fn did_transition_from_layout(
        &self,
        old_layout: &ui::CollectionViewLayout,
        new_layout: &ui::CollectionViewLayout,
    );

    #[objc::msg_send(preferredLayoutAttributesFittingAttributes:)]
    pub fn preferred_layout_attributes_fitting_attributes(
        &self,
        layout_attributes: &ui::CollectionViewLayoutAttrs,
    ) -> arc::R<ui::CollectionViewLayoutAttrs>;
}
