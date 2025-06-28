use crate::{arc, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "cf")]
use crate::cf;

use super::Class;

define_obj_type!(
    #[doc(alias = "NSURLResourceKey")]
    pub ResKey(ns::String)
);

define_obj_type!(
    #[doc(alias = "NSURL")]
    pub Url(ns::Id)
);

impl arc::A<Url> {
    #[objc::msg_send(initFileURLWithPath:isDirectory:relativeToURL:)]
    pub fn init_with_path_is_dir_relative_to_url(
        self,
        path: &ns::String,
        is_dir: bool,
        relative_to: Option<&ns::Url>,
    ) -> arc::R<Url>;

    #[objc::msg_send(initWithString:relativeToURL:)]
    pub fn init_with_string_relative_to(
        self,
        string: &ns::String,
        relative_to: Option<&ns::Url>,
    ) -> Option<arc::R<ns::Url>>;
}

impl Url {
    define_cls!(NS_URL);

    #[inline]
    pub fn file_url_relative_to(
        path: &ns::String,
        is_dir: bool,
        relative_to: Option<&ns::Url>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_path_is_dir_relative_to_url(path, is_dir, relative_to)
    }

    #[inline]
    pub fn with_fs_path_string(path: &ns::String, is_dir: bool) -> arc::R<Self> {
        Self::file_url_relative_to(path, is_dir, None)
    }

    #[inline]
    pub fn with_fs_path_str(path: &str, is_dir: bool) -> arc::R<Self> {
        let string = ns::String::with_str_no_copy(path);
        Self::file_url_relative_to(&string, is_dir, None)
    }

    #[inline]
    pub fn with_string_relative_to(
        str: &ns::String,
        relative_to: Option<&ns::Url>,
    ) -> Option<arc::R<Self>> {
        Self::alloc().init_with_string_relative_to(str, relative_to)
    }

    #[inline]
    pub fn with_string(str: &ns::String) -> Option<arc::R<Self>> {
        Self::alloc().init_with_string_relative_to(str, None)
    }

    #[inline]
    pub fn with_str_relative_to(str: &str, relative_to: Option<&ns::Url>) -> Option<arc::R<Self>> {
        let string = ns::String::with_str_no_copy(str);
        Self::with_string_relative_to(&string, relative_to)
    }

    #[inline]
    pub fn with_str(str: &str) -> Option<arc::R<Self>> {
        let string = ns::String::with_str_no_copy(str);
        Self::with_string_relative_to(&string, None)
    }

    #[objc::msg_send(absoluteString)]
    pub fn abs_string(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(resourceValuesForKeys:error:)]
    pub unsafe fn res_values_for_keys_err<'ear>(
        &self,
        keys: &ns::Array<ns::UrlResKey>,
        err: *mut Option<&ns::Error>,
    ) -> Option<arc::R<ns::Dictionary<ns::UrlResKey, ns::Id>>>;

    #[inline]
    pub fn res_values_for_keys<'ear>(
        &self,
        keys: &ns::Array<ns::UrlResKey>,
    ) -> Result<arc::R<ns::Dictionary<ns::UrlResKey, ns::Id>>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { self.res_values_for_keys_err(keys, err) })
    }

    #[objc::msg_send(startAccessingSecurityScopedResource)]
    pub fn start_accessing_security_scoped_resource(&self) -> bool;

    #[objc::msg_send(stopAccessingSecurityScopedResource)]
    pub fn stop_accessing_security_scoped_resource(&self);

    #[objc::msg_send(host)]
    pub fn host(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(port)]
    pub fn port(&self) -> Option<arc::R<ns::Number>>;

    #[objc::msg_send(user)]
    pub fn user(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(password)]
    pub fn password(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(path)]
    pub fn path(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(fragment)]
    pub fn fragment(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(query)]
    pub fn query(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(relative_path)]
    pub fn relative_path(&self) -> Option<arc::R<ns::String>>;
}

#[cfg(feature = "cf")]
impl Url {
    #[inline]
    pub fn as_cf(&self) -> &cf::Url {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(feature = "cf")]
impl AsRef<cf::Url> for Url {
    #[inline]
    fn as_ref(&self) -> &cf::Url {
        self.as_cf()
    }
}

unsafe impl Send for Url {}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_URL: &'static Class<Url>;
}

