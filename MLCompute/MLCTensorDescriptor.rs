//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MLCompute::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCTensorDescriptor")]
    pub struct MLCTensorDescriptor;

    #[cfg(feature = "MLCompute_MLCTensorDescriptor")]
    unsafe impl ClassType for MLCTensorDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCTensorDescriptor")]
unsafe impl NSCopying for MLCTensorDescriptor {}

#[cfg(feature = "MLCompute_MLCTensorDescriptor")]
unsafe impl NSObjectProtocol for MLCTensorDescriptor {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCTensorDescriptor")]
    unsafe impl MLCTensorDescriptor {
        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MLCDataType;

        #[method(dimensionCount)]
        pub unsafe fn dimensionCount(&self) -> NSUInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other shape)]
        pub unsafe fn shape(&self) -> Id<NSArray<NSNumber>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other stride)]
        pub unsafe fn stride(&self) -> Id<NSArray<NSNumber>>;

        #[method(tensorAllocationSizeInBytes)]
        pub unsafe fn tensorAllocationSizeInBytes(&self) -> NSUInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other sequenceLengths)]
        pub unsafe fn sequenceLengths(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[method(sortedSequences)]
        pub unsafe fn sortedSequences(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other batchSizePerSequenceStep)]
        pub unsafe fn batchSizePerSequenceStep(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method(maxTensorDimensions)]
        pub unsafe fn maxTensorDimensions() -> NSUInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other descriptorWithShape:dataType:)]
        pub unsafe fn descriptorWithShape_dataType(
            shape: &NSArray<NSNumber>,
            data_type: MLCDataType,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other descriptorWithShape:sequenceLengths:sortedSequences:dataType:)]
        pub unsafe fn descriptorWithShape_sequenceLengths_sortedSequences_dataType(
            shape: &NSArray<NSNumber>,
            sequence_lengths: &NSArray<NSNumber>,
            sorted_sequences: bool,
            data_type: MLCDataType,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other descriptorWithWidth:height:featureChannelCount:batchSize:)]
        pub unsafe fn descriptorWithWidth_height_featureChannelCount_batchSize(
            width: NSUInteger,
            height: NSUInteger,
            feature_channels: NSUInteger,
            batch_size: NSUInteger,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other descriptorWithWidth:height:featureChannelCount:batchSize:dataType:)]
        pub unsafe fn descriptorWithWidth_height_featureChannelCount_batchSize_dataType(
            width: NSUInteger,
            height: NSUInteger,
            feature_channel_count: NSUInteger,
            batch_size: NSUInteger,
            data_type: MLCDataType,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other convolutionWeightsDescriptorWithWidth:height:inputFeatureChannelCount:outputFeatureChannelCount:dataType:)]
        pub unsafe fn convolutionWeightsDescriptorWithWidth_height_inputFeatureChannelCount_outputFeatureChannelCount_dataType(
            width: NSUInteger,
            height: NSUInteger,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            data_type: MLCDataType,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other convolutionWeightsDescriptorWithInputFeatureChannelCount:outputFeatureChannelCount:dataType:)]
        pub unsafe fn convolutionWeightsDescriptorWithInputFeatureChannelCount_outputFeatureChannelCount_dataType(
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            data_type: MLCDataType,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other convolutionBiasesDescriptorWithFeatureChannelCount:dataType:)]
        pub unsafe fn convolutionBiasesDescriptorWithFeatureChannelCount_dataType(
            feature_channel_count: NSUInteger,
            data_type: MLCDataType,
        ) -> Option<Id<Self>>;
    }
);