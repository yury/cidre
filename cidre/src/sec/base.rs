use crate::{arc, cf, define_cf_type, os};

define_cf_type!(
    #[doc(alias = "SecCertificateRef")]
    Certificate(cf::Type)
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
    pub const NO_CERTIFICATE_MODULE: Error = Error::new_unchecked(-25313);

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
    pub const INVALID_CERTIFICATE_REF: Error = Error::new_unchecked(-67690);

    /// An invalid certificate group was encountered.
    #[doc(alias = "errSecInvalidCertificateGroup")]
    pub const INVALID_CERTIFICATE_GROUP: Error = Error::new_unchecked(-67691);

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
