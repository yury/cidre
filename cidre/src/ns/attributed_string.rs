use crate::{arc, cf, define_obj_type, define_opts, ns, objc};

define_obj_type!(
    #[doc(alias = "NSAttributedStringKey")]
    pub AttrStringKey(ns::String)
);

#[cfg(any(feature = "app", feature = "ui"))]
impl AttrStringKey {
    #[doc(alias = "NSFontAttributeName")]
    #[inline]
    pub fn font_name() -> &'static Self {
        unsafe { NSFontAttributeName }
    }

    #[doc(alias = "NSParagraphStyleAttributeName")]
    #[inline]
    pub fn paragraph_style() -> &'static Self {
        unsafe { NSParagraphStyleAttributeName }
    }

    #[doc(alias = "NSForegroundColorAttributeName")]
    #[inline]
    pub fn foreground_color() -> &'static Self {
        unsafe { NSForegroundColorAttributeName }
    }

    #[doc(alias = "NSBackgroundColorAttributeName")]
    #[inline]
    pub fn background_color() -> &'static Self {
        unsafe { NSBackgroundColorAttributeName }
    }

    #[doc(alias = "NSLigatureAttributeName")]
    #[inline]
    pub fn ligature() -> &'static Self {
        unsafe { NSLigatureAttributeName }
    }

    #[doc(alias = "NSKernAttributeName")]
    #[inline]
    pub fn kern() -> &'static Self {
        unsafe { NSKernAttributeName }
    }

    #[doc(alias = "NSTrackingAttributeName")]
    #[inline]
    pub fn tracking() -> &'static Self {
        unsafe { NSTrackingAttributeName }
    }

    #[doc(alias = "NSStrikethroughStyleAttributeName")]
    #[inline]
    pub fn strikethrough_style() -> &'static Self {
        unsafe { NSStrikethroughStyleAttributeName }
    }

    #[doc(alias = "NSUnderlineStyleAttributeName")]
    #[inline]
    pub fn underline_style() -> &'static Self {
        unsafe { NSUnderlineStyleAttributeName }
    }

    #[doc(alias = "NSStrokeColorAttributeName")]
    #[inline]
    pub fn stroke_color() -> &'static Self {
        unsafe { NSStrokeColorAttributeName }
    }

    #[doc(alias = "NSStrokeWidthAttributeName")]
    #[inline]
    pub fn stroke_width() -> &'static Self {
        unsafe { NSStrokeWidthAttributeName }
    }

    #[doc(alias = "NSShadowAttributeName")]
    #[inline]
    pub fn shadow() -> &'static Self {
        unsafe { NSShadowAttributeName }
    }

    #[doc(alias = "NSTextEffectAttributeName")]
    #[inline]
    pub fn text_effect() -> &'static Self {
        unsafe { NSTextEffectAttributeName }
    }

    #[doc(alias = "NSAttachmentAttributeName")]
    #[inline]
    pub fn attachment() -> &'static Self {
        unsafe { NSAttachmentAttributeName }
    }

    #[doc(alias = "NSLinkAttributeName")]
    #[inline]
    pub fn link() -> &'static Self {
        unsafe { NSLinkAttributeName }
    }

    #[doc(alias = "NSBaselineOffsetAttributeName")]
    #[inline]
    pub fn baseline_offset() -> &'static Self {
        unsafe { NSBaselineOffsetAttributeName }
    }

    #[doc(alias = "NSUnderlineColorAttributeName")]
    #[inline]
    pub fn underline_color() -> &'static Self {
        unsafe { NSUnderlineColorAttributeName }
    }

    #[doc(alias = "NSStrikethroughColorAttributeName")]
    #[inline]
    pub fn strikethrough_color() -> &'static Self {
        unsafe { NSStrikethroughColorAttributeName }
    }

    #[doc(alias = "NSWritingDirectionAttributeName")]
    #[inline]
    pub fn writing_direction() -> &'static Self {
        unsafe { NSWritingDirectionAttributeName }
    }
}

#[cfg(all(feature = "app", target_os = "macos"))]
#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" {
    static NSFontAttributeName: &'static AttrStringKey;
    static NSParagraphStyleAttributeName: &'static AttrStringKey;
    static NSForegroundColorAttributeName: &'static AttrStringKey;
    static NSBackgroundColorAttributeName: &'static AttrStringKey;
    static NSLigatureAttributeName: &'static AttrStringKey;
    static NSKernAttributeName: &'static AttrStringKey;
    static NSTrackingAttributeName: &'static AttrStringKey;
    static NSStrikethroughStyleAttributeName: &'static AttrStringKey;
    static NSUnderlineStyleAttributeName: &'static AttrStringKey;
    static NSStrokeColorAttributeName: &'static AttrStringKey;
    static NSStrokeWidthAttributeName: &'static AttrStringKey;
    static NSShadowAttributeName: &'static AttrStringKey;
    static NSTextEffectAttributeName: &'static AttrStringKey;
    static NSAttachmentAttributeName: &'static AttrStringKey;
    static NSLinkAttributeName: &'static AttrStringKey;
    static NSBaselineOffsetAttributeName: &'static AttrStringKey;
    static NSUnderlineColorAttributeName: &'static AttrStringKey;
    static NSStrikethroughColorAttributeName: &'static AttrStringKey;
    static NSWritingDirectionAttributeName: &'static AttrStringKey;
}

