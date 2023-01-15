//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKBackForwardList")]
    pub struct WKBackForwardList;

    #[cfg(feature = "WebKit_WKBackForwardList")]
    unsafe impl ClassType for WKBackForwardList {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_WKBackForwardList")]
    unsafe impl WKBackForwardList {
        #[cfg(feature = "WebKit_WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other currentItem)]
        pub unsafe fn currentItem(&self) -> Option<Id<WKBackForwardListItem, Shared>>;

        #[cfg(feature = "WebKit_WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other backItem)]
        pub unsafe fn backItem(&self) -> Option<Id<WKBackForwardListItem, Shared>>;

        #[cfg(feature = "WebKit_WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other forwardItem)]
        pub unsafe fn forwardItem(&self) -> Option<Id<WKBackForwardListItem, Shared>>;

        #[cfg(feature = "WebKit_WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other itemAtIndex:)]
        pub unsafe fn itemAtIndex(
            &self,
            index: NSInteger,
        ) -> Option<Id<WKBackForwardListItem, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "WebKit_WKBackForwardListItem"
        ))]
        #[method_id(@__retain_semantics Other backList)]
        pub unsafe fn backList(&self) -> Id<NSArray<WKBackForwardListItem>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "WebKit_WKBackForwardListItem"
        ))]
        #[method_id(@__retain_semantics Other forwardList)]
        pub unsafe fn forwardList(&self) -> Id<NSArray<WKBackForwardListItem>, Shared>;
    }
);
