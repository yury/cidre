use crate::{arc, cf, define_cf_type, os};

define_cf_type!(
    #[doc(alias = "SecCertificateRef")]
    Cert(cf::Type)
);

define_cf_type!(
    #[doc(alias = "SecIdentityRef")]
    Identity(cf::Type)
);

define_cf_type!(
    #[doc(alias = "SecKeyRef")]
    Key(cf::Type)
);

define_cf_type!(
    #[doc(alias = "SecPolicyRef")]
    Policy(cf::Type)
);

define_cf_type!(
    #[doc(alias = "SecAccessControlRef")]
    AccessControl(cf::Type)
);

#[cfg(target_os = "macos")]
define_cf_type!(
    #[doc(alias = "SecKeychainRef")]
    Keychain(cf::Type)
);

#[cfg(target_os = "macos")]
define_cf_type!(
    #[doc(alias = "SecKeychainItemRef")]
    KeychainItem(cf::Type)
);

#[cfg(target_os = "macos")]
define_cf_type!(
    #[doc(alias = "SecKeychainSearchRef")]
    KeychainSearch(cf::Type)
);

#[doc(alias = "SecKeychainAttrType")]
pub type KeychainAttrType = os::Type;

#[doc(alias = "SecKeychainAttribute")]
#[cfg(target_os = "macos")]
#[repr(C)]
pub struct KeychainAttribute {
    pub tag: KeychainAttrType,
    pub len: u32,
    pub data: *mut std::ffi::c_void,
}

#[doc(alias = "SecKeychainAttributeList")]
#[cfg(target_os = "macos")]
#[repr(C)]
pub struct KeychainAttributeList {
    pub count: u32,
    pub attr: *mut KeychainAttribute,
}

pub type KeychainStatus = u32;

#[cfg(target_os = "macos")]
define_cf_type!(
    #[doc(alias = "SecTrustedApplicationRef")]
    TrustedApp(cf::Type)
);

#[cfg(target_os = "macos")]
define_cf_type!(
    #[doc(alias = "SecAccessRef")]
    Access(cf::Type)
);

#[cfg(target_os = "macos")]
define_cf_type!(
    #[doc(alias = "SecACLRef")]
    Acl(cf::Type)
);

#[cfg(target_os = "macos")]
define_cf_type!(
    #[doc(alias = "SecPasswordRef")]
    Password(cf::Type)
);

#[doc(alias = "SecKeychainAttributeInfo")]
#[repr(C)]
pub struct KeychainAttributeInfo {
    pub count: u32,
    pub tag: *mut u32,
    pub format: *mut u32,
}

#[doc(alias = "SecCopyErrorMessageString")]
pub fn err_msg_string(status: os::Status) -> Option<arc::R<cf::String>> {
    unsafe { SecCopyErrorMessageString(status, std::ptr::null_mut()) }
}

pub mod err {
    use crate::os::Error;

    /// Function or operation not implemented.
    #[doc(alias = "errSecUnimplemented")]
    pub const UNIMPLEMENTED: Error = Error::new_unchecked(-4);

    /// Disk Full error.
    #[doc(alias = "errSecDiskFull")]
    pub const DISK_FULL: Error = Error::new_unchecked(-34);

    /// I/O error.
    #[doc(alias = "errSecIO")]
    pub const IO: Error = Error::new_unchecked(-36);

    /// File already open with write permission.
    #[doc(alias = "errSecOpWr")]
    pub const OP_WR: Error = Error::new_unchecked(-49);

    /// One or more parameters passed to a function were not valid.
    #[doc(alias = "errSecParam")]
    pub const PARAM: Error = Error::new_unchecked(-50);

    /// Write permissions error.
    #[doc(alias = "errSecWrPerm")]
    pub const WR_PERM: Error = Error::new_unchecked(-61);

    /// Failed to allocate memory.
    #[doc(alias = "errSecAllocate")]
    pub const ALLOCATE: Error = Error::new_unchecked(-108);

    /// User canceled the operation.
    #[doc(alias = "errSecUserCanceled")]
    pub const USER_CANCELED: Error = Error::new_unchecked(-128);

    /// Bad parameter or invalid state for operation.
    #[doc(alias = "errSecBadReq")]
    pub const BAD_REQ: Error = Error::new_unchecked(-909);

    #[doc(alias = "errSecInternalComponent")]
    pub const INTERNAL_COMPONENT: Error = Error::new_unchecked(-2070);

    #[doc(alias = "errSecCoreFoundationUnknown")]
    pub const CORE_FOUNDATION_UNKNOWN: Error = Error::new_unchecked(-4960);

    /// A required entitlement isn't present.
    #[doc(alias = "errSecMissingEntitlement")]
    pub const MISSING_ENTITLEMENT: Error = Error::new_unchecked(-34018);

    /// Client is restricted and is not permitted to perform this operation.
    #[doc(alias = "errSecRestrictedAPI")]
    pub const RESTRICTED_API: Error = Error::new_unchecked(-34020);

    /// No keychain is available. You may need to restart your computer.
    #[doc(alias = "errSecNotAvailable")]
    pub const NOT_AVAILABLE: Error = Error::new_unchecked(-25291);

    /// This keychain cannot be modified.
    #[doc(alias = "errSecReadOnly")]
    pub const READ_ONLY: Error = Error::new_unchecked(-25292);

    /// The user name or passphrase you entered is not correct.
    #[doc(alias = "errSecAuthFailed")]
    pub const AUTH_FAILED: Error = Error::new_unchecked(-25293);

    /// The specified keychain could not be found.
    #[doc(alias = "errSecNoSuchKeychain")]
    pub const NO_SUCH_KEYCHAIN: Error = Error::new_unchecked(-25294);

    /// The specified keychain is not a valid keychain file.
    #[doc(alias = "errSecInvalidKeychain")]
    pub const INVALID_KEYCHAIN: Error = Error::new_unchecked(-25295);

    /// A keychain with the same name already exists.
    #[doc(alias = "errSecDuplicateKeychain")]
    pub const DUPLICATE_KEYCHAIN: Error = Error::new_unchecked(-25296);

    /// The specified callback function is already installed.
    #[doc(alias = "errSecDuplicateCallback")]
    pub const DUPLICATE_CB: Error = Error::new_unchecked(-25297);

    /// The specified callback function is not valid.
    #[doc(alias = "errSecInvalidCallback")]
    pub const INVALID_CB: Error = Error::new_unchecked(-25298);

    /// The specified item already exists in the keychain.
    #[doc(alias = "errSecDuplicateItem")]
    pub const DUPLICATE_ITEM: Error = Error::new_unchecked(-25299);

    /// The specified item could not be found in the keychain.
    #[doc(alias = "errSecItemNotFound")]
    pub const ITEM_NOT_FOUND: Error = Error::new_unchecked(-25300);

    /// There is not enough memory available to use the specified item.
    #[doc(alias = "errSecBufferTooSmall")]
    pub const BUF_TOO_SMALL: Error = Error::new_unchecked(-25301);

    /// This item contains information which is too large or in a format that cannot be displayed.
    #[doc(alias = "errSecDataTooLarge")]
    pub const DATA_TOO_LARGE: Error = Error::new_unchecked(-25302);

    /// The specified attribute does not exist.
    #[doc(alias = "errSecNoSuchAttr")]
    pub const NO_SUCH_ATTR: Error = Error::new_unchecked(-25303);

    /// The specified item is no longer valid. It may have been deleted from the keychain.
    #[doc(alias = "errSecInvalidItemRef")]
    pub const INVALID_ITEM_REF: Error = Error::new_unchecked(-25304);

    /// Unable to search the current keychain.
    #[doc(alias = "errSecInvalidSearchRef")]
    pub const INVALID_SEARCH_REF: Error = Error::new_unchecked(-25305);

    /// The specified item does not appear to be a valid keychain item.
    #[doc(alias = "errSecNoSuchClass")]
    pub const NO_SUCH_CLASS: Error = Error::new_unchecked(-25306);

    /// A default keychain could not be found.
    #[doc(alias = "errSecNoDefaultKeychain")]
    pub const NO_DEFAULT_KEYCHAIN: Error = Error::new_unchecked(-25307);

    /// User interaction is not allowed.
    #[doc(alias = "errSecInteractionNotAllowed")]
    pub const INTERACTION_NOT_ALLOWED: Error = Error::new_unchecked(-25308);

    /// The specified attribute could not be modified.
    #[doc(alias = "errSecReadOnlyAttr")]
    pub const READ_ONLY_ATTR: Error = Error::new_unchecked(-25309);

    /// This keychain was created by a different version of the system software and cannot be opened.
    #[doc(alias = "errSecWrongSecVersion")]
    pub const WRONG_SEC_VERSION: Error = Error::new_unchecked(-25310);

    /// This item specifies a key size which is too large or too small.
    #[doc(alias = "errSecKeySizeNotAllowed")]
    pub const KEY_SIZE_NOT_ALLOWED: Error = Error::new_unchecked(-25311);

    /// A required component (data storage module) could not be loaded. You may need to restart your computer.
    #[doc(alias = "errSecNoStorageModule")]
    pub const NO_STORAGE_MODULE: Error = Error::new_unchecked(-25312);

    /// A required component (certificate module) could not be loaded. You may need to restart your computer.
    #[doc(alias = "errSecNoCertificateModule")]
    pub const NO_CERT_MODULE: Error = Error::new_unchecked(-25313);

    /// A required component (policy module) could not be loaded. You may need to restart your computer.
    #[doc(alias = "errSecNoPolicyModule")]
    pub const NO_POLICY_MODULE: Error = Error::new_unchecked(-25314);

    /// User interaction is required, but is currently not allowed.
    #[doc(alias = "errSecInteractionRequired")]
    pub const INTERACTION_REQUIRED: Error = Error::new_unchecked(-25315);

    /// The contents of this item cannot be retrieved.
    #[doc(alias = "errSecDataNotAvailable")]
    pub const DATA_NOT_AVAILABLE: Error = Error::new_unchecked(-25316);

    /// The contents of this item cannot be modified.
    #[doc(alias = "errSecDataNotModifiable")]
    pub const DATA_NOT_MODIFIABLE: Error = Error::new_unchecked(-25317);

    /// One or more certificates required to validate this certificate cannot be found.
    #[doc(alias = "errSecCreateChainFailed")]
    pub const CREATE_CHAIN_FAILED: Error = Error::new_unchecked(-25318);

    /// The specified preferences domain is not valid.
    #[doc(alias = "errSecInvalidPrefsDomain")]
    pub const INVALID_PREFS_DOMAIN: Error = Error::new_unchecked(-25319);

    /// In dark wake, no UI possible
    #[doc(alias = "errSecInDarkWake")]
    pub const IN_DARK_WAKE: Error = Error::new_unchecked(-25320);

    /// The specified access control list is not in standard (simple) form.
    #[doc(alias = "errSecACLNotSimple")]
    pub const ACL_NOT_SIMPLE: Error = Error::new_unchecked(-25240);

    /// The specified policy cannot be found.
    #[doc(alias = "errSecPolicyNotFound")]
    pub const POLICY_NOT_FOUND: Error = Error::new_unchecked(-25241);

    /// The specified trust setting is invalid.
    #[doc(alias = "errSecInvalidTrustSetting")]
    pub const INVALID_TRUST_SETTING: Error = Error::new_unchecked(-25242);

    /// The specified item has no access control.
    #[doc(alias = "errSecNoAccessForItem")]
    pub const NO_ACCESS_FOR_ITEM: Error = Error::new_unchecked(-25243);

