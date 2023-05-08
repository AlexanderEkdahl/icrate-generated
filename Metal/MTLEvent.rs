//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_protocol!(
    pub unsafe trait MTLEvent: NSObjectProtocol {
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Option<Id<ProtocolObject<dyn MTLDevice>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);
    }

    unsafe impl ProtocolType for dyn MTLEvent {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLSharedEventListener")]
    pub struct MTLSharedEventListener;

    #[cfg(feature = "Metal_MTLSharedEventListener")]
    unsafe impl ClassType for MTLSharedEventListener {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLSharedEventListener")]
unsafe impl NSObjectProtocol for MTLSharedEventListener {}

extern_methods!(
    #[cfg(feature = "Metal_MTLSharedEventListener")]
    unsafe impl MTLSharedEventListener {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLSharedEventListener")]
    unsafe impl MTLSharedEventListener {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);
#[cfg(feature = "Metal_MTLSharedEventListener")]
impl DefaultId for MTLSharedEventListener {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

pub type MTLSharedEventNotificationBlock =
    *mut Block<(NonNull<ProtocolObject<dyn MTLSharedEvent>>, u64), ()>;

extern_protocol!(
    pub unsafe trait MTLSharedEvent: MTLEvent {
        #[cfg(feature = "Metal_MTLSharedEventListener")]
        #[method(notifyListener:atValue:block:)]
        unsafe fn notifyListener_atValue_block(
            &self,
            listener: &MTLSharedEventListener,
            value: u64,
            block: MTLSharedEventNotificationBlock,
        );

        #[cfg(feature = "Metal_MTLSharedEventHandle")]
        #[method_id(@__retain_semantics New newSharedEventHandle)]
        unsafe fn newSharedEventHandle(&self) -> Id<MTLSharedEventHandle>;

        #[method(signaledValue)]
        unsafe fn signaledValue(&self) -> u64;

        #[method(setSignaledValue:)]
        unsafe fn setSignaledValue(&self, signaled_value: u64);
    }

    unsafe impl ProtocolType for dyn MTLSharedEvent {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLSharedEventHandle")]
    pub struct MTLSharedEventHandle;

    #[cfg(feature = "Metal_MTLSharedEventHandle")]
    unsafe impl ClassType for MTLSharedEventHandle {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLSharedEventHandle")]
unsafe impl NSCoding for MTLSharedEventHandle {}

#[cfg(feature = "Metal_MTLSharedEventHandle")]
unsafe impl NSObjectProtocol for MTLSharedEventHandle {}

#[cfg(feature = "Metal_MTLSharedEventHandle")]
unsafe impl NSSecureCoding for MTLSharedEventHandle {}

extern_methods!(
    #[cfg(feature = "Metal_MTLSharedEventHandle")]
    unsafe impl MTLSharedEventHandle {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLSharedEventHandle")]
    unsafe impl MTLSharedEventHandle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
