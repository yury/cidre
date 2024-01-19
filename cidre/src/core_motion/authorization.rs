#[doc(alias = "CMAuthorizationStatus")]
#[cfg(any(target_os = "ios", target_os = "watchos"))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum AuthorizationStatus {
    NotDetermined = 0,
    Restricted,
    Denied,
    Authorized,
}