    /// Invalid attempt to change the owner of this item.
    #[doc(alias = "errSecInvalidOwnerEdit")]
    pub const INVALID_OWNER_EDIT: Error = Error::new_unchecked(-25244);

    /// No trust results are available.
    #[doc(alias = "errSecTrustNotAvailable")]
    pub const TRUST_NOT_AVAILABLE: Error = Error::new_unchecked(-25245);

    /// Import/Export format unsupported.
    #[doc(alias = "errSecUnsupportedFormat")]
    pub const UNSUPPORTED_FORMAT: Error = Error::new_unchecked(-25256);

    /// Unknown format in import.
    #[doc(alias = "errSecUnknownFormat")]
    pub const UNKNOWN_FORMAT: Error = Error::new_unchecked(-25257);

    /// Key material must be wrapped for export.
    #[doc(alias = "errSecKeyIsSensitive")]
    pub const KEY_IS_SENSITIVE: Error = Error::new_unchecked(-25258);

    /// An attempt was made to import multiple private keys.
    #[doc(alias = "errSecMultiplePrivKeys")]
    pub const MULTIPLE_PRIV_KEYS: Error = Error::new_unchecked(-25259);

    /// Passphrase is required for import/export.
    #[doc(alias = "errSecPassphraseRequired")]
    pub const PASSPHRASE_REQUIRED: Error = Error::new_unchecked(-25260);

    /// The password reference was invalid.
    #[doc(alias = "errSecInvalidPasswordRef")]
    pub const INVALID_PASSWORD_REF: Error = Error::new_unchecked(-25261);

    /// The Trust Settings Record was corrupted.
    #[doc(alias = "errSecInvalidTrustSettings")]
    pub const INVALID_TRUST_SETTINGS: Error = Error::new_unchecked(-25262);

    /// No Trust Settings were found.
    #[doc(alias = "errSecNoTrustSettings")]
    pub const NO_TRUST_SETTINGS: Error = Error::new_unchecked(-25263);

    /// MAC verification failed during PKCS12 import (wrong password?)
    #[doc(alias = "errSecPkcs12VerifyFailure")]
    pub const PKCS12_VERIFY_FAILURE: Error = Error::new_unchecked(-25264);

    /// A certificate was not signed by its proposed parent.
    #[doc(alias = "errSecNotSigner")]
    pub const NOT_SIGNER: Error = Error::new_unchecked(-26267);

    /// Unable to decode the provided data.
    #[doc(alias = "errSecDecode")]
    pub const DECODE: Error = Error::new_unchecked(-26275);

    /// The required service is not available.
    #[doc(alias = "errSecServiceNotAvailable")]
    pub const SERVICE_NOT_AVAILABLE: Error = Error::new_unchecked(-67585);

    /// The client ID is not correct.
    #[doc(alias = "errSecInsufficientClientID")]
    pub const INSUFFICIENT_CLIENT_ID: Error = Error::new_unchecked(-67586);

    /// A device reset has occurred.
    #[doc(alias = "errSecDeviceReset")]
    pub const DEVICE_RESET: Error = Error::new_unchecked(-67587);

    /// A device failure has occurred.
    #[doc(alias = "errSecDeviceFailed")]
    pub const DEVICE_FAILED: Error = Error::new_unchecked(-67588);

    /// Adding an application ACL subject failed.
    #[doc(alias = "errSecAppleAddAppACLSubject")]
    pub const APPLE_ADD_APP_ACL_SUBJECT: Error = Error::new_unchecked(-67589);

    /// The public key is incomplete.
    #[doc(alias = "errSecApplePublicKeyIncomplete")]
    pub const APPLE_PUBLIC_KEY_INCOMPLETE: Error = Error::new_unchecked(-67590);

    /// A signature mismatch has occurred.
    #[doc(alias = "errSecAppleSignatureMismatch")]
    pub const APPLE_SIGNATURE_MISMATCH: Error = Error::new_unchecked(-67591);

    /// The specified key has an invalid start date.
    #[doc(alias = "errSecAppleInvalidKeyStartDate")]
    pub const APPLE_INVALID_KEY_START_DATE: Error = Error::new_unchecked(-67592);

    /// The specified key has an invalid end date.
    #[doc(alias = "errSecAppleInvalidKeyEndDate")]
    pub const APPLE_INVALID_KEY_END_DATE: Error = Error::new_unchecked(-67593);

    /// A conversion error has occurred.
    #[doc(alias = "errSecConversionError")]
    pub const CONVERSION_ERROR: Error = Error::new_unchecked(-67594);

    /// A SSLv2 rollback error has occurred.
    #[doc(alias = "errSecAppleSSLv2Rollback")]
    pub const APPLE_SSL_V2_ROLLBACK: Error = Error::new_unchecked(-67595);

    /// The quota was exceeded.
    #[doc(alias = "errSecQuotaExceeded")]
    pub const QUOTA_EXCEEDED: Error = Error::new_unchecked(-67596);

    /// The file is too big.
    #[doc(alias = "errSecFileTooBig")]
    pub const FILE_TOO_BIG: Error = Error::new_unchecked(-67597);

    /// The specified database has an invalid blob.
    #[doc(alias = "errSecInvalidDatabaseBlob")]
    pub const INVALID_DATABASE_BLOB: Error = Error::new_unchecked(-67598);

    /// The specified database has an invalid key blob.
    #[doc(alias = "errSecInvalidKeyBlob")]
    pub const INVALID_KEY_BLOB: Error = Error::new_unchecked(-67599);

    /// The specified database has an incompatible blob.
    #[doc(alias = "errSecIncompatibleDatabaseBlob")]
    pub const INCOMPATIBLE_DATABASE_BLOB: Error = Error::new_unchecked(-67600);

    /// The specified database has an incompatible key blob.
    #[doc(alias = "errSecIncompatibleKeyBlob")]
    pub const INCOMPATIBLE_KEY_BLOB: Error = Error::new_unchecked(-67601);

    /// A host name mismatch has occurred.
    #[doc(alias = "errSecHostNameMismatch")]
    pub const HOST_NAME_MISMATCH: Error = Error::new_unchecked(-67602);

    /// There is an unknown critical extension flag.
    #[doc(alias = "errSecUnknownCriticalExtensionFlag")]
    pub const UNKNOWN_CRITICAL_EXTENSION_FLAG: Error = Error::new_unchecked(-67603);

    /// No basic constraints were found.
    #[doc(alias = "errSecNoBasicConstraints")]
    pub const NO_BASIC_CONSTRAINTS: Error = Error::new_unchecked(-67604);

    /// No basic CA constraints were found.
    #[doc(alias = "errSecNoBasicConstraintsCA")]
    pub const NO_BASIC_CONSTRAINTS_CA: Error = Error::new_unchecked(-67605);

    /// The authority key ID is not valid.
    #[doc(alias = "errSecInvalidAuthorityKeyID")]
    pub const INVALID_AUTHORITY_KEY_ID: Error = Error::new_unchecked(-67606);

    /// The subject key ID is not valid.
    #[doc(alias = "errSecInvalidSubjectKeyID")]
    pub const INVALID_SUBJECT_KEY_ID: Error = Error::new_unchecked(-67607);

    /// The key usage is not valid for the specified policy.
    #[doc(alias = "errSecInvalidKeyUsageForPolicy")]
    pub const INVALID_KEY_USAGE_FOR_POLICY: Error = Error::new_unchecked(-67608);

    /// The extended key usage is not valid.
    #[doc(alias = "errSecInvalidExtendedKeyUsage")]
    pub const INVALID_EXTENDED_KEY_USAGE: Error = Error::new_unchecked(-67609);

    /// The ID linkage is not valid.
    #[doc(alias = "errSecInvalidIDLinkage")]
    pub const INVALID_ID_LINKAGE: Error = Error::new_unchecked(-67610);

    /// The path length constraint was exceeded.
    #[doc(alias = "errSecPathLengthConstraintExceeded")]
    pub const PATH_LENGTH_CONSTRAINT_EXCEEDED: Error = Error::new_unchecked(-67611);

    /// The root or anchor certificate is not valid.
    #[doc(alias = "errSecInvalidRoot")]
    pub const INVALID_ROOT: Error = Error::new_unchecked(-67612);

    /// The CRL has expired.
    #[doc(alias = "errSecCRLExpired")]
    pub const CRL_EXPIRED: Error = Error::new_unchecked(-67613);

    /// The CRL is not yet valid.
    #[doc(alias = "errSecCRLNotValidYet")]
    pub const CRL_NOT_VALID_YET: Error = Error::new_unchecked(-67614);

    /// The CRL was not found.
    #[doc(alias = "errSecCRLNotFound")]
    pub const CRL_NOT_FOUND: Error = Error::new_unchecked(-67615);

    /// The CRL server is down.
    #[doc(alias = "errSecCRLServerDown")]
    pub const CRL_SERVER_DOWN: Error = Error::new_unchecked(-67616);

    /// The CRL has a bad Uniform Resource Identifier.
    #[doc(alias = "errSecCRLBadURI")]
    pub const CRL_BAD_URI: Error = Error::new_unchecked(-67617);

    /// An unknown certificate extension was encountered.
    #[doc(alias = "errSecUnknownCertExtension")]
    pub const UNKNOWN_CERT_EXTENSION: Error = Error::new_unchecked(-67618);

    /// An unknown CRL extension was encountered.
    #[doc(alias = "errSecUnknownCRLExtension")]
    pub const UNKNOWN_CRL_EXTENSION: Error = Error::new_unchecked(-67619);

    /// The CRL is not trusted.
    #[doc(alias = "errSecCRLNotTrusted")]
    pub const CRL_NOT_TRUSTED: Error = Error::new_unchecked(-67620);

    /// The CRL policy failed.
    #[doc(alias = "errSecCRLPolicyFailed")]
    pub const CRL_POLICY_FAILED: Error = Error::new_unchecked(-67621);

    /// The issuing distribution point was not valid.
    #[doc(alias = "errSecIDPFailure")]
    pub const IDP_FAILURE: Error = Error::new_unchecked(-67622);

    /// An email address mismatch was encountered.
    #[doc(alias = "errSecSMIMEEmailAddressesNotFound")]
    pub const SMIME_EMAIL_ADDRESSES_NOT_FOUND: Error = Error::new_unchecked(-67623);

    /// The appropriate extended key usage for SMIME was not found.
    #[doc(alias = "errSecSMIMEBadExtendedKeyUsage")]
    pub const SMIME_BAD_EXTENDED_KEY_USAGE: Error = Error::new_unchecked(-67624);

    /// The key usage is not compatible with SMIME.
    #[doc(alias = "errSecSMIMEBadKeyUsage")]
    pub const SMIME_BAD_KEY_USAGE: Error = Error::new_unchecked(-67625);

    /// The key usage extension is not marked as critical.
    #[doc(alias = "errSecSMIMEKeyUsageNotCritical")]
    pub const SMIME_KEY_USAGE_NOT_CRITICAL: Error = Error::new_unchecked(-67626);

    /// No email address was found in the certificate.
    #[doc(alias = "errSecSMIMENoEmailAddress")]
    pub const SMIME_NO_EMAIL_ADDRESS: Error = Error::new_unchecked(-67627);

    /// The subject alternative name extension is not marked as critical.
    #[doc(alias = "errSecSMIMESubjAltNameNotCritical")]
    pub const SMIME_SUBJ_ALT_NAME_NOT_CRITICAL: Error = Error::new_unchecked(-67628);

