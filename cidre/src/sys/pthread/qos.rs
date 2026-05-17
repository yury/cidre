use crate::sys::qos;

#[doc(alias = "pthread_set_qos_class_self_np")]
pub fn set_qos_class_self_np(
    value: qos::Class,
    relative_priority: std::ffi::c_int,
) -> Result<(), std::ffi::c_int> {
    let res = unsafe { pthread_set_qos_class_self_np(value, relative_priority) };
    if res == 0 { Ok(()) } else { Err(res) }
}

unsafe extern "C" {
    fn pthread_set_qos_class_self_np(
        value: qos::Class,
        relative_priority: std::ffi::c_int,
    ) -> std::ffi::c_int;
}
