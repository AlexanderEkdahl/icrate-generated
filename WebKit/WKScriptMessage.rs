//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKScriptMessage")]
    pub struct WKScriptMessage;

    #[cfg(feature = "WebKit_WKScriptMessage")]
    unsafe impl ClassType for WKScriptMessage {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_WKScriptMessage")]
    unsafe impl WKScriptMessage {
        #[method_id(@__retain_semantics Other body)]
        pub unsafe fn body(&self) -> Id<Object, Shared>;

        #[cfg(feature = "WebKit_WKWebView")]
        #[method_id(@__retain_semantics Other webView)]
        pub unsafe fn webView(&self) -> Option<Id<WKWebView, Shared>>;

        #[cfg(feature = "WebKit_WKFrameInfo")]
        #[method_id(@__retain_semantics Other frameInfo)]
        pub unsafe fn frameInfo(&self) -> Id<WKFrameInfo, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "WebKit_WKContentWorld")]
        #[method_id(@__retain_semantics Other world)]
        pub unsafe fn world(&self) -> Id<WKContentWorld, Shared>;
    }
);
