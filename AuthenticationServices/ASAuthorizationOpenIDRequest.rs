//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type ASAuthorizationOpenIDOperation = NSString;
);

extern_static!(ASAuthorizationOperationImplicit: &'static ASAuthorizationOpenIDOperation);

extern_static!(ASAuthorizationOperationLogin: &'static ASAuthorizationOpenIDOperation);

extern_static!(ASAuthorizationOperationRefresh: &'static ASAuthorizationOpenIDOperation);

extern_static!(ASAuthorizationOperationLogout: &'static ASAuthorizationOpenIDOperation);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationOpenIDRequest")]
    pub struct ASAuthorizationOpenIDRequest;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationOpenIDRequest")]
    unsafe impl ClassType for ASAuthorizationOpenIDRequest {
        #[inherits(NSObject)]
        type Super = ASAuthorizationRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationOpenIDRequest")]
unsafe impl NSCoding for ASAuthorizationOpenIDRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationOpenIDRequest")]
unsafe impl NSCopying for ASAuthorizationOpenIDRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationOpenIDRequest")]
unsafe impl NSObjectProtocol for ASAuthorizationOpenIDRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationOpenIDRequest")]
unsafe impl NSSecureCoding for ASAuthorizationOpenIDRequest {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationOpenIDRequest")]
    unsafe impl ASAuthorizationOpenIDRequest {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other requestedScopes)]
        pub unsafe fn requestedScopes(&self) -> Option<Id<NSArray<ASAuthorizationScope>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setRequestedScopes:)]
        pub unsafe fn setRequestedScopes(
            &self,
            requested_scopes: Option<&NSArray<ASAuthorizationScope>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setState:)]
        pub unsafe fn setState(&self, state: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nonce)]
        pub unsafe fn nonce(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNonce:)]
        pub unsafe fn setNonce(&self, nonce: Option<&NSString>);

        #[method_id(@__retain_semantics Other requestedOperation)]
        pub unsafe fn requestedOperation(&self) -> Id<ASAuthorizationOpenIDOperation>;

        #[method(setRequestedOperation:)]
        pub unsafe fn setRequestedOperation(
            &self,
            requested_operation: &ASAuthorizationOpenIDOperation,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `ASAuthorizationRequest`
    #[cfg(feature = "AuthenticationServices_ASAuthorizationOpenIDRequest")]
    unsafe impl ASAuthorizationOpenIDRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
