//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MLCompute::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCMultiheadAttentionLayer")]
    pub struct MLCMultiheadAttentionLayer;

    #[cfg(feature = "MLCompute_MLCMultiheadAttentionLayer")]
    unsafe impl ClassType for MLCMultiheadAttentionLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCMultiheadAttentionLayer")]
unsafe impl NSObjectProtocol for MLCMultiheadAttentionLayer {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCMultiheadAttentionLayer")]
    unsafe impl MLCMultiheadAttentionLayer {
        #[cfg(feature = "MLCompute_MLCMultiheadAttentionDescriptor")]
        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor(&self) -> Id<MLCMultiheadAttentionDescriptor>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MLCompute_MLCTensor"))]
        #[method_id(@__retain_semantics Other weights)]
        pub unsafe fn weights(&self) -> Id<NSArray<MLCTensor>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MLCompute_MLCTensor"))]
        #[method_id(@__retain_semantics Other biases)]
        pub unsafe fn biases(&self) -> Option<Id<NSArray<MLCTensor>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MLCompute_MLCTensor"))]
        #[method_id(@__retain_semantics Other attentionBiases)]
        pub unsafe fn attentionBiases(&self) -> Option<Id<NSArray<MLCTensor>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MLCompute_MLCTensorParameter"
        ))]
        #[method_id(@__retain_semantics Other weightsParameters)]
        pub unsafe fn weightsParameters(&self) -> Id<NSArray<MLCTensorParameter>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MLCompute_MLCTensorParameter"
        ))]
        #[method_id(@__retain_semantics Other biasesParameters)]
        pub unsafe fn biasesParameters(&self) -> Option<Id<NSArray<MLCTensorParameter>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MLCompute_MLCMultiheadAttentionDescriptor",
            feature = "MLCompute_MLCTensor"
        ))]
        #[method_id(@__retain_semantics Other layerWithDescriptor:weights:biases:attentionBiases:)]
        pub unsafe fn layerWithDescriptor_weights_biases_attentionBiases(
            descriptor: &MLCMultiheadAttentionDescriptor,
            weights: &NSArray<MLCTensor>,
            biases: Option<&NSArray<MLCTensor>>,
            attention_biases: Option<&NSArray<MLCTensor>>,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCompute_MLCMultiheadAttentionLayer")]
    unsafe impl MLCMultiheadAttentionLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);