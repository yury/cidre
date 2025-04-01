use crate::{arc, ax, cf, define_cf_type, os, sys::Pid};

pub fn is_process_trusted() -> bool {
    is_process_trusted_with_opts(None)
}

pub fn is_process_trusted_with_prompt(prompt: &cf::String) -> bool {
    let opts = cf::DictionaryOf::with_keys_values(
        &[trusted_check_option_prompt()],
        &[prompt.as_type_ref()],
    );
    is_process_trusted_with_opts(Some(&opts))
}

#[doc(alias = "AXIsProcessTrustedWithOptions")]
pub fn is_process_trusted_with_opts(
    options: Option<&cf::DictionaryOf<cf::String, cf::Type>>,
) -> bool {
    unsafe { AXIsProcessTrustedWithOptions(options) }
}

pub fn trusted_check_option_prompt() -> &'static cf::String {
    unsafe { kAXTrustedCheckOptionPrompt }
}

define_cf_type!(
    #[doc(alias = "AXUIElementRef")]
    UiElement(cf::Type)
);

impl UiElement {
    #[doc(alias = "AXUIElementGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { AXUIElementGetTypeID() }
    }

    #[doc(alias = "AXUIElementCreateApplication")]
    #[inline]
    pub fn with_app_pid(pid: Pid) -> arc::R<Self> {
        unsafe { AXUIElementCreateApplication(pid) }
    }

    #[doc(alias = "AXUIElementCreateSystemWide")]
    #[inline]
    pub fn sys_wide() -> arc::R<Self> {
        unsafe { AXUIElementCreateSystemWide() }
    }

    #[doc(alias = "AXUIElementGetPid")]
    #[inline]
    pub fn pid(&self) -> os::Result<Pid> {
        os::result_init(|res| unsafe { AXUIElementGetPid(self, res) })
    }

    #[doc(alias = "AXUIElementCopyAttributeNames")]
    #[inline]
    pub fn attrs(&self) -> os::Result<arc::R<cf::ArrayOf<ax::Attr>>> {
        unsafe { os::result_unchecked(|res| AXUIElementCopyAttributeNames(self, res)) }
    }

    #[doc(alias = "AXUIElementCopyParameterizedAttributeNames")]
    #[inline]
    pub fn param_attrs(&self) -> os::Result<arc::R<cf::ArrayOf<ax::ParamAttr>>> {
        unsafe { os::result_unchecked(|res| AXUIElementCopyParameterizedAttributeNames(self, res)) }
    }

    #[doc(alias = "AXUIElementCopyAttributeValue")]
    #[inline]
    pub fn attr_value(&self, name: &ax::Attr) -> os::Result<arc::R<cf::Type>> {
        unsafe { os::result_unchecked(|res| AXUIElementCopyAttributeValue(self, name, res)) }
    }

    unsafe fn attr<T: arc::Retain>(&self, name: &ax::Attr) -> os::Result<arc::R<T>> {
        unsafe {
            std::mem::transmute(os::result_unchecked(|res| {
                AXUIElementCopyAttributeValue(self, name, res)
            }))
        }
    }

    #[doc(alias = "AXUIElementCopyActionNames")]
    #[inline]
    pub fn actions(&self) -> os::Result<arc::R<cf::ArrayOf<ax::Action>>> {
        unsafe { os::result_unchecked(|res| AXUIElementCopyActionNames(self, res)) }
    }

    pub fn action_desc(&self, action: &ax::Action) -> os::Result<arc::R<cf::String>> {
        unsafe { os::result_unchecked(|res| AXUIElementCopyActionDescription(self, action, res)) }
    }

    #[inline]
    pub fn set_messaging_timeout_secs(&self, timeout_in_seconds: f32) -> os::Result {
        unsafe { AXUIElementSetMessagingTimeout(self, timeout_in_seconds).result() }
    }

    #[inline]
    pub fn set_messaging_timeout(&self, val: std::time::Duration) -> os::Result {
        self.set_messaging_timeout_secs(val.as_secs() as _)
    }

    #[inline]
    pub fn is_settable(&self, attr: &ax::Attr) -> os::Result<bool> {
        os::result_init(|res| unsafe { AXUIElementIsAttributeSettable(self, attr, res) })
    }
}

