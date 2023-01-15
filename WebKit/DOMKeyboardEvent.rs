//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_enum!(
    #[underlying(c_uint)]
    #[deprecated]
    pub enum __anonymous__ {
        #[deprecated]
        DOM_KEY_LOCATION_STANDARD = 0x00,
        #[deprecated]
        DOM_KEY_LOCATION_LEFT = 0x01,
        #[deprecated]
        DOM_KEY_LOCATION_RIGHT = 0x02,
        #[deprecated]
        DOM_KEY_LOCATION_NUMPAD = 0x03,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMKeyboardEvent")]
    #[deprecated]
    pub struct DOMKeyboardEvent;

    #[cfg(feature = "WebKit_DOMKeyboardEvent")]
    unsafe impl ClassType for DOMKeyboardEvent {
        #[inherits(DOMEvent, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMUIEvent;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMKeyboardEvent")]
    unsafe impl DOMKeyboardEvent {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other keyIdentifier)]
        pub unsafe fn keyIdentifier(&self) -> Id<NSString, Shared>;

        #[method(location)]
        pub unsafe fn location(&self) -> c_uint;

        #[deprecated]
        #[method(keyLocation)]
        pub unsafe fn keyLocation(&self) -> c_uint;

        #[method(ctrlKey)]
        pub unsafe fn ctrlKey(&self) -> bool;

        #[method(shiftKey)]
        pub unsafe fn shiftKey(&self) -> bool;

        #[method(altKey)]
        pub unsafe fn altKey(&self) -> bool;

        #[method(metaKey)]
        pub unsafe fn metaKey(&self) -> bool;

        #[method(altGraphKey)]
        pub unsafe fn altGraphKey(&self) -> bool;

        #[method(keyCode)]
        pub unsafe fn keyCode(&self) -> c_int;

        #[method(charCode)]
        pub unsafe fn charCode(&self) -> c_int;

        #[cfg(feature = "Foundation_NSString")]
        #[method(getModifierState:)]
        pub unsafe fn getModifierState(&self, key_identifier_arg: Option<&NSString>) -> bool;

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMAbstractView"))]
        #[method(initKeyboardEvent:canBubble:cancelable:view:keyIdentifier:location:ctrlKey:altKey:shiftKey:metaKey:altGraphKey:)]
        pub unsafe fn initKeyboardEvent_canBubble_cancelable_view_keyIdentifier_location_ctrlKey_altKey_shiftKey_metaKey_altGraphKey(
            &self,
            r#type: Option<&NSString>,
            can_bubble: bool,
            cancelable: bool,
            view: Option<&DOMAbstractView>,
            key_identifier: Option<&NSString>,
            location: c_uint,
            ctrl_key: bool,
            alt_key: bool,
            shift_key: bool,
            meta_key: bool,
            alt_graph_key: bool,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMAbstractView"))]
        #[method(initKeyboardEvent:canBubble:cancelable:view:keyIdentifier:location:ctrlKey:altKey:shiftKey:metaKey:)]
        pub unsafe fn initKeyboardEvent_canBubble_cancelable_view_keyIdentifier_location_ctrlKey_altKey_shiftKey_metaKey(
            &self,
            r#type: Option<&NSString>,
            can_bubble: bool,
            cancelable: bool,
            view: Option<&DOMAbstractView>,
            key_identifier: Option<&NSString>,
            location: c_uint,
            ctrl_key: bool,
            alt_key: bool,
            shift_key: bool,
            meta_key: bool,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMAbstractView"))]
        #[deprecated]
        #[method(initKeyboardEvent:canBubble:cancelable:view:keyIdentifier:keyLocation:ctrlKey:altKey:shiftKey:metaKey:altGraphKey:)]
        pub unsafe fn initKeyboardEvent_canBubble_cancelable_view_keyIdentifier_keyLocation_ctrlKey_altKey_shiftKey_metaKey_altGraphKey(
            &self,
            r#type: Option<&NSString>,
            can_bubble: bool,
            cancelable: bool,
            view: Option<&DOMAbstractView>,
            key_identifier: Option<&NSString>,
            key_location: c_uint,
            ctrl_key: bool,
            alt_key: bool,
            shift_key: bool,
            meta_key: bool,
            alt_graph_key: bool,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMAbstractView"))]
        #[deprecated]
        #[method(initKeyboardEvent:canBubble:cancelable:view:keyIdentifier:keyLocation:ctrlKey:altKey:shiftKey:metaKey:)]
        pub unsafe fn initKeyboardEvent_canBubble_cancelable_view_keyIdentifier_keyLocation_ctrlKey_altKey_shiftKey_metaKey(
            &self,
            r#type: Option<&NSString>,
            can_bubble: bool,
            cancelable: bool,
            view: Option<&DOMAbstractView>,
            key_identifier: Option<&NSString>,
            key_location: c_uint,
            ctrl_key: bool,
            alt_key: bool,
            shift_key: bool,
            meta_key: bool,
        );
    }
);
