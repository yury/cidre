use crate::{arc, cf};

use super::{Device, Error, Session};

impl<'a> Session<'a> {
    /// Lookup the set of installed applications on the device.
    ///
    /// The info argument will point to a cf::Dictionary. Each key in the dictionary is an application
    /// identifier and the value is the Info.plist of the installed application.
    ///
    /// The client can specify options to filter the results.
    //// If the value of kLookupApplicationTypeKey is:
    ////     kApplicationTypeAny: All application types are returned.
    ////     kApplicationTypeSystem: Only applications installed with the OS are returned.
    ////     kApplicationTypeUser: Only third party applications are returned.
    ////
    /// If the value of kLookupAttributeKey is:
    ////    kAttributeHasSettingsBundle: Only applications with the "HasSettingsBundle" attribute are returned.
    ////     kAttributeHasSettingsBundleIcon: Only applications with the "HasSettingsBundleIcon" attribute are returned.
    ////
    ////     kLookupBundleIDsKey
    ////         CFString or array of strings that indicate the bundle IDs to lookup
    ////     kLookupReturnAttributesKey
    ////         CFString or array of strings indicating which of the keys from the Info.plist should be returned
    ////
    /// IMPORTANT NOTE:
    /// You should specify kLookupReturnAttributesKey in the options dictionary with an array value containing those
    /// keys you would like returned. If you do not specify this attribute, MobileDevice will log every time you make
    /// an offending call and performance will be poor. When specifying this key, you MUST include at least
    /// kCFBundleIdentifierKey in the array of keys to be returned or the call will not work properly.
    pub fn lookup_apps(&self, options: &cf::Dictionary) -> Result<arc::R<cf::Dictionary>, Error> {
        unsafe {
            let mut info = None;
            AMDeviceLookupApplications(self, options, &mut info).to_result(info)
        }
    }

    pub fn apps_lookup(&'a self) -> AppsLookupBuilder<'a> {
        AppsLookupBuilder {
            session: self,
            application_type: None,
            attribute: None,
        }
    }
}

pub struct AppsLookupBuilder<'a> {
    session: &'a Session<'a>,
    application_type: Option<arc::R<cf::String>>,
    attribute: Option<arc::R<cf::String>>,
}

impl<'a> AppsLookupBuilder<'a> {
    pub fn app_type_any(&mut self) -> &mut Self {
        self.application_type = Some(cf::str!(c"Any").retained());
        self
    }

    pub fn app_type_system(&mut self) -> &mut Self {
        self.application_type = Some(cf::str!(c"System").retained());
        self
    }

    pub fn app_type_user(&mut self) -> &mut Self {
        self.application_type = Some(cf::str!(c"User").retained());
        self
    }

    pub fn app_type_internal(&mut self) -> &mut Self {
        self.application_type = Some(cf::str!(c"Internal").retained());
        self
    }

    pub fn has_settings_bundle(&mut self) -> &mut Self {
        self.attribute = Some(cf::str!(c"HasSettingsBundle").retained());
        self
    }

    pub fn has_settings_bundle_icon(&mut self) -> &mut Self {
        self.attribute = Some(cf::str!(c"HasSettingsBundleIcon").retained());
        self
    }

    pub fn lookup(&self) -> Result<arc::R<cf::Dictionary>, Error> {
        let mut options = cf::DictionaryMut::with_capacity(3);
        if let Some(ref app_type) = self.application_type {
            options.insert(cf::str!(c"ApplicationType"), app_type);
        }

        if let Some(ref attribute) = self.attribute {
            options.insert(cf::str!(c"Attribute"), attribute)
        }

        self.session.lookup_apps(&options)
    }
}

#[link(name = "MobileDevice", kind = "framework")]
unsafe extern "C" {
    fn AMDeviceLookupApplications(
        device: &Device,
        options: &cf::Dictionary,
        info: *mut Option<arc::R<cf::Dictionary>>,
    ) -> Error;
}