    /// The appropriate extended key usage for SSL was not found.
    #[doc(alias = "errSecSSLBadExtendedKeyUsage")]
    pub const SSL_BAD_EXTENDED_KEY_USAGE: Error = Error::new_unchecked(-67629);

    /// The OCSP response was incorrect or could not be parsed.
    #[doc(alias = "errSecOCSPBadResponse")]
    pub const OCSP_BAD_RESPONSE: Error = Error::new_unchecked(-67630);

    /// The OCSP request was incorrect or could not be parsed.
    #[doc(alias = "errSecOCSPBadRequest")]
    pub const OCSP_BAD_REQUEST: Error = Error::new_unchecked(-67631);

    /// OCSP service is unavailable.
    #[doc(alias = "errSecOCSPUnavailable")]
    pub const OCSP_UNAVAILABLE: Error = Error::new_unchecked(-67632);

    /// The OCSP server did not recognize this certificate.
    #[doc(alias = "errSecOCSPStatusUnrecognized")]
    pub const OCSP_STATUS_UNRECOGNIZED: Error = Error::new_unchecked(-67633);

    /// An end-of-data was detected.
    #[doc(alias = "errSecEndOfData")]
    pub const END_OF_DATA: Error = Error::new_unchecked(-67634);

    /// An incomplete certificate revocation check occurred.
    #[doc(alias = "errSecIncompleteCertRevocationCheck")]
    pub const INCOMPLETE_CERT_REVOCATION_CHECK: Error = Error::new_unchecked(-67635);

    /// A network failure occurred.
    #[doc(alias = "errSecNetworkFailure")]
    pub const NETWORK_FAILURE: Error = Error::new_unchecked(-67636);

    /// The OCSP response was not trusted to a root or anchor certificate.
    #[doc(alias = "errSecOCSPNotTrustedToAnchor")]
    pub const OCSP_NOT_TRUSTED_TO_ANCHOR: Error = Error::new_unchecked(-67637);

    /// The record was modified.
    #[doc(alias = "errSecRecordModified")]
    pub const RECORD_MODIFIED: Error = Error::new_unchecked(-67638);

    /// The OCSP response had an invalid signature.
    #[doc(alias = "errSecOCSPSignatureError")]
    pub const OCSP_SIGNATURE_ERROR: Error = Error::new_unchecked(-67639);

    /// The OCSP response had no signer.
    #[doc(alias = "errSecOCSPNoSigner")]
    pub const OCSP_NO_SIGNER: Error = Error::new_unchecked(-67640);

    /// The OCSP responder was given a malformed request.
    #[doc(alias = "errSecOCSPResponderMalformedReq")]
    pub const OCSP_RESPONDER_MALFORMED_REQ: Error = Error::new_unchecked(-67641);

    /// The OCSP responder encountered an internal error.
    #[doc(alias = "errSecOCSPResponderInternalError")]
    pub const OCSP_RESPONDER_INTERNAL_ERROR: Error = Error::new_unchecked(-67642);

    /// The OCSP responder is busy, try again later.
    #[doc(alias = "errSecOCSPResponderTryLater")]
    pub const OCSP_RESPONDER_TRY_LATER: Error = Error::new_unchecked(-67643);

    /// The OCSP responder requires a signature.
    #[doc(alias = "errSecOCSPResponderSignatureRequired")]
    pub const OCSP_RESPONDER_SIGNATURE_REQUIRED: Error = Error::new_unchecked(-67644);

    /// The OCSP responder rejected this request as unauthorized.
    #[doc(alias = "errSecOCSPResponderUnauthorized")]
    pub const OCSP_RESPONDER_UNAUTHORIZED: Error = Error::new_unchecked(-67645);

    /// The OCSP response nonce did not match the request.
    #[doc(alias = "errSecOCSPResponseNonceMismatch")]
    pub const OCSP_RESPONSE_NONCE_MISMATCH: Error = Error::new_unchecked(-67646);

    /// Code signing encountered an incorrect certificate chain length.
    #[doc(alias = "errSecCodeSigningBadCertChainLength")]
    pub const CODE_SIGNING_BAD_CERT_CHAIN_LENGTH: Error = Error::new_unchecked(-67647);

    /// Code signing found no basic constraints.
    #[doc(alias = "errSecCodeSigningNoBasicConstraints")]
    pub const CODE_SIGNING_NO_BASIC_CONSTRAINTS: Error = Error::new_unchecked(-67648);

    /// Code signing encountered an incorrect path length constraint.
    #[doc(alias = "errSecCodeSigningBadPathLengthConstraint")]
    pub const CODE_SIGNING_BAD_PATH_LENGTH_CONSTRAINT: Error = Error::new_unchecked(-67649);

    /// Code signing found no extended key usage.
    #[doc(alias = "errSecCodeSigningNoExtendedKeyUsage")]
    pub const CODE_SIGNING_NO_EXTENDED_KEY_USAGE: Error = Error::new_unchecked(-67650);

    /// Code signing indicated use of a development-only certificate.
    #[doc(alias = "errSecCodeSigningDevelopment")]
    pub const CODE_SIGNING_DEVELOPMENT: Error = Error::new_unchecked(-67651);

    /// Resource signing has encountered an incorrect certificate chain length.
    #[doc(alias = "errSecResourceSignBadCertChainLength")]
    pub const RESOURCE_SIGN_BAD_CERT_CHAIN_LENGTH: Error = Error::new_unchecked(-67652);

    /// Resource signing has encountered an error in the extended key usage.
    #[doc(alias = "errSecResourceSignBadExtKeyUsage")]
    pub const RESOURCE_SIGN_BAD_EXT_KEY_USAGE: Error = Error::new_unchecked(-67653);

    /// The trust setting for this policy was set to Deny.
    #[doc(alias = "errSecTrustSettingDeny")]
    pub const TRUST_SETTING_DENY: Error = Error::new_unchecked(-67654);

    /// An invalid certificate subject name was encountered.
    #[doc(alias = "errSecInvalidSubjectName")]
    pub const INVALID_SUBJECT_NAME: Error = Error::new_unchecked(-67655);

    /// An unknown qualified certificate statement was encountered.
    #[doc(alias = "errSecUnknownQualifiedCertStatement")]
    pub const UNKNOWN_QUALIFIED_CERT_STATEMENT: Error = Error::new_unchecked(-67656);

    #[doc(alias = "errSecMobileMeRequestQueued")]
    pub const MOBILE_ME_REQUEST_QUEUED: Error = Error::new_unchecked(-67657);

    #[doc(alias = "errSecMobileMeRequestRedirected")]
    pub const MOBILE_ME_REQUEST_REDIRECTED: Error = Error::new_unchecked(-67658);

    #[doc(alias = "errSecMobileMeServerError")]
    pub const MOBILE_ME_SERVER_ERROR: Error = Error::new_unchecked(-67659);

    #[doc(alias = "errSecMobileMeServerNotAvailable")]
    pub const MOBILE_ME_SERVER_NOT_AVAILABLE: Error = Error::new_unchecked(-67660);

    #[doc(alias = "errSecMobileMeServerAlreadyExists")]
    pub const MOBILE_ME_SERVER_ALREADY_EXISTS: Error = Error::new_unchecked(-67661);

    #[doc(alias = "errSecMobileMeServerServiceErr")]
    pub const MOBILE_ME_SERVER_SERVICE_ERR: Error = Error::new_unchecked(-67662);

    #[doc(alias = "errSecMobileMeRequestAlreadyPending")]
    pub const MOBILE_ME_REQUEST_ALREADY_PENDING: Error = Error::new_unchecked(-67663);

    #[doc(alias = "errSecMobileMeNoRequestPending")]
    pub const MOBILE_ME_NO_REQUEST_PENDING: Error = Error::new_unchecked(-67664);

    #[doc(alias = "errSecMobileMeCSRVerifyFailure")]
    pub const MOBILE_ME_CSR_VERIFY_FAILURE: Error = Error::new_unchecked(-67665);

    #[doc(alias = "errSecMobileMeFailedConsistencyCheck")]
    pub const MOBILE_ME_FAILED_CONSISTENCY_CHECK: Error = Error::new_unchecked(-67666);

    /// A function was called without initializing CSSM.
    #[doc(alias = "errSecNotInitialized")]
    pub const NOT_INITIALIZED: Error = Error::new_unchecked(-67667);

    /// The CSSM handle does not match with the service type.
    #[doc(alias = "errSecInvalidHandleUsage")]
    pub const INVALID_HANDLE_USAGE: Error = Error::new_unchecked(-67668);

    /// A reference to the calling module was not found in the list of authorized callers.
    #[doc(alias = "errSecPVCReferentNotFound")]
    pub const PVC_REFERENT_NOT_FOUND: Error = Error::new_unchecked(-67669);

    /// A function address was not within the verified module.
    #[doc(alias = "errSecFunctionIntegrityFail")]
    pub const FUNCTION_INTEGRITY_FAIL: Error = Error::new_unchecked(-67670);

    /// An internal error has occurred.
    #[doc(alias = "errSecInternalError")]
    pub const INTERNAL_ERROR: Error = Error::new_unchecked(-67671);

    /// A memory error has occurred.
    #[doc(alias = "errSecMemoryError")]
    pub const MEMORY_ERROR: Error = Error::new_unchecked(-67672);

    /// Invalid data was encountered.
    #[doc(alias = "errSecInvalidData")]
    pub const INVALID_DATA: Error = Error::new_unchecked(-67673);

    /// A Module Directory Service error has occurred.
    #[doc(alias = "errSecMDSError")]
    pub const MDS_ERROR: Error = Error::new_unchecked(-67674);

    /// An invalid pointer was encountered.
    #[doc(alias = "errSecInvalidPointer")]
    pub const INVALID_POINTER: Error = Error::new_unchecked(-67675);

    /// Self-check has failed.
    #[doc(alias = "errSecSelfCheckFailed")]
    pub const SELF_CHECK_FAILED: Error = Error::new_unchecked(-67676);

    /// A function has failed.
    #[doc(alias = "errSecFunctionFailed")]
    pub const FUNCTION_FAILED: Error = Error::new_unchecked(-67677);

    /// A module manifest verification failure has occurred.
    #[doc(alias = "errSecModuleManifestVerifyFailed")]
    pub const MODULE_MANIFEST_VERIFY_FAILED: Error = Error::new_unchecked(-67678);

    /// An invalid GUID was encountered.
    #[doc(alias = "errSecInvalidGUID")]
    pub const INVALID_GUID: Error = Error::new_unchecked(-67679);

    /// An invalid handle was encountered.
    #[doc(alias = "errSecInvalidHandle")]
    pub const INVALID_HANDLE: Error = Error::new_unchecked(-67680);

    /// An invalid DB list was encountered.
    #[doc(alias = "errSecInvalidDBList")]
    pub const INVALID_DB_LIST: Error = Error::new_unchecked(-67681);

    /// An invalid passthrough ID was encountered.
    #[doc(alias = "errSecInvalidPassthroughID")]
    pub const INVALID_PASSTHROUGH_ID: Error = Error::new_unchecked(-67682);

    /// An invalid network address was encountered.
    #[doc(alias = "errSecInvalidNetworkAddress")]
    pub const INVALID_NETWORK_ADDRESS: Error = Error::new_unchecked(-67683);

    /// The certificate revocation list is already signed.
    #[doc(alias = "errSecCRLAlreadySigned")]
    pub const CRL_ALREADY_SIGNED: Error = Error::new_unchecked(-67684);