#[cfg(any(target_os = "ios", target_os = "tvos", target_os = "watchos"))]
#[link(name = "UIKit", kind = "framework")]
unsafe extern "C" {
    static NSFontAttributeName: &'static AttrStringKey;
    static NSParagraphStyleAttributeName: &'static AttrStringKey;
    static NSForegroundColorAttributeName: &'static AttrStringKey;
    static NSBackgroundColorAttributeName: &'static AttrStringKey;
    static NSLigatureAttributeName: &'static AttrStringKey;
    static NSKernAttributeName: &'static AttrStringKey;
    static NSTrackingAttributeName: &'static AttrStringKey;
    static NSStrikethroughStyleAttributeName: &'static AttrStringKey;
    static NSUnderlineStyleAttributeName: &'static AttrStringKey;
    static NSStrokeColorAttributeName: &'static AttrStringKey;
    static NSStrokeWidthAttributeName: &'static AttrStringKey;
    static NSShadowAttributeName: &'static AttrStringKey;
    static NSTextEffectAttributeName: &'static AttrStringKey;
    static NSAttachmentAttributeName: &'static AttrStringKey;
    static NSLinkAttributeName: &'static AttrStringKey;
    static NSBaselineOffsetAttributeName: &'static AttrStringKey;
    static NSUnderlineColorAttributeName: &'static AttrStringKey;
    static NSStrikethroughColorAttributeName: &'static AttrStringKey;
    static NSWritingDirectionAttributeName: &'static AttrStringKey;
}

define_obj_type!(
    #[doc(alias = "NSAttributedStringFormattingContextKey")]
    pub AttrStringFormatCtxKey(ns::String)
);

define_obj_type!(
    #[doc(alias = "NSAttributedString")]
    pub AttrString(ns::Id),
    NS_ATTRIBUTED_STRING
);

impl arc::A<AttrString> {
    #[objc::msg_send(initWithString:)]
    pub fn init_with_string(self, str: &ns::String) -> arc::R<AttrString>;

    #[objc::msg_send(initWithAttributedString:)]
    pub fn init_with_attr_string(self, str: &ns::AttrString) -> arc::R<AttrString>;
}

impl AttrString {
    /// For performance reasons, this property returns the current backing store of
    /// the attributed string object. If you want to maintain a snapshot of this as
    /// you manipulate the returned string, you should make a copy of the appropriate substring.
    #[objc::msg_send(string)]
    pub fn string(&self) -> arc::R<ns::String>;

    #[objc::msg_send(length)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[objc::msg_send(attributesAtIndex:effectiveRange:)]
    pub unsafe fn attrs_at_throws(
        &self,
        index: usize,
        effective_range: Option<&ns::Range>,
    ) -> arc::R<ns::Dictionary<AttrStringKey, ns::Id>>;

    pub fn attrs_at<'ear>(
        &self,
        index: usize,
        effective_range: Option<&ns::Range>,
    ) -> ns::ExResult<'ear, arc::R<ns::Dictionary<AttrStringKey, ns::Id>>> {
        ns::try_catch(|| unsafe { self.attrs_at_throws(index, effective_range) })
    }

    #[inline]
    pub fn with_str(str: &str) -> arc::R<Self> {
        let str = ns::String::with_str(str);
        Self::alloc().init_with_string(&str)
    }

    #[inline]
    pub fn with_string(str: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_string(str)
    }

    #[inline]
    pub fn with_attr_string(str: &ns::AttrString) -> arc::R<Self> {
        Self::alloc().init_with_attr_string(str)
    }

    #[inline]
    pub fn as_cf(&self) -> &cf::AttrString {
        unsafe { std::mem::transmute(self) }
    }

    #[objc::msg_send(copy)]
    pub fn copy(&self) -> arc::R<AttrString>;

    #[objc::msg_send(mutableCopy)]
    pub fn copy_mut(&self) -> arc::R<AttrStringMut>;
}

define_obj_type!(
    #[doc(alias = "NSMutableAttributedString")]
    pub AttrStringMut(AttrString),
    NS_MUTABLE_ATTRIBUTED_STRING
);

impl arc::A<AttrStringMut> {
    #[objc::msg_send(initWithString:)]
    pub fn init_with_string(self, str: &ns::String) -> arc::R<AttrStringMut>;

    #[objc::msg_send(initWithAttributedString:)]
    pub fn init_with_attr_string(self, str: &ns::AttrString) -> arc::R<AttrStringMut>;
}

impl AttrStringMut {
    #[inline]
    pub fn with_str(str: &str) -> arc::R<Self> {
        let str = ns::String::with_str(str);
        Self::alloc().init_with_string(&str)
    }

