//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::MLCompute::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCDevice")]
    pub struct MLCDevice;

    #[cfg(feature = "MLCompute_MLCDevice")]
    unsafe impl ClassType for MLCDevice {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCDevice")]
unsafe impl NSCopying for MLCDevice {}

#[cfg(feature = "MLCompute_MLCDevice")]
unsafe impl NSObjectProtocol for MLCDevice {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCDevice")]
    unsafe impl MLCDevice {
        #[method(type)]
        pub unsafe fn r#type(&self, ) -> MLCDeviceType;

        #[method(actualDeviceType)]
        pub unsafe fn actualDeviceType(&self, ) -> MLCDeviceType;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other gpuDevices)]
        pub unsafe fn gpuDevices(&self, ) -> Id<NSArray<ProtocolObject<dyn MTLDevice>,>>;

        #[method_id(@__retain_semantics Other cpuDevice)]
        pub unsafe fn cpuDevice() -> Id<Self>;

        #[method_id(@__retain_semantics Other gpuDevice)]
        pub unsafe fn gpuDevice() -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other aneDevice)]
        pub unsafe fn aneDevice() -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other deviceWithType:)]
        pub unsafe fn deviceWithType(r#type: MLCDeviceType,) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other deviceWithType:selectsMultipleComputeDevices:)]
        pub unsafe fn deviceWithType_selectsMultipleComputeDevices(r#type: MLCDeviceType,selects_multiple_compute_devices: bool,) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other deviceWithGPUDevices:)]
        pub unsafe fn deviceWithGPUDevices(gpus: &NSArray<ProtocolObject<dyn MTLDevice>,>,) -> Option<Id<Self>>;

    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MLCompute_MLCDevice")]
    unsafe impl MLCDevice {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>, ) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

    }
);

