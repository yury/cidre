use crate::{arc, cf, sec};

impl sec::Key {
    #[doc(alias = "SecKeyCreateWithData")]
    #[inline]
    pub fn with_data(
        data: &cf::Data,
        attrs: &cf::Dictionary,
    ) -> Result<arc::R<Self>, arc::R<cf::Error>> {
        cf::if_none(|err| unsafe { SecKeyCreateWithData(data, attrs, err) })
    }
}

unsafe extern "C-unwind" {
    fn SecKeyCreateWithData(
        key_data: &cf::Data,
        attrs: &cf::Dictionary,
        error: *mut arc::R<cf::Error>,
    ) -> Option<arc::R<sec::Key>>;
}

pub mod attrs {
    use crate::cf;

    #[doc(alias = "kSecAttrKeyClass")]
    #[inline]
    pub const fn key_class() -> &'static cf::String {
        unsafe { kSecAttrKeyClass }
    }

    #[doc(alias = "kSecAttrApplicationLabel")]
    #[inline]
    pub const fn application_label() -> &'static cf::String {
        unsafe { kSecAttrApplicationLabel }
    }

    #[doc(alias = "kSecAttrIsPermanent")]
    #[inline]
    pub const fn is_permanent() -> &'static cf::String {
        unsafe { kSecAttrIsPermanent }
    }

    #[doc(alias = "kSecAttrIsSensitive")]
    #[inline]
    pub const fn is_sensitive() -> &'static cf::String {
        unsafe { kSecAttrIsSensitive }
    }

    #[doc(alias = "kSecAttrIsExtractable")]
    #[inline]
    pub const fn is_extractable() -> &'static cf::String {
        unsafe { kSecAttrIsExtractable }
    }

    #[doc(alias = "kSecAttrApplicationTag")]
    #[inline]
    pub const fn application_tag() -> &'static cf::String {
        unsafe { kSecAttrApplicationTag }
    }

    #[doc(alias = "kSecAttrKeyType")]
    #[inline]
    pub const fn key_type() -> &'static cf::String {
        unsafe { kSecAttrKeyType }
    }

    #[doc(alias = "kSecAttrPRF")]
    #[inline]
    pub const fn prf() -> &'static cf::String {
        unsafe { kSecAttrPRF }
    }

    #[doc(alias = "kSecAttrSalt")]
    #[inline]
    pub const fn salt() -> &'static cf::String {
        unsafe { kSecAttrSalt }
    }

    #[doc(alias = "kSecAttrRounds")]
    #[inline]
    pub const fn rounds() -> &'static cf::String {
        unsafe { kSecAttrRounds }
    }

    #[doc(alias = "kSecAttrKeySizeInBits")]
    #[inline]
    pub const fn key_size_in_bits() -> &'static cf::String {
        unsafe { kSecAttrKeySizeInBits }
    }

    #[doc(alias = "kSecAttrEffectiveKeySize")]
    #[inline]
    pub const fn effective_key_size() -> &'static cf::String {
        unsafe { kSecAttrEffectiveKeySize }
    }

    #[doc(alias = "kSecAttrCanEncrypt")]
    #[inline]
    pub const fn can_encrypt() -> &'static cf::String {
        unsafe { kSecAttrCanEncrypt }
    }

    #[doc(alias = "kSecAttrCanDecrypt")]
    #[inline]
    pub const fn can_decrypt() -> &'static cf::String {
        unsafe { kSecAttrCanDecrypt }
    }

    #[doc(alias = "kSecAttrCanDerive")]
    #[inline]
    pub const fn can_derive() -> &'static cf::String {
        unsafe { kSecAttrCanDerive }
    }

    #[doc(alias = "kSecAttrCanSign")]
    #[inline]
    pub const fn can_sign() -> &'static cf::String {
        unsafe { kSecAttrCanSign }
    }

    #[doc(alias = "kSecAttrCanVerify")]
    #[inline]
    pub const fn can_verify() -> &'static cf::String {
        unsafe { kSecAttrCanVerify }
    }

    #[doc(alias = "kSecAttrCanWrap")]
    #[inline]
    pub const fn can_wrap() -> &'static cf::String {
        unsafe { kSecAttrCanWrap }
    }

    #[doc(alias = "kSecAttrCanUnwrap")]
    #[inline]
    pub const fn can_unwrap() -> &'static cf::String {
        unsafe { kSecAttrCanUnwrap }
    }

    #[doc(alias = "kSecAttrSyncViewHint")]
    #[inline]
    pub const fn sync_view_hint() -> &'static cf::String {
        unsafe { kSecAttrSyncViewHint }
    }

    #[doc(alias = "kSecAttrTokenID")]
    #[inline]
    pub const fn token_id() -> &'static cf::String {
        unsafe { kSecAttrTokenID }
    }

    unsafe extern "C" {
        static kSecAttrKeyClass: &'static cf::String;
        static kSecAttrApplicationLabel: &'static cf::String;
        static kSecAttrIsPermanent: &'static cf::String;
        static kSecAttrIsSensitive: &'static cf::String;
        static kSecAttrIsExtractable: &'static cf::String;
        static kSecAttrApplicationTag: &'static cf::String;
        static kSecAttrKeyType: &'static cf::String;
        static kSecAttrPRF: &'static cf::String;
        static kSecAttrSalt: &'static cf::String;
        static kSecAttrRounds: &'static cf::String;
        static kSecAttrKeySizeInBits: &'static cf::String;
        static kSecAttrEffectiveKeySize: &'static cf::String;
        static kSecAttrCanEncrypt: &'static cf::String;
        static kSecAttrCanDecrypt: &'static cf::String;
        static kSecAttrCanDerive: &'static cf::String;
        static kSecAttrCanSign: &'static cf::String;
        static kSecAttrCanVerify: &'static cf::String;
        static kSecAttrCanWrap: &'static cf::String;
        static kSecAttrCanUnwrap: &'static cf::String;
        static kSecAttrSyncViewHint: &'static cf::String;
        static kSecAttrTokenID: &'static cf::String;
    }
}

