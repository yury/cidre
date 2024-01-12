use std::ffi::{c_char, CStr};

use crate::{arc, define_obj_type, ns, nw};

define_obj_type!(
    #[doc(alias = "nw_privacy_context")]
    #[doc(alias = "nw_privacy_context_t")]
    pub PrivacyContext(ns::Id)
);

impl PrivacyContext {
    #[doc(alias = "nw_privacy_context_create")]
    #[inline]
    pub fn with_desc(val: &CStr) -> arc::R<Self> {
        unsafe { nw_privacy_context_create(val.as_ptr()) }
    }

    #[inline]
    pub fn default() -> &'static mut Self {
        unsafe { _nw_privacy_context_default_context }
    }

    #[doc(alias = "nw_privacy_context_flush_cache")]
    #[inline]
    pub fn flush_cache(&mut self) {
        unsafe { nw_privacy_context_flush_cache(self) }
    }

    #[doc(alias = "nw_privacy_context_disable_logging")]
    #[inline]
    pub fn disable_logging(&mut self) {
        unsafe { nw_privacy_context_disable_logging(self) }
    }

    #[doc(alias = "nw_privacy_context_require_encrypted_name_resolution")]
    #[inline]
    pub fn require_encrypted_name_resolution(
        &mut self,
        val: bool,
        fallback_resolver_cfg: Option<&nw::ResolverCfg>,
    ) {
        unsafe {
            nw_privacy_context_require_encrypted_name_resolution(self, val, fallback_resolver_cfg)
        }
    }

    #[doc(alias = "nw_privacy_context_add_proxy")]
    #[inline]
    pub fn add_proxy(&mut self, proxy_cfg: &nw::ProxyCfg) {
        unsafe { nw_privacy_context_add_proxy(self, proxy_cfg) }
    }

    #[doc(alias = "nw_privacy_context_clear_proxies")]
    #[inline]
    pub fn clear_proxies(&mut self) {
        unsafe { nw_privacy_context_clear_proxies(self) }
    }
}

#[link(name = "Network", kind = "framework")]
extern "C" {
    fn nw_privacy_context_create(description: *const c_char) -> arc::R<PrivacyContext>;
    fn nw_privacy_context_flush_cache(context: &mut PrivacyContext);
    fn nw_privacy_context_disable_logging(context: &mut PrivacyContext);
    fn nw_privacy_context_require_encrypted_name_resolution(
        context: &mut PrivacyContext,
        require_encrypted_name_resolution: bool,
        fallback_resolver_config: Option<&nw::ResolverCfg>,
    );

    fn nw_privacy_context_add_proxy(context: &mut PrivacyContext, proxy_config: &nw::ProxyCfg);
    fn nw_privacy_context_clear_proxies(context: &mut PrivacyContext);

    static mut _nw_privacy_context_default_context: &'static mut PrivacyContext;
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use crate::nw;

    #[test]
    fn basics() {
        let desc = CString::new("test").unwrap();
        let mut ctx = nw::PrivacyContext::with_desc(&desc);
        ctx.flush_cache();
        ctx.disable_logging();
        eprintln!("{:?}", ctx);

        let ctx = nw::PrivacyContext::default();
        ctx.disable_logging();
        ctx.flush_cache();
        eprintln!("{:?}", ctx);
    }
}
