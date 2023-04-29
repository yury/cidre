use crate::define_options;

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

define_options!(SearchPathDomainMask(usize));

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

// FOUNDATION_EXPORT NSArray<NSString *> *NSSearchPathForDirectoriesInDomains(NSSearchPathDirectory directory, NSSearchPathDomainMask domainMask, BOOL expandTilde);

// extern "C" {

// }
