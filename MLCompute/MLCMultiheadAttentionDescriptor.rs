//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MLCompute::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCMultiheadAttentionDescriptor")]
    pub struct MLCMultiheadAttentionDescriptor;

    #[cfg(feature = "MLCompute_MLCMultiheadAttentionDescriptor")]
    unsafe impl ClassType for MLCMultiheadAttentionDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCMultiheadAttentionDescriptor")]
unsafe impl NSCopying for MLCMultiheadAttentionDescriptor {}

#[cfg(feature = "MLCompute_MLCMultiheadAttentionDescriptor")]
unsafe impl NSObjectProtocol for MLCMultiheadAttentionDescriptor {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCMultiheadAttentionDescriptor")]
    unsafe impl MLCMultiheadAttentionDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method(modelDimension)]
        pub unsafe fn modelDimension(&self) -> NSUInteger;

        #[method(keyDimension)]
        pub unsafe fn keyDimension(&self) -> NSUInteger;

        #[method(valueDimension)]
        pub unsafe fn valueDimension(&self) -> NSUInteger;

        #[method(headCount)]
        pub unsafe fn headCount(&self) -> NSUInteger;

        #[method(dropout)]
        pub unsafe fn dropout(&self) -> c_float;

        #[method(hasBiases)]
        pub unsafe fn hasBiases(&self) -> bool;

        #[method(hasAttentionBiases)]
        pub unsafe fn hasAttentionBiases(&self) -> bool;

        #[method(addsZeroAttention)]
        pub unsafe fn addsZeroAttention(&self) -> bool;

        #[method_id(@__retain_semantics Other descriptorWithModelDimension:keyDimension:valueDimension:headCount:dropout:hasBiases:hasAttentionBiases:addsZeroAttention:)]
        pub unsafe fn descriptorWithModelDimension_keyDimension_valueDimension_headCount_dropout_hasBiases_hasAttentionBiases_addsZeroAttention(
            model_dimension: NSUInteger,
            key_dimension: NSUInteger,
            value_dimension: NSUInteger,
            head_count: NSUInteger,
            dropout: c_float,
            has_biases: bool,
            has_attention_biases: bool,
            adds_zero_attention: bool,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other descriptorWithModelDimension:headCount:)]
        pub unsafe fn descriptorWithModelDimension_headCount(
            model_dimension: NSUInteger,
            head_count: NSUInteger,
        ) -> Id<Self>;
    }
);