    /// An invalid number of fields were encountered.
    #[doc(alias = "errSecInvalidNumberOfFields")]
    pub const INVALID_NUMBER_OF_FIELDS: Error = Error::new_unchecked(-67685);

    /// A verification failure occurred.
    #[doc(alias = "errSecVerificationFailure")]
    pub const VERIFICATION_FAILURE: Error = Error::new_unchecked(-67686);

    /// An unknown tag was encountered.
    #[doc(alias = "errSecUnknownTag")]
    pub const UNKNOWN_TAG: Error = Error::new_unchecked(-67687);

    /// An invalid signature was encountered.
    #[doc(alias = "errSecInvalidSignature")]
    pub const INVALID_SIGNATURE: Error = Error::new_unchecked(-67688);

    /// An invalid name was encountered.
    #[doc(alias = "errSecInvalidName")]
    pub const INVALID_NAME: Error = Error::new_unchecked(-67689);

    /// An invalid certificate reference was encountered.
    #[doc(alias = "errSecInvalidCertificateRef")]
    pub const INVALID_CERT_REF: Error = Error::new_unchecked(-67690);

    /// An invalid certificate group was encountered.
    #[doc(alias = "errSecInvalidCertificateGroup")]
    pub const INVALID_CERT_GROUP: Error = Error::new_unchecked(-67691);

    /// The specified tag was not found.
    #[doc(alias = "errSecTagNotFound")]
    pub const TAG_NOT_FOUND: Error = Error::new_unchecked(-67692);

    /// The specified query was not valid.
    #[doc(alias = "errSecInvalidQuery")]
    pub const INVALID_QUERY: Error = Error::new_unchecked(-67693);

    /// An invalid value was detected.
    #[doc(alias = "errSecInvalidValue")]
    pub const INVALID_VALUE: Error = Error::new_unchecked(-67694);

    /// A callback has failed.
    #[doc(alias = "errSecCallbackFailed")]
    pub const CALLBACK_FAILED: Error = Error::new_unchecked(-67695);

    /// An ACL delete operation has failed.
    #[doc(alias = "errSecACLDeleteFailed")]
    pub const ACL_DELETE_FAILED: Error = Error::new_unchecked(-67696);

    /// An ACL replace operation has failed.
    #[doc(alias = "errSecACLReplaceFailed")]
    pub const ACL_REPLACE_FAILED: Error = Error::new_unchecked(-67697);

    /// An ACL add operation has failed.
    #[doc(alias = "errSecACLAddFailed")]
    pub const ACL_ADD_FAILED: Error = Error::new_unchecked(-67698);

    /// An ACL change operation has failed.
    #[doc(alias = "errSecACLChangeFailed")]
    pub const ACL_CHANGE_FAILED: Error = Error::new_unchecked(-67699);

    #[doc(alias = "errSecInvalidAccessCredentials")]
    pub const INVALID_ACCESS_CREDENTIALS: Error = Error::new_unchecked(-67700);

    /// An invalid record was encountered.
    #[doc(alias = "errSecInvalidRecord")]
    pub const INVALID_RECORD: Error = Error::new_unchecked(-67701);

    /// An invalid ACL was encountered.
    #[doc(alias = "errSecInvalidACL")]
    pub const INVALID_ACL: Error = Error::new_unchecked(-67702);

    /// An invalid sample value was encountered.
    #[doc(alias = "errSecInvalidSampleValue")]
    pub const INVALID_SAMPLE_VALUE: Error = Error::new_unchecked(-67703);

    /// An incompatible version was encountered.
    #[doc(alias = "errSecIncompatibleVersion")]
    pub const INCOMPATIBLE_VERSION: Error = Error::new_unchecked(-67704);

    /// The privilege was not granted.
    #[doc(alias = "errSecPrivilegeNotGranted")]
    pub const PRIVILEGE_NOT_GRANTED: Error = Error::new_unchecked(-67705);

    /// An invalid scope was encountered.
    #[doc(alias = "errSecInvalidScope")]
    pub const INVALID_SCOPE: Error = Error::new_unchecked(-67706);

    /// The PVC is already configured.
    #[doc(alias = "errSecPVCAlreadyConfigured")]
    pub const PVC_ALREADY_CONFIGURED: Error = Error::new_unchecked(-67707);

    /// An invalid PVC was encountered.
    #[doc(alias = "errSecInvalidPVC")]
    pub const INVALID_PVC: Error = Error::new_unchecked(-67708);

    /// The EMM load has failed.
    #[doc(alias = "errSecEMMLoadFailed")]
    pub const EMM_LOAD_FAILED: Error = Error::new_unchecked(-67709);

    /// The EMM unload has failed.
    #[doc(alias = "errSecEMMUnloadFailed")]
    pub const EMM_UNLOAD_FAILED: Error = Error::new_unchecked(-67710);

    /// The add-in load operation has failed.
    #[doc(alias = "errSecAddinLoadFailed")]
    pub const ADDIN_LOAD_FAILED: Error = Error::new_unchecked(-67711);

    /// An invalid key was encountered.
    #[doc(alias = "errSecInvalidKeyRef")]
    pub const INVALID_KEY_REF: Error = Error::new_unchecked(-67712);

    /// An invalid key hierarchy was encountered.
    #[doc(alias = "errSecInvalidKeyHierarchy")]
    pub const INVALID_KEY_HIERARCHY: Error = Error::new_unchecked(-67713);

    /// The add-in unload operation has failed.
    #[doc(alias = "errSecAddinUnloadFailed")]
    pub const ADDIN_UNLOAD_FAILED: Error = Error::new_unchecked(-67714);

    /// A library reference was not found.
    #[doc(alias = "errSecLibraryReferenceNotFound")]
    pub const LIBRARY_REFERENCE_NOT_FOUND: Error = Error::new_unchecked(-67715);

    /// An invalid add-in function table was encountered.
    #[doc(alias = "errSecInvalidAddinFunctionTable")]
    pub const INVALID_ADDIN_FUNCTION_TABLE: Error = Error::new_unchecked(-67716);

    /// An invalid service mask was encountered.
    #[doc(alias = "errSecInvalidServiceMask")]
    pub const INVALID_SERVICE_MASK: Error = Error::new_unchecked(-67717);

    /// A module was not loaded.
    #[doc(alias = "errSecModuleNotLoaded")]
    pub const MODULE_NOT_LOADED: Error = Error::new_unchecked(-67718);

    /// An invalid subservice ID was encountered.
    #[doc(alias = "errSecInvalidSubServiceID")]
    pub const INVALID_SUB_SERVICE_ID: Error = Error::new_unchecked(-67719);

    /// An attribute was not in the context.
    #[doc(alias = "errSecAttributeNotInContext")]
    pub const ATTRIBUTE_NOT_IN_CONTEXT: Error = Error::new_unchecked(-67720);

    /// A module failed to initialize.
    #[doc(alias = "errSecModuleManagerInitializeFailed")]
    pub const MODULE_MANAGER_INITIALIZE_FAILED: Error = Error::new_unchecked(-67721);

    /// A module was not found.
    #[doc(alias = "errSecModuleManagerNotFound")]
    pub const MODULE_MANAGER_NOT_FOUND: Error = Error::new_unchecked(-67722);

    /// An event notification callback was not found.
    #[doc(alias = "errSecEventNotificationCallbackNotFound")]
    pub const EVENT_NOTIFICATION_CALLBACK_NOT_FOUND: Error = Error::new_unchecked(-67723);

    /// An input length error was encountered.
    #[doc(alias = "errSecInputLengthError")]
    pub const INPUT_LENGTH_ERROR: Error = Error::new_unchecked(-67724);

    /// An output length error was encountered.
    #[doc(alias = "errSecOutputLengthError")]
    pub const OUTPUT_LENGTH_ERROR: Error = Error::new_unchecked(-67725);

    /// The privilege is not supported.
    #[doc(alias = "errSecPrivilegeNotSupported")]
    pub const PRIVILEGE_NOT_SUPPORTED: Error = Error::new_unchecked(-67726);

    /// A device error was encountered.
    #[doc(alias = "errSecDeviceError")]
    pub const DEVICE_ERROR: Error = Error::new_unchecked(-67727);

    /// The CSP handle was busy.
    #[doc(alias = "errSecAttachHandleBusy")]
    pub const ATTACH_HANDLE_BUSY: Error = Error::new_unchecked(-67728);

    /// You are not logged in.
    #[doc(alias = "errSecNotLoggedIn")]
    pub const NOT_LOGGED_IN: Error = Error::new_unchecked(-67729);

    /// An algorithm mismatch was encountered.
    #[doc(alias = "errSecAlgorithmMismatch")]
    pub const ALGORITHM_MISMATCH: Error = Error::new_unchecked(-67730);

    /// The key usage is incorrect.
    #[doc(alias = "errSecKeyUsageIncorrect")]
    pub const KEY_USAGE_INCORRECT: Error = Error::new_unchecked(-67731);

    /// The key blob type is incorrect.
    #[doc(alias = "errSecKeyBlobTypeIncorrect")]
    pub const KEY_BLOB_TYPE_INCORRECT: Error = Error::new_unchecked(-67732);

    /// The key header is inconsistent.
    #[doc(alias = "errSecKeyHeaderInconsistent")]
    pub const KEY_HEADER_INCONSISTENT: Error = Error::new_unchecked(-67733);

    /// The key header format is not supported.
    #[doc(alias = "errSecUnsupportedKeyFormat")]
    pub const UNSUPPORTED_KEY_FORMAT: Error = Error::new_unchecked(-67734);

    /// The key size is not supported.
    #[doc(alias = "errSecUnsupportedKeySize")]
    pub const UNSUPPORTED_KEY_SIZE: Error = Error::new_unchecked(-67735);

    /// The key usage mask is not valid.
    #[doc(alias = "errSecInvalidKeyUsageMask")]
    pub const INVALID_KEY_USAGE_MASK: Error = Error::new_unchecked(-67736);

    /// The key usage mask is not supported.
    #[doc(alias = "errSecUnsupportedKeyUsageMask")]
    pub const UNSUPPORTED_KEY_USAGE_MASK: Error = Error::new_unchecked(-67737);

    /// The key attribute mask is not valid.
    #[doc(alias = "errSecInvalidKeyAttributeMask")]
    pub const INVALID_KEY_ATTRIBUTE_MASK: Error = Error::new_unchecked(-67738);

    /// The key attribute mask is not supported.
    #[doc(alias = "errSecUnsupportedKeyAttributeMask")]
    pub const UNSUPPORTED_KEY_ATTRIBUTE_MASK: Error = Error::new_unchecked(-67739);

    /// The key label is not valid.
    #[doc(alias = "errSecInvalidKeyLabel")]
    pub const INVALID_KEY_LABEL: Error = Error::new_unchecked(-67740);

    /// The key label is not supported.
    #[doc(alias = "errSecUnsupportedKeyLabel")]
    pub const ERR_SEC_UNSUPPORTED_KEY_LABEL: Error = Error::new_unchecked(-67741);

    /// The key format is not valid.
    #[doc(alias = "errSecInvalidKeyFormat")]
    pub const INVALID_KEY_FORMAT: Error = Error::new_unchecked(-67742);

    /// The vector of buffers is not supported.
    #[doc(alias = "errSecUnsupportedVectorOfBuffers")]
    pub const UNSUPPORTED_VECTOR_OF_BUFFERS: Error = Error::new_unchecked(-67743);

