use crate::{arc, define_cls, define_obj_type, ns, objc, ui};

#[cfg(feature = "blocks")]
use crate::blocks;

/// Configures a dequeued cell for an index path and item.
#[cfg(feature = "blocks")]
pub type CollectionViewCellRegistrationCfgHandler = blocks::EscBlock<
    fn(cell: &mut ui::CollectionViewCell, index_path: &ns::IndexPath, item: &ns::Id),
>;

define_obj_type!(
    #[doc(alias = "UICollectionViewCellRegistration")]
    pub CollectionViewCellRegistration(ns::Id)
);

impl CollectionViewCellRegistration {
    define_cls!(UI_COLLECTION_VIEW_CELL_REGISTRATION);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(registrationWithCellClass:configurationHandler:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn with_cell_class_cfg_handler_block(
        cell_class: &objc::Class<ns::Id>,
        handler: &mut CollectionViewCellRegistrationCfgHandler,
    ) -> arc::R<Self>;

    /// A registration for cells of `cell_class`, configured by `handler` when dequeued.
    #[cfg(feature = "blocks")]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn with_cell_class_cfg_handler<C: objc::Obj>(
        cell_class: &objc::Class<C>,
        handler: impl FnMut(&mut ui::CollectionViewCell, &ns::IndexPath, &ns::Id) + 'static,
    ) -> arc::R<Self> {
        let cell_class: &objc::Class<ns::Id> = unsafe { std::mem::transmute(cell_class) };
        let mut handler = CollectionViewCellRegistrationCfgHandler::new3(handler);
        Self::with_cell_class_cfg_handler_block(cell_class, &mut handler)
    }
}

define_obj_type!(
    #[doc(alias = "UICollectionViewSupplementaryRegistration")]
    pub CollectionViewSupplementaryRegistration(ns::Id)
);

unsafe extern "C" {
    static UI_COLLECTION_VIEW_CELL_REGISTRATION:
        &'static objc::Class<CollectionViewCellRegistration>;
}
