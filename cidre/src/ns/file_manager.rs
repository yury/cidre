use crate::{arc, define_cls, define_obj_type, define_opts, ns, objc, os};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    #[doc(alias = "NSFileAttributeKey")]
    pub FileAttrKey(ns::String)
);

define_obj_type!(
    #[doc(alias = "NSFileAttributeType")]
    pub FileAttrType(ns::String)
);

define_obj_type!(
    #[doc(alias = "NSFileProtectionType")]
    pub FileProtectionType(ns::String)
);

define_opts!(
    #[doc(alias = "NSVolumeEnumerationOptions")]
    pub VolumeEnumOpts(usize)
);

impl VolumeEnumOpts {
    ///  The mounted volume enumeration will skip hidden volumes.
    #[doc(alias = "NSVolumeEnumerationSkipHiddenVolumes")]
    pub const SKIP_HIDDEN_VOLUMES: Self = Self(1 << 1);

    /// The mounted volume enumeration will produce file reference URLs rather than path-based URLs.
    #[doc(alias = "NSVolumeEnumerationProduceFileReferenceURLs")]
    pub const PRODUCE_FILE_REFERENCE_URLS: Self = Self(1 << 2);
}

define_opts!(
    #[doc(alias = "NSDirectoryEnumerationOptions")]
    pub DirEnumOpts(usize)
);

impl DirEnumOpts {
    /// Causes the to perform a shallow enumeration and not descend into directories it encounters.
    #[doc(alias = "NSDirectoryEnumerationSkipsSubdirectoryDescendants")]
    pub const SKIPS_SUBDIRECTORY_DESCENDANTS: Self = Self(1 << 0);

    /// Will cause the to not descend into packages.
    #[doc(alias = "NSDirectoryEnumerationSkipsPackageDescendants")]
    pub const SKIPS_PACKAGE_DESCENDANTS: Self = Self(1 << 1);

    /// Causes the to not enumerate hidden files.
    #[doc(alias = "NSDirectoryEnumerationSkipsHiddenFiles")]
    pub const SKIPS_HIDDEN_FILES: Self = Self(1 << 2);

    /// Causes the to enumerate each directory a second time after all of its contained files have been enumerated.
    /// Use isEnumeratingDirectoryPostOrder to differentiate a post-order enumerated directory from a pre-order one.
    #[doc(alias = "NSDirectoryEnumerationIncludesDirectoriesPostOrder")]
    pub const INCLUDES_DIRECTORIES_POST_ORDER: Self = Self(1 << 3);

    /// Causes the to always produce file path URLs relative to the directoryURL. This can reduce the size of each URL object returned during enumeration.
    #[doc(alias = "NSDirectoryEnumerationProducesRelativePathURLs")]
    pub const PRODUCES_RELATIVE_PATH_URLS: Self = Self(1 << 4);
}

define_opts!(
    #[doc(alias = "NSFileManagerItemReplacementOptions")]
    pub ItemReplacementOpts(usize)
);

impl ItemReplacementOpts {
    #[doc(alias = "NSFileManagerItemReplacementUsingNewMetadataOnly")]
    pub const USING_NEW_METADATA_ONLY: Self = Self(1 << 0);

    #[doc(alias = "NSFileManagerItemReplacementWithoutDeletingBackupItem")]
    pub const WITHOUT_DELETING_BACKUP_ITEM: Self = Self(1 << 1);
}

#[doc(alias = "NSURLRelationship")]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(isize)]
pub enum UrlRelationship {
    #[doc(alias = "NSURLRelationshipContains")]
    Contains,

    #[doc(alias = "NSURLRelationshipSame")]
    Same,

    #[doc(alias = "NSURLRelationshipOther")]
    Other,
}

define_opts!(
    #[doc(alias = "NSFileManagerUnmountOptions")]
    pub UnmountOpts(usize)
);

impl UnmountOpts {
    #[doc(alias = "NSFileManagerUnmountAllPartitionsAndEjectDisk")]
    pub const ALL_PARTITIONS_AND_EJECT_DISK: Self = Self(1 << 0);

