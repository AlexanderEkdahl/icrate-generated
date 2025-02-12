//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSFetchedResultsController")]
    pub struct NSFetchedResultsController<ResultType: Message = Object> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut ResultType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "CoreData_NSFetchedResultsController")]
    unsafe impl<ResultType: Message> ClassType for NSFetchedResultsController<ResultType> {
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

#[cfg(feature = "CoreData_NSFetchedResultsController")]
unsafe impl<ResultType: Message> NSObjectProtocol for NSFetchedResultsController<ResultType> {}

extern_methods!(
    #[cfg(feature = "CoreData_NSFetchedResultsController")]
    unsafe impl<ResultType: Message> NSFetchedResultsController<ResultType> {
        #[cfg(all(
            feature = "CoreData_NSFetchRequest",
            feature = "CoreData_NSManagedObjectContext",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithFetchRequest:managedObjectContext:sectionNameKeyPath:cacheName:)]
        pub unsafe fn initWithFetchRequest_managedObjectContext_sectionNameKeyPath_cacheName(
            this: Option<Allocated<Self>>,
            fetch_request: &NSFetchRequest<ResultType>,
            context: &NSManagedObjectContext,
            section_name_key_path: Option<&NSString>,
            name: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(performFetch:_)]
        pub unsafe fn performFetch(&self) -> Result<(), Id<NSError>>;

        #[cfg(feature = "CoreData_NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest(&self) -> Id<NSFetchRequest<ResultType>>;

        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Id<NSManagedObjectContext>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sectionNameKeyPath)]
        pub unsafe fn sectionNameKeyPath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other cacheName)]
        pub unsafe fn cacheName(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSFetchedResultsControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSFetchedResultsControllerDelegate>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(deleteCacheWithName:)]
        pub unsafe fn deleteCacheWithName(name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other fetchedObjects)]
        pub unsafe fn fetchedObjects(&self) -> Option<Id<NSArray<ResultType>>>;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other objectAtIndexPath:)]
        pub unsafe fn objectAtIndexPath(&self, index_path: &NSIndexPath) -> Id<ResultType>;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other indexPathForObject:)]
        pub unsafe fn indexPathForObject(&self, object: &ResultType) -> Option<Id<NSIndexPath>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sectionIndexTitleForSectionName:)]
        pub unsafe fn sectionIndexTitleForSectionName(
            &self,
            section_name: &NSString,
        ) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other sectionIndexTitles)]
        pub unsafe fn sectionIndexTitles(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other sections)]
        pub unsafe fn sections(
            &self,
        ) -> Option<Id<NSArray<ProtocolObject<dyn NSFetchedResultsSectionInfo>>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(sectionForSectionIndexTitle:atIndex:)]
        pub unsafe fn sectionForSectionIndexTitle_atIndex(
            &self,
            title: &NSString,
            section_index: NSInteger,
        ) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSFetchedResultsController")]
    unsafe impl<ResultType: Message> NSFetchedResultsController<ResultType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSFetchedResultsSectionInfo {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other indexTitle)]
        unsafe fn indexTitle(&self) -> Option<Id<NSString>>;

        #[method(numberOfObjects)]
        unsafe fn numberOfObjects(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other objects)]
        unsafe fn objects(&self) -> Option<Id<NSArray>>;
    }

    unsafe impl ProtocolType for dyn NSFetchedResultsSectionInfo {}
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSFetchedResultsChangeType {
        NSFetchedResultsChangeInsert = 1,
        NSFetchedResultsChangeDelete = 2,
        NSFetchedResultsChangeMove = 3,
        NSFetchedResultsChangeUpdate = 4,
    }
);

extern_protocol!(
    pub unsafe trait NSFetchedResultsControllerDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "CoreData_NSFetchedResultsController",
            feature = "CoreData_NSManagedObjectID",
            feature = "Foundation_NSOrderedCollectionDifference"
        ))]
        #[optional]
        #[method(controller:didChangeContentWithDifference:)]
        unsafe fn controller_didChangeContentWithDifference(
            &self,
            controller: &NSFetchedResultsController,
            diff: &NSOrderedCollectionDifference<NSManagedObjectID>,
        );

        #[cfg(all(
            feature = "CoreData_NSFetchedResultsController",
            feature = "Foundation_NSIndexPath"
        ))]
        #[optional]
        #[method(controller:didChangeObject:atIndexPath:forChangeType:newIndexPath:)]
        unsafe fn controller_didChangeObject_atIndexPath_forChangeType_newIndexPath(
            &self,
            controller: &NSFetchedResultsController,
            an_object: &Object,
            index_path: Option<&NSIndexPath>,
            r#type: NSFetchedResultsChangeType,
            new_index_path: Option<&NSIndexPath>,
        );

        #[cfg(feature = "CoreData_NSFetchedResultsController")]
        #[optional]
        #[method(controller:didChangeSection:atIndex:forChangeType:)]
        unsafe fn controller_didChangeSection_atIndex_forChangeType(
            &self,
            controller: &NSFetchedResultsController,
            section_info: &ProtocolObject<dyn NSFetchedResultsSectionInfo>,
            section_index: NSUInteger,
            r#type: NSFetchedResultsChangeType,
        );

        #[cfg(feature = "CoreData_NSFetchedResultsController")]
        #[optional]
        #[method(controllerWillChangeContent:)]
        unsafe fn controllerWillChangeContent(&self, controller: &NSFetchedResultsController);

        #[cfg(feature = "CoreData_NSFetchedResultsController")]
        #[optional]
        #[method(controllerDidChangeContent:)]
        unsafe fn controllerDidChangeContent(&self, controller: &NSFetchedResultsController);

        #[cfg(all(
            feature = "CoreData_NSFetchedResultsController",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other controller:sectionIndexTitleForSectionName:)]
        unsafe fn controller_sectionIndexTitleForSectionName(
            &self,
            controller: &NSFetchedResultsController,
            section_name: &NSString,
        ) -> Option<Id<NSString>>;
    }

    unsafe impl ProtocolType for dyn NSFetchedResultsControllerDelegate {}
);
