use crate::{arc, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    #[doc(alias = "NSAppearanceName")]
    pub AppearanceName(ns::String)
);

impl AppearanceName {
    #[objc::available(macos = 10.9)]
    pub fn aqua() -> &'static Self {
        unsafe { NSAppearanceNameAqua }
    }

    #[objc::available(macos = 10.14)]
    pub fn dark_aqua() -> &'static Self {
        unsafe { NSAppearanceNameDarkAqua }
    }
}

unsafe extern "C" {
    static NSAppearanceNameAqua: &'static AppearanceName;
    static NSAppearanceNameDarkAqua: &'static AppearanceName;
}

define_obj_type!(
    #[doc(alias = "NSAppearance")]
    pub Appearance(ns::Id),
    sym NSAppearance
);

impl Appearance {
    #[objc::msg_send(name)]
    #[objc::available(macos = 10.9)]
    pub fn name(&self) -> arc::R<AppearanceName>;

    #[objc::msg_send(appearanceNamed:)]
    #[objc::available(macos = 10.9)]
    pub fn named(name: &AppearanceName) -> Option<arc::R<Self>>;

    /// The ambient drawing appearance, which may differ from a view's effective appearance.
    #[objc::msg_send(currentDrawingAppearance)]
    #[objc::available(macos = 11.0)]
    pub fn current_drawing() -> arc::R<Self>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(performAsCurrentDrawingAppearance:)]
    #[objc::available(macos = 11.0)]
    pub fn perform_as_current_drawing_appearance_block(&self, block: &mut blocks::NoEscBlock<fn()>);

    /// Resolves dynamic colors and assets using this appearance, restoring the previous one
    /// after the synchronous callback returns.
    #[cfg(feature = "blocks")]
    #[objc::available(macos = 11.0)]
    #[allow(unused_unsafe)]
    pub fn perform_as_current_drawing_appearance(&self, mut f: impl FnMut()) {
        let mut block = unsafe { blocks::NoEscBlock::stack0(&mut f) };
        unsafe { self.perform_as_current_drawing_appearance_block(&mut block) };
    }
}

// NSAppearanceCustomization, adopted by NSView.
impl ns::View {
    #[objc::msg_send(appearance)]
    #[objc::available(macos = 10.9)]
    pub fn appearance(&self) -> Option<arc::R<Appearance>>;

    #[objc::msg_send(setAppearance:)]
    #[objc::available(macos = 10.9)]
    pub fn set_appearance(&mut self, val: Option<&Appearance>);

    #[objc::msg_send(effectiveAppearance)]
    #[objc::available(macos = 10.9)]
    pub fn effective_appearance(&self) -> arc::R<Appearance>;
}
