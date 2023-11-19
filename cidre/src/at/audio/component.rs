use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use crate::{arc, cf, define_options, os};

define_options!(pub Flags(u32));

/// AudioComponentFlags
impl Flags {
    /// When this bit in AudioComponentDescription's componentFlags is set, AudioComponentFindNext
    /// will only return this component when performing a specific, non-wildcard search for the
    /// component, i.e. with non-zero values of componentType, componentSubType, and
    /// componentManufacturer. This can be useful when privately registering a component.
    pub const UNSEARCHABLE: Self = Self(1);

    /// An AudioComponent sets this bit in its componentFlags to indicate to the system that the
    /// AudioComponent is safe to open in a sandboxed process.
    pub const SANDBOX_SAFE: Self = Self(2);

    /// The system sets this flag automatically when registering components which implement a version 3
    /// Audio Unit.
    pub const IS_V3_AUDIO_UNIT: Self = Self(4);

    /// The system sets this flag automatically when registering components which require asynchronous
    /// instantiation via AudioComponentInstantiate (v3 audio units with views).
    pub const REQUIRES_ASYNC_INSTANTIATION: Self = Self(8);

    /// The system sets this flag automatically when registering components which can be loaded into
    /// the current process. This is always true for V2 audio units; it depends on the packaging
    /// in the case of a V3 audio unit.
    pub const CAN_LOAD_IN_PROCESS: Self = Self(0x10);
}

define_options!(pub InstantiationOptions(u32));

/// AudioComponentInstantiationOptions
///
/// Most component instances are loaded into the calling process.
///
/// A version 3 audio unit, however, can be loaded into a separate extension service process,
/// and this is the default behavior for these components. To be able to load one in-process
/// requires that the developer package the audio unit in a bundle separate from the application
/// extension, since an extension's main binary cannot be dynamically loaded into another
/// process.
///
/// A macOS host may request in-process loading of such audio units using
/// LOAD_IN_PROCESS.
///
/// Flag::IS_V3_AUDIO_UNIT specifies whether an audio unit is implemented using API
/// version 3.
///
/// These options are just requests to the implementation. It may fail and fall back to the
/// default.
impl InstantiationOptions {
    /// Attempt to load the component into a separate extension process.
    pub const LOAD_OUT_OF_PROCESS: Self = Self(1);

    /// Attempt to load the component into the current process. Only available on macOS.
    pub const LOAD_IN_PROCESS: Self = Self(2);
    pub const LOADED_REMOTELY: Self = Self(1 << 31);
}

/// AudioComponentDescription
///
/// A structure describing the unique and identifying IDs of an audio component
///
#[derive(Default, Debug)]
#[repr(C, align(4))]
pub struct Desc {
    /// A 4-char code identifying the generic type of an audio component.
    /// `aenc` for example
    pub type_: os::Type,

    /// A 4-char code identifying the a specific individual component. type/
    /// subtype/manufacturer triples must be globally unique.
    /// `aac ` for example
    pub sub_type: os::Type,

    ///  Vendor identification.
    pub manufacturer: os::Type,

    /// Must be set to zero unless a known specific value is requested.
    pub flags: u32,

    /// Must be set to zero unless a known specific value is requested.
    pub flags_mask: u32,
}

impl Desc {
    #[doc(alias = "AudioComponentCount")]
    pub fn components_count(&self) -> u32 {
        unsafe { AudioComponentCount(self) }
    }
}

impl IntoIterator for Desc {
    type Item = &'static Component;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            desc: self,
            component: None,
        }
    }
}

pub struct Iter {
    desc: Desc,
    component: Option<&'static Component>,
}

impl Iterator for Iter {
    type Item = &'static Component;

    fn next(&mut self) -> Option<Self::Item> {
        self.component = unsafe { AudioComponentFindNext(self.component, &self.desc) };
        self.component
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Component(c_void);

#[derive(Debug)]
#[repr(transparent)]
pub struct ComponentInstance(c_void);

#[derive(Debug)]
#[repr(transparent)]
pub struct ComponentInstanceRef(&'static mut ComponentInstance);

impl Drop for ComponentInstanceRef {
    fn drop(&mut self) {
        let res = unsafe { AudioComponentInstanceDispose(self.0) };
        debug_assert!(res.is_ok());
    }
}

impl Deref for ComponentInstanceRef {
    type Target = ComponentInstance;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl DerefMut for ComponentInstanceRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl Component {
    #[inline]
    pub fn name(&self) -> Result<arc::R<cf::String>, os::Status> {
        let mut res = None;
        unsafe { AudioComponentCopyName(self, &mut res).to_result_unchecked(res) }
    }

    #[inline]
    pub fn desc(&self) -> Result<Desc, os::Status> {
        let mut desc = Desc::default();
        unsafe {
            let res = AudioComponentGetDescription(self, &mut desc);
            if res.is_ok() {
                Ok(desc)
            } else {
                Err(res)
            }
        }
    }

    #[inline]
    pub fn version(&self) -> Result<u32, os::Status> {
        unsafe {
            let mut version = 0;
            let res = AudioComponentGetVersion(self, &mut version);
            if res.is_ok() {
                Ok(version)
            } else {
                Err(res)
            }
        }
    }

    pub fn new_instance(&self) -> Result<ComponentInstanceRef, os::Status> {
        unsafe {
            let mut instance = None;
            let res = AudioComponentInstanceNew(self, &mut instance);
            if res.is_ok() {
                Ok(ComponentInstanceRef(instance.unwrap_unchecked()))
            } else {
                Err(res)
            }
        }
    }
}

extern "C" {
    fn AudioComponentFindNext(
        in_component: Option<&Component>,
        in_desc: &Desc,
    ) -> Option<&'static Component>;

    fn AudioComponentCount(in_desc: &Desc) -> u32;

    fn AudioComponentCopyName(
        in_component: &Component,
        out_name: &mut Option<arc::R<cf::String>>,
    ) -> os::Status;

    fn AudioComponentGetDescription(component: &Component, out_desc: &mut Desc) -> os::Status;

    fn AudioComponentGetVersion(component: &Component, out_version: &mut u32) -> os::Status;

    fn AudioComponentInstanceNew(
        component: &Component,
        out_instance: &mut Option<&'static mut ComponentInstance>,
    ) -> os::Status;

    fn AudioComponentInstanceDispose(instance: &mut ComponentInstance) -> os::Status;

    //fn CAShow(in_object: *const c_void);
}

#[cfg(test)]
mod tests {
    use crate::at::audio;

    #[test]
    fn basics() {
        let desc = audio::ComponentDesc {
            // type_: u32::from_be_bytes(*b"aenc"),
            // sub_type: u32::from_be_bytes(*b"aac "),
            ..Default::default()
        };

        let count = desc.components_count();

        for c in desc.into_iter() {
            let name = c.name().unwrap();
            let version = c.version().unwrap();
            let desc = c.desc().unwrap();
            println!("v. {version}: {name:?}\n {desc:?}");
        }

        assert!(count > 0);
    }

    #[test]
    fn aac() {
        let desc = audio::ComponentDesc {
            type_: u32::from_be_bytes(*b"aenc"),
            sub_type: u32::from_be_bytes(*b"aac "),
            ..Default::default()
        };

        let _inst = desc.into_iter().last().unwrap().new_instance().unwrap();
    }
}
