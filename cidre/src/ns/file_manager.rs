use crate::{arc, define_obj_type, define_options, ns, objc, os};

define_obj_type!(FileAttrKey(ns::String));
define_obj_type!(FileAttrType(ns::String));
define_obj_type!(FileProtectionType(ns::String));

define_options!(VolumeEnumerationOptions(usize));

impl VolumeEnumerationOptions {
    ///  The mounted volume enumeration will skip hidden volumes.
    pub const SKIP_HIDDEN_VOLUMES: Self = Self(1 << 1);

    /// The mounted volume enumeration will produce file reference URLs rather than path-based URLs.
    pub const PRODUCE_FILE_REFERENCE_URLS: Self = Self(1 << 2);
}

define_options!(DirectoryEnumerationOptions(usize));
impl DirectoryEnumerationOptions {
    /// Causes the to perform a shallow enumeration and not descend into directories it encounters.
    pub const SKIPS_SUBDIRECTORY_DESCENDANTS: Self = Self(1 << 0);

    /// Will cause the to not descend into packages.
    pub const SKIPS_PACKAGE_DESCENDANTS: Self = Self(1 << 1);

    /// Causes the to not enumerate hidden files.
    pub const SKIPS_HIDDEN_FILES: Self = Self(1 << 2);

    /// Causes the to enumerate each directory a second time after all of its contained files have been enumerated.
    /// Use isEnumeratingDirectoryPostOrder to differentiate a post-order enumerated directory from a pre-order one.
    pub const INCLUDES_DIRECTORIES_POST_ORDER: Self = Self(1 << 3);

    /// Causes the to always produce file path URLs relative to the directoryURL. This can reduce the size of each URL object returned during enumeration.
    pub const PRODUCES_RELATIVE_PATH_URLS: Self = Self(1 << 4);
}

define_options!(ItemReplacementOptions(usize));

impl ItemReplacementOptions {
    pub const USING_NEW_METADATA_ONLY: Self = Self(1 << 0);

    pub const WITHOUT_DELETING_BACKUP_ITEM: Self = Self(1 << 1);
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(isize)]
pub enum URLRelationship {
    Contains,
    Same,
    Other,
}

define_obj_type!(FileManager(ns::Id));

