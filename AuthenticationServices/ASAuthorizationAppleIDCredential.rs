//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ASUserDetectionStatus {
        ASUserDetectionStatusUnsupported = 0,
        ASUserDetectionStatusUnknown = 1,
        ASUserDetectionStatusLikelyReal = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
    pub struct ASAuthorizationAppleIDCredential;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
    unsafe impl ClassType for ASAuthorizationAppleIDCredential {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
unsafe impl ASAuthorizationCredential for ASAuthorizationAppleIDCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
unsafe impl NSCoding for ASAuthorizationAppleIDCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
unsafe impl NSCopying for ASAuthorizationAppleIDCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
unsafe impl NSObjectProtocol for ASAuthorizationAppleIDCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
unsafe impl NSSecureCoding for ASAuthorizationAppleIDCredential {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
    unsafe impl ASAuthorizationAppleIDCredential {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other authorizedScopes)]
        pub unsafe fn authorizedScopes(&self) -> Id<NSArray<ASAuthorizationScope>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other authorizationCode)]
        pub unsafe fn authorizationCode(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other identityToken)]
        pub unsafe fn identityToken(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other email)]
        pub unsafe fn email(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSPersonNameComponents")]
        #[method_id(@__retain_semantics Other fullName)]
        pub unsafe fn fullName(&self) -> Option<Id<NSPersonNameComponents>>;

        #[method(realUserStatus)]
        pub unsafe fn realUserStatus(&self) -> ASUserDetectionStatus;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