    /// The input vector is not valid.
    #[doc(alias = "errSecInvalidInputVector")]
    pub const INVALID_INPUT_VECTOR: Error = Error::new_unchecked(-67744);

    /// The output vector is not valid.
    #[doc(alias = "errSecInvalidOutputVector")]
    pub const INVALID_OUTPUT_VECTOR: Error = Error::new_unchecked(-67745);

    /// An invalid context was encountered.
    #[doc(alias = "errSecInvalidContext")]
    pub const INVALID_CONTEXT: Error = Error::new_unchecked(-67746);

    /// An invalid algorithm was encountered.
    #[doc(alias = "errSecInvalidAlgorithm")]
    pub const INVALID_ALGORITHM: Error = Error::new_unchecked(-67747);

    /// A key attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeKey")]
    pub const INVALID_ATTRIBUTE_KEY: Error = Error::new_unchecked(-67748);

    /// A key attribute was missing.
    #[doc(alias = "errSecMissingAttributeKey")]
    pub const MISSING_ATTRIBUTE_KEY: Error = Error::new_unchecked(-67749);

    /// An init vector attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeInitVector")]
    pub const INVALID_ATTRIBUTE_INIT_VECTOR: Error = Error::new_unchecked(-67750);

    /// An init vector attribute was missing.
    #[doc(alias = "errSecMissingAttributeInitVector")]
    pub const MISSING_ATTRIBUTE_INIT_VECTOR: Error = Error::new_unchecked(-67751);

    /// A salt attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeSalt")]
    pub const INVALID_ATTRIBUTE_SALT: Error = Error::new_unchecked(-67752);

    /// A salt attribute was missing.
    #[doc(alias = "errSecMissingAttributeSalt")]
    pub const MISSING_ATTRIBUTE_SALT: Error = Error::new_unchecked(-67753);

    /// A padding attribute was not valid.
    #[doc(alias = "errSecInvalidAttributePadding")]
    pub const INVALID_ATTRIBUTE_PADDING: Error = Error::new_unchecked(-67754);

    /// A padding attribute was missing.
    #[doc(alias = "errSecMissingAttributePadding")]
    pub const MISSING_ATTRIBUTE_PADDING: Error = Error::new_unchecked(-67755);

    /// A random number attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeRandom")]
    pub const INVALID_ATTRIBUTE_RANDOM: Error = Error::new_unchecked(-67756);

    /// A random number attribute was missing.
    #[doc(alias = "errSecMissingAttributeRandom")]
    pub const MISSING_ATTRIBUTE_RANDOM: Error = Error::new_unchecked(-67757);

    /// A seed attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeSeed")]
    pub const INVALID_ATTRIBUTE_SEED: Error = Error::new_unchecked(-67758);

    /// A seed attribute was missing.
    #[doc(alias = "errSecMissingAttributeSeed")]
    pub const MISSING_ATTRIBUTE_SEED: Error = Error::new_unchecked(-67759);

    /// A passphrase attribute was not valid.
    #[doc(alias = "errSecInvalidAttributePassphrase")]
    pub const INVALID_ATTRIBUTE_PASSPHRASE: Error = Error::new_unchecked(-67760);

    /// A passphrase attribute was missing.
    #[doc(alias = "errSecMissingAttributePassphrase")]
    pub const MISSING_ATTRIBUTE_PASSPHRASE: Error = Error::new_unchecked(-67761);

    /// A key length attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeKeyLength")]
    pub const INVALID_ATTRIBUTE_KEY_LENGTH: Error = Error::new_unchecked(-67762);

    /// A key length attribute was missing.
    #[doc(alias = "errSecMissingAttributeKeyLength")]
    pub const MISSING_ATTRIBUTE_KEY_LENGTH: Error = Error::new_unchecked(-67763);

    /// A block size attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeBlockSize")]
    pub const INVALID_ATTRIBUTE_BLOCK_SIZE: Error = Error::new_unchecked(-67764);

    /// A block size attribute was missing.
    #[doc(alias = "errSecMissingAttributeBlockSize")]
    pub const MISSING_ATTRIBUTE_BLOCK_SIZE: Error = Error::new_unchecked(-67765);

    /// An output size attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeOutputSize")]
    pub const INVALID_ATTRIBUTE_OUTPUT_SIZE: Error = Error::new_unchecked(-67766);

    /// An output size attribute was missing.
    #[doc(alias = "errSecMissingAttributeOutputSize")]
    pub const MISSING_ATTRIBUTE_OUTPUT_SIZE: Error = Error::new_unchecked(-67767);

    /// The number of rounds attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeRounds")]
    pub const INVALID_ATTRIBUTE_ROUNDS: Error = Error::new_unchecked(-67768);

    /// The number of rounds attribute was missing.
    #[doc(alias = "errSecMissingAttributeRounds")]
    pub const MISSING_ATTRIBUTE_ROUNDS: Error = Error::new_unchecked(-67769);

    /// An algorithm parameters attribute was not valid.
    #[doc(alias = "errSecInvalidAlgorithmParms")]
    pub const INVALID_ALGORITHM_PARMS: Error = Error::new_unchecked(-67770);

    /// An algorithm parameters attribute was missing.
    #[doc(alias = "errSecMissingAlgorithmParms")]
    pub const MISSING_ALGORITHM_PARMS: Error = Error::new_unchecked(-67771);

    /// A label attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeLabel")]
    pub const INVALID_ATTRIBUTE_LABEL: Error = Error::new_unchecked(-67772);

    /// A label attribute was missing.
    #[doc(alias = "errSecMissingAttributeLabel")]
    pub const MISSING_ATTRIBUTE_LABEL: Error = Error::new_unchecked(-67773);

    /// A key type attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeKeyType")]
    pub const INVALID_ATTRIBUTE_KEY_TYPE: Error = Error::new_unchecked(-67774);

    /// A key type attribute was missing.
    #[doc(alias = "errSecMissingAttributeKeyType")]
    pub const MISSING_ATTRIBUTE_KEY_TYPE: Error = Error::new_unchecked(-67775);

    /// A mode attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeMode")]
    pub const INVALID_ATTRIBUTE_MODE: Error = Error::new_unchecked(-67776);

    /// A mode attribute was missing.
    #[doc(alias = "errSecMissingAttributeMode")]
    pub const MISSING_ATTRIBUTE_MODE: Error = Error::new_unchecked(-67777);

    /// An effective bits attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeEffectiveBits")]
    pub const INVALID_ATTRIBUTE_EFFECTIVE_BITS: Error = Error::new_unchecked(-67778);

    /// An effective bits attribute was missing.
    #[doc(alias = "errSecMissingAttributeEffectiveBits")]
    pub const MISSING_ATTRIBUTE_EFFECTIVE_BITS: Error = Error::new_unchecked(-67779);

    /// A start date attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeStartDate")]
    pub const INVALID_ATTRIBUTE_START_DATE: Error = Error::new_unchecked(-67780);

    /// A start date attribute was missing.
    #[doc(alias = "errSecMissingAttributeStartDate")]
    pub const MISSING_ATTRIBUTE_START_DATE: Error = Error::new_unchecked(-67781);

    /// An end date attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeEndDate")]
    pub const INVALID_ATTRIBUTE_END_DATE: Error = Error::new_unchecked(-67782);

    /// An end date attribute was missing.
    #[doc(alias = "errSecMissingAttributeEndDate")]
    pub const MISSING_ATTRIBUTE_END_DATE: Error = Error::new_unchecked(-67783);

    /// A version attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeVersion")]
    pub const INVALID_ATTRIBUTE_VERSION: Error = Error::new_unchecked(-67784);

    /// A version attribute was missing.
    #[doc(alias = "errSecMissingAttributeVersion")]
    pub const MISSING_ATTRIBUTE_VERSION: Error = Error::new_unchecked(-67785);

    /// A prime attribute was not valid.
    #[doc(alias = "errSecInvalidAttributePrime")]
    pub const INVALID_ATTRIBUTE_PRIME: Error = Error::new_unchecked(-67786);

    /// A prime attribute was missing.
    #[doc(alias = "errSecMissingAttributePrime")]
    pub const MISSING_ATTRIBUTE_PRIME: Error = Error::new_unchecked(-67787);

    /// A base attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeBase")]
    pub const INVALID_ATTRIBUTE_BASE: Error = Error::new_unchecked(-67788);

    /// A base attribute was missing.
    #[doc(alias = "errSecMissingAttributeBase")]
    pub const MISSING_ATTRIBUTE_BASE: Error = Error::new_unchecked(-67789);

    /// A subprime attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeSubprime")]
    pub const INVALID_ATTRIBUTE_SUBPRIME: Error = Error::new_unchecked(-67790);

    /// A subprime attribute was missing.
    #[doc(alias = "errSecMissingAttributeSubprime")]
    pub const MISSING_ATTRIBUTE_SUBPRIME: Error = Error::new_unchecked(-67791);

    /// An iteration count attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeIterationCount")]
    pub const INVALID_ATTRIBUTE_ITERATION_COUNT: Error = Error::new_unchecked(-67792);

    /// An iteration count attribute was missing.
    #[doc(alias = "errSecMissingAttributeIterationCount")]
    pub const MISSING_ATTRIBUTE_ITERATION_COUNT: Error = Error::new_unchecked(-67793);

    /// A database handle attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeDLDBHandle")]
    pub const INVALID_ATTRIBUTE_DLDB_HANDLE: Error = Error::new_unchecked(-67794);

    /// A database handle attribute was missing.
    #[doc(alias = "errSecMissingAttributeDLDBHandle")]
    pub const MISSING_ATTRIBUTE_DLDBHANDLE: Error = Error::new_unchecked(-67795);

    /// An access credentials attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeAccessCredentials")]
    pub const INVALID_ATTRIBUTE_ACCESS_CREDENTIALS: Error = Error::new_unchecked(-67796);

    /// An access credentials attribute was missing.
    #[doc(alias = "errSecMissingAttributeAccessCredentials")]
    pub const MISSING_ATTRIBUTE_ACCESS_CREDENTIALS: Error = Error::new_unchecked(-67797);

    /// A public key format attribute was not valid.
    #[doc(alias = "errSecInvalidAttributePublicKeyFormat")]
    pub const INVALID_ATTRIBUTE_PUBLIC_KEY_FORMAT: Error = Error::new_unchecked(-67798);

    /// A public key format attribute was missing.
    #[doc(alias = "errSecMissingAttributePublicKeyFormat")]
    pub const MISSING_ATTRIBUTE_PUBLIC_KEY_FORMAT: Error = Error::new_unchecked(-67799);

    /// A private key format attribute was not valid.
    #[doc(alias = "errSecInvalidAttributePrivateKeyFormat")]
    pub const INVALID_ATTRIBUTE_PRIVATE_KEY_FORMAT: Error = Error::new_unchecked(-67800);

    /// A private key format attribute was missing.
    #[doc(alias = "errSecMissingAttributePrivateKeyFormat")]
    pub const MISSING_ATTRIBUTE_PRIVATE_KEY_FORMAT: Error = Error::new_unchecked(-67801);

    /// A symmetric key format attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeSymmetricKeyFormat")]
    pub const INVALID_ATTRIBUTE_SYMMETRIC_KEY_FORMAT: Error = Error::new_unchecked(-67802);

    /// A symmetric key format attribute was missing.
    #[doc(alias = "errSecMissingAttributeSymmetricKeyFormat")]
    pub const MISSING_ATTRIBUTE_SYMMETRIC_KEY_FORMAT: Error = Error::new_unchecked(-67803);

