use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use crate::{arc, cf, define_opts, os};

define_opts!(pub Flags(u32));

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

define_opts!(pub InstantiationOpts(u32));

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
impl InstantiationOpts {
    /// Attempt to load the component into a separate extension process.
    pub const LOAD_OUT_OF_PROCESS: Self = Self(1);

    /// Attempt to load the component into the current process. Only available on macOS.
    pub const LOAD_IN_PROCESS: Self = Self(2);
    pub const LOADED_REMOTELY: Self = Self(1 << 31);
}

/// A structure describing the unique and identifying IDs of an audio component
#[doc(alias = "AudioComponentDescription")]
#[derive(Default)]
#[repr(C)]
pub struct Desc {
    /// A 4-char code identifying the generic type of an audio component.
    /// `aenc` for example
    pub type_: os::Type,

    /// A 4-char code identifying the a specific individual component. type/
    /// subtype/manufacturer triples must be globally unique.
    /// `aac ` for example
    pub sub_type: os::Type,

    /// Vendor identification.
    pub manufacturer: os::Type,

    /// Must be set to zero unless a known specific value is requested.
    pub flags: u32,

    /// Must be set to zero unless a known specific value is requested.
    pub flags_mask: u32,
}

impl std::fmt::Debug for Desc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut type_ = self.type_.to_be_bytes();
        let type_ = crate::four_cc_to_str(&mut type_);

        let mut sub_type = self.sub_type.to_be_bytes();
        let sub_type = crate::four_cc_to_str(&mut sub_type);

        let mut manufacturer = self.manufacturer.to_be_bytes();
        let manufacturer = crate::four_cc_to_str(&mut manufacturer);
        f.debug_struct("AudioComponentDescription")
            .field("type_", &type_)
            .field("sub_type", &sub_type)
            .field("manufacturer", &manufacturer)
            .field("flags", &self.flags)
            .field("flags_mask", &self.flags_mask)
            .finish()
    }
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

#[doc(alias = "AudioComponent")]
#[derive(Debug)]
#[repr(transparent)]
pub struct Component(c_void);

/// OpaqueInstance
#[doc(alias = "OpaqueAudioComponentInstance")]
#[derive(Debug)]
#[repr(transparent)]
pub struct Instance(c_void);

#[doc(alias = "AudioComponentInstance")]
#[derive(Debug)]
#[repr(transparent)]
pub struct InstanceRef(&'static mut Instance);

pub trait State<T> {
    fn release_resources(_instance: &mut T) -> os::Result {
        Ok(())
    }
}

#[derive(Debug)]
pub struct UninitializedState;

#[derive(Debug)]
pub struct InitializedState;

impl Component {
    #[inline]
    pub fn name(&self) -> os::Result<arc::R<cf::String>> {
        unsafe { os::result_unchecked(|val| AudioComponentCopyName(self, val)) }
    }

    #[inline]
    pub fn desc(&self) -> os::Result<Desc> {
        let mut desc = Desc::default();
        unsafe { AudioComponentGetDescription(self, &mut desc).result()? };
        Ok(desc)
    }

    #[inline]
    pub fn version(&self) -> os::Result<u32> {
        let mut version = 0;
        unsafe { AudioComponentGetVersion(self, &mut version).result()? };
        Ok(version)
    }

    #[doc(alias = "AudioComponentInstanceNew")]
    pub fn open(&self) -> os::Result<InstanceRef> {
        let mut instance = None;
        unsafe {
            AudioComponentInstanceNew(self, &mut instance).result()?;
            Ok(InstanceRef(instance.unwrap_unchecked()))
        }
    }
}

impl Instance {
    pub fn component(&self) -> Option<&Component> {
        unsafe { AudioComponentInstanceGetComponent(self) }
    }

    pub unsafe fn dispose(&mut self) -> os::Result {
        unsafe { AudioComponentInstanceDispose(self).into() }
    }
}

impl Drop for InstanceRef {
    fn drop(&mut self) {
        let res = unsafe { self.0.dispose() };
        debug_assert!(res.is_ok());
    }
}

impl Deref for InstanceRef {
    type Target = Instance;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl DerefMut for InstanceRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

unsafe extern "C-unwind" {
    fn AudioComponentFindNext(
        in_component: Option<&Component>,
        in_desc: &Desc,
    ) -> Option<&'static Component>;

    fn AudioComponentCount(in_desc: &Desc) -> u32;

    fn AudioComponentCopyName(
        in_component: &Component,
        out_name: *mut Option<arc::R<cf::String>>,
    ) -> os::Status;

    fn AudioComponentGetDescription(component: &Component, out_desc: &mut Desc) -> os::Status;

    fn AudioComponentGetVersion(component: &Component, out_version: &mut u32) -> os::Status;

    fn AudioComponentInstanceNew(
        component: &Component,
        out_instance: *mut Option<&'static mut Instance>,
    ) -> os::Status;

    fn AudioComponentInstanceDispose(instance: &mut Instance) -> os::Status;

    fn AudioComponentInstanceGetComponent(instance: &Instance) -> Option<&Component>;
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

        let _inst = desc.into_iter().next().unwrap().open().unwrap();
    }
}