pub mod key_types {
    use crate::cf;

    #[doc(alias = "kSecAttrKeyTypeRSA")]
    #[inline]
    pub const fn rsa() -> &'static cf::String {
        unsafe { kSecAttrKeyTypeRSA }
    }

    #[doc(alias = "kSecAttrKeyTypeDSA")]
    #[inline]
    pub const fn dsa() -> &'static cf::String {
        unsafe { kSecAttrKeyTypeDSA }
    }

    #[doc(alias = "kSecAttrKeyTypeAES")]
    #[inline]
    pub const fn aes() -> &'static cf::String {
        unsafe { kSecAttrKeyTypeAES }
    }

    #[doc(alias = "kSecAttrKeyTypeDES")]
    #[inline]
    pub const fn des() -> &'static cf::String {
        unsafe { kSecAttrKeyTypeDES }
    }

    #[doc(alias = "kSecAttrKeyType3DES")]
    #[inline]
    pub const fn triple_des() -> &'static cf::String {
        unsafe { kSecAttrKeyType3DES }
    }

    #[doc(alias = "kSecAttrKeyTypeRC4")]
    #[inline]
    pub const fn rc4() -> &'static cf::String {
        unsafe { kSecAttrKeyTypeRC4 }
    }

    #[doc(alias = "kSecAttrKeyTypeRC2")]
    #[inline]
    pub const fn rc2() -> &'static cf::String {
        unsafe { kSecAttrKeyTypeRC2 }
    }

    #[doc(alias = "kSecAttrKeyTypeCAST")]
    #[inline]
    pub const fn cast() -> &'static cf::String {
        unsafe { kSecAttrKeyTypeCAST }
    }

    #[doc(alias = "kSecAttrKeyTypeECDSA")]
    #[inline]
    pub const fn ecdsa() -> &'static cf::String {
        unsafe { kSecAttrKeyTypeECDSA }
    }

    #[doc(alias = "kSecAttrKeyTypeEC")]
    #[inline]
    pub const fn ec() -> &'static cf::String {
        unsafe { kSecAttrKeyTypeEC }
    }

    #[doc(alias = "kSecAttrKeyTypeECSECPrimeRandom")]
    #[inline]
    pub const fn ec_sec_prime_random() -> &'static cf::String {
        unsafe { kSecAttrKeyTypeECSECPrimeRandom }
    }

    unsafe extern "C" {
        static kSecAttrKeyTypeRSA: &'static cf::String;
        static kSecAttrKeyTypeDSA: &'static cf::String;
        static kSecAttrKeyTypeAES: &'static cf::String;
        static kSecAttrKeyTypeDES: &'static cf::String;
        static kSecAttrKeyType3DES: &'static cf::String;
        static kSecAttrKeyTypeRC4: &'static cf::String;
        static kSecAttrKeyTypeRC2: &'static cf::String;
        static kSecAttrKeyTypeCAST: &'static cf::String;
        static kSecAttrKeyTypeECDSA: &'static cf::String;
        static kSecAttrKeyTypeEC: &'static cf::String;
        static kSecAttrKeyTypeECSECPrimeRandom: &'static cf::String;
    }
}

pub mod key_classes {
    use crate::cf;

    #[doc(alias = "kSecAttrKeyClassPublic")]
    #[inline]
    pub const fn public() -> &'static cf::String {
        unsafe { kSecAttrKeyClassPublic }
    }

    #[doc(alias = "kSecAttrKeyClassPrivate")]
    #[inline]
    pub const fn private() -> &'static cf::String {
        unsafe { kSecAttrKeyClassPrivate }
    }

    #[doc(alias = "kSecAttrKeyClassSymmetric")]
    #[inline]
    pub const fn symmetric() -> &'static cf::String {
        unsafe { kSecAttrKeyClassSymmetric }
    }

    unsafe extern "C" {
        static kSecAttrKeyClassPublic: &'static cf::String;
        static kSecAttrKeyClassPrivate: &'static cf::String;
        static kSecAttrKeyClassSymmetric: &'static cf::String;
    }
}