    #[doc(alias = "NSFileManagerUnmountWithoutUI")]
    pub const WITHOUT_UI: Self = Self(1 << 1);
}

define_obj_type!(
    #[doc(alias = "NSFileManager")]
    pub FileManager(ns::Id)
);

impl FileManager {
    define_cls!(NS_FILE_MANAGER);

    #[objc::msg_send(defaultManager)]
    pub fn default() -> arc::R<FileManager>;

    #[objc::msg_send(contentsOfDirectoryAtURL:includingPropertiesForKeys:options:error:)]
    pub unsafe fn contents_of_dir_at_url_err<'ear>(
        &self,
        url: &ns::Url,
        including_props_for_keys: Option<&ns::Array<ns::UrlResKey>>,
        options: DirEnumOpts,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ns::Array<ns::Url>>>;

    pub fn contents_of_dir_at_url<'ear>(
        &self,
        url: &ns::Url,
        including_props_for_keys: Option<&ns::Array<ns::UrlResKey>>,
        options: DirEnumOpts,
    ) -> Result<arc::R<ns::Array<ns::Url>>, &'ear ns::Error> {
        ns::if_none(|err| unsafe {
            self.contents_of_dir_at_url_err(url, including_props_for_keys, options, err)
        })
    }

    #[objc::msg_send(URLForDirectory:inDomain:appropriateForURL:create:error:)]
    pub unsafe fn url_for_dir_err<'ear>(
        &self,
        directory: ns::SearchPathDirectory,
        in_domain: ns::SearchPathDomainMask,
        appropriate_for_url: Option<&ns::Url>,
        create: bool,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ns::Url>>;

    pub fn url_for_dir<'ear>(
        &self,
        directory: ns::SearchPathDirectory,
        in_domain: ns::SearchPathDomainMask,
        appropriate_for_url: Option<&ns::Url>,
        create: bool,
    ) -> Result<arc::R<ns::Url>, Option<&'ear ns::Error>> {
        let mut error = None;
        if let Some(res) = unsafe {
            self.url_for_dir_err(
                directory,
                in_domain,
                appropriate_for_url,
                create,
                &mut error,
            )
        } {
            Ok(res)
        } else {
            Err(error)
        }
    }

    #[objc::msg_send(createDirectoryAtURL:withIntermediateDirectories:attributes:error:)]
    pub unsafe fn create_dir_at_url_err<'ear>(
        &self,
        url: &ns::Url,
        create_intermediates: bool,
        attributes: Option<&ns::Dictionary<ns::FileAttrKey, ns::Id>>,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn create_dir_at_url<'ear>(
        &self,
        url: &ns::Url,
        create_intermediates: bool,
        attributes: Option<&ns::Dictionary<ns::FileAttrKey, ns::Id>>,
    ) -> ns::Result {
        ns::if_false(|err| unsafe {
            self.create_dir_at_url_err(url, create_intermediates, attributes, err)
        })
    }

    #[objc::msg_send(createDirectoryAtPath:withIntermediateDirectories:attributes:error:)]
    pub unsafe fn create_dir_at_path_err<'ear>(
        &self,
        path: &ns::String,
        create_intermediates: bool,
        attributes: Option<&ns::Dictionary<ns::FileAttrKey, ns::Id>>,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    #[inline]
    pub fn create_dir_at_path<'ear>(
        &self,
        path: &ns::String,
        create_intermediates: bool,
        attributes: Option<&ns::Dictionary<ns::FileAttrKey, ns::Id>>,
    ) -> ns::Result {
        ns::if_false(|err| unsafe {
            self.create_dir_at_path_err(path, create_intermediates, attributes, err)
        })
    }

    #[objc::msg_send(removeItemAtPath:error:)]
    pub unsafe fn remove_item_at_path_err<'ear>(
        &self,
        path: &ns::String,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    #[inline]
    pub fn remove_item_at_path<'ear>(&self, path: &ns::String) -> ns::Result {
        ns::if_false(|err| unsafe { self.remove_item_at_path_err(path, err) })
    }

    #[objc::msg_send(currentDirectoryPath)]
    pub fn current_dir_path(&self) -> arc::R<ns::String>;

    #[objc::msg_send(changeCurrentDirectoryPath:)]
    pub fn change_current_dir_path(&mut self, path: &ns::String) -> bool;

    #[objc::msg_send(fileExistsAtPath:)]
    pub fn file_exists_at_path(&self, path: &ns::String) -> bool;

    #[objc::msg_send(fileExistsAtPath:isDirectory:)]
    pub fn file_exists_at_path_is_dir(&self, path: &ns::String, is_dir: &mut bool) -> bool;

    #[objc::msg_send(isReadableFileAtPath:)]
    pub fn is_readable_file_at_path(&self, path: &ns::String) -> bool;

    #[objc::msg_send(isWritableFileAtPath:)]
    pub fn is_writable_file_at_path(&self, path: &ns::String) -> bool;

    #[objc::msg_send(isExecutableFileAtPath:)]
    pub fn is_executable_file_at_path(&self, path: &ns::String) -> bool;

    #[objc::msg_send(isDeletableFileAtPath:)]
    pub fn is_deletable_file_at_path(&self, path: &ns::String) -> bool;

    #[objc::msg_send(setUbiquitous:itemAtURL:destinationURL:error:)]
    pub unsafe fn set_ubiquitous_item_err<'ar>(
        &mut self,
        value: bool,
        item_at_url: &ns::Url,
        dest_url: &ns::Url,
        error: *mut Option<&'ar ns::Error>,
    ) -> bool;

    #[inline]
    pub fn set_ubiquitous_item<'ear>(
        &mut self,
        value: bool,
        item_at_url: &ns::Url,
        dest_url: &ns::Url,
    ) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe {
            self.set_ubiquitous_item_err(value, item_at_url, dest_url, err)
        })
    }

    #[objc::msg_send(isUbiquitousItemAtURL:)]
    pub fn is_ubiquitous_item(&self, item_at_url: &ns::Url) -> bool;

    #[objc::msg_send(startDownloadingUbiquitousItemAtURL:error:)]
    pub fn start_downloading_ubquitous_item_err<'ar>(
        &mut self,
        item_at_url: &ns::Url,
        error: *mut Option<&'ar ns::Error>,
    ) -> bool;

    pub fn start_downloading_ubquitous_item<'ear>(
        &mut self,
        item_at_url: &ns::Url,
    ) -> ns::Result<'ear> {
        ns::if_false(|err| self.start_downloading_ubquitous_item_err(item_at_url, err))
    }

    #[objc::msg_send(evictUbiquitousItemAtURL:error:)]
    pub fn evict_ubiquitous_item_err<'ar>(
        &mut self,
        item_at_url: &ns::Url,
        error: *mut Option<&'ar ns::Error>,
    ) -> bool;

    #[inline]
    pub fn evict_ubiquitous_item<'ear>(&mut self, item_at_url: &ns::Url) -> ns::Result<'ear> {
        ns::if_false(|err| self.evict_ubiquitous_item_err(item_at_url, err))
    }

    #[cfg(target_os = "macos")]
    #[objc::msg_send(mountedVolumeURLsIncludingResourceValuesForKeys:options:)]
    pub fn mounted_volume_urls(
        &self,
        keys: Option<&ns::Array<ns::UrlResKey>>,
        options: ns::VolumeEnumOpts,
    ) -> arc::R<ns::Array<ns::Url>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(unmountVolumeAtURL:options:completionHandler:)]
    pub fn unmount_volume_at_url_ch_block(
        &self,
        url: &ns::Url,
        options: ns::FileManagerUnmountOpts,
        ch: &mut blocks::ErrCh,
    );

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn unmount_volume_at_url_ch(
        &self,
        url: &ns::Url,
        options: ns::FileManagerUnmountOpts,
        ch: impl FnMut(Option<&ns::Error>) + 'static,
    ) {
        let mut ch = blocks::ErrCh::new1(ch);
        self.unmount_volume_at_url_ch_block(url, options, &mut ch)
    }

    #[cfg(all(feature = "blocks", feature = "async"))]
    #[inline]
    pub async fn unmount_volume_at_url(
        &self,
        url: &ns::Url,
        options: ns::FileManagerUnmountOpts,
    ) -> Result<(), arc::R<ns::Error>> {
        let (future, mut ch) = blocks::ok();
        self.unmount_volume_at_url_ch_block(url, options, &mut ch);
        future.await
    }
}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_FILE_MANAGER: &'static objc::Class<FileManager>;
}

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    static NSFileProtectionNone: &'static ns::FileProtectionType;
    static NSFileProtectionComplete: &'static ns::FileProtectionType;
    static NSFileProtectionCompleteUnlessOpen: &'static ns::FileProtectionType;
    static NSFileProtectionCompleteUntilFirstUserAuthentication: &'static ns::FileProtectionType;

    static NSFileSystemSize: &'static ns::FileAttrKey;
    static NSFileSystemFreeSize: &'static ns::FileAttrKey;
    static NSFileSystemNodes: &'static ns::FileAttrKey;
    static NSFileSystemFreeNodes: &'static ns::FileAttrKey;
    static NSFileType: &'static ns::FileAttrKey;
    static NSFileProtectionKey: &'static ns::FileAttrKey;

    static NSFileSize: &'static ns::FileAttrKey;
    static NSFileModificationDate: &'static ns::FileAttrKey;
    static NSFileReferenceCount: &'static ns::FileAttrKey;
    static NSFileDeviceIdentifier: &'static ns::FileAttrKey;
    static NSFileOwnerAccountName: &'static ns::FileAttrKey;
    static NSFileGroupOwnerAccountName: &'static ns::FileAttrKey;
    static NSFilePosixPermissions: &'static ns::FileAttrKey;
    static NSFileSystemNumber: &'static ns::FileAttrKey;
    static NSFileSystemFileNumber: &'static ns::FileAttrKey;
    static NSFileExtensionHidden: &'static ns::FileAttrKey;
    static NSFileHFSCreatorCode: &'static ns::FileAttrKey;
    static NSFileHFSTypeCode: &'static ns::FileAttrKey;
    static NSFileImmutable: &'static ns::FileAttrKey;
    static NSFileAppendOnly: &'static ns::FileAttrKey;
    static NSFileCreationDate: &'static ns::FileAttrKey;
    static NSFileOwnerAccountID: &'static ns::FileAttrKey;
    static NSFileGroupOwnerAccountID: &'static ns::FileAttrKey;
    static NSFileBusy: &'static ns::FileAttrKey;

    static NSFileTypeDirectory: &'static ns::FileAttrType;
    static NSFileTypeRegular: &'static ns::FileAttrType;
    static NSFileTypeSymbolicLink: &'static ns::FileAttrType;
    static NSFileTypeSocket: &'static ns::FileAttrType;
    static NSFileTypeCharacterSpecial: &'static ns::FileAttrType;
    static NSFileTypeBlockSpecial: &'static ns::FileAttrType;
    static NSFileTypeUnknown: &'static ns::FileAttrType;
}