impl ResKey {
    #[inline]
    pub fn name() -> &'static Self {
        unsafe { NSURLNameKey }
    }

    #[inline]
    pub fn localized_name() -> &'static Self {
        unsafe { NSURLLocalizedNameKey }
    }

    #[inline]
    pub fn is_regular_file() -> &'static Self {
        unsafe { NSURLIsRegularFileKey }
    }

    #[inline]
    pub fn is_directory() -> &'static Self {
        unsafe { NSURLIsDirectoryKey }
    }

    #[inline]
    pub fn is_symbolic_link() -> &'static Self {
        unsafe { NSURLIsSymbolicLinkKey }
    }

    #[inline]
    pub fn is_volume() -> &'static Self {
        unsafe { NSURLIsVolumeKey }
    }

    #[inline]
    pub fn is_package() -> &'static Self {
        unsafe { NSURLIsPackageKey }
    }

    #[inline]
    pub fn is_application() -> &'static Self {
        unsafe { NSURLIsApplicationKey }
    }

    #[inline]
    pub fn application_is_scriptable() -> &'static Self {
        unsafe { NSURLApplicationIsScriptableKey }
    }

    #[inline]
    pub fn is_system_immutable() -> &'static Self {
        unsafe { NSURLIsSystemImmutableKey }
    }

    #[inline]
    pub fn is_user_immutable() -> &'static Self {
        unsafe { NSURLIsUserImmutableKey }
    }

    #[inline]
    pub fn is_hidden() -> &'static Self {
        unsafe { NSURLIsHiddenKey }
    }

    #[inline]
    pub fn has_hidden_extension() -> &'static Self {
        unsafe { NSURLHasHiddenExtensionKey }
    }

    #[inline]
    pub fn creation_date() -> &'static Self {
        unsafe { NSURLCreationDateKey }
    }

    #[inline]
    pub fn access_date() -> &'static Self {
        unsafe { NSURLContentAccessDateKey }
    }

    #[inline]
    pub fn content_modification_date() -> &'static Self {
        unsafe { NSURLContentModificationDateKey }
    }

    #[inline]
    pub fn attribute_modification_date() -> &'static Self {
        unsafe { NSURLAttributeModificationDateKey }
    }

    #[inline]
    pub fn link_count() -> &'static Self {
        unsafe { NSURLLinkCountKey }
    }

    #[inline]
    pub fn parent_directory() -> &'static Self {
        unsafe { NSURLParentDirectoryURLKey }
    }

    #[inline]
    pub fn volume_url() -> &'static Self {
        unsafe { NSURLVolumeURLKey }
    }

    #[inline]
    pub fn content_type() -> &'static Self {
        unsafe { NSURLContentTypeKey }
    }

    #[inline]
    pub fn localized_type_desc() -> &'static Self {
        unsafe { NSURLLocalizedTypeDescriptionKey }
    }

    #[inline]
    pub fn label_number() -> &'static Self {
        unsafe { NSURLLabelNumberKey }
    }

    #[inline]
    pub fn label_color() -> &'static Self {
        unsafe { NSURLLabelColorKey }
    }

    #[inline]
    pub fn localized_label() -> &'static Self {
        unsafe { NSURLLocalizedLabelKey }
    }

    #[inline]
    pub fn effective_icon() -> &'static Self {
        unsafe { NSURLEffectiveIconKey }
    }

    #[inline]
    pub fn custom_icon() -> &'static Self {
        unsafe { NSURLCustomIconKey }
    }

    #[inline]
    pub fn file_resource_id() -> &'static Self {
        unsafe { NSURLFileResourceIdentifierKey }
    }

    #[inline]
    pub fn volume_id() -> &'static Self {
        unsafe { NSURLVolumeIdentifierKey }
    }

    #[inline]
    pub fn preferred_io_block_size() -> &'static Self {
        unsafe { NSURLPreferredIOBlockSizeKey }
    }

    #[inline]
    pub fn is_readable() -> &'static Self {
        unsafe { NSURLIsReadableKey }
    }

    #[inline]
    pub fn is_writable() -> &'static Self {
        unsafe { NSURLIsWritableKey }
    }

    #[inline]
    pub fn is_executable() -> &'static Self {
        unsafe { NSURLIsExecutableKey }
    }

    #[inline]
    pub fn file_security() -> &'static Self {
        unsafe { NSURLFileSecurityKey }
    }

    #[inline]
    pub fn is_excluded_from_backup() -> &'static Self {
        unsafe { NSURLIsExcludedFromBackupKey }
    }

    #[inline]
    pub fn tag_names() -> &'static Self {
        unsafe { NSURLTagNamesKey }
    }

    #[inline]
    pub fn path() -> &'static Self {
        unsafe { NSURLPathKey }
    }

    #[inline]
    pub fn cannonical_path() -> &'static Self {
        unsafe { NSURLCanonicalPathKey }
    }

    #[inline]
    pub fn is_mount_trigger() -> &'static Self {
        unsafe { NSURLIsMountTriggerKey }
    }

    #[inline]
    pub fn generation_id() -> &'static Self {
        unsafe { NSURLGenerationIdentifierKey }
    }

    #[inline]
    pub fn document_id() -> &'static Self {
        unsafe { NSURLDocumentIdentifierKey }
    }

    #[inline]
    pub fn added_to_directory_date() -> &'static Self {
        unsafe { NSURLAddedToDirectoryDateKey }
    }

    #[inline]
    pub fn quarantine_props() -> &'static Self {
        unsafe { NSURLQuarantinePropertiesKey }
    }

    #[inline]
    pub fn file_resource_type() -> &'static Self {
        unsafe { NSURLFileResourceTypeKey }
    }

    #[inline]
    pub fn file_id() -> &'static Self {
        unsafe { NSURLFileIdentifierKey }
    }

    #[inline]
    pub fn file_content_id() -> &'static Self {
        unsafe { NSURLFileContentIdentifierKey }
    }

    #[inline]
    pub fn may_share_file_content() -> &'static Self {
        unsafe { NSURLMayShareFileContentKey }
    }

    #[inline]
    pub fn may_hane_extended_attrs() -> &'static Self {
        unsafe { NSURLMayHaveExtendedAttributesKey }
    }

    #[inline]
    pub fn is_purgeable() -> &'static Self {
        unsafe { NSURLIsPurgeableKey }
    }

    #[inline]
    pub fn is_sparse() -> &'static Self {
        unsafe { NSURLIsSparseKey }
    }

    #[inline]
    pub fn directory_entry_count() -> &'static Self {
        unsafe { NSURLDirectoryEntryCountKey }
    }

    #[inline]
    pub fn volume_localized_format_desc() -> &'static Self {
        unsafe { NSURLVolumeLocalizedFormatDescriptionKey }
    }

    #[inline]
    pub fn volume_total_capacity() -> &'static Self {
        unsafe { NSURLVolumeTotalCapacityKey }
    }

    #[inline]
    pub fn volume_available_capacity() -> &'static Self {
        unsafe { NSURLVolumeAvailableCapacityKey }
    }

    #[inline]
    pub fn volume_resource_count() -> &'static Self {
        unsafe { NSURLVolumeResourceCountKey }
    }

    #[inline]
    pub fn volume_supports_persistent_ids() -> &'static Self {
        unsafe { NSURLVolumeSupportsPersistentIDsKey }
    }

    #[inline]
    pub fn volume_supports_symbolic_links() -> &'static Self {
        unsafe { NSURLVolumeSupportsSymbolicLinksKey }
    }

    #[inline]
    pub fn volume_supports_hard_links() -> &'static Self {
        unsafe { NSURLVolumeSupportsHardLinksKey }
    }

    #[inline]
    pub fn volume_supports_journaling() -> &'static Self {
        unsafe { NSURLVolumeSupportsJournalingKey }
    }

    #[inline]
    pub fn volume_is_journaling() -> &'static Self {
        unsafe { NSURLVolumeIsJournalingKey }
    }

    #[inline]
    pub fn volume_supports_sparce_files() -> &'static Self {
        unsafe { NSURLVolumeSupportsSparseFilesKey }
    }

    #[inline]
    pub fn volume_supports_zero_runs() -> &'static Self {
        unsafe { NSURLVolumeSupportsZeroRunsKey }
    }

    #[inline]
    pub fn volume_supports_case_sensitive_names() -> &'static Self {
        unsafe { NSURLVolumeSupportsCaseSensitiveNamesKey }
    }

    #[inline]
    pub fn volume_supports_case_preserved_names() -> &'static Self {
        unsafe { NSURLVolumeSupportsCasePreservedNamesKey }
    }

    #[inline]
    pub fn volume_supports_root_directory_dates() -> &'static Self {
        unsafe { NSURLVolumeSupportsRootDirectoryDatesKey }
    }

    #[inline]
    pub fn volume_supports_volume_sizes() -> &'static Self {
        unsafe { NSURLVolumeSupportsVolumeSizesKey }
    }

    #[inline]
    pub fn volume_supports_renaming() -> &'static Self {
        unsafe { NSURLVolumeSupportsRenamingKey }
    }

    #[inline]
    pub fn volume_supports_advisory_file_locking() -> &'static Self {
        unsafe { NSURLVolumeSupportsAdvisoryFileLockingKey }
    }

    #[inline]
    pub fn volume_supports_extended_security() -> &'static Self {
        unsafe { NSURLVolumeSupportsExtendedSecurityKey }
    }

    #[inline]
    pub fn volume_is_browsable() -> &'static Self {
        unsafe { NSURLVolumeIsBrowsableKey }
    }

    #[inline]
    pub fn volume_is_ejectable() -> &'static Self {
        unsafe { NSURLVolumeIsEjectableKey }
    }

    #[inline]
    pub fn volume_is_removable() -> &'static Self {
        unsafe { NSURLVolumeIsRemovableKey }
    }

    #[inline]
    pub fn volume_is_internal() -> &'static Self {
        unsafe { NSURLVolumeIsInternalKey }
    }

    #[inline]
    pub fn volume_is_automounted() -> &'static Self {
        unsafe { NSURLVolumeIsAutomountedKey }
    }

    #[inline]
    pub fn volume_is_local() -> &'static Self {
        unsafe { NSURLVolumeIsLocalKey }
    }

    #[inline]
    pub fn volume_is_read_only() -> &'static Self {
        unsafe { NSURLVolumeIsReadOnlyKey }
    }

    #[inline]
    pub fn volume_creation_date() -> &'static Self {
        unsafe { NSURLVolumeCreationDateKey }
    }

    #[inline]
    pub fn volume_url_for_remounting() -> &'static Self {
        unsafe { NSURLVolumeURLForRemountingKey }
    }

    #[inline]
    pub fn volume_uuid_string() -> &'static Self {
        unsafe { NSURLVolumeUUIDStringKey }
    }

    #[inline]
    pub fn volume_name() -> &'static Self {
        unsafe { NSURLVolumeNameKey }
    }

    #[inline]
    pub fn volume_localized_name() -> &'static Self {
        unsafe { NSURLVolumeLocalizedNameKey }
    }

    #[inline]
    pub fn volume_is_encrypted() -> &'static Self {
        unsafe { NSURLVolumeIsEncryptedKey }
    }

    #[inline]
    pub fn volume_is_root_file_system() -> &'static Self {
        unsafe { NSURLVolumeIsRootFileSystemKey }
    }
    #[inline]
    pub fn volume_supports_compression() -> &'static Self {
        unsafe { NSURLVolumeSupportsCompressionKey }
    }

    #[inline]
    pub fn volume_supports_file_cloning() -> &'static Self {
        unsafe { NSURLVolumeSupportsFileCloningKey }
    }

    #[inline]
    pub fn volume_supports_swap_renaming() -> &'static Self {
        unsafe { NSURLVolumeSupportsSwapRenamingKey }
    }

    #[inline]
    pub fn volume_supports_exclusive_renaming() -> &'static Self {
        unsafe { NSURLVolumeSupportsExclusiveRenamingKey }
    }

    #[inline]
    pub fn volume_supports_immutable_files() -> &'static Self {
        unsafe { NSURLVolumeSupportsImmutableFilesKey }
    }

    #[inline]
    pub fn volume_supports_access_permissions() -> &'static Self {
        unsafe { NSURLVolumeSupportsAccessPermissionsKey }
    }

    #[inline]
    pub fn volume_supports_file_protection() -> &'static Self {
        unsafe { NSURLVolumeSupportsFileProtectionKey }
    }

    #[inline]
    pub fn volume_available_capacity_for_important_usage() -> &'static Self {
        unsafe { NSURLVolumeAvailableCapacityForImportantUsageKey }
    }

    #[inline]
    pub fn volume_available_capacity_for_opportunistic_usage() -> &'static Self {
        unsafe { NSURLVolumeAvailableCapacityForOpportunisticUsageKey }
    }

    #[inline]
    pub fn volume_type_name() -> &'static Self {
        unsafe { NSURLVolumeTypeNameKey }
    }

    #[inline]
    pub fn volume_sub_type_name() -> &'static Self {
        unsafe { NSURLVolumeSubtypeKey }
    }

    #[inline]
    pub fn volume_mount_from_location() -> &'static Self {
        unsafe { NSURLVolumeMountFromLocationKey }
    }
}

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    static NSURLNameKey: &'static ResKey;
    static NSURLLocalizedNameKey: &'static ResKey;
    static NSURLIsRegularFileKey: &'static ResKey;
    static NSURLIsDirectoryKey: &'static ResKey;
    static NSURLIsSymbolicLinkKey: &'static ResKey;
    static NSURLIsVolumeKey: &'static ResKey;
    static NSURLIsPackageKey: &'static ResKey;
    static NSURLIsApplicationKey: &'static ResKey;
    static NSURLApplicationIsScriptableKey: &'static ResKey;
    static NSURLIsSystemImmutableKey: &'static ResKey;
    static NSURLIsUserImmutableKey: &'static ResKey;
    static NSURLIsHiddenKey: &'static ResKey;
    static NSURLHasHiddenExtensionKey: &'static ResKey;
    static NSURLCreationDateKey: &'static ResKey;
    static NSURLContentAccessDateKey: &'static ResKey;
    static NSURLContentModificationDateKey: &'static ResKey;
    static NSURLAttributeModificationDateKey: &'static ResKey;
    static NSURLLinkCountKey: &'static ResKey;
    static NSURLParentDirectoryURLKey: &'static ResKey;
    static NSURLVolumeURLKey: &'static ResKey;
    static NSURLContentTypeKey: &'static ResKey;
    static NSURLLocalizedTypeDescriptionKey: &'static ResKey;
    static NSURLLabelNumberKey: &'static ResKey;
    static NSURLLabelColorKey: &'static ResKey;
    static NSURLLocalizedLabelKey: &'static ResKey;
    static NSURLEffectiveIconKey: &'static ResKey;
    static NSURLCustomIconKey: &'static ResKey;
    static NSURLFileResourceIdentifierKey: &'static ResKey;
    static NSURLVolumeIdentifierKey: &'static ResKey;
    static NSURLPreferredIOBlockSizeKey: &'static ResKey;
    static NSURLIsReadableKey: &'static ResKey;
    static NSURLIsWritableKey: &'static ResKey;
    static NSURLIsExecutableKey: &'static ResKey;
    static NSURLFileSecurityKey: &'static ResKey;
    static NSURLIsExcludedFromBackupKey: &'static ResKey;
    static NSURLTagNamesKey: &'static ResKey;
    static NSURLPathKey: &'static ResKey;
    static NSURLCanonicalPathKey: &'static ResKey;
    static NSURLIsMountTriggerKey: &'static ResKey;
    static NSURLGenerationIdentifierKey: &'static ResKey;
    static NSURLDocumentIdentifierKey: &'static ResKey;
    static NSURLAddedToDirectoryDateKey: &'static ResKey;
    static NSURLQuarantinePropertiesKey: &'static ResKey;
    static NSURLFileResourceTypeKey: &'static ResKey;
    static NSURLFileIdentifierKey: &'static ResKey;
    static NSURLFileContentIdentifierKey: &'static ResKey;
    static NSURLMayShareFileContentKey: &'static ResKey;
    static NSURLMayHaveExtendedAttributesKey: &'static ResKey;
    static NSURLIsPurgeableKey: &'static ResKey;
    static NSURLIsSparseKey: &'static ResKey;
    static NSURLDirectoryEntryCountKey: &'static ResKey;
    static NSURLVolumeLocalizedFormatDescriptionKey: &'static ResKey;
    static NSURLVolumeTotalCapacityKey: &'static ResKey;
    static NSURLVolumeAvailableCapacityKey: &'static ResKey;
    static NSURLVolumeResourceCountKey: &'static ResKey;
    static NSURLVolumeSupportsPersistentIDsKey: &'static ResKey;
    static NSURLVolumeSupportsSymbolicLinksKey: &'static ResKey;
    static NSURLVolumeSupportsHardLinksKey: &'static ResKey;
    static NSURLVolumeSupportsJournalingKey: &'static ResKey;
    static NSURLVolumeIsJournalingKey: &'static ResKey;
    static NSURLVolumeSupportsSparseFilesKey: &'static ResKey;
    static NSURLVolumeSupportsZeroRunsKey: &'static ResKey;
    static NSURLVolumeSupportsCaseSensitiveNamesKey: &'static ResKey;
    static NSURLVolumeSupportsCasePreservedNamesKey: &'static ResKey;
    static NSURLVolumeSupportsRootDirectoryDatesKey: &'static ResKey;
    static NSURLVolumeSupportsVolumeSizesKey: &'static ResKey;
    static NSURLVolumeSupportsRenamingKey: &'static ResKey;
    static NSURLVolumeSupportsAdvisoryFileLockingKey: &'static ResKey;
    static NSURLVolumeSupportsExtendedSecurityKey: &'static ResKey;
    static NSURLVolumeIsBrowsableKey: &'static ResKey;
    static NSURLVolumeIsEjectableKey: &'static ResKey;
    static NSURLVolumeIsRemovableKey: &'static ResKey;
    static NSURLVolumeIsInternalKey: &'static ResKey;
    static NSURLVolumeIsAutomountedKey: &'static ResKey;
    static NSURLVolumeIsLocalKey: &'static ResKey;
    static NSURLVolumeIsReadOnlyKey: &'static ResKey;
    static NSURLVolumeCreationDateKey: &'static ResKey;
    static NSURLVolumeURLForRemountingKey: &'static ResKey;
    static NSURLVolumeUUIDStringKey: &'static ResKey;
    static NSURLVolumeNameKey: &'static ResKey;
    static NSURLVolumeLocalizedNameKey: &'static ResKey;
    static NSURLVolumeIsEncryptedKey: &'static ResKey;
    static NSURLVolumeIsRootFileSystemKey: &'static ResKey;
    static NSURLVolumeSupportsCompressionKey: &'static ResKey;
    static NSURLVolumeSupportsFileCloningKey: &'static ResKey;
    static NSURLVolumeSupportsSwapRenamingKey: &'static ResKey;
    static NSURLVolumeSupportsExclusiveRenamingKey: &'static ResKey;
    static NSURLVolumeSupportsImmutableFilesKey: &'static ResKey;
    static NSURLVolumeSupportsAccessPermissionsKey: &'static ResKey;
    static NSURLVolumeSupportsFileProtectionKey: &'static ResKey;
    static NSURLVolumeAvailableCapacityForImportantUsageKey: &'static ResKey;
    static NSURLVolumeAvailableCapacityForOpportunisticUsageKey: &'static ResKey;
    static NSURLVolumeTypeNameKey: &'static ResKey;
    static NSURLVolumeSubtypeKey: &'static ResKey;
    static NSURLVolumeMountFromLocationKey: &'static ResKey;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let url = ns::Url::with_fs_path_str("/tmp", true);
        let abs_str = url.abs_string().unwrap();

        let url2 = ns::Url::with_str("file:///tmp/").unwrap();
        let abs_str2 = url2.abs_string().unwrap();

        assert!(abs_str.eq(&abs_str2));
    }
}
