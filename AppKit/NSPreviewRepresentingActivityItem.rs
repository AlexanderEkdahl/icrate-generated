//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSPreviewRepresentableActivityItem: NSObjectProtocol {
        #[method_id(@__retain_semantics Other item)]
        unsafe fn item(&self) -> Id<Object>;

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method_id(@__retain_semantics Other title)]
        unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSItemProvider")]
        #[optional]
        #[method_id(@__retain_semantics Other imageProvider)]
        unsafe fn imageProvider(&self) -> Option<Id<NSItemProvider>>;

        #[cfg(feature = "Foundation_NSItemProvider")]
        #[optional]
        #[method_id(@__retain_semantics Other iconProvider)]
        unsafe fn iconProvider(&self) -> Option<Id<NSItemProvider>>;
    }

    unsafe impl ProtocolType for dyn NSPreviewRepresentableActivityItem {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPreviewRepresentingActivityItem")]
    pub struct NSPreviewRepresentingActivityItem;

    #[cfg(feature = "AppKit_NSPreviewRepresentingActivityItem")]
    unsafe impl ClassType for NSPreviewRepresentingActivityItem {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSPreviewRepresentingActivityItem")]
unsafe impl NSObjectProtocol for NSPreviewRepresentingActivityItem {}

#[cfg(feature = "AppKit_NSPreviewRepresentingActivityItem")]
unsafe impl NSPreviewRepresentableActivityItem for NSPreviewRepresentingActivityItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPreviewRepresentingActivityItem")]
    unsafe impl NSPreviewRepresentingActivityItem {
        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithItem:title:image:icon:)]
        pub unsafe fn initWithItem_title_image_icon(
            this: Option<Allocated<Self>>,
            item: &Object,
            title: Option<&NSString>,
            image: Option<&NSImage>,
            icon: Option<&NSImage>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSItemProvider", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithItem:title:imageProvider:iconProvider:)]
        pub unsafe fn initWithItem_title_imageProvider_iconProvider(
            this: Option<Allocated<Self>>,
            item: &Object,
            title: Option<&NSString>,
            image_provider: Option<&NSItemProvider>,
            icon_provider: Option<&NSItemProvider>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