impl FileAttrKey {
    #[inline]
    pub fn file_type() -> &'static Self {
        unsafe { NSFileType }
    }

    #[inline]
    pub fn file_protection() -> &'static Self {
        unsafe { NSFileProtectionKey }
    }

    #[inline]
    pub fn file_system_size() -> &'static Self {
        unsafe { NSFileSystemSize }
    }

    #[inline]
    pub fn file_system_free_size() -> &'static Self {
        unsafe { NSFileSystemFreeSize }
    }

    #[inline]
    pub fn file_system_nodes() -> &'static Self {
        unsafe { NSFileSystemNodes }
    }

    #[inline]
    pub fn file_system_free_nodes() -> &'static Self {
        unsafe { NSFileSystemFreeNodes }
    }

    #[inline]
    pub fn size() -> &'static Self {
        unsafe { NSFileSize }
    }

    #[inline]
    pub fn modification_date() -> &'static Self {
        unsafe { NSFileModificationDate }
    }
    #[inline]
    pub fn reference_count() -> &'static Self {
        unsafe { NSFileReferenceCount }
    }

    #[inline]
    pub fn device_id() -> &'static Self {
        unsafe { NSFileDeviceIdentifier }
    }

    #[inline]
    pub fn owner_account_name() -> &'static Self {
        unsafe { NSFileOwnerAccountName }
    }

    #[inline]
    pub fn group_owner_account_name() -> &'static Self {
        unsafe { NSFileGroupOwnerAccountName }
    }

    #[inline]
    pub fn posix_permissions() -> &'static Self {
        unsafe { NSFilePosixPermissions }
    }

    #[inline]
    pub fn file_system_number() -> &'static Self {
        unsafe { NSFileSystemNumber }
    }

    #[inline]
    pub fn file_system_file_number() -> &'static Self {
        unsafe { NSFileSystemFileNumber }
    }

    #[inline]
    pub fn extension_hidden() -> &'static Self {
        unsafe { NSFileExtensionHidden }
    }

    #[inline]
    pub fn hfs_creator_code() -> &'static Self {
        unsafe { NSFileHFSCreatorCode }
    }

    #[inline]
    pub fn hfs_type_code() -> &'static Self {
        unsafe { NSFileHFSTypeCode }
    }

    #[inline]
    pub fn immutable() -> &'static Self {
        unsafe { NSFileImmutable }
    }

    #[inline]
    pub fn append_only() -> &'static Self {
        unsafe { NSFileAppendOnly }
    }

    #[inline]
    pub fn creation_date() -> &'static Self {
        unsafe { NSFileCreationDate }
    }

    #[inline]
    pub fn owner_account_id() -> &'static Self {
        unsafe { NSFileOwnerAccountID }
    }

    #[inline]
    pub fn group_owner_account_id() -> &'static Self {
        unsafe { NSFileGroupOwnerAccountID }
    }
    #[inline]
    pub fn busy() -> &'static Self {
        unsafe { NSFileBusy }
    }
}

