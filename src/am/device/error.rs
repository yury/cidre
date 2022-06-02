use std::fmt::Debug;

use crate::cf;

use super::base::Error;

impl Error {
    pub const fn make(num: i32) -> Error {
        Self(0x3A << 26 | num)
    }

    #[inline]
    pub fn text<'a>(&self) -> Option<cf::Retained<'a, cf::String>> {
        unsafe { AMDCopyErrorText(*self) }
    }

    #[inline]
    pub fn is_ok(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    #[inline]
    pub unsafe fn to_result<T>(self, option: Option<T>) -> Result<T, Self> {
        if self.is_ok() {
            debug_assert!(option.is_some());
            Ok(option.unwrap_unchecked())
        } else {
            Err(self)
        }
    }

    #[inline]
    pub fn result(self) -> Result<(), Self> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self)
        }
    }

    pub const SUCCESS: Self = Self::make(0);
    pub const UNDEFINED: Self = Self::make(1);
    pub const BAD_HEADER: Self = Self::make(2);
    pub const NO_RESOURCES: Self = Self::make(3);
    pub const READ: Self = Self::make(4);
    pub const WRITE: Self = Self::make(5);
    pub const UNKNOWN_PACKET: Self = Self::make(6);
    pub const INVALID_ARGUMENT: Self = Self::make(7);
    pub const NOT_FOUND: Self = Self::make(8);
    pub const IS_DIRECTORY: Self = Self::make(9);
    pub const PERMISSION: Self = Self::make(10);
    pub const NOT_CONNECTED: Self = Self::make(11);
    pub const TIME_OUT: Self = Self::make(12);
    pub const OVERRUN: Self = Self::make(13);
    pub const EOF: Self = Self::make(14);
    pub const UNSUPPORTED: Self = Self::make(15);
    pub const FILE_EXISTS: Self = Self::make(16);
    pub const BUSY: Self = Self::make(17);
    pub const CRYPTO: Self = Self::make(18);
    pub const TOO_BIG: Self = Self::make(133);

    ///
    /// Error codes received from Lockdownd over the connection. MobileDevice itself
    /// should only use these to relay information about Lockdownd errors to clients.
    ///
    /// There should be an analogous AMDError for each kLDErrorXXX declared at the top
    /// of MobileDevice.c.
    ///

    pub const INVALID_RESPONSE: Self = Self::make(19);
    pub const MISSING_KEY: Self = Self::make(20);
    pub const MISSING_VALUE: Self = Self::make(21);
    pub const GET_PROHIBITED: Self = Self::make(22);
    pub const SET_PROHIBITED: Self = Self::make(23);
    pub const REMOVE_PROHIBITED: Self = Self::make(24);
    pub const IMMUTABLE_VALUE: Self = Self::make(25);
    pub const PASSWORD_PROTECTED: Self = Self::make(26);
    pub const USER_DENIED_PAIRING: Self = Self::make(149);
    pub const PAIRING_DIALOG_RESPONSE_PENDING: Self = Self::make(150);
    pub const MISSING_HOST_ID: Self = Self::make(27);
    pub const INVALID_HOST_ID: Self = Self::make(28);
    pub const SESSION_ACTIVE: Self = Self::make(29);
    pub const SESSION_INACTIVE: Self = Self::make(30);
    pub const MISSING_SESSION_ID: Self = Self::make(31);
    pub const INVALID_SESSION_ID: Self = Self::make(32);
    pub const MISSING_SERVICE: Self = Self::make(33);
    pub const INVALID_SERVICE: Self = Self::make(34);
    pub const SERVICE_LIMIT: Self = Self::make(91);
    pub const CHECKIN_SETUP_FAILED: Self = Self::make(94);
    pub const INVALID_CHECKIN: Self = Self::make(35);
    pub const CHECKIN_TIMEOUT: Self = Self::make(36);
    pub const CHECKIN_CONNECTION_FAILED: Self = Self::make(95);
    pub const CHECKIN_RECEIVE_FAILED: Self = Self::make(96);
    pub const CHECKIN_RESPONSE_FAILED: Self = Self::make(97);
    pub const CHECKIN_OUT_OF_MEMORY: Self = Self::make(105);
    pub const CHECKIN_SEND_FAILED: Self = Self::make(98);
    pub const MISSING_PAIR_RECORD: Self = Self::make(37);
    pub const INVALID_PAIR_RECORD: Self = Self::make(92);
    pub const SAVE_PAIR_RECORD_FAILED: Self = Self::make(104);
    pub const INVALID_ACTIVATION_RECORD: Self = Self::make(38);
    pub const MISSING_ACTIVATION_RECORD: Self = Self::make(39);
    pub const SERVICE_PROHIBITED: Self = Self::make(93);
    pub const ESCROW_LOCKED: Self = Self::make(129);
    pub const PAIRING_PROHIBITED: Self = Self::make(130);
    pub const PROHIBITED_BY_SUPERVISION: Self = Self::make(131);
    pub const FMI_PPROTECTED: Self = Self::make(153);
    pub const MC_PROTECTED: Self = Self::make(154);
    pub const MC_CHALLENGE_REQUIRED: Self = Self::make(155);

    /* End of Lockdown errors. */

    pub const WRONG_DROID: Self = Self::make(40);

    pub const SU_VERIFICATION: Self = Self::make(41);
    pub const SU_PATCH: Self = Self::make(42);
    pub const SU_FIRMWARE: Self = Self::make(43);

    pub const PROVISIONING_PROFILE_NOT_VALID: Self = Self::make(44);

    pub const SEND_MESSAGE: Self = Self::make(45);
    pub const RECEIVE_MESSAGE: Self = Self::make(46);
    pub const MISSING_OPTIONS: Self = Self::make(47);
    pub const MISSING_IMAGE_TYPE: Self = Self::make(48);
    pub const DIGEST_FAILED: Self = Self::make(49);
    pub const START_SERVICE: Self = Self::make(50);
    pub const INVALID_DISK_IMAGE: Self = Self::make(51);
    pub const MISSING_DIGEST: Self = Self::make(52);
    pub const MUX: Self = Self::make(53);

    ///
    /// Errors defined by either the MobileInstallationProxy or MobileInstallation.framework.
    ///

    pub const APPLICATION_ALREADY_INSTALLED: Self = Self::make(54);
    pub const APPLICATION_MOVE_FAILED: Self = Self::make(55);
    pub const APPLICATION_SINFCAPTURE_FAILED: Self = Self::make(56);
    /// OBSOLETE
    pub const APPLICATION_SANDBOX_FAILED: Self = Self::make(57);
    pub const APPLICATION_VERIFICATION_FAILED: Self = Self::make(58);
    /// OBSOLETE
    pub const ARCHIVE_DESTRUCTION_FAILED: Self = Self::make(59);
    /// OBSOLETE
    pub const BUNDLE_VERIFICATION_FAILED: Self = Self::make(60);
    pub const CARRIER_BUNDLE_COPY_FAILED: Self = Self::make(61);
    pub const CARRIER_BUNDLE_DIRECTORY_CREATION_FAILED: Self = Self::make(62);
    pub const CARRIER_BUNDLE_MISSING_SUPPORTED_SIMS: Self = Self::make(63);
    pub const COMM_CENTER_NOTIFICATION_FAILED: Self = Self::make(64);
    pub const CONTAINER_CREATION_FAILED: Self = Self::make(65);
    pub const CONTAINER_P0WN_FAILED: Self = Self::make(66);
    /// OBSOLETE
    pub const CONTAINER_REMOVAL_FAILED: Self = Self::make(67);
    /// OBSOLETE
    pub const EMBEDDED_PROFILE_INSTALL_FAILED: Self = Self::make(68);
    // pub const ERROR_ERROR: Self = Self::make(69); //Huh?
    pub const ERROR: Self = Self::make(69);
    pub const EXECUTABLE_TWIDDLE_FAILED: Self = Self::make(70);
    /// OBSOLETE
    pub const EXISTENCE_CHECK_FAILED: Self = Self::make(71);
    pub const INSTALL_MAP_UPDATE_FAILED: Self = Self::make(72);
    pub const MANIFEST_CAPTURE_FAILED: Self = Self::make(73);
    /// OBSOLETE
    pub const MAP_GENERATION_FAILED: Self = Self::make(74);
    pub const MISSING_BUNDLE_EXECUTABLE: Self = Self::make(75);
    pub const MISSING_BUNDLE_IDENTIFIER: Self = Self::make(76);
    pub const MISSING_BUNDLE_PATH: Self = Self::make(77);
    pub const MISSING_CONTAINER: Self = Self::make(78);
    /// OBSOLETE
    pub const NOTIFICATION_FAILED: Self = Self::make(79);
    pub const PACKAGE_EXTRACTION_FAILED: Self = Self::make(80);
    pub const PACKAGE_INSPECTION_FAILED: Self = Self::make(81);
    /// OBSOLETE
    pub const PACKAGE_MOVE_FAILED: Self = Self::make(82);
    pub const PATH_CONVERSION_FAILED: Self = Self::make(83);
    /// OBSOLETE
    pub const RESTORE_CONTAINER_FAILED: Self = Self::make(84);
    /// OBSOLETE
    pub const SEATBELT_PROFILE_REMOVAL_FAILED: Self = Self::make(85);
    pub const STAGE_CREATION_FAILED: Self = Self::make(86);
    pub const SYMLINK_FAILED: Self = Self::make(87);
    pub const ITUNES_ARTWORK_CAPTURE_FAILED: Self = Self::make(88);
    pub const ITUNES_METADATA_CAPTURE_FAILED: Self = Self::make(89);
    pub const ALREADY_ARCHIVED: Self = Self::make(90);
    pub const UNKNOWN_COMMAND: Self = Self::make(102);
    pub const API_INTERNAL: Self = Self::make(103);
    pub const DEVICE_OS_VERSION_TOO_LOW: Self = Self::make(126);
    pub const DEVICE_FAMILY_NOT_SUPPORTED: Self = Self::make(128);
    pub const PACKAGE_PATCH_FAILED: Self = Self::make(134);
    pub const INCORRECT_ARCHITECTURE: Self = Self::make(135);
    pub const PLUGIN_COPY_FAILED: Self = Self::make(136);
    pub const BREADCRUMB_FAILED: Self = Self::make(137);
    pub const BREADCRUMB_UNLOCK: Self = Self::make(138);
    pub const GEO_JSON_CAPTURE_FAILED: Self = Self::make(139);
    pub const NEWSSTAND_ARTWORK_CAPTURE_FAILED: Self = Self::make(140);
    pub const MISSING_COMMAND: Self = Self::make(141);
    pub const NOT_ENTITLED: Self = Self::make(142);
    pub const MISSING_PACKAGE_PATH: Self = Self::make(143);
    pub const MISSING_CONTAINER_PATH: Self = Self::make(144);
    pub const MISSING_APPLICATION_IDENTIFIER: Self = Self::make(145);
    pub const MISSING_ATTRIBUTE_VALUE: Self = Self::make(146);
    pub const LOOKUP_FAILED: Self = Self::make(147);
    pub const DICT_CREATION_FAILED: Self = Self::make(148);
    pub const INSTALL_PROHIBITED: Self = Self::make(151);
    pub const UNINSTALL_PROHIBITED: Self = Self::make(152);
    pub const MISSING_BUNDLE_VERSION: Self = Self::make(156);

    pub const APP_BLACKLISTED: Self = Self::make(157);
    pub const APPEX_BUNDLE_ID_NOT_PREFIXED: Self = Self::make(158);
    pub const APPEX_BUNDLE_ILLEGAL_XPC_SERVICE_DICT: Self = Self::make(159);
    pub const APPEX_BUNDLE_MISSING_NS_EXTENSION_DICT: Self = Self::make(160);
    pub const APPEX_BUNDLE_ILLEGAL_PACKAGE_TYPE_VALUE: Self = Self::make(161);
    pub const APPEX_BUNDLE_MISSING_CLASS_OR_STORYBOARD: Self = Self::make(162);
    pub const APPEX_BUNDLE_ILLEGAL_EXTENSION_CONTEXT_CLASS: Self = Self::make(163);
    pub const APPEX_BUNDLE_ILLEGAL_EXTENSION_CONTEXT_HOST_CLASS: Self = Self::make(164);
    pub const APPEX_BUNDLE_ILLEGAL_EXTENSION_VIEW_CONTROLLER_HOST_CLASS: Self = Self::make(165);
    pub const APPEX_BUNDLE_MISSING_EXTENSION_POINT_IDENTIFIER_STRING: Self = Self::make(166);
    pub const APPEX_BUNDLE_ILLEGAL_EXTENSION_POINT_IDENTIFIER_STRING: Self = Self::make(167);
    pub const APPEX_BUNDLE_ILLEGAL_EXTENSION_ATTRIBUTES_DICT: Self = Self::make(168);
    pub const APPEX_BUNDLE_ILLEGAL_EXTENSION_POINT_NAME_STRING: Self = Self::make(169);
    pub const APPEX_BUNDLE_ILLEGAL_EXTENSION_POINT_VERSION_STRING: Self = Self::make(170);
    pub const MISSING_BUNDLE_NAME_STRING: Self = Self::make(171);
    pub const MISSING_BUNDLE_DISPLAY_NAME_STRING: Self = Self::make(172);
    pub const ILLEGAL_BUNDLE_SHORT_VERSION_STRING: Self = Self::make(173);
    pub const ILLEGAL_XPCSERVICE_RUN_LOOP_TYPE: Self = Self::make(174);
    pub const ILLEGAL_XPCSERVICE_SERVICE_TYPE: Self = Self::make(175);
    pub const DUPLICATE_IDENTIFIER: Self = Self::make(176);
    pub const APPEX_BUNDLE_UNKNOWN_EXTENSION_POINT_IDENTIFIER_STRING: Self = Self::make(177);
    pub const MULTIPLE_FILE_PROVIDER_APPEX_BUNDLES: Self = Self::make(178);

    /* End MobileInstallation Errors. */

    /* Mux errors. */

    pub const MUX_CREATE_LISTENER_ERROR: Self = Self::make(99);
    pub const MUX_GET_LISTENER: Self = Self::make(100);
    pub const MUX_CONNECT: Self = Self::make(101);
    pub const DEVICE_TOO_NEW: Self = Self::make(106);
    pub const DEVICE_REF_NO_GOOD: Self = Self::make(107);
    pub const DEVICE_DISCONNECTED: Self = Self::make(132);

    /* Use this for the error conversion functions, not kAMDUndefinedError. */
    pub const CANNOT_TRANSLATE: Self = Self::make(108);

    /* Errors from the Mobile Image Mounter */

    pub const MOBILE_IMAGE_MOUNTER_MISSING_IMAGE_SIGNATURE: Self = Self::make(109);
    pub const MOBILE_IMAGE_MOUNTER_RESPONSE_CREATION_FAILED: Self = Self::make(110);
    pub const MOBILE_IMAGE_MOUNTER_MISSING_IMAGE_TYPE: Self = Self::make(111);
    pub const MOBILE_IMAGE_MOUNTER_MISSING_IMAGE_PATH: Self = Self::make(112);
    pub const MOBILE_IMAGE_MOUNTER_IMAGE_MAP_LOAD_FAILED: Self = Self::make(113);
    pub const MOBILE_IMAGE_MOUNTER_ALREADY_MOUNTED: Self = Self::make(114);
    pub const MOBILE_IMAGE_MOUNTER_IMAGE_MOVE_FAILED: Self = Self::make(115);
    pub const MOBILE_IMAGE_MOUNTER_MOUNT_PATH_MISSING: Self = Self::make(116);
    pub const MOBILE_IMAGE_MOUNTER_MOUNT_PATH_NOT_EMPTY: Self = Self::make(117);
    pub const MOBILE_IMAGE_MOUNTER_IMAGE_MOUNT_FAILED: Self = Self::make(118);
    pub const MOBILE_IMAGE_MOUNTER_TRUST_CACHE_LOAD_FAILED: Self = Self::make(119);
    pub const MOBILE_IMAGE_MOUNTER_DIGEST_FAILED: Self = Self::make(120);
    pub const MOBILE_IMAGE_MOUNTER_DIGEST_CREATION_FAILED: Self = Self::make(121);
    pub const MOBILE_IMAGE_MOUNTER_IMAGE_VERIFICATION_FAILED: Self = Self::make(122);
    pub const MOBILE_IMAGE_MOUNTER_IMAGE_INFO_CREATION_FAILED: Self = Self::make(123);
    pub const MOBILE_IMAGE_MOUNTER_IMAGE_MAP_STORE_FAILED: Self = Self::make(124);

    /* End of Mobile Image Mounter errors. */

    /* Bonjour errors. */

    pub const BONJOUR_SETUP: Self = Self::make(125);
    pub const NO_WIFI_SYNC_SUPPORT: Self = Self::make(127);

    ///
    /// This should always be the last error. When adding new error code, steal whatever
    /// value is currently being used by kAMDMaximumError, and then bump the value of
    /// kAMDMaximumError by one.
    ///
    pub const MAXIMUM: Self = Self::make(277);
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self.text() {
            Some(text) => text.to_string(),
            None => "<no description>".to_string(),
        };
        f.debug_struct("am::DeviceError")
            .field("code", &self.0)
            .field("text", &text)
            .finish()
    }
}

#[link(name = "MobileDevice", kind = "framework")]
extern "C" {
    fn AMDCopyErrorText<'a>(error: Error) -> Option<cf::Retained<'a, cf::String>>;
}

#[cfg(test)]
pub mod tests {
    use crate::cf;

    #[test]
    pub fn description() {
        let text = super::Error::TIME_OUT
            .text()
            .expect("description for error");
        let str = cf::String::from_str_no_copy("The operation timed out.");
        assert!(str.equal(&text));
        text.show();
        // let text = super::Error::MAXIMUM_ERROR.text().expect("description for error");
        // text.show();
        // let str = cf::String::from_str_no_copy("The operation timed out.");
        // assert!(str.equal(&text));
    }
}