    #[inline]
    pub fn with_string(str: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_string(str)
    }

    #[inline]
    pub fn with_attr_string(str: &ns::AttrString) -> arc::R<Self> {
        Self::alloc().init_with_attr_string(str)
    }

    #[objc::msg_send(mutableString)]
    pub fn string_mut(&mut self) -> &mut ns::StringMut;

    #[objc::msg_send(addAttribute:value:range:)]
    pub unsafe fn add_attr_throws(
        &mut self,
        name: &ns::AttrStringKey,
        val: &ns::Id,
        range: ns::Range,
    );

    pub fn add_attr<'ear>(
        &mut self,
        name: &ns::AttrStringKey,
        val: &ns::Id,
        range: ns::Range,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.add_attr_throws(name, val, range) })
    }

    #[objc::msg_send(addAttributes:range:)]
    pub unsafe fn add_attrs_throws(
        &mut self,
        attrs: &ns::Dictionary<ns::AttrStringKey, ns::Id>,
        range: ns::Range,
    );

    pub fn add_attrs<'ear>(
        &mut self,
        attrs: &ns::Dictionary<ns::AttrStringKey, ns::Id>,
        range: ns::Range,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.add_attrs_throws(attrs, range) })
    }

    #[objc::msg_send(removeAttribute:range:)]
    pub unsafe fn remove_attr_throws(&mut self, name: &ns::AttrStringKey, range: ns::Range);

    pub fn remove_attr<'ear>(
        &mut self,
        name: &ns::AttrStringKey,
        range: ns::Range,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.remove_attr_throws(name, range) })
    }

    #[objc::msg_send(insertAttributedString:atIndex:)]
    pub fn insert_attr_string(&mut self, val: &ns::AttrString, at: usize);

    #[objc::msg_send(appendAttributedString:)]
    pub fn append_attr_string(&mut self, val: &ns::AttrString);

    #[objc::msg_send(replaceCharactersInRange:withAttributedString:)]
    pub unsafe fn replace_with_attr_string_throws(
        &mut self,
        range: ns::Range,
        attr_str: &ns::AttrString,
    );

    pub fn replace_with_attr_string<'ear>(
        &mut self,
        range: ns::Range,
        attr_str: &ns::AttrString,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.replace_with_attr_string_throws(range, attr_str) })
    }
}

define_opts!(
    #[doc(alias = "NSInlinePresentationIntent")]
    pub InlinePresentationIntent(usize)
);

impl InlinePresentationIntent {
    pub const EMPHASIZED: Self = Self(1 << 0);
    pub const STRONGLY_EMPHASIZED: Self = Self(1 << 1);
    pub const CODE: Self = Self(1 << 2);
    pub const STRIKETHROUGH: Self = Self(1 << 5);
    pub const SOFT_BREAK: Self = Self(1 << 6);
    pub const LINE_BREAK: Self = Self(1 << 7);
    pub const INLINE_HTML: Self = Self(1 << 8);
    pub const BLOCK_HTML: Self = Self(1 << 9);
}

unsafe extern "C" {
    static NS_ATTRIBUTED_STRING: &'static objc::Class<AttrString>;
    static NS_MUTABLE_ATTRIBUTED_STRING: &'static objc::Class<AttrStringMut>;
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use crate::{ns, objc::Obj};

    #[test]
    fn basics() {
        let str = ns::String::with_str("Hello");
        let astr = ns::AttrString::with_string(&str);

        assert_eq!(&astr.string(), &str);
        let attrs = astr.attrs_at(0, None).unwrap();
        assert!(attrs.is_empty());

        let copy = astr.copy_mut();
        let copy = copy.copy();
        let copy = ns::AttrString::with_attr_string(&copy);

        assert_eq!(copy.len(), 5);

        let mut mcopy = copy.copy_mut();

        // let mut mstr = mcopy.string_mut();

        let color_key = ns::AttrStringKey::foreground_color();
        let color_val = ns::Color::with_rgba(0.5, 0.5, 1.0, 1.0);

        let attrs = ns::Dictionary::with_keys_values(&[color_key], &[color_val.as_id_ref()]);

        mcopy.add_attrs(&attrs, ns::Range::new(0, 5)).expect("ok");
        mcopy
            .add_attr(color_key, &color_val, ns::Range::new(1000, 10))
            .expect_err("ok");
        mcopy
            .add_attr(color_key, &color_val, ns::Range::new(0, 3))
            .expect("ok");

        mcopy
            .remove_attr(color_key, ns::Range::new(100, 200))
            .expect_err("should be out of bounds");

        mcopy
            .remove_attr(color_key, ns::Range::new(0, 5))
            .expect("failed to remove valid range");

        let mstr = ns::AttrStringMut::with_str("nice");
        assert_eq!(mstr.len(), 4);
        // TODO: investigate
        // astr.attrs_at(1000, Some(&ns::Range::new(1000, 1000)))
        // .expect_err("Should be Exception");
    }
}
