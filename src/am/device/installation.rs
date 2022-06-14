use crate::{cf, cfstr};

use super::{base::ServiceConnection, Device, Error, Session};

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
    pub fn lookup_applications(
        &self,
        options: &cf::Dictionary,
    ) -> Result<cf::Retained<'a, cf::Dictionary>, Error> {
        unsafe {
            let mut info = None;
            let res = AMDeviceLookupApplications(self, options, &mut info);
            res.to_result(info)
        }
    }

    pub fn apps_lookup(&self) -> AppsLookupBuilder {
        AppsLookupBuilder {
            session: self,
            application_type: None,
            attribute: None,
        }
    }
}

pub struct AppsLookupBuilder<'a> {
    session: &'a Session<'a>,
    application_type: Option<cf::Retained<'a, cf::String>>,
    attribute: Option<cf::Retained<'a, cf::String>>,
}

impl<'a> AppsLookupBuilder<'a> {
    pub fn app_type_any(&mut self) -> &mut Self {
        self.application_type = Some(cfstr!("Any").retained());
        self
    }

    pub fn app_type_system(&mut self) -> &mut Self {
        self.application_type = Some(cfstr!("System").retained());
        self
    }

    pub fn app_type_user(&mut self) -> &mut Self {
        self.application_type = Some(cfstr!("User").retained());
        self
    }

    pub fn app_type_internal(&mut self) -> &mut Self {
        self.application_type = Some(cfstr!("Internal").retained());
        self
    }

    pub fn has_settings_bundle(&mut self) -> &mut Self {
        self.attribute = Some(cfstr!("HasSettingsBundle").retained());
        self
    }

    pub fn has_settings_bundle_icon(&mut self) -> &mut Self {
        self.attribute = Some(cfstr!("HasSettingsBundleIcon").retained());
        self
    }

    pub fn lookup(&self) -> Result<cf::Retained<'a, cf::Dictionary>, Error> {
        let mut options = cf::MutableDictionary::with_capacity(3);
        if let Some(ref app_type) = self.application_type {
            options.insert(cfstr!("ApplicationType"), app_type);
        }

        if let Some(ref attribute) = self.attribute {
            options.insert(cfstr!("Attribute"), attribute)
        }

        self.session.lookup_applications(&options)
    }
}

#[link(name = "MobileDevice", kind = "framework")]
extern "C" {
    fn AMDeviceLookupApplications<'a>(
        device: &Device,
        options: &cf::Dictionary,
        info: &mut Option<cf::Retained<'a, cf::Dictionary>>,
    ) -> Error;
}
