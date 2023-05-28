//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MLCompute::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCGroupNormalizationLayer")]
    pub struct MLCGroupNormalizationLayer;

    #[cfg(feature = "MLCompute_MLCGroupNormalizationLayer")]
    unsafe impl ClassType for MLCGroupNormalizationLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCGroupNormalizationLayer")]
unsafe impl NSObjectProtocol for MLCGroupNormalizationLayer {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCGroupNormalizationLayer")]
    unsafe impl MLCGroupNormalizationLayer {
        #[method(featureChannelCount)]
        pub unsafe fn featureChannelCount(&self) -> NSUInteger;

        #[method(groupCount)]
        pub unsafe fn groupCount(&self) -> NSUInteger;

        #[cfg(feature = "MLCompute_MLCTensor")]
        #[method_id(@__retain_semantics Other beta)]
        pub unsafe fn beta(&self) -> Option<Id<MLCTensor>>;

        #[cfg(feature = "MLCompute_MLCTensor")]
        #[method_id(@__retain_semantics Other gamma)]
        pub unsafe fn gamma(&self) -> Option<Id<MLCTensor>>;

        #[cfg(feature = "MLCompute_MLCTensorParameter")]
        #[method_id(@__retain_semantics Other betaParameter)]
        pub unsafe fn betaParameter(&self) -> Option<Id<MLCTensorParameter>>;

        #[cfg(feature = "MLCompute_MLCTensorParameter")]
        #[method_id(@__retain_semantics Other gammaParameter)]
        pub unsafe fn gammaParameter(&self) -> Option<Id<MLCTensorParameter>>;

        #[method(varianceEpsilon)]
        pub unsafe fn varianceEpsilon(&self) -> c_float;

        #[cfg(feature = "MLCompute_MLCTensor")]
        #[method_id(@__retain_semantics Other layerWithFeatureChannelCount:groupCount:beta:gamma:varianceEpsilon:)]
        pub unsafe fn layerWithFeatureChannelCount_groupCount_beta_gamma_varianceEpsilon(
            feature_channel_count: NSUInteger,
            group_count: NSUInteger,
            beta: Option<&MLCTensor>,
            gamma: Option<&MLCTensor>,
            variance_epsilon: c_float,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCompute_MLCGroupNormalizationLayer")]
    unsafe impl MLCGroupNormalizationLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);