use crate::swift::{self, abi, value::Storage};

unsafe extern "C" {

    #[link_name = "$s10Foundation16AttributedStringV13CharacterViewVMa"]
    fn character_view_metadata();

    #[link_name = "$s10Foundation16AttributedStringV10charactersAC13CharacterViewVvg"]
    fn attr_string_characters();

    #[link_name = "$sSS10FoundationE11_charactersSSAA16AttributedStringV13CharacterViewV_tcfC"]
    fn string_from_characters();
}

crate::define_swift_marker!(CharacterViewValue = accessor character_view_metadata);

crate::define_swift!(
    #[swift::struct("Foundation.AttributedString", size(8), align(8), sendable)]
    pub AttrString
);

impl AttrString {
    /// The text without its attributes, via `String(_characters:)`.
    #[doc(alias = "AttributedString.characters")]
    pub fn to_swift_string(&self) -> swift::String {
        unsafe {
            let mut characters = Storage::<CharacterViewValue>::new();
            abi::call::value_to_value(
                attr_string_characters as *const (),
                self.as_ptr(),
                characters.as_mut_ptr(),
            );

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

    /// The text without its attributes, as a Rust string.
    pub fn to_string(&self) -> std::string::String {
        self.to_swift_string().to_string()
    }
}

#[allow(clippy::inherent_to_string_shadow_display)]
impl std::fmt::Display for AttrString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl std::fmt::Debug for AttrString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AttrString({:?})", self.to_string())
    }
}
