use crate::{arc, blocks, define_cls, define_obj_type, define_opts, ns, objc};

#[objc::obj_trait]
pub trait ProxyCreating: objc::Obj {
    #[objc::msg_send(remoteObjectProxy)]
    fn remote_object_proxy(&self) -> arc::R<ns::Id>;

    #[objc::msg_send(remoteObjectProxyWithErrorHandler:)]
    fn remote_object_proxy_with_err_handler(
        &self,
        handler: &mut blocks::SendBlock<fn(&ns::Error)>,
    ) -> arc::R<ns::Id>;

    #[objc::optional]
    #[objc::msg_send(synchronousRemoteObjectProxyWithErrorHandler:)]
    fn sync_remote_object_proxy_with_err_handler(
        &self,
        handler: &mut blocks::SendBlock<fn(&ns::Error)>,
    ) -> arc::R<ns::Id>;
}

define_opts!(
    pub ConnectionOptions(usize)
);

impl ConnectionOptions {
    pub const PRIVILEGED: Self = Self(1 << 12);
}

define_obj_type!(
    #[doc(alias = "NSXPCConnection")]
    pub Connection(ns::Id)
);

impl ProxyCreating for Connection {}

impl arc::A<Connection> {
    #[cfg(target_os = "macos")]
    #[objc::msg_send(initWithServiceName:)]
    pub fn init_with_service_name(self, name: &ns::String) -> arc::R<Connection>;

    #[cfg(target_os = "macos")]
    #[objc::msg_send(initWithServiceName:options:)]
    pub fn init_with_mach_service(
        self,
        name: &ns::String,
        options: ConnectionOptions,
    ) -> arc::R<Connection>;

    #[cfg(target_os = "macos")]
    #[objc::msg_send(initWithListenerEndpoint:)]
    pub fn init_with_listener_endpoint(self, endpoint: &ListenerEndpoint) -> arc::R<Connection>;
}

impl Connection {
    define_cls!(NS_XPC_CONNECTION);

    #[cfg(target_os = "macos")]
    pub fn with_service_name(name: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_service_name(name)
    }

    #[cfg(target_os = "macos")]
    pub fn with_mach_service(name: &ns::String, options: ConnectionOptions) -> arc::R<Self> {
        Self::alloc().init_with_mach_service(name, options)
    }

    #[cfg(target_os = "macos")]
    pub fn with_listener_endpoint(endpoint: &ListenerEndpoint) -> arc::R<Self> {
        Self::alloc().init_with_listener_endpoint(endpoint)
    }

    #[objc::msg_send(serviceName)]
    pub fn service_name(&self) -> Option<&ns::String>;

    #[objc::msg_send(endpoint)]
    pub fn endpoint(&self) -> Option<&ListenerEndpoint>;

    #[objc::msg_send(exportedInterface)]
    pub fn exported_iface(&self) -> Option<&Iface>;

    #[objc::msg_send(setExportedInterface:)]
    pub fn set_exported_iface(&mut self, val: Option<&Iface>);

    #[objc::msg_send(exportedObject)]
    pub fn exported_object(&self) -> Option<&ns::Id>;

    #[objc::msg_send(setExportedObject:)]
    pub fn set_exported_object(&mut self, val: Option<&ns::Id>);

    #[objc::msg_send(remoteObjectInterface)]
    pub fn remote_object_iface(&self) -> Option<&Iface>;

    #[objc::msg_send(setRemoteObjectInterface:)]
    pub fn set_remote_object_iface(&mut self, val: Option<&Iface>);

    #[objc::msg_send(remoteObjectProxy)]
    pub fn remote_object_proxy(&self) -> Option<&ns::Id>;

    #[objc::msg_send(setRemoteObjectProxy:)]
    pub fn set_remote_object_proxy(&mut self, val: Option<&ns::Id>);

    #[objc::msg_send(remoteObjectProxyWithErrorHandler:)]
    pub fn remote_object_proxy_with_err_handler(
        &self,
        handler: &mut blocks::SendBlock<fn(&ns::Error)>,
    ) -> arc::R<ns::Id>;

    #[objc::msg_send(synchronousRemoteObjectProxyWithErrorHandler:)]
    pub fn sync_remote_object_proxy_with_err_handler(
        &self,
        handler: &mut blocks::SendBlock<fn(&ns::Error)>,
    ) -> arc::R<ns::Id>;

    #[objc::msg_send(interruptionHandler)]
    pub fn interruption_handler(&mut self) -> Option<&mut blocks::EscBlock<fn()>>;

    #[objc::msg_send(setInterruptionHandler:)]
    pub fn set_interruption_handler(&mut self, val: Option<&mut blocks::EscBlock<fn()>>);

    #[objc::msg_send(invalidationHandler)]
    pub fn invalidation_handler(&mut self) -> Option<&mut blocks::EscBlock<fn()>>;

    #[objc::msg_send(setInvalidationHandler:)]
    pub fn set_invalidation_handler(&mut self, val: Option<&mut blocks::EscBlock<fn()>>);

    #[objc::msg_send(resume)]
    pub fn resume(&mut self);

    #[objc::msg_send(suspend)]
    pub fn suspend(&mut self);

    #[objc::msg_send(activate)]
    pub fn activate(&mut self);

    #[objc::msg_send(invalidate)]
    pub fn invalidate(&mut self);

    #[cfg(target_os = "macos")]
    #[objc::msg_send(setConnectionCodeSigningRequirement:)]
    pub fn set_connection_code_signing_requirement(&mut self, val: &ns::String);
}

define_obj_type!(
    #[doc(alias = "NSXPCListenerEndpoint")]
    pub ListenerEndpoint(ns::Id)
);

define_obj_type!(
    #[doc(alias = "NSXPCInterface")]
    pub Iface(ns::Id)
);

impl Iface {
    define_cls!(NS_XPC_INTERFACE);

    #[objc::msg_send(interfaceWithProtocol:)]
    pub fn interface_with_protocol(protocol: &ns::Id) -> arc::R<Self>;

    #[objc::msg_send(protocol)]
    pub fn protocol(&self) -> &ns::Id;

    #[objc::msg_send(setProtocol:)]
    pub fn set_protocol(&mut self, val: &ns::Id);
}

define_obj_type!(
    #[doc(alias = "NSXPCListener")]
    pub Listener(ns::Id)
);

impl arc::A<Listener> {
    #[objc::msg_send(initWithMachServiceName:)]
    pub fn init_with_mach_service_name(self, name: &ns::String) -> arc::R<Listener>;
}

impl Listener {
    define_cls!(NS_XPC_LISTENER);

    #[objc::msg_send(serviceListener)]
    pub fn service() -> arc::R<Self>;

    #[objc::msg_send(anonymousListener)]
    pub fn anonymous() -> arc::R<Self>;

    pub fn with_mach_service_name(name: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_mach_service_name(name)
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_XPC_CONNECTION: &'static objc::Class<Connection>;
    static NS_XPC_LISTENER: &'static objc::Class<Listener>;
    static NS_XPC_INTERFACE: &'static objc::Class<Iface>;
}
