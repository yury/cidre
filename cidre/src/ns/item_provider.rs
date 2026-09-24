use crate::{arc, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    /// Data or files another app offers, in drag and drop and sharing, loaded on demand.
    #[doc(alias = "NSItemProvider")]
    pub ItemProvider(ns::Id),
    NS_ITEM_PROVIDER
);

impl ItemProvider {
    /// The type identifiers the provider can load, the richest first.
    #[objc::msg_send(registeredTypeIdentifiers)]
    pub fn registered_type_ids(&self) -> arc::R<ns::Array<ns::String>>;

    /// Whether the provider has something of type `type_id`, or of a type conforming to it.
    #[objc::msg_send(hasItemConformingToTypeIdentifier:)]
    pub fn has_item_conforming_to_type_id(&self, type_id: &ns::String) -> bool;

    /// A name for the item, a file's name without its extension for one.
    #[objc::msg_send(suggestedName)]
    pub fn suggested_name(&self) -> Option<arc::R<ns::String>>;

    /// Writes a copy of the item as `type_id` to a temporary file, and calls `ch` with its URL
    /// on a background queue. The file is deleted once `ch` returns: open or copy it there.
    #[cfg(feature = "blocks")]
    #[objc::msg_send(loadFileRepresentationForTypeIdentifier:completionHandler:)]
    pub fn load_file_repr_for_type_id_ch_block(
        &self,
        type_id: &ns::String,
        ch: &mut blocks::ResultCh<ns::Url>,
    ) -> arc::R<ns::Progress>;

    /// Like [`Self::load_file_repr_for_type_id_ch_block`] with a closure.
    #[cfg(feature = "blocks")]
    pub fn load_file_repr_for_type_id(
        &self,
        type_id: &ns::String,
        ch: impl FnMut(Option<&ns::Url>, Option<&ns::Error>) + 'static,
    ) -> arc::R<ns::Progress> {
        let mut ch = blocks::ResultCh::new2(ch);
        self.load_file_repr_for_type_id_ch_block(type_id, &mut ch)
    }
}

unsafe extern "C" {
    static NS_ITEM_PROVIDER: &'static objc::Class<ItemProvider>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let provider = ns::ItemProvider::new();
        assert!(provider.registered_type_ids().is_empty());
        assert!(!provider.has_item_conforming_to_type_id(ns::str!(c"public.image")));
    }
}