    /// A wrapped key format attribute was not valid.
    #[doc(alias = "errSecInvalidAttributeWrappedKeyFormat")]
    pub const INVALID_ATTRIBUTE_WRAPPED_KEY_FORMAT: Error = Error::new_unchecked(-67804);

    /// A wrapped key format attribute was missing.
    #[doc(alias = "errSecMissingAttributeWrappedKeyFormat")]
    pub const MISSING_ATTRIBUTE_WRAPPED_KEY_FORMAT: Error = Error::new_unchecked(-67805);

    /// A staged operation is in progress.
    #[doc(alias = "errSecStagedOperationInProgress")]
    pub const STAGED_OPERATION_IN_PROGRESS: Error = Error::new_unchecked(-67806);

    /// A staged operation was not started.
    #[doc(alias = "errSecStagedOperationNotStarted")]
    pub const STAGED_OPERATION_NOT_STARTED: Error = Error::new_unchecked(-67807);

    /// A cryptographic verification failure has occurred.
    #[doc(alias = "errSecVerifyFailed")]
    pub const VERIFY_FAILED: Error = Error::new_unchecked(-67808);

    /// The query size is unknown.
    #[doc(alias = "errSecQuerySizeUnknown")]
    pub const QUERY_SIZE_UNKNOWN: Error = Error::new_unchecked(-67809);

    /// A block size mismatch occurred.
    #[doc(alias = "errSecBlockSizeMismatch")]
    pub const BLOCK_SIZE_MISMATCH: Error = Error::new_unchecked(-67810);

    /// The public key was inconsistent.
    #[doc(alias = "errSecPublicKeyInconsistent")]
    pub const PUBLIC_KEY_INCONSISTENT: Error = Error::new_unchecked(-67811);

    /// A device verification failure has occurred.
    #[doc(alias = "errSecDeviceVerifyFailed")]
    pub const DEVICE_VERIFY_FAILED: Error = Error::new_unchecked(-67812);

    /// An invalid login name was detected.
    #[doc(alias = "errSecInvalidLoginName")]
    pub const INVALID_LOGIN_NAME: Error = Error::new_unchecked(-67813);

    /// The user is already logged in.
    #[doc(alias = "errSecAlreadyLoggedIn")]
    pub const ALREADY_LOGGED_IN: Error = Error::new_unchecked(-67814);

    /// An invalid digest algorithm was detected.
    #[doc(alias = "errSecInvalidDigestAlgorithm")]
    pub const INVALID_DIGEST_ALGORITHM: Error = Error::new_unchecked(-67815);

    /// An invalid CRL group was detected.
    #[doc(alias = "errSecInvalidCRLGroup")]
    pub const INVALID_CRL_GROUP: Error = Error::new_unchecked(-67816);

    /// The certificate cannot operate.
    #[doc(alias = "errSecCertificateCannotOperate")]
    pub const CERT_CANNOT_OPERATE: Error = Error::new_unchecked(-67817);

    /// An expired certificate was detected.
    #[doc(alias = "errSecCertificateExpired")]
    pub const CERT_EXPIRED: Error = Error::new_unchecked(-67818);

    /// The certificate is not yet valid.
    #[doc(alias = "errSecCertificateNotValidYet")]
    pub const CERT_NOT_VALID_YET: Error = Error::new_unchecked(-67819);

    /// The certificate was revoked.
    #[doc(alias = "errSecCertificateRevoked")]
    pub const CERT_REVOKED: Error = Error::new_unchecked(-67820);

    /// The certificate was suspended.
    #[doc(alias = "errSecCertificateSuspended")]
    pub const CERT_SUSPENDED: Error = Error::new_unchecked(-67821);

    /// Insufficient credentials were detected.
    #[doc(alias = "errSecInsufficientCredentials")]
    pub const INSUFFICIENT_CREDENTIALS: Error = Error::new_unchecked(-67822);

    /// The action was not valid.
    #[doc(alias = "errSecInvalidAction")]
    pub const INVALID_ACTION: Error = Error::new_unchecked(-67823);

    /// The authority was not valid.
    #[doc(alias = "errSecInvalidAuthority")]
    pub const INVALID_AUTHORITY: Error = Error::new_unchecked(-67824);

    /// A verify action has failed.
    #[doc(alias = "errSecVerifyActionFailed")]
    pub const VERIFY_ACTION_FAILED: Error = Error::new_unchecked(-67825);

    /// The certificate authority was not valid.
    #[doc(alias = "errSecInvalidCertAuthority")]
    pub const INVALID_CERT_AUTHORITY: Error = Error::new_unchecked(-67826);

    /// The CRL authority was not valid.
    #[doc(alias = "errSecInvalidCRLAuthority")]
    pub const INVALID_CRL_AUTHORITY: Error = Error::new_unchecked(-67827);

    /// The CRL encoding was not valid.
    #[doc(alias = "errSecInvalidCRLEncoding")]
    pub const INVALID_CRL_ENCODING: Error = Error::new_unchecked(-67828);

    /// The CRL type was not valid.
    #[doc(alias = "errSecInvalidCRLType")]
    pub const INVALID_CRL_TYPE: Error = Error::new_unchecked(-67829);

    /// The CRL was not valid.
    #[doc(alias = "errSecInvalidCRL")]
    pub const INVALID_CRL: Error = Error::new_unchecked(-67830);

    /// The form type was not valid.
    #[doc(alias = "errSecInvalidFormType")]
    pub const INVALID_FORM_TYPE: Error = Error::new_unchecked(-67831);

    /// The ID was not valid.
    #[doc(alias = "errSecInvalidID")]
    pub const INVALID_ID: Error = Error::new_unchecked(-67832);

    /// The identifier was not valid.
    #[doc(alias = "errSecInvalidIdentifier")]
    pub const INVALID_IDENTIFIER: Error = Error::new_unchecked(-67833);

    /// The index was not valid.
    #[doc(alias = "errSecInvalidIndex")]
    pub const INVALID_INDEX: Error = Error::new_unchecked(-67834);

    /// The policy identifiers are not valid.
    #[doc(alias = "errSecInvalidPolicyIdentifiers")]
    pub const INVALID_POLICY_IDENTIFIERS: Error = Error::new_unchecked(-67835);

    /// The time specified was not valid.
    #[doc(alias = "errSecInvalidTimeString")]
    pub const INVALID_TIME_STRING: Error = Error::new_unchecked(-67836);

    /// The trust policy reason was not valid.
    #[doc(alias = "errSecInvalidReason")]
    pub const INVALID_REASON: Error = Error::new_unchecked(-67837);

    /// The request inputs are not valid.
    #[doc(alias = "errSecInvalidRequestInputs")]
    pub const INVALID_REQUEST_INPUTS: Error = Error::new_unchecked(-67838);

    /// The response vector was not valid.
    #[doc(alias = "errSecInvalidResponseVector")]
    pub const INVALID_RESPONSE_VECTOR: Error = Error::new_unchecked(-67839);

    /// The stop-on policy was not valid.
    #[doc(alias = "errSecInvalidStopOnPolicy")]
    pub const INVALID_STOP_ON_POLICY: Error = Error::new_unchecked(-67840);

    /// The tuple was not valid.
    #[doc(alias = "errSecInvalidTuple")]
    pub const INVALID_TUPLE: Error = Error::new_unchecked(-67841);

    /// Multiple values are not supported.
    #[doc(alias = "errSecMultipleValuesUnsupported")]
    pub const MULTIPLE_VALUES_UNSUPPORTED: Error = Error::new_unchecked(-67842);

    /// The certificate was not trusted.
    #[doc(alias = "errSecNotTrusted")]
    pub const NOT_TRUSTED: Error = Error::new_unchecked(-67843);

    /// No default authority was detected.
    #[doc(alias = "errSecNoDefaultAuthority")]
    pub const NO_DEFAULT_AUTHORITY: Error = Error::new_unchecked(-67844);

    /// The trust policy had a rejected form.
    #[doc(alias = "errSecRejectedForm")]
    pub const REJECTED_FORM: Error = Error::new_unchecked(-67845);

    /// The request was lost.
    #[doc(alias = "errSecRequestLost")]
    pub const REQUEST_LOST: Error = Error::new_unchecked(-67846);

    /// The request was rejected.
    #[doc(alias = "errSecRequestRejected")]
    pub const REQUEST_REJECTED: Error = Error::new_unchecked(-67847);

    /// The address type is not supported.
    #[doc(alias = "errSecUnsupportedAddressType")]
    pub const UNSUPPORTED_ADDRESS_TYPE: Error = Error::new_unchecked(-67848);

    /// The service is not supported.
    #[doc(alias = "errSecUnsupportedService")]
    pub const UNSUPPORTED_SERVICE: Error = Error::new_unchecked(-67849);

    /// The tuple group was not valid.
    #[doc(alias = "errSecInvalidTupleGroup")]
    pub const INVALID_TUPLE_GROUP: Error = Error::new_unchecked(-67850);

    /// The base ACLs are not valid.
    #[doc(alias = "errSecInvalidBaseACLs")]
    pub const INVALID_BASE_ACLS: Error = Error::new_unchecked(-67851);

    /// The tuple credentials are not valid.
    #[doc(alias = "errSecInvalidTupleCredentials")]
    pub const INVALID_TUPLE_CREDENTIALS: Error = Error::new_unchecked(-67852);

    /// The encoding was not valid.
    #[doc(alias = "errSecInvalidEncoding")]
    pub const INVALID_ENCODING: Error = Error::new_unchecked(-67853);

    /// The validity period was not valid.
    #[doc(alias = "errSecInvalidValidityPeriod")]
    pub const INVALID_VALIDITY_PERIOD: Error = Error::new_unchecked(-67854);

    /// The requestor was not valid.
    #[doc(alias = "errSecInvalidRequestor")]
    pub const INVALID_REQUESTOR: Error = Error::new_unchecked(-67855);

    /// The request descriptor was not valid.
    #[doc(alias = "errSecRequestDescriptor")]
    pub const REQUEST_DESCRIPTOR: Error = Error::new_unchecked(-67856);

    /// The bundle information was not valid.
    #[doc(alias = "errSecInvalidBundleInfo")]
    pub const INVALID_BUNDLE_INFO: Error = Error::new_unchecked(-67857);

    /// The CRL index was not valid.
    #[doc(alias = "errSecInvalidCRLIndex")]
    pub const INVALID_CRL_INDEX: Error = Error::new_unchecked(-67858);

    /// No field values were detected.
    #[doc(alias = "errSecNoFieldValues")]
    pub const NO_FIELD_VALUES: Error = Error::new_unchecked(-67859);

    /// The field format is not supported.
    #[doc(alias = "errSecUnsupportedFieldFormat")]
    pub const UNSUPPORTED_FIELD_FORMAT: Error = Error::new_unchecked(-67860);

    /// The index information is not supported.
    #[doc(alias = "errSecUnsupportedIndexInfo")]
    pub const UNSUPPORTED_INDEX_INFO: Error = Error::new_unchecked(-67861);

    /// The locality is not supported.
    #[doc(alias = "errSecUnsupportedLocality")]
    pub const UNSUPPORTED_LOCALITY: Error = Error::new_unchecked(-67862);

    /// The number of attributes is not supported.
    #[doc(alias = "errSecUnsupportedNumAttributes")]
    pub const UNSUPPORTED_NUM_ATTRIBUTES: Error = Error::new_unchecked(-67863);

    /// The number of indexes is not supported.
    #[doc(alias = "errSecUnsupportedNumIndexes")]
    pub const UNSUPPORTED_NUM_INDEXES: Error = Error::new_unchecked(-67864);

