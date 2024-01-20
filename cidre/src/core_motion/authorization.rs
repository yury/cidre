#[doc(alias = "CMAuthorizationStatus")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum AuthorizationStatus {
    NotDetermined = 0,
    Restricted,
    Denied,
    Authorized,
}