impl FileAttrType {
    #[inline]
    pub fn directory() -> &'static Self {
        unsafe { NSFileTypeDirectory }
    }

    #[inline]
    pub fn regular() -> &'static Self {
        unsafe { NSFileTypeRegular }
    }

    #[inline]
    pub fn symbolic_link() -> &'static Self {
        unsafe { NSFileTypeSymbolicLink }
    }

    #[inline]
    pub fn socket() -> &'static Self {
        unsafe { NSFileTypeSocket }
    }

    #[inline]
    pub fn character_special() -> &'static Self {
        unsafe { NSFileTypeCharacterSpecial }
    }

    #[inline]
    pub fn block_special() -> &'static Self {
        unsafe { NSFileTypeBlockSpecial }
    }

    #[inline]
    pub fn unknown() -> &'static Self {
        unsafe { NSFileTypeUnknown }
    }
}

impl FileProtectionType {
    #[inline]
    pub fn none() -> &'static Self {
        unsafe { NSFileProtectionNone }
    }

    #[inline]
    pub fn complete() -> &'static Self {
        unsafe { NSFileProtectionComplete }
    }
    #[inline]
    pub fn complete_unless_open() -> &'static Self {
        unsafe { NSFileProtectionCompleteUnlessOpen }
    }

    #[inline]
    pub fn complete_until_first_user_authentication() -> &'static Self {
        unsafe { NSFileProtectionCompleteUntilFirstUserAuthentication }
    }
}

