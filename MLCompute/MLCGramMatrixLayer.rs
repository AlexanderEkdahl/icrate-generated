//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::MLCompute::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCGramMatrixLayer")]
    pub struct MLCGramMatrixLayer;

    #[cfg(feature = "MLCompute_MLCGramMatrixLayer")]
    unsafe impl ClassType for MLCGramMatrixLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCGramMatrixLayer")]
unsafe impl NSObjectProtocol for MLCGramMatrixLayer {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCGramMatrixLayer")]
    unsafe impl MLCGramMatrixLayer {
        #[method(scale)]
        pub unsafe fn scale(&self, ) -> c_float;

        #[method_id(@__retain_semantics Other layerWithScale:)]
        pub unsafe fn layerWithScale(scale: c_float,) -> Id<Self>;

    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCompute_MLCGramMatrixLayer")]
    unsafe impl MLCGramMatrixLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>, ) -> Id<Self>;

    }
);

