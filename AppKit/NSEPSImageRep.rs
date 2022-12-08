//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSEPSImageRep;

    unsafe impl ClassType for NSEPSImageRep {
        type Super = NSImageRep;
    }
);

extern_methods!(
    unsafe impl NSEPSImageRep {
        #[method_id(@__retain_semantics Other imageRepWithData:)]
        pub unsafe fn imageRepWithData(epsData: &NSData) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            epsData: &NSData,
        ) -> Option<Id<Self, Shared>>;

        #[method(prepareGState)]
        pub unsafe fn prepareGState(&self);

        #[method_id(@__retain_semantics Other EPSRepresentation)]
        pub unsafe fn EPSRepresentation(&self) -> Id<NSData, Shared>;

        #[method(boundingBox)]
        pub unsafe fn boundingBox(&self) -> NSRect;
    }
);