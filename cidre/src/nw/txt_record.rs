use std::ffi::CStr;

use crate::{arc, define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_txt_record")]
    #[doc(alias = "nw_txt_record_t")]
    pub TxtRecord(ns::Id)
);

#[doc(alias = "nw_txt_record_find_key_t")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum TxtRecordFindKey {
    #[doc(alias = "nw_txt_record_find_key_invalid")]
    Invalid = 0,
    #[doc(alias = "nw_txt_record_find_key_not_present")]
    NotPresent = 1,
    #[doc(alias = "nw_txt_record_find_key_no_value")]
    NoValue = 2,
    #[doc(alias = "nw_txt_record_find_key_empty_value")]
    EmptyValue = 3,
    #[doc(alias = "nw_txt_record_find_key_non_empty_value")]
    NonEmptyValue = 4,
}

#[cfg(feature = "blocks")]
#[doc(alias = "nw_txt_record_access_key_t")]
pub type TxtRecordAccessKey = crate::blocks::NoEscBlock<
    fn(
        key: *const std::ffi::c_char,
        found: TxtRecordFindKey,
        value: *const u8,
        value_len: usize,
    ) -> bool,
>;

#[cfg(feature = "blocks")]
#[doc(alias = "nw_txt_record_applier_t")]
pub type TxtRecordApplier = TxtRecordAccessKey;

impl TxtRecord {
    #[doc(alias = "nw_txt_record_create_dictionary")]
    #[inline]
    pub fn dictionary() -> arc::R<Self> {
        unsafe { nw_txt_record_create_dictionary() }
    }

    #[doc(alias = "nw_txt_record_create_with_bytes")]
    #[inline]
    pub fn with_bytes(bytes: &[u8]) -> Option<arc::R<Self>> {
        unsafe { nw_txt_record_create_with_bytes(bytes.as_ptr(), bytes.len()) }
    }

    pub fn copy(&self) -> arc::R<Self> {
        unsafe { nw_txt_record_copy(self) }
    }

    pub fn find_key<K: AsRef<CStr>>(&self, key: K) -> TxtRecordFindKey {
        unsafe { nw_txt_record_find_key(self, key.as_ref().as_ptr()) }
    }

    #[cfg(feature = "blocks")]
    pub fn access_key_block<K: AsRef<CStr>>(
        &self,
        key: K,
        access_value: &mut TxtRecordAccessKey,
    ) -> bool {
        unsafe { nw_txt_record_access_key(self, key.as_ref().as_ptr(), access_value) }
    }

    #[cfg(feature = "blocks")]
    pub fn access_key<K: AsRef<CStr>>(
        &self,
        key: K,
        mut handler: impl FnMut(
            /* key: */ *const std::ffi::c_char,
            /* found: */ TxtRecordFindKey,
            /* value: */ *const u8,
            /* value_len: */ usize,
        ) -> bool,
    ) -> bool {
        let mut block = unsafe { TxtRecordAccessKey::stack4(&mut handler) };
        self.access_key_block(key, &mut block)
    }

    pub fn set_key<K: AsRef<CStr>>(&mut self, key: K, value: &[u8]) -> Result<(), ()> {
        unsafe {
            if nw_txt_record_set_key(self, key.as_ref().as_ptr(), value.as_ptr(), value.len()) {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    pub fn remove_key<K: AsRef<CStr>>(&mut self, key: K) -> Result<(), ()> {
        unsafe {
            if nw_txt_record_remove_key(self, key.as_ref().as_ptr()) {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    /// Apply the block to every key-value pair in the TXT record object.
    #[cfg(feature = "blocks")]
    pub fn apply_block(&self, applier: &mut TxtRecordApplier) -> bool {
        unsafe { nw_txt_record_apply(self, applier) }
    }

    #[cfg(feature = "blocks")]
    pub fn apply(
        &self,
        mut handler: impl FnMut(
            /* key: */ *const std::ffi::c_char,
            /* found: */ TxtRecordFindKey,
            /* value: */ *const u8,
            /* value_len: */ usize,
        ) -> bool,
    ) -> bool {
        let mut block = unsafe { TxtRecordApplier::stack4(&mut handler) };

        unsafe { nw_txt_record_apply(self, &mut block) }
    }

    pub fn is_equal(&self, other: Option<&Self>) -> bool {
        unsafe { nw_txt_record_is_equal(Some(self), other) }
    }

    #[doc(alias = "nw_txt_record_is_dictionary")]
    pub fn is_dictionary(&self) -> bool {
        unsafe { nw_txt_record_is_dictionary(self) }
    }
}

#[link(name = "Network", kind = "framework")]
unsafe extern "C-unwind" {
    fn nw_txt_record_create_with_bytes(
        txt_bytes: *const u8,
        txt_len: usize,
    ) -> Option<arc::R<TxtRecord>>;

    fn nw_txt_record_create_dictionary() -> arc::R<TxtRecord>;

    fn nw_txt_record_copy(record: &TxtRecord) -> arc::R<TxtRecord>;

    fn nw_txt_record_find_key(record: &TxtRecord, key: *const std::ffi::c_char)
    -> TxtRecordFindKey;

    #[cfg(feature = "blocks")]
    fn nw_txt_record_access_key(
        record: &TxtRecord,
        key: *const std::ffi::c_char,
        access_value: &mut TxtRecordAccessKey,
    ) -> bool;

    fn nw_txt_record_remove_key(record: &mut TxtRecord, key: *const std::ffi::c_char) -> bool;

    fn nw_txt_record_set_key(
        record: &mut TxtRecord,
        key: *const std::ffi::c_char,
        value: *const u8,
        value_len: usize,
    ) -> bool;

    fn nw_txt_record_is_equal(left: Option<&TxtRecord>, right: Option<&TxtRecord>) -> bool;
    fn nw_txt_record_is_dictionary(record: &TxtRecord) -> bool;
    #[cfg(feature = "blocks")]
    fn nw_txt_record_apply(record: &TxtRecord, applier: &mut TxtRecordApplier) -> bool;
}
