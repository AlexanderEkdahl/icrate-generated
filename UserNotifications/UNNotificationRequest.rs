//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNNotificationRequest")]
    pub struct UNNotificationRequest;

    #[cfg(feature = "UserNotifications_UNNotificationRequest")]
    unsafe impl ClassType for UNNotificationRequest {
        type Super = NSObject;
    }
);

#[cfg(feature = "UserNotifications_UNNotificationRequest")]
unsafe impl NSCoding for UNNotificationRequest {}

#[cfg(feature = "UserNotifications_UNNotificationRequest")]
unsafe impl NSObjectProtocol for UNNotificationRequest {}

#[cfg(feature = "UserNotifications_UNNotificationRequest")]
unsafe impl NSSecureCoding for UNNotificationRequest {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNNotificationRequest")]
    unsafe impl UNNotificationRequest {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "UserNotifications_UNNotificationContent")]
        #[method_id(@__retain_semantics Other content)]
        pub unsafe fn content(&self) -> Id<UNNotificationContent, Shared>;

        #[cfg(feature = "UserNotifications_UNNotificationTrigger")]
        #[method_id(@__retain_semantics Other trigger)]
        pub unsafe fn trigger(&self) -> Option<Id<UNNotificationTrigger, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "UserNotifications_UNNotificationContent",
            feature = "UserNotifications_UNNotificationTrigger"
        ))]
        #[method_id(@__retain_semantics Other requestWithIdentifier:content:trigger:)]
        pub unsafe fn requestWithIdentifier_content_trigger(
            identifier: &NSString,
            content: &UNNotificationContent,
            trigger: Option<&UNNotificationTrigger>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);