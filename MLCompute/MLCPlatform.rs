//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::MLCompute::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCPlatform")]
    pub struct MLCPlatform;

    #[cfg(feature = "MLCompute_MLCPlatform")]
    unsafe impl ClassType for MLCPlatform {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCPlatform")]
unsafe impl NSObjectProtocol for MLCPlatform {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCPlatform")]
    unsafe impl MLCPlatform {
        #[cfg(feature = "Foundation_NSNumber")]
        #[method(setRNGSeedTo:)]
        pub unsafe fn setRNGSeedTo(seed: &NSNumber,);

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other getRNGseed)]
        pub unsafe fn getRNGseed() -> Option<Id<NSNumber>>;

    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MLCompute_MLCPlatform")]
    unsafe impl MLCPlatform {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>, ) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

    }
);

