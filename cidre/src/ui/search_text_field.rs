use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    /// The text field of a search bar, usable on its own: text after tokens, and a menu of
    /// suggestions under it.
    #[doc(alias = "UISearchTextField")]
    pub SearchTextField(ui::TextField),
    sym UISearchTextField
);

impl SearchTextField {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: crate::cg::Rect) -> arc::R<SearchTextField>;

    pub fn with_frame(frame: crate::cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(tokens)]
    pub fn tokens(&self) -> arc::R<ns::Array<SearchToken>>;

    #[objc::msg_send(setTokens:)]
    pub fn set_tokens(&mut self, val: &ns::Array<SearchToken>);

    #[objc::msg_send(insertToken:atIndex:)]
    pub fn insert_token_at(&mut self, token: &SearchToken, index: ns::Integer);

    #[objc::msg_send(removeTokenAtIndex:)]
    pub fn remove_token_at(&mut self, index: ns::Integer);

    #[objc::msg_send(tokenBackgroundColor)]
    pub fn token_bg_color(&self) -> arc::R<ui::Color>;

    /// `None` for the default color.
    #[objc::msg_send(setTokenBackgroundColor:)]
    pub fn set_token_bg_color(&mut self, val: Option<&ui::Color>);

    #[objc::msg_send(allowsDeletingTokens)]
    pub fn allows_deleting_tokens(&self) -> bool;

    #[objc::msg_send(setAllowsDeletingTokens:)]
    pub fn set_allows_deleting_tokens(&mut self, val: bool);

    #[objc::msg_send(allowsCopyingTokens)]
    pub fn allows_copying_tokens(&self) -> bool;

    #[objc::msg_send(setAllowsCopyingTokens:)]
    pub fn set_allows_copying_tokens(&mut self, val: bool);

    #[objc::msg_send(searchSuggestions)]
    #[objc::available(ios = 16.0)]
    pub fn search_suggestions(&self) -> Option<arc::R<ns::Array<SearchSuggestionItem>>>;

    /// Shown as a menu under the field while not empty; `None` or empty dismisses it. The
    /// field sets it to `None` once one is picked.
    #[objc::msg_send(setSearchSuggestions:)]
    #[objc::available(ios = 16.0)]
    pub fn set_search_suggestions(&mut self, val: Option<&ns::Array<SearchSuggestionItem>>);
}

define_obj_type!(
    /// A token of a [`SearchTextField`]: an icon and a text, before the field's text.
    #[doc(alias = "UISearchToken")]
    pub SearchToken(ns::Id),
    sym UISearchToken
);

impl SearchToken {
    #[objc::msg_send(tokenWithIcon:text:)]
    pub fn with_icon_text(icon: Option<&ui::Image>, text: &ns::String) -> arc::R<Self>;

    #[objc::msg_send(representedObject)]
    pub fn represented_obj(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setRepresentedObject:)]
    pub fn set_represented_obj(&mut self, val: Option<&ns::Id>);
}

define_obj_type!(
    #[doc(alias = "UISearchSuggestionItem")]
    pub SearchSuggestionItem(ns::Id),
    sym UISearchSuggestionItem
);

impl SearchSuggestionItem {
    #[objc::init(initWithLocalizedSuggestion:localizedDescription:iconImage:)]
    pub fn init_with_suggestion_desc_icon(
        self,
        suggestion: &ns::String,
        desc: Option<&ns::String>,
        icon: Option<&ui::Image>,
    ) -> arc::R<SearchSuggestionItem>;

    pub fn with_suggestion_desc_icon(
        suggestion: &ns::String,
        desc: Option<&ns::String>,
        icon: Option<&ui::Image>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_suggestion_desc_icon(suggestion, desc, icon)
    }

    #[objc::msg_send(localizedSuggestion)]
    pub fn localized_suggestion(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(localizedDescription)]
    pub fn localized_desc(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(iconImage)]
    pub fn icon_image(&self) -> Option<arc::R<ui::Image>>;

    #[objc::msg_send(representedObject)]
    pub fn represented_obj(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setRepresentedObject:)]
    pub fn set_represented_obj(&mut self, val: Option<&ns::Id>);
}

#[objc::protocol(UISearchTextFieldDelegate)]
pub trait SearchTextFieldDelegate: ui::TextFieldDelegate {
    /// One of the field's suggestions was picked; the field has set them to `None`.
    #[objc::optional]
    #[objc::msg_send(searchTextField:didSelectSuggestion:)]
    fn search_text_field_did_select_suggestion(
        &mut self,
        search_text_field: &mut SearchTextField,
        suggestion: &SearchSuggestionItem,
    );
}

define_obj_type!(
    pub AnySearchTextFieldDelegate(ns::Id)
);

impl ui::TextFieldDelegate for AnySearchTextFieldDelegate {}
impl SearchTextFieldDelegate for AnySearchTextFieldDelegate {}

impl SearchTextField {
    #[objc::msg_send(setDelegate:)]
    pub fn set_search_delegate<D: SearchTextFieldDelegate>(&mut self, val: Option<&D>);
}
