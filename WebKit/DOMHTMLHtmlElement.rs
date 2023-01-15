//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLHtmlElement")]
    #[deprecated]
    pub struct DOMHTMLHtmlElement;

    #[cfg(feature = "WebKit_DOMHTMLHtmlElement")]
    unsafe impl ClassType for DOMHTMLHtmlElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLHtmlElement")]
    unsafe impl DOMHTMLHtmlElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other version)]
        pub unsafe fn version(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setVersion:)]
        pub unsafe fn setVersion(&self, version: Option<&NSString>);
    }
);
