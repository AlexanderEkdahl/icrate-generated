//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSThread")]
    pub struct NSThread;

    #[cfg(feature = "Foundation_NSThread")]
    unsafe impl ClassType for NSThread {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSThread")]
unsafe impl NSObjectProtocol for NSThread {}

extern_methods!(
    #[cfg(feature = "Foundation_NSThread")]
    unsafe impl NSThread {
        #[method_id(@__retain_semantics Other currentThread)]
        pub fn currentThread() -> Id<NSThread>;

        #[method(detachNewThreadWithBlock:)]
        pub unsafe fn detachNewThreadWithBlock(block: &Block<(), ()>);

        #[method(detachNewThreadSelector:toTarget:withObject:)]
        pub unsafe fn detachNewThreadSelector_toTarget_withObject(
            selector: Sel,
            target: &Object,
            argument: Option<&Object>,
        );

        #[method(isMultiThreaded)]
        pub fn isMultiThreaded() -> bool;

        #[cfg(feature = "Foundation_NSMutableDictionary")]
        #[method_id(@__retain_semantics Other threadDictionary)]
        pub unsafe fn threadDictionary(&self) -> Id<NSMutableDictionary>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(sleepUntilDate:)]
        pub unsafe fn sleepUntilDate(date: &NSDate);

        #[method(sleepForTimeInterval:)]
        pub unsafe fn sleepForTimeInterval(ti: NSTimeInterval);

        #[method(exit)]
        pub unsafe fn exit();

        #[method(threadPriority)]
        pub unsafe fn threadPriority_class() -> c_double;

        #[method(setThreadPriority:)]
        pub unsafe fn setThreadPriority_class(p: c_double) -> bool;

        #[method(threadPriority)]
        pub unsafe fn threadPriority(&self) -> c_double;

        #[method(setThreadPriority:)]
        pub unsafe fn setThreadPriority(&self, thread_priority: c_double);

        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;

        #[method(setQualityOfService:)]
        pub unsafe fn setQualityOfService(&self, quality_of_service: NSQualityOfService);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other callStackReturnAddresses)]
        pub unsafe fn callStackReturnAddresses() -> Id<NSArray<NSNumber>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other callStackSymbols)]
        pub unsafe fn callStackSymbols() -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(stackSize)]
        pub unsafe fn stackSize(&self) -> NSUInteger;

        #[method(setStackSize:)]
        pub unsafe fn setStackSize(&self, stack_size: NSUInteger);

        #[method(isMainThread)]
        pub fn isMainThread(&self) -> bool;

        #[method(isMainThread)]
        pub fn isMainThread_class() -> bool;

        #[method_id(@__retain_semantics Other mainThread)]
        pub fn mainThread() -> Id<NSThread>;

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithTarget:selector:object:)]
        pub unsafe fn initWithTarget_selector_object(
            this: Option<Allocated<Self>>,
            target: &Object,
            selector: Sel,
            argument: Option<&Object>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithBlock:)]
        pub unsafe fn initWithBlock(
            this: Option<Allocated<Self>>,
            block: &Block<(), ()>,
        ) -> Id<Self>;

        #[method(isExecuting)]
        pub unsafe fn isExecuting(&self) -> bool;

        #[method(isFinished)]
        pub unsafe fn isFinished(&self) -> bool;

        #[method(isCancelled)]
        pub unsafe fn isCancelled(&self) -> bool;

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(start)]
        pub unsafe fn start(&self);

        #[method(main)]
        pub unsafe fn main(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSThread")]
    unsafe impl NSThread {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);
#[cfg(feature = "Foundation_NSThread")]
impl DefaultId for NSThread {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

extern_static!(NSWillBecomeMultiThreadedNotification: &'static NSNotificationName);

extern_static!(NSDidBecomeSingleThreadedNotification: &'static NSNotificationName);

extern_static!(NSThreadWillExitNotification: &'static NSNotificationName);