    /// The number of record types is not supported.
    #[doc(alias = "errSecUnsupportedNumRecordTypes")]
    pub const UNSUPPORTED_NUM_RECORD_TYPES: Error = Error::new_unchecked(-67865);

    /// Too many fields were specified.
    #[doc(alias = "errSecFieldSpecifiedMultiple")]
    pub const FIELD_SPECIFIED_MULTIPLE: Error = Error::new_unchecked(-67866);

    /// The field format was incompatible.
    #[doc(alias = "errSecIncompatibleFieldFormat")]
    pub const INCOMPATIBLE_FIELD_FORMAT: Error = Error::new_unchecked(-67867);

    /// The parsing module was not valid.
    #[doc(alias = "errSecInvalidParsingModule")]
    pub const INVALID_PARSING_MODULE: Error = Error::new_unchecked(-67868);

    /// The database is locked.
    #[doc(alias = "errSecDatabaseLocked")]
    pub const DATABASE_LOCKED: Error = Error::new_unchecked(-67869);

    /// The data store is open.
    #[doc(alias = "errSecDatastoreIsOpen")]
    pub const DATASTORE_IS_OPEN: Error = Error::new_unchecked(-67870);

    /// A missing value was detected.
    #[doc(alias = "errSecMissingValue")]
    pub const MISSING_VALUE: Error = Error::new_unchecked(-67871);

    /// The query limits are not supported.
    #[doc(alias = "errSecUnsupportedQueryLimits")]
    pub const UNSUPPORTED_QUERY_LIMITS: Error = Error::new_unchecked(-67872);

    /// The number of selection predicates is not supported.
    #[doc(alias = "errSecUnsupportedNumSelectionPreds")]
    pub const UNSUPPORTED_NUM_SELECTION_PREDS: Error = Error::new_unchecked(-67873);

    /// The operator is not supported.
    #[doc(alias = "errSecUnsupportedOperator")]
    pub const UNSUPPORTED_OPERATOR: Error = Error::new_unchecked(-67874);

    /// The database location is not valid.
    #[doc(alias = "errSecInvalidDBLocation")]
    pub const INVALID_DB_LOCATION: Error = Error::new_unchecked(-67875);

    /// The access request is not valid.
    #[doc(alias = "errSecInvalidAccessRequest")]
    pub const INVALID_ACCESS_REQUEST: Error = Error::new_unchecked(-67876);

    /// The index information is not valid.
    #[doc(alias = "errSecInvalidIndexInfo")]
    pub const INVALID_INDEX_INFO: Error = Error::new_unchecked(-67877);

    /// The new owner is not valid.
    #[doc(alias = "errSecInvalidNewOwner")]
    pub const INVALID_NEW_OWNER: Error = Error::new_unchecked(-67878);

    /// The modify mode is not valid.
    #[doc(alias = "errSecInvalidModifyMode")]
    pub const INVALID_MODIFY_MODE: Error = Error::new_unchecked(-67879);

    /// A required certificate extension is missing.
    #[doc(alias = "errSecMissingRequiredExtension")]
    pub const MISSING_REQUIRED_EXTENSION: Error = Error::new_unchecked(-67880);

    /// The extended key usage extension was not marked critical.
    #[doc(alias = "errSecExtendedKeyUsageNotCritical")]
    pub const EXTENDED_KEY_USAGE_NOT_CRITICAL: Error = Error::new_unchecked(-67881);

    /// A timestamp was expected but was not found.
    #[doc(alias = "errSecTimestampMissing")]
    pub const TIMESTAMP_MISSING: Error = Error::new_unchecked(-67882);

    /// The timestamp was not valid.
    #[doc(alias = "errSecTimestampInvalid")]
    pub const TIMESTAMP_INVALID: Error = Error::new_unchecked(-67883);

    /// The timestamp was not trusted.
    #[doc(alias = "errSecTimestampNotTrusted")]
    pub const TIMESTAMP_NOT_TRUSTED: Error = Error::new_unchecked(-67884);

    /// The timestamp service is not available.
    #[doc(alias = "errSecTimestampServiceNotAvailable")]
    pub const TIMESTAMP_SERVICE_NOT_AVAILABLE: Error = Error::new_unchecked(-67885);

    /// An unrecognized or unsupported Algorithm Identifier in timestamp.
    #[doc(alias = "errSecTimestampBadAlg")]
    pub const TIMESTAMP_BAD_ALG: Error = Error::new_unchecked(-67886);

    /// The timestamp transaction is not permitted or supported.
    #[doc(alias = "errSecTimestampBadRequest")]
    pub const TIMESTAMP_BAD_REQUEST: Error = Error::new_unchecked(-67887);

    /// The timestamp data submitted has the wrong format.
    #[doc(alias = "errSecTimestampBadDataFormat")]
    pub const TIMESTAMP_BAD_DATA_FORMAT: Error = Error::new_unchecked(-67888);

    /// The time source for the Timestamp Authority is not available.
    #[doc(alias = "errSecTimestampTimeNotAvailable")]
    pub const TIMESTAMP_TIME_NOT_AVAILABLE: Error = Error::new_unchecked(-67889);

    /// The requested policy is not supported by the Timestamp Authority.
    #[doc(alias = "errSecTimestampUnacceptedPolicy")]
    pub const TIMESTAMP_UNACCEPTED_POLICY: Error = Error::new_unchecked(-67890);

    /// The requested extension is not supported by the Timestamp Authority.
    #[doc(alias = "errSecTimestampUnacceptedExtension")]
    pub const TIMESTAMP_UNACCEPTED_EXTENSION: Error = Error::new_unchecked(-67891);

    /// The additional information requested is not available.
    #[doc(alias = "errSecTimestampAddInfoNotAvailable")]
    pub const TIMESTAMP_ADD_INFO_NOT_AVAILABLE: Error = Error::new_unchecked(-67892);

    /// The timestamp request cannot be handled due to system failure.
    #[doc(alias = "errSecTimestampSystemFailure")]
    pub const TIMESTAMP_SYSTEM_FAILURE: Error = Error::new_unchecked(-67893);

    /// A signing time was expected but was not found.
    #[doc(alias = "errSecSigningTimeMissing")]
    pub const SIGNING_TIME_MISSING: Error = Error::new_unchecked(-67894);

    /// A timestamp transaction was rejected.
    #[doc(alias = "errSecTimestampRejection")]
    pub const TIMESTAMP_REJECTION: Error = Error::new_unchecked(-67895);

    /// A timestamp transaction is waiting.
    #[doc(alias = "errSecTimestampWaiting")]
    pub const TIMESTAMP_WAITING: Error = Error::new_unchecked(-67896);

    /// A timestamp authority revocation warning was issued.
    #[doc(alias = "errSecTimestampRevocationWarning")]
    pub const TIMESTAMP_REVOCATION_WARNING: Error = Error::new_unchecked(-67897);

    /// A timestamp authority revocation notification was issued.
    #[doc(alias = "errSecTimestampRevocationNotification")]
    pub const TIMESTAMP_REVOCATION_NOTIFICATION: Error = Error::new_unchecked(-67898);

    /// The requested policy is not allowed for this certificate.
    #[doc(alias = "errSecCertificatePolicyNotAllowed")]
    pub const CERT_POLICY_NOT_ALLOWED: Error = Error::new_unchecked(-67899);

    /// The requested name is not allowed for this certificate.
    #[doc(alias = "errSecCertificateNameNotAllowed")]
    pub const CERT_NAME_NOT_ALLOWED: Error = Error::new_unchecked(-67900);

    /// The validity period in the certificate exceeds the maximum allowed.
    #[doc(alias = "errSecCertificateValidityPeriodTooLong")]
    pub const CERT_VALIDITY_PERIOD_TOO_LONG: Error = Error::new_unchecked(-67901);

    /// The verified certificate is a CA rather than an end-entity
    #[doc(alias = "errSecCertificateIsCA")]
    pub const CERT_IS_CA: Error = Error::new_unchecked(-67902);

    /// The certificate contains multiple extensions with the same extension ID.
    #[doc(alias = "errSecCertificateDuplicateExtension")]
    pub const CERT_DUPLICATE_EXTENSION: Error = Error::new_unchecked(-67903);
}

pub mod ssl_err {
    use crate::os::Error;

    /// SSL protocol error
    #[doc(alias = "errSSLProtocol")]
    pub const PROTOCOL: Error = Error::new_unchecked(-9800);

    /// Cipher Suite negotiation failure
    #[doc(alias = "errSSLNegotiation")]
    pub const NEGOTIATION: Error = Error::new_unchecked(-9801);

    /// Fatal alert
    #[doc(alias = "errSSLFatalAlert")]
    pub const FATAL_ALERT: Error = Error::new_unchecked(-9802);

    /// I/O would block (not fatal)
    #[doc(alias = "errSSLWouldBlock")]
    pub const WOULD_BLOCK: Error = Error::new_unchecked(-9803);

    /// Attempt to restore an unknown session
    #[doc(alias = "errSSLSessionNotFound")]
    pub const SESSION_NOT_FOUND: Error = Error::new_unchecked(-9804);

    /// Connection closed gracefully
    #[doc(alias = "errSSLClosedGraceful")]
    pub const CLOSED_GRACEFUL: Error = Error::new_unchecked(-9805);

    /// Connection closed via error
    #[doc(alias = "errSSLClosedAbort")]
    pub const CLOSED_ABORT: Error = Error::new_unchecked(-9806);

    /// Invalid certificate chain
    #[doc(alias = "errSSLXCertChainInvalid")]
    pub const CERT_CHAIN_INVALID: Error = Error::new_unchecked(-9807);

    /// Bad certificate format
    #[doc(alias = "errSSLBadCert")]
    pub const BAD_CERT: Error = Error::new_unchecked(-9808);

    /// Underlying cryptographic error
    #[doc(alias = "errSSLCrypto")]
    pub const CRYPTO: Error = Error::new_unchecked(-9809);

    /// Internal error
    #[doc(alias = "errSSLInternal")]
    pub const INTERNAL: Error = Error::new_unchecked(-9810);

    /// Module attach failure
    #[doc(alias = "errSSLModuleAttach")]
    pub const MODULE_ATTACH: Error = Error::new_unchecked(-9811);

    /// Valid cert chain, untrusted root
    #[doc(alias = "errSSLUnknownRootCert")]
    pub const UNKNOWN_ROOT_CERT: Error = Error::new_unchecked(-9812);

    /// Cert chain not verified by root
    #[doc(alias = "errSSLNoRootCert")]
    pub const NO_ROOT_CERT: Error = Error::new_unchecked(-9813);

    /// Chain had an expired cert
    #[doc(alias = "errSSLCertExpired")]
    pub const CERT_EXPIRED: Error = Error::new_unchecked(-9814);

    /// Chain had a cert not yet valid
    #[doc(alias = "errSSLCertNotYetValid")]
    pub const CERT_NOT_YET_VALID: Error = Error::new_unchecked(-9815);

    /// Server closed session with no notification
    #[doc(alias = "errSSLClosedNoNotify")]
    pub const CLOSED_NO_NOTIFY: Error = Error::new_unchecked(-9816);

    /// Insufficient buffer provided
    #[doc(alias = "errSSLBufferOverflow")]
    pub const BUF_OVERFLOW: Error = Error::new_unchecked(-9817);

    /// bad SSLCipherSuite
    #[doc(alias = "errSSLBadCipherSuite")]
    pub const BAD_CIPHER_SUITE: Error = Error::new_unchecked(-9818);

