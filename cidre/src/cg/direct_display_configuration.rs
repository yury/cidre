use crate::cg;

#[cfg(target_os = "macos")]
impl cg::DirectDisplayId {
    /// Return true if the display is connected, awake, and available for
    /// drawing; false otherwise.
    pub fn is_active(self) -> bool {
        unsafe { CGDisplayIsActive(self) }
    }

    /// Return true if the display is asleep (and is therefore not drawable);
    /// false otherwise.
    pub fn is_asleep(self) -> bool {
        unsafe { CGDisplayIsAsleep(self) }
    }

    /// Return true if the display is connected or online; false otherwise.
    pub fn is_online(self) -> bool {
        unsafe { CGDisplayIsOnline(self) }
    }

    /// Return true if the display is the current main display; false
    /// otherwise
    pub fn is_main(self) -> bool {
        unsafe { CGDisplayIsMain(self) }
    }

    /// Return true if the display is built-in, such as the internal display in
    /// portable systems; false otherwise.
    pub fn is_builtin(self) -> bool {
        unsafe { CGDisplayIsBuiltin(self) }
    }
}

#[cfg(target_os = "macos")]
unsafe extern "C-unwind" {
    fn CGDisplayIsActive(display: cg::DirectDisplayId) -> bool;
    fn CGDisplayIsAsleep(display: cg::DirectDisplayId) -> bool;
    fn CGDisplayIsOnline(display: cg::DirectDisplayId) -> bool;
    fn CGDisplayIsMain(display: cg::DirectDisplayId) -> bool;
    fn CGDisplayIsBuiltin(display: cg::DirectDisplayId) -> bool;
}
