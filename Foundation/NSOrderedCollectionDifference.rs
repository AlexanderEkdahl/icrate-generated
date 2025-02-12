//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSOrderedCollectionDifferenceCalculationOptions {
        NSOrderedCollectionDifferenceCalculationOmitInsertedObjects = 1 << 0,
        NSOrderedCollectionDifferenceCalculationOmitRemovedObjects = 1 << 1,
        NSOrderedCollectionDifferenceCalculationInferMoves = 1 << 2,
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
    pub struct NSOrderedCollectionDifference<ObjectType: Message = Object> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut ObjectType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
    unsafe impl<ObjectType: Message> ClassType for NSOrderedCollectionDifference<ObjectType> {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

#[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
unsafe impl<ObjectType: Message> NSFastEnumeration for NSOrderedCollectionDifference<ObjectType> {}

#[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
unsafe impl<ObjectType: Message> NSObjectProtocol for NSOrderedCollectionDifference<ObjectType> {}

extern_methods!(
    #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
    unsafe impl<ObjectType: Message> NSOrderedCollectionDifference<ObjectType> {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSOrderedCollectionChange"
        ))]
        #[method_id(@__retain_semantics Init initWithChanges:)]
        pub unsafe fn initWithChanges(
            this: Option<Allocated<Self>>,
            changes: &NSArray<NSOrderedCollectionChange<ObjectType>>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSIndexSet",
            feature = "Foundation_NSOrderedCollectionChange"
        ))]
        #[method_id(@__retain_semantics Init initWithInsertIndexes:insertedObjects:removeIndexes:removedObjects:additionalChanges:)]
        pub unsafe fn initWithInsertIndexes_insertedObjects_removeIndexes_removedObjects_additionalChanges(
            this: Option<Allocated<Self>>,
            inserts: &NSIndexSet,
            inserted_objects: Option<&NSArray<ObjectType>>,
            removes: &NSIndexSet,
            removed_objects: Option<&NSArray<ObjectType>>,
            changes: &NSArray<NSOrderedCollectionChange<ObjectType>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexSet"))]
        #[method_id(@__retain_semantics Init initWithInsertIndexes:insertedObjects:removeIndexes:removedObjects:)]
        pub unsafe fn initWithInsertIndexes_insertedObjects_removeIndexes_removedObjects(
            this: Option<Allocated<Self>>,
            inserts: &NSIndexSet,
            inserted_objects: Option<&NSArray<ObjectType>>,
            removes: &NSIndexSet,
            removed_objects: Option<&NSArray<ObjectType>>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSOrderedCollectionChange"
        ))]
        #[method_id(@__retain_semantics Other insertions)]
        pub unsafe fn insertions(&self) -> Id<NSArray<NSOrderedCollectionChange<ObjectType>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSOrderedCollectionChange"
        ))]
        #[method_id(@__retain_semantics Other removals)]
        pub unsafe fn removals(&self) -> Id<NSArray<NSOrderedCollectionChange<ObjectType>>>;

        #[method(hasChanges)]
        pub unsafe fn hasChanges(&self) -> bool;

        #[cfg(feature = "Foundation_NSOrderedCollectionChange")]
        #[method_id(@__retain_semantics Other differenceByTransformingChangesWithBlock:)]
        pub unsafe fn differenceByTransformingChangesWithBlock(
            &self,
            block: &Block<
                (NonNull<NSOrderedCollectionChange<ObjectType>>,),
                NonNull<NSOrderedCollectionChange<Object>>,
            >,
        ) -> Id<NSOrderedCollectionDifference<Object>>;

        #[method_id(@__retain_semantics Other inverseDifference)]
        pub unsafe fn inverseDifference(&self) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
    unsafe impl<ObjectType: Message> NSOrderedCollectionDifference<ObjectType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