impl FileManager {
    #[inline]
    pub fn cls() -> &'static objc::Class<Self> {
        unsafe { NS_FILE_MANAGER }
    }

    #[objc::cls_msg_send(defaultManager)]
    pub fn default() -> &'static mut FileManager;

    #[objc::msg_send(contentsOfDirectoryAtURL:includingPropertiesForKeys:options:error:)]
    pub fn contents_of_dir_at_url_err_ar(
        &self,
        url: &ns::URL,
        including_props_for_keys: Option<&ns::Array<ns::URLResourceKey>>,
        options: DirectoryEnumerationOptions,
        error: *mut Option<&'ar ns::Error>,
    ) -> Option<arc::Rar<ns::Array<ns::URL>>>;

    #[objc::rar_retain]
    pub fn contents_of_dir_at_url_err<'ear>(
        &self,
        url: &ns::URL,
        including_props_for_keys: Option<&ns::Array<ns::URLResourceKey>>,
        options: DirectoryEnumerationOptions,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ns::Array<ns::URL>>>;

    pub fn contents_of_dir_at_url<'ar>(
        &self,
        url: &ns::URL,
        including_props_for_keys: Option<&ns::Array<ns::URLResourceKey>>,
        options: DirectoryEnumerationOptions,
    ) -> Result<arc::R<ns::Array<ns::URL>>, &'ar ns::Error> {
        let mut error = None;
        let res =
            self.contents_of_dir_at_url_err(url, including_props_for_keys, options, &mut error);
        if res.is_none() {
            return Err(unsafe { error.unwrap_unchecked() });
        }
        unsafe { Ok(res.unwrap_unchecked()) }
    }

    #[objc::msg_send(URLForDirectory:inDomain:appropriateForURL:create:error:)]
    pub fn url_for_dir_err_ar<'ear>(
        &self,
        directory: ns::SearchPathDirectory,
        in_domain: ns::SearchPathDomainMask,
        appropriate_for_url: Option<&ns::URL>,
        create: bool,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::Rar<ns::URL>>;

    #[objc::rar_retain]
    pub fn url_for_dir_err<'ear>(
        &self,
        directory: ns::SearchPathDirectory,
        in_domain: ns::SearchPathDomainMask,
        appropriate_for_url: Option<&ns::URL>,
        create: bool,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ns::URL>>;

    pub fn url_for_dir<'ear>(
        &self,
        directory: ns::SearchPathDirectory,
        in_domain: ns::SearchPathDomainMask,
        appropriate_for_url: Option<&ns::URL>,
        create: bool,
    ) -> Result<arc::R<ns::URL>, Option<&'ear ns::Error>> {
        let mut error = None;
        let url = self.url_for_dir_err(
            directory,
            in_domain,
            appropriate_for_url,
            create,
            &mut error,
        );
        if let Some(url) = url {
            return Ok(url);
        }
        Err(error)
    }

    #[objc::msg_send(createDirectoryAtURL:withIntermediateDirectories:attributes:error:)]
    pub fn create_dir_at_url_err<'ear>(
        &self,
        url: &ns::URL,
        create_intermediates: bool,
        attributes: Option<&ns::Dictionary<ns::FileAttrKey, ns::Id>>,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn create_dir_at_url<'ear>(
        &self,
        url: &ns::URL,
        create_intermediates: bool,
        attributes: Option<&ns::Dictionary<ns::FileAttrKey, ns::Id>>,
    ) -> Result<(), &'ear ns::Error> {
        let mut error = None;
        if self.create_dir_at_url_err(url, create_intermediates, attributes, &mut error) {
            Ok(())
        } else {
            Err(unsafe { error.unwrap_unchecked() })
        }
    }

    #[objc::msg_send(createDirectoryAtPath:withIntermediateDirectories:attributes:error:)]
    pub fn create_dir_at_path_err<'ear>(
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
    ) -> Result<(), &'ear ns::Error> {
        let mut error = None;
        if self.create_dir_at_path_err(path, create_intermediates, attributes, &mut error) {
            Ok(())
        } else {
            Err(unsafe { error.unwrap_unchecked() })
        }
    }

    #[objc::msg_send(removeItemAtPath:error:)]
    pub fn remove_item_at_path_err<'ear>(
        &self,
        path: &ns::String,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    #[inline]
    pub fn remove_item_at_path<'ear>(&self, path: &ns::String) -> Result<(), &'ear ns::Error> {
        let mut error = None;
        if self.remove_item_at_path_err(path, &mut error) {
            Ok(())
        } else {
            Err(unsafe { error.unwrap_unchecked() })
        }
    }

    #[objc::msg_send(currentDirectoryPath)]
    pub fn current_dir_path(&self) -> &ns::String;

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
    pub fn set_ubiquitous_item_err<'ar>(
        &mut self,
        value: bool,
        item_at_url: &ns::URL,
        dest_url: &ns::URL,
        error: *mut Option<&'ar ns::Error>,
    ) -> bool;

    #[inline]
    pub fn set_ubiquitous_item<'ar>(
        &mut self,
        value: bool,
        item_at_url: &ns::URL,
        dest_url: &ns::URL,
    ) -> Result<(), &'ar ns::Error> {
        let mut error = None;
        if self.set_ubiquitous_item_err(value, item_at_url, dest_url, &mut error) {
            Ok(())
        } else {
            Err(unsafe { error.unwrap_unchecked() })
        }
    }

    #[objc::msg_send(isUbiquitousItemAtURL:)]
    pub fn is_ubiquitous_item(&self, item_at_url: &ns::URL) -> bool;

    #[objc::msg_send(startDownloadingUbiquitousItemAtURL:error:)]
    pub fn start_downloading_ubquitous_item_err<'ar>(
        &mut self,
        item_at_url: &ns::URL,
        error: *mut Option<&'ar ns::Error>,
    ) -> bool;

    pub fn start_downloading_ubquitous_item<'ar>(
        &mut self,
        item_at_url: &ns::URL,
    ) -> Result<(), &'ar ns::Error> {
        let mut error = None;
        if self.start_downloading_ubquitous_item_err(item_at_url, &mut error) {
            Ok(())
        } else {
            Err(unsafe { error.unwrap_unchecked() })
        }
    }

    #[objc::msg_send(evictUbiquitousItemAtURL:error:)]
    pub fn evict_ubiquitous_item_err<'ar>(
        &mut self,
        item_at_url: &ns::URL,
        error: *mut Option<&'ar ns::Error>,
    ) -> bool;

    #[inline]
    pub fn evict_ubiquitous_item<'ar>(
        &mut self,
        item_at_url: &ns::URL,
    ) -> Result<(), &'ar ns::Error> {
        let mut error = None;
        if self.evict_ubiquitous_item_err(item_at_url, &mut error) {
            Ok(())
        } else {
            Err(unsafe { error.unwrap_unchecked() })
        }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_FILE_MANAGER: &'static objc::Class<FileManager>;
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
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
    pub fn device_identifier() -> &'static Self {
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
    pub fn file_type(&self) -> Option<&ns::String>;

    #[objc::msg_send(file_type)]
    pub fn file_posix_permissions(&self) -> ns::UInteger;

    #[objc::msg_send(fileOwnerAccountName)]
    pub fn file_owner_account_name(&self) -> Option<&ns::String>;

    #[objc::msg_send(fileGroupOwnerAccountName)]
    pub fn file_group_owner_account_name(&self) -> Option<&ns::String>;

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
    pub fn file_modification_date(&self) -> Option<&ns::Date>;

    #[objc::msg_send(fileCreationDate)]
    pub fn file_creation_date(&self) -> Option<&ns::Date>;

    #[objc::msg_send(fileOwnerAccountID)]
    pub fn file_owner_account_id(&self) -> Option<&ns::Number>;

    #[objc::msg_send(fileGroupOwnerAccountID)]
    pub fn file_group_owner_account_id(&self) -> Option<&ns::Number>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let fm = ns::FileManager::default();
        println!("{fm:?}");
        let url = ns::URL::with_fs_path_str("/tmp/", true);
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
        let fm = ns::FileManager::default();

        let pwd = fm.current_dir_path();
        assert!(!pwd.is_empty());

        let err = ns::try_catch(|| {
            fm.url_for_dir(
                ns::SearchPathDirectory::Desktop,
                ns::SearchPathDomainMask::NETWORK,
                None,
                true,
            )
            .err()
            .unwrap()
        });
        assert_eq!(Ok(None), err);
    }

    #[test]
    pub fn create_dir() {
        let parent = ns::String::with_str("/tmp/foo");

        let path = ns::String::with_str("/tmp/foo/nest");
        let fm = ns::FileManager::default();

        let _r = fm.remove_item_at_path(&parent); // don't care about result for now

        fm.create_dir_at_path(&path, false, None)
            .expect_err("should fail");

        fm.create_dir_at_path(&path, true, None).unwrap();
        assert!(fm.file_exists_at_path(&path));

        fm.remove_item_at_path(&parent).unwrap();
        assert!(!fm.file_exists_at_path(&parent));
    }
}
