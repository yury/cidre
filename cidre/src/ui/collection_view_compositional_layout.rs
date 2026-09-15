use crate::{arc, cg, define_obj_type, ns, objc, ui};

#[cfg(feature = "blocks")]
use crate::blocks;

/// The layout of a section, given its index and the environment: the
/// container's size and insets, and the traits.
#[cfg(feature = "blocks")]
pub type CollectionLayoutSectionProvider = blocks::EscBlock<
    fn(
        section_index: ns::Integer,
        env: &ui::CollectionLayoutEnvironment,
    ) -> Option<arc::Rar<ui::CollectionLayoutSection>>,
>;

define_obj_type!(
    #[doc(alias = "UICollectionViewCompositionalLayout")]
    pub CollectionViewCompositionalLayout(ui::CollectionViewLayout),
    UI_COLLECTION_VIEW_COMPOSITIONAL_LAYOUT
);

impl CollectionViewCompositionalLayout {
    /// A layout whose every section is laid out as `section`.
    #[objc::init(initWithSection:)]
    #[objc::available(ios = 13.0, tvos = 13.0)]
    pub fn init_with_section(
        self,
        section: &ui::CollectionLayoutSection,
    ) -> arc::R<CollectionViewCompositionalLayout>;

    #[objc::available(ios = 13.0, tvos = 13.0)]
    pub fn with_section(section: &ui::CollectionLayoutSection) -> arc::R<Self> {
        Self::alloc().init_with_section(section)
    }

    /// A layout asking `provider` for each section's layout.
    #[cfg(feature = "blocks")]
    #[objc::init(initWithSectionProvider:)]
    #[objc::available(ios = 13.0, tvos = 13.0)]
    pub fn init_with_section_provider(
        self,
        provider: &mut CollectionLayoutSectionProvider,
    ) -> arc::R<CollectionViewCompositionalLayout>;

    #[cfg(feature = "blocks")]
    #[objc::available(ios = 13.0, tvos = 13.0)]
    pub fn with_section_provider(
        provider: impl FnMut(
            ns::Integer,
            &ui::CollectionLayoutEnvironment,
        ) -> Option<arc::Rar<ui::CollectionLayoutSection>>
        + 'static,
    ) -> arc::R<Self> {
        let mut provider = CollectionLayoutSectionProvider::new2(provider);
        Self::alloc().init_with_section_provider(&mut provider)
    }
}

define_obj_type!(
    /// What a section is laid out in.
    #[doc(alias = "NSCollectionLayoutContainer")]
    pub CollectionLayoutContainer(ns::Id)
);

impl CollectionLayoutContainer {
    #[objc::msg_send(contentSize)]
    pub fn content_size(&self) -> cg::Size;

    #[objc::msg_send(effectiveContentSize)]
    pub fn effective_content_size(&self) -> cg::Size;

    #[objc::msg_send(contentInsets)]
    pub fn content_insets(&self) -> ui::DirectionalEdgeInsets;

    #[objc::msg_send(effectiveContentInsets)]
    pub fn effective_content_insets(&self) -> ui::DirectionalEdgeInsets;
}

define_obj_type!(
    /// What a section provider lays out for.
    #[doc(alias = "NSCollectionLayoutEnvironment")]
    pub CollectionLayoutEnvironment(ns::Id)
);

impl CollectionLayoutEnvironment {
    #[objc::msg_send(container)]
    pub fn container(&self) -> arc::R<CollectionLayoutContainer>;

    #[objc::msg_send(traitCollection)]
    pub fn trait_collection(&self) -> arc::R<ui::TraitCollection>;
}

/// How a section's groups scroll against the layout's main axis.
#[doc(alias = "UICollectionLayoutSectionOrthogonalScrollingBehavior")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CollectionLayoutSectionOrthogonalScrollingBehavior {
    None = 0,
    Continuous = 1,
    ContinuousGroupLeadingBoundary = 2,
    Paging = 3,
    GroupPaging = 4,
    GroupPagingCentered = 5,
}

#[doc(alias = "NSRectAlignment")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum RectAlignment {
    None = 0,
    Top = 1,
    TopLeading = 2,
    Leading = 3,
    BottomLeading = 4,
    Bottom = 5,
    BottomTrailing = 6,
    Trailing = 7,
    TopTrailing = 8,
}

define_obj_type!(
    #[doc(alias = "NSCollectionLayoutDimension")]
    pub CollectionLayoutDimension(ns::Id),
    NS_COLLECTION_LAYOUT_DIMENSION
);

impl CollectionLayoutDimension {
    #[objc::msg_send(fractionalWidthDimension:)]
    pub fn fractional_width(val: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(fractionalHeightDimension:)]
    pub fn fractional_height(val: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(absoluteDimension:)]
    pub fn absolute(val: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(estimatedDimension:)]
    pub fn estimated(val: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(dimension)]
    pub fn dimension(&self) -> cg::Float;
}

define_obj_type!(
    #[doc(alias = "NSCollectionLayoutSize")]
    pub CollectionLayoutSize(ns::Id),
    NS_COLLECTION_LAYOUT_SIZE
);

