//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

pub type ASAuthorizationScope = NSString;

extern_static!(ASAuthorizationScopeFullName: &'static ASAuthorizationScope);

extern_static!(ASAuthorizationScopeEmail: &'static ASAuthorizationScope);

extern_class!(
    #[derive(Debug)]
    pub struct ASAuthorization;

    unsafe impl ClassType for ASAuthorization {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl ASAuthorization {
        #[method_id(@__retain_semantics Other provider)]
        pub unsafe fn provider(&self) -> Id<ASAuthorizationProvider, Shared>;

        #[method_id(@__retain_semantics Other credential)]
        pub unsafe fn credential(&self) -> Id<ASAuthorizationCredential, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);