/// Shortcuts for attributes
impl UiElement {
    pub fn role(&self) -> os::Result<arc::R<ax::Role>> {
        unsafe { self.attr(ax::attr::role()) }
    }

    pub fn role_desc(&self) -> os::Result<arc::R<cf::String>> {
        unsafe { self.attr(ax::attr::role_desc()) }
    }

    pub fn focused_app(&self) -> os::Result<arc::R<Self>> {
        unsafe { self.attr(ax::attr::focused_app()) }
    }

    pub fn focused_ui_element(&self) -> os::Result<arc::R<Self>> {
        unsafe { self.attr(ax::attr::focused_ui_element()) }
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    fn AXIsProcessTrustedWithOptions(
        options: Option<&cf::DictionaryOf<cf::String, cf::Type>>,
    ) -> bool;

    static kAXTrustedCheckOptionPrompt: &'static cf::String;

    fn AXUIElementGetTypeID() -> cf::TypeId;

    fn AXUIElementCreateApplication(pid: Pid) -> arc::R<UiElement>;
    fn AXUIElementCreateSystemWide() -> arc::R<UiElement>;
    fn AXUIElementGetPid(elem: &UiElement, pid: *mut Pid) -> ax::Error;

    fn AXUIElementCopyAttributeNames(
        elem: &UiElement,
        names: *mut Option<arc::R<cf::ArrayOf<ax::Attr>>>,
    ) -> ax::Error;

    fn AXUIElementCopyAttributeValue(
        elem: &UiElement,
        name: &cf::String,
        value: *mut Option<arc::R<cf::Type>>,
    ) -> ax::Error;

    fn AXUIElementCopyActionNames(
        elem: &UiElement,
        names: *mut Option<arc::R<cf::ArrayOf<ax::Action>>>,
    ) -> ax::Error;

    fn AXUIElementCopyActionDescription(
        elem: &UiElement,
        action: &ax::Action,
        desc: *mut Option<arc::R<cf::String>>,
    ) -> ax::Error;

    fn AXUIElementCopyParameterizedAttributeNames(
        elem: &UiElement,
        names: *mut Option<arc::R<cf::ArrayOf<ax::ParamAttr>>>,
    ) -> ax::Error;

    fn AXUIElementSetMessagingTimeout(elem: &UiElement, timeout_in_seconds: f32) -> ax::Error;

    fn AXUIElementIsAttributeSettable(
        elem: &UiElement,
        attr: &ax::Attr,
        val: *mut bool,
    ) -> ax::Error;
}

#[cfg(test)]
mod tests {
    use crate::{ax, cf};

    #[test]
    fn basics() {
        let sys_wide = ax::UiElement::sys_wide();
        sys_wide
            .set_messaging_timeout(std::time::Duration::from_secs(10))
            .unwrap();

        assert!(!sys_wide.is_settable(ax::attr::role()).unwrap());
        assert!(!sys_wide.is_settable(ax::attr::role_desc()).unwrap());

        let err = sys_wide.pid().unwrap_err();
        assert_eq!(err, ax::err::INVALID_UI_ELEMENT);

        let attrs = sys_wide.attrs().unwrap();
        println!("attrs {attrs:?}");

        let role = sys_wide.attr_value(ax::attr::role()).unwrap();

        let role_desc = sys_wide.role_desc().unwrap();

        println!("role {role:?} {role_desc:?}");

        let actions = sys_wide.actions().unwrap();
        assert!(actions.is_empty());

        let fake_action = ax::Action::with_string(cf::str!(c"fake"));
        let err = sys_wide.action_desc(fake_action).err().unwrap();
        assert_eq!(err, ax::err::INVALID_UI_ELEMENT);

        let err = sys_wide.param_attrs().err().unwrap();
        assert_eq!(err, ax::err::INVALID_UI_ELEMENT);
    }
}