impl CollectionLayoutSize {
    #[objc::msg_send(sizeWithWidthDimension:heightDimension:)]
    pub fn with_width_height(
        width: &CollectionLayoutDimension,
        height: &CollectionLayoutDimension,
    ) -> arc::R<Self>;
}

define_obj_type!(
    #[doc(alias = "NSCollectionLayoutItem")]
    pub CollectionLayoutItem(ns::Id),
    NS_COLLECTION_LAYOUT_ITEM
);

impl CollectionLayoutItem {
    #[objc::msg_send(itemWithLayoutSize:)]
    pub fn with_layout_size(size: &CollectionLayoutSize) -> arc::R<Self>;

    #[objc::msg_send(setContentInsets:)]
    pub fn set_content_insets(&mut self, val: ui::DirectionalEdgeInsets);
}

define_obj_type!(
    #[doc(alias = "NSCollectionLayoutGroup")]
    pub CollectionLayoutGroup(CollectionLayoutItem),
    NS_COLLECTION_LAYOUT_GROUP
);

impl CollectionLayoutGroup {
    #[objc::msg_send(horizontalGroupWithLayoutSize:subitems:)]
    pub fn horizontal_with_layout_size_subitems(
        size: &CollectionLayoutSize,
        subitems: &ns::Array<CollectionLayoutItem>,
    ) -> arc::R<Self>;

    #[objc::msg_send(verticalGroupWithLayoutSize:subitems:)]
    pub fn vertical_with_layout_size_subitems(
        size: &CollectionLayoutSize,
        subitems: &ns::Array<CollectionLayoutItem>,
    ) -> arc::R<Self>;

    #[objc::msg_send(setInterItemSpacing:)]
    pub fn set_inter_item_spacing(&mut self, val: Option<&ns::Id>);
}

define_obj_type!(
    #[doc(alias = "NSCollectionLayoutBoundarySupplementaryItem")]
    pub CollectionLayoutBoundarySupplementaryItem(ns::Id),
    NS_COLLECTION_LAYOUT_BOUNDARY_SUPPLEMENTARY_ITEM
);

impl CollectionLayoutBoundarySupplementaryItem {
    #[objc::msg_send(boundarySupplementaryItemWithLayoutSize:elementKind:alignment:)]
    pub fn with_layout_size_element_kind_alignment(
        size: &CollectionLayoutSize,
        element_kind: &ns::String,
        alignment: RectAlignment,
    ) -> arc::R<Self>;

    #[objc::msg_send(pinToVisibleBounds)]
    pub fn pin_to_visible_bounds(&self) -> bool;

    #[objc::msg_send(setPinToVisibleBounds:)]
    pub fn set_pin_to_visible_bounds(&mut self, val: bool);
}

define_obj_type!(
    #[doc(alias = "NSCollectionLayoutSection")]
    pub CollectionLayoutSection(ns::Id),
    NS_COLLECTION_LAYOUT_SECTION
);

impl CollectionLayoutSection {
    #[objc::msg_send(sectionWithGroup:)]
    pub fn with_group(group: &CollectionLayoutGroup) -> arc::R<Self>;

    #[objc::msg_send(contentInsets)]
    pub fn content_insets(&self) -> ui::DirectionalEdgeInsets;

    #[objc::msg_send(setContentInsets:)]
    pub fn set_content_insets(&mut self, val: ui::DirectionalEdgeInsets);

    #[objc::msg_send(interGroupSpacing)]
    pub fn inter_group_spacing(&self) -> cg::Float;

    #[objc::msg_send(setInterGroupSpacing:)]
    pub fn set_inter_group_spacing(&mut self, val: cg::Float);

    #[objc::msg_send(orthogonalScrollingBehavior)]
    pub fn orthogonal_scrolling_behavior(
        &self,
    ) -> CollectionLayoutSectionOrthogonalScrollingBehavior;

    #[objc::msg_send(setOrthogonalScrollingBehavior:)]
    pub fn set_orthogonal_scrolling_behavior(
        &mut self,
        val: CollectionLayoutSectionOrthogonalScrollingBehavior,
    );

    #[objc::msg_send(setBoundarySupplementaryItems:)]
    pub fn set_boundary_supplementary_items(
        &mut self,
        val: &ns::Array<CollectionLayoutBoundarySupplementaryItem>,
    );
}

unsafe extern "C" {
    static UI_COLLECTION_VIEW_COMPOSITIONAL_LAYOUT:
        &'static objc::Class<CollectionViewCompositionalLayout>;
    static NS_COLLECTION_LAYOUT_DIMENSION: &'static objc::Class<CollectionLayoutDimension>;
    static NS_COLLECTION_LAYOUT_SIZE: &'static objc::Class<CollectionLayoutSize>;
    static NS_COLLECTION_LAYOUT_ITEM: &'static objc::Class<CollectionLayoutItem>;
    static NS_COLLECTION_LAYOUT_GROUP: &'static objc::Class<CollectionLayoutGroup>;
    static NS_COLLECTION_LAYOUT_BOUNDARY_SUPPLEMENTARY_ITEM:
        &'static objc::Class<CollectionLayoutBoundarySupplementaryItem>;
    static NS_COLLECTION_LAYOUT_SECTION: &'static objc::Class<CollectionLayoutSection>;
}
