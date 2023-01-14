//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSHTTPPropertyStatusCodeKey: Option<&'static NSString>);

extern_static!(NSHTTPPropertyStatusReasonKey: Option<&'static NSString>);

extern_static!(NSHTTPPropertyServerHTTPVersionKey: Option<&'static NSString>);

extern_static!(NSHTTPPropertyRedirectionHeadersKey: Option<&'static NSString>);

extern_static!(NSHTTPPropertyErrorPageDataKey: Option<&'static NSString>);

extern_static!(NSHTTPPropertyHTTPProxy: Option<&'static NSString>);

extern_static!(NSFTPPropertyUserLoginKey: Option<&'static NSString>);

extern_static!(NSFTPPropertyUserPasswordKey: Option<&'static NSString>);

extern_static!(NSFTPPropertyActiveTransferModeKey: Option<&'static NSString>);

extern_static!(NSFTPPropertyFileOffsetKey: Option<&'static NSString>);

extern_static!(NSFTPPropertyFTPProxy: Option<&'static NSString>);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSURLHandleStatus {
        NSURLHandleNotLoaded = 0,
        NSURLHandleLoadSucceeded = 1,
        NSURLHandleLoadInProgress = 2,
        NSURLHandleLoadFailed = 3,
    }
);

extern_protocol!(
    #[deprecated]
    pub struct NSURLHandleClient;

    unsafe impl ProtocolType for NSURLHandleClient {
        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSURLHandle"))]
        #[deprecated]
        #[method(URLHandle:resourceDataDidBecomeAvailable:)]
        pub unsafe fn URLHandle_resourceDataDidBecomeAvailable(
            &self,
            sender: Option<&NSURLHandle>,
            newBytes: Option<&NSData>,
        );

        #[cfg(feature = "Foundation_NSURLHandle")]
        #[deprecated]
        #[method(URLHandleResourceDidBeginLoading:)]
        pub unsafe fn URLHandleResourceDidBeginLoading(&self, sender: Option<&NSURLHandle>);

        #[cfg(feature = "Foundation_NSURLHandle")]
        #[deprecated]
        #[method(URLHandleResourceDidFinishLoading:)]
        pub unsafe fn URLHandleResourceDidFinishLoading(&self, sender: Option<&NSURLHandle>);

        #[cfg(feature = "Foundation_NSURLHandle")]
        #[deprecated]
        #[method(URLHandleResourceDidCancelLoading:)]
        pub unsafe fn URLHandleResourceDidCancelLoading(&self, sender: Option<&NSURLHandle>);

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURLHandle"))]
        #[deprecated]
        #[method(URLHandle:resourceDidFailLoadingWithReason:)]
        pub unsafe fn URLHandle_resourceDidFailLoadingWithReason(
            &self,
            sender: Option<&NSURLHandle>,
            reason: Option<&NSString>,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLHandle")]
    pub struct NSURLHandle;

    #[cfg(feature = "Foundation_NSURLHandle")]
    unsafe impl ClassType for NSURLHandle {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSURLHandle")]
    unsafe impl NSURLHandle {
        #[deprecated]
        #[method(registerURLHandleClass:)]
        pub unsafe fn registerURLHandleClass(anURLHandleSubclass: Option<&Class>);

        #[cfg(feature = "Foundation_NSURL")]
        #[deprecated]
        #[method(URLHandleClassForURL:)]
        pub unsafe fn URLHandleClassForURL(anURL: Option<&NSURL>) -> Option<&'static Class>;

        #[deprecated]
        #[method(status)]
        pub unsafe fn status(&self) -> NSURLHandleStatus;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other failureReason)]
        pub unsafe fn failureReason(&self) -> Option<Id<NSString, Shared>>;

        #[deprecated]
        #[method(addClient:)]
        pub unsafe fn addClient(&self, client: Option<&NSURLHandleClient>);

        #[deprecated]
        #[method(removeClient:)]
        pub unsafe fn removeClient(&self, client: Option<&NSURLHandleClient>);

        #[deprecated]
        #[method(loadInBackground)]
        pub unsafe fn loadInBackground(&self);

        #[deprecated]
        #[method(cancelLoadInBackground)]
        pub unsafe fn cancelLoadInBackground(&self);

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated]
        #[method_id(@__retain_semantics Other resourceData)]
        pub unsafe fn resourceData(&self) -> Option<Id<NSData, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated]
        #[method_id(@__retain_semantics Other availableResourceData)]
        pub unsafe fn availableResourceData(&self) -> Option<Id<NSData, Shared>>;

        #[deprecated]
        #[method(expectedResourceDataSize)]
        pub unsafe fn expectedResourceDataSize(&self) -> c_longlong;

        #[deprecated]
        #[method(flushCachedData)]
        pub unsafe fn flushCachedData(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(backgroundLoadDidFailWithReason:)]
        pub unsafe fn backgroundLoadDidFailWithReason(&self, reason: Option<&NSString>);

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated]
        #[method(didLoadBytes:loadComplete:)]
        pub unsafe fn didLoadBytes_loadComplete(&self, newBytes: Option<&NSData>, yorn: bool);

        #[cfg(feature = "Foundation_NSURL")]
        #[deprecated]
        #[method(canInitWithURL:)]
        pub unsafe fn canInitWithURL(anURL: Option<&NSURL>) -> bool;

        #[cfg(feature = "Foundation_NSURL")]
        #[deprecated]
        #[method_id(@__retain_semantics Other cachedHandleForURL:)]
        pub unsafe fn cachedHandleForURL(anURL: Option<&NSURL>) -> Option<Id<NSURLHandle, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithURL:cached:)]
        pub unsafe fn initWithURL_cached(
            this: Option<Allocated<Self>>,
            anURL: Option<&NSURL>,
            willCache: bool,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other propertyForKey:)]
        pub unsafe fn propertyForKey(
            &self,
            propertyKey: Option<&NSString>,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other propertyForKeyIfAvailable:)]
        pub unsafe fn propertyForKeyIfAvailable(
            &self,
            propertyKey: Option<&NSString>,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(writeProperty:forKey:)]
        pub unsafe fn writeProperty_forKey(
            &self,
            propertyValue: Option<&Object>,
            propertyKey: Option<&NSString>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated]
        #[method(writeData:)]
        pub unsafe fn writeData(&self, data: Option<&NSData>) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated]
        #[method_id(@__retain_semantics Other loadInForeground)]
        pub unsafe fn loadInForeground(&self) -> Option<Id<NSData, Shared>>;

        #[deprecated]
        #[method(beginLoadInBackground)]
        pub unsafe fn beginLoadInBackground(&self);

        #[deprecated]
        #[method(endLoadInBackground)]
        pub unsafe fn endLoadInBackground(&self);
    }
);
