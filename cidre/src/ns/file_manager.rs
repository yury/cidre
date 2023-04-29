use crate::{arc, define_obj_type, define_options, ns, objc};

define_obj_type!(URLResourceKey(ns::String));

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
    pub fn default() -> &'static FileManager;

    #[objc::msg_send(contentsOfDirectoryAtURL:includingPropertiesForKeys:options:error:)]
    pub fn contents_of_dir_at_url_error_ar(
        &self,
        url: &ns::URL,
        including_props_for_keys: Option<&ns::Array<ns::URLResourceKey>>,
        options: DirectoryEnumerationOptions,
        error: *mut Option<&'ar ns::Error>,
    ) -> Option<&'ar ns::Array<ns::URL>>;

    #[objc::rar_retain]
    pub fn contents_of_dir_at_url_error<'ear>(
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
            self.contents_of_dir_at_url_error(url, including_props_for_keys, options, &mut error);
        if res.is_none() {
            return Err(unsafe { error.unwrap_unchecked() });
        }
        unsafe { Ok(res.unwrap_unchecked()) }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_FILE_MANAGER: &'static objc::Class<FileManager>;
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
}
