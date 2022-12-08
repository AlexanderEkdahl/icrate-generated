//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSDateIntervalFormatterStyle {
        NSDateIntervalFormatterNoStyle = 0,
        NSDateIntervalFormatterShortStyle = 1,
        NSDateIntervalFormatterMediumStyle = 2,
        NSDateIntervalFormatterLongStyle = 3,
        NSDateIntervalFormatterFullStyle = 4,
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSDateIntervalFormatter;

    unsafe impl ClassType for NSDateIntervalFormatter {
        type Super = NSFormatter;
    }
);

extern_methods!(
    unsafe impl NSDateIntervalFormatter {
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale, Shared>;

        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Id<NSCalendar, Shared>;

        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared>;

        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, timeZone: Option<&NSTimeZone>);

        #[method_id(@__retain_semantics Other dateTemplate)]
        pub unsafe fn dateTemplate(&self) -> Id<NSString, Shared>;

        #[method(setDateTemplate:)]
        pub unsafe fn setDateTemplate(&self, dateTemplate: Option<&NSString>);

        #[method(dateStyle)]
        pub unsafe fn dateStyle(&self) -> NSDateIntervalFormatterStyle;

        #[method(setDateStyle:)]
        pub unsafe fn setDateStyle(&self, dateStyle: NSDateIntervalFormatterStyle);

        #[method(timeStyle)]
        pub unsafe fn timeStyle(&self) -> NSDateIntervalFormatterStyle;

        #[method(setTimeStyle:)]
        pub unsafe fn setTimeStyle(&self, timeStyle: NSDateIntervalFormatterStyle);

        #[method_id(@__retain_semantics Other stringFromDate:toDate:)]
        pub unsafe fn stringFromDate_toDate(
            &self,
            fromDate: &NSDate,
            toDate: &NSDate,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringFromDateInterval:)]
        pub unsafe fn stringFromDateInterval(
            &self,
            dateInterval: &NSDateInterval,
        ) -> Option<Id<NSString, Shared>>;
    }
);