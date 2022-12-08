//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct NSColorPickingDefault;

    unsafe impl NSColorPickingDefault {
        #[method_id(@__retain_semantics Init initWithPickerMask:colorPanel:)]
        pub unsafe fn initWithPickerMask_colorPanel(
            this: Option<Allocated<Self>>,
            mask: NSUInteger,
            owningColorPanel: &NSColorPanel,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other provideNewButtonImage)]
        pub unsafe fn provideNewButtonImage(&self) -> Id<NSImage, Shared>;

        #[method(insertNewButtonImage:in:)]
        pub unsafe fn insertNewButtonImage_in(
            &self,
            newButtonImage: &NSImage,
            buttonCell: &NSButtonCell,
        );

        #[method(viewSizeChanged:)]
        pub unsafe fn viewSizeChanged(&self, sender: Option<&Object>);

        #[method(alphaControlAddedOrRemoved:)]
        pub unsafe fn alphaControlAddedOrRemoved(&self, sender: Option<&Object>);

        #[method(attachColorList:)]
        pub unsafe fn attachColorList(&self, colorList: &NSColorList);

        #[method(detachColorList:)]
        pub unsafe fn detachColorList(&self, colorList: &NSColorList);

        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: NSColorPanelMode);

        #[method_id(@__retain_semantics Other buttonToolTip)]
        pub unsafe fn buttonToolTip(&self) -> Id<NSString, Shared>;

        #[method(minContentSize)]
        pub unsafe fn minContentSize(&self) -> NSSize;
    }
);

extern_protocol!(
    pub struct NSColorPickingCustom;

    unsafe impl NSColorPickingCustom {
        #[method(supportsMode:)]
        pub unsafe fn supportsMode(&self, mode: NSColorPanelMode) -> bool;

        #[method(currentMode)]
        pub unsafe fn currentMode(&self) -> NSColorPanelMode;

        #[method_id(@__retain_semantics Other provideNewView:)]
        pub unsafe fn provideNewView(&self, initialRequest: bool) -> Id<NSView, Shared>;

        #[method(setColor:)]
        pub unsafe fn setColor(&self, newColor: &NSColor);
    }
);