#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
#[inline]
pub fn available_memory() -> usize {
    unsafe { os_proc_available_memory() }
}

unsafe extern "C-unwind" {
    #[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
    fn os_proc_available_memory() -> usize;
}
