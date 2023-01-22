//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "BAAppExtensionInfo.rs"]
mod __BAAppExtensionInfo;
#[path = "BABase.rs"]
mod __BABase;
#[path = "BADownload.rs"]
mod __BADownload;
#[path = "BADownloadManager.rs"]
mod __BADownloadManager;
#[path = "BADownloaderExtension.rs"]
mod __BADownloaderExtension;
#[path = "BATypes.rs"]
mod __BATypes;
#[path = "BAURLDownload.rs"]
mod __BAURLDownload;

#[cfg(feature = "BackgroundAssets_BAAppExtensionInfo")]
pub use self::__BAAppExtensionInfo::BAAppExtensionInfo;
#[cfg(feature = "BackgroundAssets_BADownload")]
pub use self::__BADownload::BADownload;
#[cfg(feature = "BackgroundAssets_BADownloadManager")]
pub use self::__BADownloadManager::BADownloadManager;
pub use self::__BADownloadManager::BADownloadManagerDelegate;
pub use self::__BADownloaderExtension::BADownloaderExtension;
pub use self::__BATypes::BADownloaderPriority;
pub use self::__BATypes::BADownloaderPriorityDefault;
pub use self::__BATypes::BADownloaderPriorityMax;
pub use self::__BATypes::BADownloaderPriorityMin;
pub use self::__BATypes::{
    BAContentRequest, BAContentRequestInstall, BAContentRequestPeriodic, BAContentRequestUpdate,
};
pub use self::__BATypes::{
    BADownloadState, BADownloadStateCreated, BADownloadStateDownloading, BADownloadStateFailed,
    BADownloadStateFinished, BADownloadStateWaiting,
};
#[cfg(feature = "BackgroundAssets_BAURLDownload")]
pub use self::__BAURLDownload::BAURLDownload;