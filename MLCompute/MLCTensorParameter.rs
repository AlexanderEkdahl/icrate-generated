//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MLCompute::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCTensorParameter")]
    pub struct MLCTensorParameter;

    #[cfg(feature = "MLCompute_MLCTensorParameter")]
    unsafe impl ClassType for MLCTensorParameter {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCTensorParameter")]
unsafe impl NSObjectProtocol for MLCTensorParameter {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCTensorParameter")]
    unsafe impl MLCTensorParameter {
        #[cfg(feature = "MLCompute_MLCTensor")]
        #[method_id(@__retain_semantics Other tensor)]
        pub unsafe fn tensor(&self) -> Id<MLCTensor>;

        #[method(isUpdatable)]
        pub unsafe fn isUpdatable(&self) -> bool;

        #[method(setIsUpdatable:)]
        pub unsafe fn setIsUpdatable(&self, is_updatable: bool);

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "MLCompute_MLCTensor")]
        #[method_id(@__retain_semantics Other parameterWithTensor:)]
        pub unsafe fn parameterWithTensor(tensor: &MLCTensor) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MLCompute_MLCTensor",
            feature = "MLCompute_MLCTensorData"
        ))]
        #[method_id(@__retain_semantics Other parameterWithTensor:optimizerData:)]
        pub unsafe fn parameterWithTensor_optimizerData(
            tensor: &MLCTensor,
            optimizer_data: Option<&NSArray<MLCTensorData>>,
        ) -> Id<Self>;
    }
);