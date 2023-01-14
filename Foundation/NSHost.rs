//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSHost")]
    #[deprecated = "Use Network framework instead, see deprecation notice in <Foundation/NSHost.h>"]
    pub struct NSHost;

    #[cfg(feature = "Foundation_NSHost")]
    unsafe impl ClassType for NSHost {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSHost")]
    unsafe impl NSHost {
        #[method_id(@__retain_semantics Other currentHost)]
        pub unsafe fn currentHost() -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other hostWithName:)]
        pub unsafe fn hostWithName(name: Option<&NSString>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other hostWithAddress:)]
        pub unsafe fn hostWithAddress(address: &NSString) -> Id<Self, Shared>;

        #[method(isEqualToHost:)]
        pub unsafe fn isEqualToHost(&self, aHost: &NSHost) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other names)]
        pub unsafe fn names(&self) -> Id<NSArray<NSString>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other address)]
        pub unsafe fn address(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other addresses)]
        pub unsafe fn addresses(&self) -> Id<NSArray<NSString>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Id<NSString, Shared>>;

        #[deprecated = "Caching no longer supported"]
        #[method(setHostCacheEnabled:)]
        pub unsafe fn setHostCacheEnabled(flag: bool);

        #[deprecated = "Caching no longer supported"]
        #[method(isHostCacheEnabled)]
        pub unsafe fn isHostCacheEnabled() -> bool;

        #[deprecated = "Caching no longer supported"]
        #[method(flushHostCache)]
        pub unsafe fn flushHostCache();
    }
);
