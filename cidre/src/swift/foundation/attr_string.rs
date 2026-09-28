use crate::swift::{self, abi, value::Storage};

unsafe extern "C" {
    /// `String(_characters:)`, declared in Foundation's extension of `String`,
    /// which the mangler does not spell.
    #[link_name = "$sSS10FoundationE11_charactersSSAA16AttributedStringV13CharacterViewV_tcfC"]
    fn string_from_characters();
}

crate::define_swift!(
    #[swift::struct("Foundation.AttributedString(struct).CharacterView")]
    CharacterViewValue
);

crate::define_swift!(
    #[swift::struct("Foundation.AttributedString", size(8), align(8), sendable)]
    pub AttrString
);

impl AttrString {
    /// The text without its attributes, via `String(_characters:)`.
    #[doc(alias = "AttributedString.characters")]
    pub fn to_swift_string(&self) -> swift::String {
        let characters = self.characters();
        unsafe {
            swift::String::from_raw(swift::value::call_with_owned_value(
                characters,
                |characters| {
                    abi::call::value_to_string(
                        string_from_characters as *const (),
                        characters.cast_const(),
                    )
                },
            ))
        }
    }
}

impl AttrString {
    #[swift::call(
        "Foundation.AttributedString(struct).characters: \
         Foundation.AttributedString(struct).CharacterView(struct) { get }"
    )]
    fn characters(&self) -> Storage<CharacterViewValue>;
}

impl std::fmt::Display for AttrString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.to_swift_string(), f)
    }
}

impl std::fmt::Debug for AttrString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AttrString")
            .field(&self.to_swift_string())
            .finish()
    }
}
