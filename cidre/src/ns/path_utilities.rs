use crate::{
    arc, define_opts, ns,
    objc::{self, Obj},
};

impl ns::String {
    #[objc::msg_send(pathWithComponents:)]
    pub fn path_with_components(components: ns::Array<ns::String>) -> arc::R<Self>;

    #[objc::msg_send(pathComponents)]
    pub fn path_components(&self) -> arc::R<ns::Array<ns::String>>;

    #[objc::msg_send(isAbsolutePath)]
    pub fn is_abs_path(&self) -> bool;

    #[objc::msg_send(lastPathComponent)]
    pub fn last_path_component(&self) -> arc::R<ns::String>;

    #[objc::msg_send(stringByDeletingLastPathComponent)]
    pub fn string_by_deleting_last_path_component(&self) -> arc::R<ns::String>;

    #[objc::msg_send(stringByAppendingPathComponent:)]
    pub fn string_by_appending_path_component(&self, str: &ns::String) -> arc::R<ns::String>;

    #[objc::msg_send(pathExtension)]
    pub fn path_extension(&self) -> arc::R<ns::String>;

    #[objc::msg_send(stringByDeletingPathExtension)]
    pub fn string_by_deleting_path_extension(&self) -> arc::R<ns::String>;

    #[objc::msg_send(stringByAppendingPathExtension:)]
    pub fn string_by_appending_path_extension(&self, str: &ns::String) -> arc::R<ns::String>;

    #[objc::msg_send(stringByAbbreviatingWithTildeInPath)]
    pub fn string_by_abbreviating_with_tilde_in_path(&self) -> arc::R<ns::String>;

    #[objc::msg_send(stringByExpandingTildeInPath)]
    pub fn string_by_expanding_tilde_in_path(&self) -> arc::R<ns::String>;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum SearchPathDirectory {
    /// Supported applications (Applications)
    Application = 1,

    /// Unsupported applications, demonstration versions (Demos)
    DemoApplication,

    /// developer applications (Developer/Applications). DEPRECATED - there is no one single Developer directory.
    #[deprecated(note = "there is no one single Developer directory")]
    DeveloperApplication,

    /// System and network administration applications (Administration)
    AdminApplication,

    /// Various documentation, support, and configuration files, resources (Library)
    Library,

    /// Developer resources (Developer) DEPRECATED - there is no one single Developer directory.
    #[deprecated(note = "there is no one single Developer directory")]
    Developer,

    /// User home directories (Users)
    User,

    /// User home directories (Users)
    Documentation,

    /// documents (Documents)
    Document,

    /// Location of CoreServices directory (System/Library/CoreServices)
    CoreService,

    /// Location of autosaved documents (Documents/Autosaved)
    AutosavedInformation = 11,

    /// Location of user's desktop
    Desktop = 12,

    /// Location of discardable cache files (Library/Caches)
    Caches = 13,

    /// Location of application support files (plug-ins, etc) (Library/Application Support)
    ApplicationSupport = 14,

    /// Location of the user's "Downloads" directory
    Downloads = 15,

    /// Input methods (Library/Input Methods)
    InputMethods = 16,

    /// Location of user's Movies directory (~/Movies)
    Movies = 17,

    /// Location of user's Music directory (~/Music)
    Music = 18,

    /// Location of user's Pictures directory (~/Pictures)
    Pictures = 19,

    /// Location of CoreServices directory (System/Library/CoreServices)
    PrinterDescription = 20,

    /// Location of CoreServices directory (System/Library/CoreServices)
    SharedPublic = 21,

    /// Location of CoreServices directory (System/Library/CoreServices)
    PreferencePanes = 22,

    /// Location of CoreServices directory (System/Library/CoreServices)
    ApplicationScripts = 23,

    /// Location of CoreServices directory (System/Library/CoreServices)
    ItemReplacement = 99,

    /// User home directories (Users)
    AllApplications = 100,

    /// User home directories (Users)
    AllLibraries = 101,

    /// Location of Trash directory
    Trash = 102,
}

define_opts!(pub SearchPathDomainMask(usize));

impl SearchPathDomainMask {
    /// User's home directory --- place to install user's personal items (~)
    pub const USER: Self = Self(1);

