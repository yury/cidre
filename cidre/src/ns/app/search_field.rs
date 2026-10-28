use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    /// A text field for searching: a magnifying glass before the text, a button that clears
    /// it after.
    #[doc(alias = "NSSearchField")]
    pub SearchField(ns::TextField),
    sym NSSearchField
);

impl SearchField {
    /// Whether the search is sent only when Return is pressed (`false` by default: as typed).
    #[objc::msg_send(sendsWholeSearchString)]
    pub fn sends_whole_search_string(&self) -> bool;

    #[objc::msg_send(setSendsWholeSearchString:)]
    pub fn set_sends_whole_search_string(&mut self, val: bool);

    /// Whether each change is sent at once rather than after a pause.
    #[objc::msg_send(sendsSearchStringImmediately)]
    pub fn sends_search_string_immediately(&self) -> bool;

    #[objc::msg_send(setSendsSearchStringImmediately:)]
    pub fn set_sends_search_string_immediately(&mut self, val: bool);
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut field = ns::SearchField::new();
        field.set_sends_search_string_immediately(true);
        assert!(field.sends_search_string_immediately());
    }
}
