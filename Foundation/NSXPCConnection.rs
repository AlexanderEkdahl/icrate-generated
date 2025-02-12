//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSXPCProxyCreating {
        #[method_id(@__retain_semantics Other remoteObjectProxy)]
        unsafe fn remoteObjectProxy(&self) -> Id<Object>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other remoteObjectProxyWithErrorHandler:)]
        unsafe fn remoteObjectProxyWithErrorHandler(
            &self,
            handler: &Block<(NonNull<NSError>,), ()>,
        ) -> Id<Object>;

        #[cfg(feature = "Foundation_NSError")]
        #[optional]
        #[method_id(@__retain_semantics Other synchronousRemoteObjectProxyWithErrorHandler:)]
        unsafe fn synchronousRemoteObjectProxyWithErrorHandler(
            &self,
            handler: &Block<(NonNull<NSError>,), ()>,
        ) -> Id<Object>;
    }

    unsafe impl ProtocolType for dyn NSXPCProxyCreating {}
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSXPCConnectionOptions {
        NSXPCConnectionPrivileged = 1 << 12,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSXPCConnection")]
    pub struct NSXPCConnection;

    #[cfg(feature = "Foundation_NSXPCConnection")]
    unsafe impl ClassType for NSXPCConnection {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSXPCConnection")]
unsafe impl NSObjectProtocol for NSXPCConnection {}

#[cfg(feature = "Foundation_NSXPCConnection")]
unsafe impl NSXPCProxyCreating for NSXPCConnection {}

extern_methods!(
    #[cfg(feature = "Foundation_NSXPCConnection")]
    unsafe impl NSXPCConnection {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithServiceName:)]
        pub unsafe fn initWithServiceName(
            this: Option<Allocated<Self>>,
            service_name: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other serviceName)]
        pub unsafe fn serviceName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithMachServiceName:options:)]
        pub unsafe fn initWithMachServiceName_options(
            this: Option<Allocated<Self>>,
            name: &NSString,
            options: NSXPCConnectionOptions,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSXPCListenerEndpoint")]
        #[method_id(@__retain_semantics Init initWithListenerEndpoint:)]
        pub unsafe fn initWithListenerEndpoint(
            this: Option<Allocated<Self>>,
            endpoint: &NSXPCListenerEndpoint,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSXPCListenerEndpoint")]
        #[method_id(@__retain_semantics Other endpoint)]
        pub unsafe fn endpoint(&self) -> Id<NSXPCListenerEndpoint>;

        #[cfg(feature = "Foundation_NSXPCInterface")]
        #[method_id(@__retain_semantics Other exportedInterface)]
        pub unsafe fn exportedInterface(&self) -> Option<Id<NSXPCInterface>>;

        #[cfg(feature = "Foundation_NSXPCInterface")]
        #[method(setExportedInterface:)]
        pub unsafe fn setExportedInterface(&self, exported_interface: Option<&NSXPCInterface>);

        #[method_id(@__retain_semantics Other exportedObject)]
        pub unsafe fn exportedObject(&self) -> Option<Id<Object>>;

        #[method(setExportedObject:)]
        pub unsafe fn setExportedObject(&self, exported_object: Option<&Object>);

        #[cfg(feature = "Foundation_NSXPCInterface")]
        #[method_id(@__retain_semantics Other remoteObjectInterface)]
        pub unsafe fn remoteObjectInterface(&self) -> Option<Id<NSXPCInterface>>;

        #[cfg(feature = "Foundation_NSXPCInterface")]
        #[method(setRemoteObjectInterface:)]
        pub unsafe fn setRemoteObjectInterface(
            &self,
            remote_object_interface: Option<&NSXPCInterface>,
        );

        #[method_id(@__retain_semantics Other remoteObjectProxy)]
        pub unsafe fn remoteObjectProxy(&self) -> Id<Object>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other remoteObjectProxyWithErrorHandler:)]
        pub unsafe fn remoteObjectProxyWithErrorHandler(
            &self,
            handler: &Block<(NonNull<NSError>,), ()>,
        ) -> Id<Object>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other synchronousRemoteObjectProxyWithErrorHandler:)]
        pub unsafe fn synchronousRemoteObjectProxyWithErrorHandler(
            &self,
            handler: &Block<(NonNull<NSError>,), ()>,
        ) -> Id<Object>;

        #[method(interruptionHandler)]
        pub unsafe fn interruptionHandler(&self) -> *mut Block<(), ()>;

        #[method(setInterruptionHandler:)]
        pub unsafe fn setInterruptionHandler(&self, interruption_handler: Option<&Block<(), ()>>);

        #[method(invalidationHandler)]
        pub unsafe fn invalidationHandler(&self) -> *mut Block<(), ()>;

        #[method(setInvalidationHandler:)]
        pub unsafe fn setInvalidationHandler(&self, invalidation_handler: Option<&Block<(), ()>>);

        #[method(resume)]
        pub unsafe fn resume(&self);

        #[method(suspend)]
        pub unsafe fn suspend(&self);

        #[method(activate)]
        pub unsafe fn activate(&self);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method_id(@__retain_semantics Other currentConnection)]
        pub unsafe fn currentConnection() -> Option<Id<NSXPCConnection>>;

        #[method(scheduleSendBarrierBlock:)]
        pub unsafe fn scheduleSendBarrierBlock(&self, block: &Block<(), ()>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCodeSigningRequirement:)]
        pub unsafe fn setCodeSigningRequirement(&self, requirement: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSXPCConnection")]
    unsafe impl NSXPCConnection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSXPCListener")]
    pub struct NSXPCListener;

    #[cfg(feature = "Foundation_NSXPCListener")]
    unsafe impl ClassType for NSXPCListener {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSXPCListener")]
unsafe impl NSObjectProtocol for NSXPCListener {}

extern_methods!(
    #[cfg(feature = "Foundation_NSXPCListener")]
    unsafe impl NSXPCListener {
        #[method_id(@__retain_semantics Other serviceListener)]
        pub unsafe fn serviceListener() -> Id<NSXPCListener>;

        #[method_id(@__retain_semantics Other anonymousListener)]
        pub unsafe fn anonymousListener() -> Id<NSXPCListener>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithMachServiceName:)]
        pub unsafe fn initWithMachServiceName(
            this: Option<Allocated<Self>>,
            name: &NSString,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSXPCListenerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSXPCListenerDelegate>>,
        );

        #[cfg(feature = "Foundation_NSXPCListenerEndpoint")]
        #[method_id(@__retain_semantics Other endpoint)]
        pub unsafe fn endpoint(&self) -> Id<NSXPCListenerEndpoint>;

        #[method(resume)]
        pub unsafe fn resume(&self);

        #[method(suspend)]
        pub unsafe fn suspend(&self);

        #[method(activate)]
        pub unsafe fn activate(&self);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setConnectionCodeSigningRequirement:)]
        pub unsafe fn setConnectionCodeSigningRequirement(&self, requirement: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSXPCListener")]
    unsafe impl NSXPCListener {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSXPCListenerDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "Foundation_NSXPCConnection",
            feature = "Foundation_NSXPCListener"
        ))]
        #[optional]
        #[method(listener:shouldAcceptNewConnection:)]
        unsafe fn listener_shouldAcceptNewConnection(
            &self,
            listener: &NSXPCListener,
            new_connection: &NSXPCConnection,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn NSXPCListenerDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSXPCInterface")]
    pub struct NSXPCInterface;

    #[cfg(feature = "Foundation_NSXPCInterface")]
    unsafe impl ClassType for NSXPCInterface {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSXPCInterface")]
unsafe impl NSObjectProtocol for NSXPCInterface {}

extern_methods!(
    #[cfg(feature = "Foundation_NSXPCInterface")]
    unsafe impl NSXPCInterface {
        #[method_id(@__retain_semantics Other interfaceWithProtocol:)]
        pub unsafe fn interfaceWithProtocol(protocol: &Protocol) -> Id<NSXPCInterface>;

        #[method_id(@__retain_semantics Other protocol)]
        pub unsafe fn protocol(&self) -> Id<Protocol>;

        #[method(setProtocol:)]
        pub unsafe fn setProtocol(&self, protocol: &Protocol);

        #[cfg(feature = "Foundation_NSSet")]
        #[method(setClasses:forSelector:argumentIndex:ofReply:)]
        pub unsafe fn setClasses_forSelector_argumentIndex_ofReply(
            &self,
            classes: &NSSet<TodoClass>,
            sel: Sel,
            arg: NSUInteger,
            of_reply: bool,
        );

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other classesForSelector:argumentIndex:ofReply:)]
        pub unsafe fn classesForSelector_argumentIndex_ofReply(
            &self,
            sel: Sel,
            arg: NSUInteger,
            of_reply: bool,
        ) -> Id<NSSet<TodoClass>>;

        #[method(setInterface:forSelector:argumentIndex:ofReply:)]
        pub unsafe fn setInterface_forSelector_argumentIndex_ofReply(
            &self,
            ifc: &NSXPCInterface,
            sel: Sel,
            arg: NSUInteger,
            of_reply: bool,
        );

        #[method_id(@__retain_semantics Other interfaceForSelector:argumentIndex:ofReply:)]
        pub unsafe fn interfaceForSelector_argumentIndex_ofReply(
            &self,
            sel: Sel,
            arg: NSUInteger,
            of_reply: bool,
        ) -> Option<Id<NSXPCInterface>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSXPCInterface")]
    unsafe impl NSXPCInterface {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSXPCListenerEndpoint")]
    pub struct NSXPCListenerEndpoint;

    #[cfg(feature = "Foundation_NSXPCListenerEndpoint")]
    unsafe impl ClassType for NSXPCListenerEndpoint {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSXPCListenerEndpoint")]
unsafe impl NSCoding for NSXPCListenerEndpoint {}

#[cfg(feature = "Foundation_NSXPCListenerEndpoint")]
unsafe impl NSObjectProtocol for NSXPCListenerEndpoint {}

#[cfg(feature = "Foundation_NSXPCListenerEndpoint")]
unsafe impl NSSecureCoding for NSXPCListenerEndpoint {}

extern_methods!(
    #[cfg(feature = "Foundation_NSXPCListenerEndpoint")]
    unsafe impl NSXPCListenerEndpoint {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSXPCListenerEndpoint")]
    unsafe impl NSXPCListenerEndpoint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSXPCCoder")]
    pub struct NSXPCCoder;

    #[cfg(feature = "Foundation_NSXPCCoder")]
    unsafe impl ClassType for NSXPCCoder {
        #[inherits(NSObject)]
        type Super = NSCoder;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSXPCCoder")]
unsafe impl NSObjectProtocol for NSXPCCoder {}

extern_methods!(
    #[cfg(feature = "Foundation_NSXPCCoder")]
    unsafe impl NSXPCCoder {
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSObject>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSObject>);

        #[cfg(feature = "Foundation_NSXPCConnection")]
        #[method_id(@__retain_semantics Other connection)]
        pub unsafe fn connection(&self) -> Option<Id<NSXPCConnection>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSXPCCoder")]
    unsafe impl NSXPCCoder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