impl<K: objc::Obj, V: objc::Obj> ns::Dictionary<K, V> {
    #[objc::msg_send(fileSize)]
    pub fn file_size(&self) -> usize;

    #[objc::msg_send(file_type)]
    pub fn file_type(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(file_type)]
    pub fn file_posix_permissions(&self) -> ns::UInteger;

    #[objc::msg_send(fileOwnerAccountName)]
    pub fn file_owner_account_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(fileGroupOwnerAccountName)]
    pub fn file_group_owner_account_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(fileSystemNumber)]
    pub fn file_system_number(&self) -> ns::Integer;

    #[objc::msg_send(fileSystemFileNumber)]
    pub fn file_system_file_number(&self) -> ns::UInteger;

    #[objc::msg_send(fileExtensionHidden)]
    pub fn file_extension_hidden(&self) -> bool;

    #[objc::msg_send(fileHFSCreatorCode)]
    pub fn file_hfs_creator_code(&self) -> os::Type;

    #[objc::msg_send(fileHFSTypeCode)]
    pub fn file_hfs_type_code(&self) -> os::Type;

    #[objc::msg_send(fileIsImmutable)]
    pub fn file_is_immutable(&self) -> bool;

    #[objc::msg_send(fileIsAppendOnly)]
    pub fn file_is_append_only(&self) -> bool;

    #[objc::msg_send(fileModificationDate)]
    pub fn file_modification_date(&self) -> Option<arc::R<ns::Date>>;

    #[objc::msg_send(fileCreationDate)]
    pub fn file_creation_date(&self) -> Option<arc::R<ns::Date>>;

    #[objc::msg_send(fileOwnerAccountID)]
    pub fn file_owner_account_id(&self) -> Option<arc::R<ns::Number>>;

    #[objc::msg_send(fileGroupOwnerAccountID)]
    pub fn file_group_owner_account_id(&self) -> Option<arc::R<ns::Number>>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let fm = ns::FileManager::default();
        println!("{fm:?}");
        let url = ns::Url::with_fs_path_str("/tmp/", true);
        let list = fm
            .contents_of_dir_at_url(&url, None, Default::default())
            .expect("Failed to list {url:?}");
        assert!(!list.is_empty());
    }

    #[test]
    fn throws() {
        //> Passing a directory and domain pair that makes no sense
        //> (for example NSDesktopDirectory and NSNetworkDomainMask) raises an exception.
        //
        // but actually it doesn't raises an exception. It just return none as error and none as result

        // NOTE: on macos 15.1 Beta (24B5035e) it is not a case

        // let fm = ns::FileManager::default();

        // let pwd = fm.current_dir_path();
        // assert!(!pwd.is_empty());

        // let err = ns::try_catch(|| {
        //     let f = fm.url_for_dir(
        //         ns::SearchPathDirectory::Desktop,
        //         ns::SearchPathDomainMask::NETWORK,
        //         None,
        //         true,
        //     );
        //     println!("{f:?}");
        //     f.err().unwrap()
        // });

        // assert_eq!(Ok(None), err);
    }

    #[test]
    pub fn create_dir() {
        let parent = ns::str!(c"/tmp/foo");

        let path = ns::str!(c"/tmp/foo/nest");
        let fm = ns::FileManager::default();

        let _r = fm.remove_item_at_path(parent); // don't care about result for now

        fm.create_dir_at_path(path, false, None)
            .expect_err("should fail");

        fm.create_dir_at_path(path, true, None).unwrap();
        assert!(fm.file_exists_at_path(&path));

        fm.remove_item_at_path(parent).unwrap();
        assert!(!fm.file_exists_at_path(parent));
    }

    #[cfg(target_os = "macos")]
    #[test]
    pub fn list_mounts() {
        let keys = ns::Array::from_slice(&[
            ns::UrlResKey::name(),
            ns::UrlResKey::is_volume(),
            ns::UrlResKey::path(),
            ns::UrlResKey::volume_url(),
            ns::UrlResKey::volume_id(),
            ns::UrlResKey::volume_localized_format_desc(),
            ns::UrlResKey::volume_total_capacity(),
            ns::UrlResKey::volume_url_for_remounting(),
            ns::UrlResKey::volume_mount_from_location(),
            ns::UrlResKey::volume_type_name(),
            ns::UrlResKey::volume_sub_type_name(),
        ]);

        let fm = ns::FileManager::default();
        let list = fm.mounted_volume_urls(Some(&keys), Default::default());

        for url in list.iter() {
            let values = url.res_values_for_keys(&keys);
            eprintln!("{values:?}");
        }
        // list.first().unwrap().att
        eprintln!("{list:?}");
    }
}
