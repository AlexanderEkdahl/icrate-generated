//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNGroup")]
    pub struct CNGroup;

    #[cfg(feature = "Contacts_CNGroup")]
    unsafe impl ClassType for CNGroup {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Contacts_CNGroup")]
    unsafe impl CNGroup {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;
    }
);

extern_static!(CNGroupIdentifierKey: &'static NSString);

extern_static!(CNGroupNameKey: &'static NSString);