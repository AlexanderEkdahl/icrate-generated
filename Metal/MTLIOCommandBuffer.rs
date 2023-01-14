//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLIOStatus {
        MTLIOStatusPending = 0,
        MTLIOStatusCancelled = 1,
        MTLIOStatusError = 2,
        MTLIOStatusComplete = 3,
    }
);

pub type MTLIOCommandBufferHandler = *mut Block<(NonNull<MTLIOCommandBuffer>,), ()>;

extern_protocol!(
    pub struct MTLIOCommandBuffer;

    unsafe impl ProtocolType for MTLIOCommandBuffer {
        #[method(addCompletedHandler:)]
        pub unsafe fn addCompletedHandler(&self, block: MTLIOCommandBufferHandler);

        #[method(loadBytes:size:sourceHandle:sourceHandleOffset:)]
        pub unsafe fn loadBytes_size_sourceHandle_sourceHandleOffset(
            &self,
            pointer: NonNull<c_void>,
            size: NSUInteger,
            sourceHandle: &MTLIOFileHandle,
            sourceHandleOffset: NSUInteger,
        );

        #[method(loadBuffer:offset:size:sourceHandle:sourceHandleOffset:)]
        pub unsafe fn loadBuffer_offset_size_sourceHandle_sourceHandleOffset(
            &self,
            buffer: &MTLBuffer,
            offset: NSUInteger,
            size: NSUInteger,
            sourceHandle: &MTLIOFileHandle,
            sourceHandleOffset: NSUInteger,
        );

        #[method(loadTexture:slice:level:size:sourceBytesPerRow:sourceBytesPerImage:destinationOrigin:sourceHandle:sourceHandleOffset:)]
        pub unsafe fn loadTexture_slice_level_size_sourceBytesPerRow_sourceBytesPerImage_destinationOrigin_sourceHandle_sourceHandleOffset(
            &self,
            texture: &MTLTexture,
            slice: NSUInteger,
            level: NSUInteger,
            size: MTLSize,
            sourceBytesPerRow: NSUInteger,
            sourceBytesPerImage: NSUInteger,
            destinationOrigin: MTLOrigin,
            sourceHandle: &MTLIOFileHandle,
            sourceHandleOffset: NSUInteger,
        );

        #[method(copyStatusToBuffer:offset:)]
        pub unsafe fn copyStatusToBuffer_offset(&self, buffer: &MTLBuffer, offset: NSUInteger);

        #[method(commit)]
        pub unsafe fn commit(&self);

        #[method(waitUntilCompleted)]
        pub unsafe fn waitUntilCompleted(&self);

        #[method(tryCancel)]
        pub unsafe fn tryCancel(&self);

        #[method(addBarrier)]
        pub unsafe fn addBarrier(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(pushDebugGroup:)]
        pub unsafe fn pushDebugGroup(&self, string: &NSString);

        #[method(popDebugGroup)]
        pub unsafe fn popDebugGroup(&self);

        #[method(enqueue)]
        pub unsafe fn enqueue(&self);

        #[method(waitForEvent:value:)]
        pub unsafe fn waitForEvent_value(&self, event: &MTLSharedEvent, value: u64);

        #[method(signalEvent:value:)]
        pub unsafe fn signalEvent_value(&self, event: &MTLSharedEvent, value: u64);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: Option<&NSString>);

        #[method(status)]
        pub unsafe fn status(&self) -> MTLIOStatus;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Id<NSError, Shared>>;
    }
);