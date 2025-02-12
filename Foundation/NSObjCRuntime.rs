//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSFoundationVersionNumber: c_double);

typed_extensible_enum!(
    pub type NSExceptionName = NSString;
);

typed_extensible_enum!(
    pub type NSRunLoopMode = NSString;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSStringFromSelector(a_selector: Sel) -> NonNull<NSString>;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSSelectorFromString(a_selector_name: &NSString) -> Sel;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSStringFromClass(a_class: &Class) -> NonNull<NSString>;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSClassFromString(a_class_name: &NSString) -> *const Class;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSStringFromProtocol(proto: &Protocol) -> NonNull<NSString>;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSProtocolFromString(namestr: &NSString) -> *mut Protocol;
);

extern_fn!(
    pub unsafe fn NSGetSizeAndAlignment(
        type_ptr: NonNull<c_char>,
        sizep: *mut NSUInteger,
        alignp: *mut NSUInteger,
    ) -> NonNull<c_char>;
);

pub type NSComparator = *mut Block<(NonNull<Object>, NonNull<Object>), NSComparisonResult>;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSEnumerationOptions {
        NSEnumerationConcurrent = 1 << 0,
        NSEnumerationReverse = 1 << 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSSortOptions {
        NSSortConcurrent = 1 << 0,
        NSSortStable = 1 << 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSQualityOfService {
        NSQualityOfServiceUserInteractive = 0x21,
        NSQualityOfServiceUserInitiated = 0x19,
        NSQualityOfServiceUtility = 0x11,
        NSQualityOfServiceBackground = 0x09,
        NSQualityOfServiceDefault = -1,
    }
);
