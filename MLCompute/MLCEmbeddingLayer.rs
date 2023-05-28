//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::MLCompute::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCEmbeddingLayer")]
    pub struct MLCEmbeddingLayer;

    #[cfg(feature = "MLCompute_MLCEmbeddingLayer")]
    unsafe impl ClassType for MLCEmbeddingLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCEmbeddingLayer")]
unsafe impl NSObjectProtocol for MLCEmbeddingLayer {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCEmbeddingLayer")]
    unsafe impl MLCEmbeddingLayer {
        #[cfg(feature = "MLCompute_MLCEmbeddingDescriptor")]
        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor(&self, ) -> Id<MLCEmbeddingDescriptor>;

        #[cfg(feature = "MLCompute_MLCTensor")]
        #[method_id(@__retain_semantics Other weights)]
        pub unsafe fn weights(&self, ) -> Id<MLCTensor>;

        #[cfg(feature = "MLCompute_MLCTensorParameter")]
        #[method_id(@__retain_semantics Other weightsParameter)]
        pub unsafe fn weightsParameter(&self, ) -> Id<MLCTensorParameter>;

        #[cfg(all(feature = "MLCompute_MLCEmbeddingDescriptor",feature = "MLCompute_MLCTensor"))]
        #[method_id(@__retain_semantics Other layerWithDescriptor:weights:)]
        pub unsafe fn layerWithDescriptor_weights(descriptor: &MLCEmbeddingDescriptor,weights: &MLCTensor,) -> Id<Self>;

    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCompute_MLCEmbeddingLayer")]
    unsafe impl MLCEmbeddingLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>, ) -> Id<Self>;

    }
);

