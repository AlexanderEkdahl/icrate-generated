//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMBlob")]
    #[deprecated]
    pub struct DOMBlob;

    #[cfg(feature = "WebKit_DOMBlob")]
    unsafe impl ClassType for DOMBlob {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMBlob")]
    unsafe impl DOMBlob {
        #[method(size)]
        pub unsafe fn size(&self) -> c_ulonglong;
    }
);