    /* fatal errors detected by peer */

    /// Unexpected message received
    #[doc(alias = "errSSLPeerUnexpectedMsg")]
    pub const PEER_UNEXPECTED_MSG: Error = Error::new_unchecked(-9819);

    /// Bad MAC
    #[doc(alias = "errSSLPeerBadRecordMac")]
    pub const PEER_BAD_RECORD_MAC: Error = Error::new_unchecked(-9820);

    /// Decryption failed
    #[doc(alias = "errSSLPeerDecryptionFail")]
    pub const PEER_DECRYPTION_FAIL: Error = Error::new_unchecked(-9821);

    /// Record overflow
    #[doc(alias = "errSSLPeerRecordOverflow")]
    pub const PEER_RECORD_OVERFLOW: Error = Error::new_unchecked(-9822);

    /// Decompression failure
    #[doc(alias = "errSSLPeerDecompressFail")]
    pub const PEER_DECOMPRESS_FAIL: Error = Error::new_unchecked(-9823);

    /// Handshake failure
    #[doc(alias = "errSSLPeerHandshakeFail")]
    pub const PEER_HANDSHAKE_FAIL: Error = Error::new_unchecked(-9824);

    /// Misc. bad certificate
    #[doc(alias = "errSSLPeerBadCert")]
    pub const PEER_BAD_CERT: Error = Error::new_unchecked(-9825);

    /// Bad unsupported cert format
    #[doc(alias = "errSSLPeerUnsupportedCert")]
    pub const PEER_UNSUPPORTED_CERT: Error = Error::new_unchecked(-9826);

    /// Certificate revoked
    #[doc(alias = "errSSLPeerCertRevoked")]
    pub const PEER_CERT_REVOKED: Error = Error::new_unchecked(-9827);

    /// Certificate expired
    #[doc(alias = "errSSLPeerCertExpired")]
    pub const PEER_CERT_EXPIRED: Error = Error::new_unchecked(-9828);

    /// Unknown certificate
    #[doc(alias = "errSSLPeerCertUnknown")]
    pub const PEER_CERT_UNKNOWN: Error = Error::new_unchecked(-9829);

    /// Illegal parameter
    #[doc(alias = "errSSLIllegalParam")]
    pub const ILLEGAL_PARAM: Error = Error::new_unchecked(-9830);

    /// Unknown Cert Authority
    #[doc(alias = "errSSLPeerUnknownCA")]
    pub const PEER_UNKNOWN_CA: Error = Error::new_unchecked(-9831);

    /// Access denied
    #[doc(alias = "errSSLPeerAccessDenied")]
    pub const PEER_ACCESS_DENIED: Error = Error::new_unchecked(-9832);

    /// Decoding error
    #[doc(alias = "errSSLPeerDecodeError")]
    pub const PEER_DECODE_ERROR: Error = Error::new_unchecked(-9833);

    /// Decryption error
    #[doc(alias = "errSSLPeerDecryptError")]
    pub const PEER_DECRYPT_ERROR: Error = Error::new_unchecked(-9834);

    /// Export restriction
    #[doc(alias = "errSSLPeerExportRestriction")]
    pub const PEER_EXPORT_RESTRICTION: Error = Error::new_unchecked(-9835);

    /// Bad protocol version
    #[doc(alias = "errSSLPeerProtocolVersion")]
    pub const PEER_PROTOCOL_VERSION: Error = Error::new_unchecked(-9836);

    /// Insufficient security
    #[doc(alias = "errSSLPeerInsufficientSecurity")]
    pub const PEER_INSUFFICIENT_SECURITY: Error = Error::new_unchecked(-9837);

    /// Internal error
    #[doc(alias = "errSSLPeerInternalError")]
    pub const PEER_INTERNAL_ERROR: Error = Error::new_unchecked(-9838);

    /// User canceled
    #[doc(alias = "errSSLPeerUserCancelled")]
    pub const PEER_USER_CANCELLED: Error = Error::new_unchecked(-9839);

    /// No renegotiation allowed
    #[doc(alias = "errSSLPeerNoRenegotiation")]
    pub const PEER_NO_RENEGOTIATION: Error = Error::new_unchecked(-9840);

    /* non-fatal result codes */

    /// Peer cert is valid, or was ignored if verification disabled
    #[doc(alias = "errSSLPeerAuthCompleted")]
    pub const PEER_AUTH_COMPLETED: Error = Error::new_unchecked(-9841);

    /// Server has requested a client cert
    #[doc(alias = "errSSLClientCertRequested")]
    pub const CLIENT_CERT_REQUESTED: Error = Error::new_unchecked(-9842);

    /* more errors detected by us */

    /// Peer host name mismatch
    #[doc(alias = "errSSLHostNameMismatch")]
    pub const HOST_NAME_MISMATCH: Error = Error::new_unchecked(-9843);

    /// Peer dropped connection before responding
    #[doc(alias = "errSSLConnectionRefused")]
    pub const CONNECTION_REFUSED: Error = Error::new_unchecked(-9844);

    /// Decryption failure
    #[doc(alias = "errSSLDecryptionFail")]
    pub const DECRYPTION_FAIL: Error = Error::new_unchecked(-9845);

    /// Bad MAC
    #[doc(alias = "errSSLBadRecordMac")]
    pub const BAD_RECORD_MAC: Error = Error::new_unchecked(-9846);

    /// Record overflow
    #[doc(alias = "errSSLRecordOverflow")]
    pub const RECORD_OVERFLOW: Error = Error::new_unchecked(-9847);

    /// Configuration error
    #[doc(alias = "errSSLBadConfiguration")]
    pub const BAD_CONFIGURATION: Error = Error::new_unchecked(-9848);

    /// Unexpected (skipped) record in DTLS
    #[doc(alias = "errSSLUnexpectedRecord")]
    pub const UNEXPECTED_RECORD: Error = Error::new_unchecked(-9849);

    /// Weak ephemeral dh key  
    #[doc(alias = "errSSLWeakPeerEphemeralDHKey")]
    pub const WEAK_PEER_EPHEMERAL_DH_KEY: Error = Error::new_unchecked(-9850);

    /* non-fatal result codes */

    /// SNI
    #[doc(alias = "errSSLClientHelloReceived")]
    pub const CLIENT_HELLO_RECEIVED: Error = Error::new_unchecked(-9851);

    /* fatal errors resulting from transport or networking errors */

    /// Transport (socket) shutdown, e.g., TCP RST or FIN.
    #[doc(alias = "errSSLTransportReset")]
    pub const TRANSPORT_RESET: Error = Error::new_unchecked(-9852);

    /// Network timeout triggered
    #[doc(alias = "errSSLNetworkTimeout")]
    pub const NETWORK_TIMEOUT: Error = Error::new_unchecked(-9853);

    /* fatal errors resulting from software misconfiguration */

    /// TLS configuration failed
    #[doc(alias = "errSSLConfigurationFailed")]
    pub const CONFIGURATION_FAILED: Error = Error::new_unchecked(-9854);

    /* additional errors */

    /// Unsupported TLS extension
    #[doc(alias = "errSSLUnsupportedExtension")]
    pub const UNSUPPORTED_EXTENSION: Error = Error::new_unchecked(-9855);

    /// Peer rejected unexpected message
    #[doc(alias = "errSSLUnexpectedMessage")]
    pub const UNEXPECTED_MESSAGE: Error = Error::new_unchecked(-9856);

    /// Decompression failed
    #[doc(alias = "errSSLDecompressFail")]
    pub const DECOMPRESS_FAIL: Error = Error::new_unchecked(-9857);

    /// Handshake failed
    #[doc(alias = "errSSLHandshakeFail")]
    pub const HANDSHAKE_FAIL: Error = Error::new_unchecked(-9858);

    /// Decode failed
    #[doc(alias = "errSSLDecodeError")]
    pub const DECODE_ERROR: Error = Error::new_unchecked(-9859);

    /// Inappropriate fallback
    #[doc(alias = "errSSLInappropriateFallback")]
    pub const INAPPROPRIATE_FALLBACK: Error = Error::new_unchecked(-9860);

    /// Missing extension
    #[doc(alias = "errSSLMissingExtension")]
    pub const MISSING_EXTENSION: Error = Error::new_unchecked(-9861);

    /// Bad OCSP response
    #[doc(alias = "errSSLBadCertificateStatusResponse")]
    pub const BAD_CERT_STATUS_RESPONSE: Error = Error::new_unchecked(-9862);

    /// Certificate required
    #[doc(alias = "errSSLCertificateRequired")]
    pub const CERT_REQUIRED: Error = Error::new_unchecked(-9863);

    /// Unknown PSK identity
    #[doc(alias = "errSSLUnknownPSKIdentity")]
    pub const UNKNOWN_PSK_IDENTITY: Error = Error::new_unchecked(-9864);

    /// Unknown or unrecognized name
    #[doc(alias = "errSSLUnrecognizedName")]
    pub const UNRECOGNIZED_NAME: Error = Error::new_unchecked(-9865);

    /* ATS compliance violation errors */

    /// ATS violation
    #[doc(alias = "errSSLATSViolation")]
    pub const ATS_VIOLATION: Error = Error::new_unchecked(-9880);

    /// ATS violation: minimum protocol version is not ATS compliant
    #[doc(alias = "errSSLATSMinimumVersionViolation")]
    pub const ATS_MIN_VERSION_VIOLATION: Error = Error::new_unchecked(-9881);

    /// ATS violation: selected ciphersuite is not ATS compliant
    #[doc(alias = "errSSLATSCiphersuiteViolation")]
    pub const ATS_CIPHERSUITE_VIOLATION: Error = Error::new_unchecked(-9882);

    /// ATS violation: peer key size is not ATS compliant
    #[doc(alias = "errSSLATSMinimumKeySizeViolation")]
    pub const ATS_MIN_KEY_SIZE_VIOLATION: Error = Error::new_unchecked(-9883);

    /// ATS violation: peer leaf certificate hash algorithm is not ATS compliant
    #[doc(alias = "errSSLATSLeafCertificateHashAlgorithmViolation")]
    pub const ATS_LEAF_CERT_HASH_ALGORITHM_VIOLATION: Error = Error::new_unchecked(-9884);

    /// ATS violation: peer certificate hash algorithm is not ATS compliant
    #[doc(alias = "errSSLATSCertificateHashAlgorithmViolation")]
    pub const ATS_CERT_HASH_ALGORITHM_VIOLATION: Error = Error::new_unchecked(-9885);

    /// ATS violation: peer certificate is not issued by trusted peer
    #[doc(alias = "errSSLATSCertificateTrustViolation")]
    pub const ATS_CERT_TRUST_VIOLATION: Error = Error::new_unchecked(-9886);

    /* early data errors */

    /// Early application data rejected by peer
    #[doc(alias = "errSSLEarlyDataRejected")]
    pub const EARLY_DATA_REJECTED: Error = Error::new_unchecked(-9890);
}

#[link(name = "Security", kind = "framework")]
extern "C-unwind" {
    fn SecCopyErrorMessageString(
        status: os::Status,
        reserved: *mut std::ffi::c_void,
    ) -> Option<arc::R<cf::String>>;
}

#[cfg(test)]
mod tests {
    use crate::{os, sec};
    #[test]
    fn basics() {
        assert!(sec::err_msg_string(os::Status(0)).is_some());
        assert!(sec::err_msg_string(sec::err::NOT_SIGNER.status()).is_some());
    }
}
