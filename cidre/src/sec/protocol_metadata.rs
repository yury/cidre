use crate::{arc, define_obj_type, dispatch, ns};

define_obj_type!(
    #[doc(alias = "sec_protocol_metadata")]
    #[doc(alias = "sec_protocol_metadata_t")]
    pub ProtocolMetadata(ns::Id)
);

impl ProtocolMetadata {
    #[doc(alias = "sec_protocol_metadata_copy_peer_public_key")]
    #[inline]
    pub fn peer_public_key(&self) -> Option<arc::R<dispatch::Data>> {
        unsafe { sec_protocol_metadata_copy_peer_public_key(self) }
    }

    #[inline]
    pub fn peer_public_key_bytes(&self) -> Option<Vec<u8>> {
        let public_key = self.peer_public_key()?;
        Some(public_key.map().as_slice().to_vec())
    }
}

unsafe extern "C-unwind" {
    fn sec_protocol_metadata_copy_peer_public_key(
        metadata: &ProtocolMetadata,
    ) -> Option<arc::R<dispatch::Data>>;
}
