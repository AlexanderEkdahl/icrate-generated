//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_protocol!(
    pub unsafe trait MEMessageEncoder: NSObjectProtocol {
        #[cfg(all(
            feature = "MailKit_MEComposeContext",
            feature = "MailKit_MEMessage",
            feature = "MailKit_MEOutgoingMessageEncodingStatus"
        ))]
        #[method(getEncodingStatusForMessage:composeContext:completionHandler:)]
        unsafe fn getEncodingStatusForMessage_composeContext_completionHandler(
            &self,
            message: &MEMessage,
            compose_context: &MEComposeContext,
            completion_handler: &Block<(NonNull<MEOutgoingMessageEncodingStatus>,), ()>,
        );

        #[cfg(all(
            feature = "MailKit_MEComposeContext",
            feature = "MailKit_MEMessage",
            feature = "MailKit_MEMessageEncodingResult"
        ))]
        #[method(encodeMessage:composeContext:completionHandler:)]
        unsafe fn encodeMessage_composeContext_completionHandler(
            &self,
            message: &MEMessage,
            compose_context: &MEComposeContext,
            completion_handler: &Block<(NonNull<MEMessageEncodingResult>,), ()>,
        );
    }

    unsafe impl ProtocolType for dyn MEMessageEncoder {}
);