    /// Local to the current machine --- place to install items available to everyone on this machine (/Library)
    pub const LOCAL: Self = Self(2);

    /// Publically available location in the local area network --- place to install items available on the network (/Network)
    pub const NETWORK: Self = Self(4);

    /// Provided by Apple, unmodifiable (/System)
    pub const SYSTEM: Self = Self(8);

    /// All domains: all of the above and future items
    pub const ALL: Self = Self(0x0ffff);
}

#[inline]
pub fn search_path_for_dirs_in_domains_ar(
    directory: SearchPathDirectory,
    domain_mask: SearchPathDomainMask,
    expand_tilde: bool,
) -> arc::Rar<ns::Array<ns::String>> {
    unsafe { NSSearchPathForDirectoriesInDomains(directory, domain_mask, expand_tilde) }
}

#[inline]
pub fn search_path_for_dirs_in_domains(
    directory: SearchPathDirectory,
    domain_mask: SearchPathDomainMask,
    expand_tilde: bool,
) -> arc::R<ns::Array<ns::String>> {
    arc::rar_retain(search_path_for_dirs_in_domains_ar(
        directory,
        domain_mask,
        expand_tilde,
    ))
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    fn NSSearchPathForDirectoriesInDomains(
        directory: SearchPathDirectory,
        domain_mask: SearchPathDomainMask,
        expand_tilde: bool,
    ) -> arc::Rar<ns::Array<ns::String>>;

    fn NSUserName() -> &'static ns::String;
    fn NSFullUserName() -> &'static ns::String;
    fn NSHomeDirectory() -> &'static ns::String;
    fn NSTemporaryDirectory() -> &'static ns::String;
    fn NSHomeDirectoryForUser(user_name: Option<&ns::String>) -> Option<arc::Rar<ns::String>>;
}

#[doc(alias = "NSUserName")]
#[inline]
pub fn user_name() -> &'static ns::String {
    unsafe { NSUserName() }
}

#[doc(alias = "NSFullUserName")]
#[inline]
pub fn full_user_name() -> &'static ns::String {
    unsafe { NSFullUserName() }
}

#[doc(alias = "NSHomeDirectory")]
#[inline]
pub fn home_dir() -> &'static ns::String {
    unsafe { NSHomeDirectory() }
}

#[doc(alias = "NSTemporaryDirectory")]
#[inline]
pub fn tmp_dir() -> &'static ns::String {
    unsafe { NSTemporaryDirectory() }
}

#[doc(alias = "NSHomeDirectoryForUser")]
#[inline]
pub fn home_dir_for_user_ar(user_name: Option<&ns::String>) -> Option<arc::Rar<ns::String>> {
    unsafe { NSHomeDirectoryForUser(user_name) }
}

#[doc(alias = "NSHomeDirectoryForUser")]
#[inline]
pub fn home_dir_for_user(user_name: Option<&ns::String>) -> Option<arc::R<ns::String>> {
    arc::rar_retain_option(home_dir_for_user_ar(user_name))
}

impl<T: Obj> ns::Array<T> {
    #[objc::msg_send(pathsMatchingExtensions:)]
    pub fn paths_matching_extensions(
        &self,
        filter_types: &ns::Array<ns::String>,
    ) -> arc::R<ns::Array<ns::String>>;
}

#[cfg(test)]
mod tests {
    use crate::{ns, objc};

    #[test]
    fn basics() {
        let uname1 = ns::user_name();
        let uname2 = ns::user_name();

        unsafe {
            assert_eq!(
                uname1.as_type_ref().as_type_ptr(),
                uname2.as_type_ref().as_type_ptr()
            );
        }
        objc::ar_pool(|| {
            let _user_name = ns::user_name();
        });

        let _fun = ns::full_user_name();

        let home = ns::home_dir();
        let parent = home.string_by_deleting_last_path_component();
        assert_eq!(parent.to_string(), "/Users");

        let home2 = ns::home_dir_for_user(Some(ns::user_name()))
            .expect("Failed to get current user home folder");

        assert!(home.is_equal(home2.as_ref()));

        let res = ns::search_path_for_dirs_in_domains(
            ns::SearchPathDirectory::User,
            ns::SearchPathDomainMask::LOCAL,
            false,
        );

        assert!(!res.is_empty());
    }
}
