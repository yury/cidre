use crate::{arc, define_obj_type, ns, objc};

#[doc(alias = "NSTokenStyle")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum TokenStyle {
    /// Rounded tokens, the style of the field's choice.
    Default = 0,
    /// Plain text, no token look.
    None = 1,
    Rounded = 2,
    Squared = 3,
    PlainSquared = 4,
}

define_obj_type!(
    /// A text field whose text is tokens, made as a separator (a comma) is typed.
    #[doc(alias = "NSTokenField")]
    pub TokenField(ns::TextField),
    sym NSTokenField
);

impl TokenField {
    #[objc::msg_send(tokenStyle)]
    pub fn token_style(&self) -> TokenStyle;

    #[objc::msg_send(setTokenStyle:)]
    pub fn set_token_style(&mut self, val: TokenStyle);

    /// Seconds from a key typed to the completions shown.
    #[objc::msg_send(completionDelay)]
    pub fn completion_delay(&self) -> ns::TimeInterval;

    #[objc::msg_send(setCompletionDelay:)]
    pub fn set_completion_delay(&mut self, val: ns::TimeInterval);

    #[objc::msg_send(setDelegate:)]
    pub fn set_token_delegate<D: TokenFieldDelegate>(&mut self, val: Option<&D>);
}

#[objc::protocol(NSTokenFieldDelegate)]
pub trait TokenFieldDelegate: ns::TextFieldDelegate {
    /// What the text being typed in token `token_index` completes to; `selected_index` the
    /// one picked first (-1: none).
    #[objc::optional]
    #[objc::msg_send(tokenField:completionsForSubstring:indexOfToken:indexOfSelectedItem:)]
    fn token_field_completions_for_substring(
        &mut self,
        token_field: &mut TokenField,
        substring: &ns::String,
        token_index: ns::Integer,
        selected_index: *mut ns::Integer,
    ) -> Option<arc::R<ns::Array<ns::String>>>;

    /// The tokens to add at `index` of those typed or pasted (strings, unless the delegate
    /// represents them otherwise).
    #[objc::optional]
    #[objc::msg_send(tokenField:shouldAddObjects:atIndex:)]
    fn token_field_should_add_objs_at(
        &mut self,
        token_field: &mut TokenField,
        tokens: &ns::Array<ns::Id>,
        index: ns::UInteger,
    ) -> arc::R<ns::Array<ns::Id>>;
}

define_obj_type!(
    pub AnyTokenFieldDelegate(ns::Id)
);

impl ns::TextFieldDelegate for AnyTokenFieldDelegate {}
impl TokenFieldDelegate for AnyTokenFieldDelegate {}
