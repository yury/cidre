use crate::{
    api, arc, av, ns, swift,
    swift::{
        SwiftMetadata,
        concurrency::{self, AsyncCallArgs},
        value::Storage,
    },
};

use super::analysis_type::AnalysisType;
use super::results::{SessionResult, SessionResultValue};

crate::define_swift!(
    #[swift::class("MusicUnderstanding.MusicUnderstandingSession")]
    pub MusicUnderstandingSession
);

#[link(name = "MusicUnderstanding", kind = "framework")]
unsafe extern "C" {
    /// The allocating initializer, which takes the `AVAsset` at +1.
    #[link_name = "$s18MusicUnderstanding0aB7SessionC5assetACs8Sendable_So7AVAssetCXc_tYaKcfC"]
    fn session_init_with_asset();

    #[link_name = "$s18MusicUnderstanding0aB7SessionC5assetACs8Sendable_So7AVAssetCXc_tYaKcfCTu"]
    static SESSION_INIT_WITH_ASSET_ASYNC_FN: u8;

    #[link_name = "$s18MusicUnderstanding0aB7SessionC7analyzeAC0C6ResultVyYaKFTj"]
    fn session_analyze();

    #[link_name = "$s18MusicUnderstanding0aB7SessionC7analyzeAC0C6ResultVyYaKFTjTu"]
    static SESSION_ANALYZE_ASYNC_FN: u8;

    #[link_name = "$s18MusicUnderstanding0aB7SessionC7analyze3forAC0C6ResultVShyAA12AnalysisTypeVG_tYaKFTj"]
    fn session_analyze_for();

    #[link_name = "$s18MusicUnderstanding0aB7SessionC7analyze3forAC0C6ResultVShyAA12AnalysisTypeVG_tYaKFTjTu"]
    static SESSION_ANALYZE_FOR_ASYNC_FN: u8;
}

impl MusicUnderstandingSession {
    /// Creates a session for an asset.
    #[doc(alias = "MusicUnderstandingSession.init(asset:)")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn with_asset_handler<F>(asset: &av::Asset, callback: F)
    where
        F: FnOnce(Result<arc::R<Self>, arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            concurrency::call_async_result(
                session_init_with_asset as *const (),
                &raw const SESSION_INIT_WITH_ASSET_ASYNC_FN,
                // The initializer consumes its argument, so it gets a reference
                // of its own rather than the caller's.
                asset.retained(),
                |asset| {
                    AsyncCallArgs::new()
                        // A static initializer's `self` is the class metadata.
                        .swift_self(<Self as SwiftMetadata>::metadata().cast_mut().cast())
                        .arg(0, asset.retained().into_raw().cast())
                },
                |_, session| arc::R::from_raw(session.cast()),
                callback,
            );
        }
    }

    #[doc(alias = "MusicUnderstandingSession.init(asset:)")]
    #[cfg(feature = "async")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn with_asset(
        asset: &av::Asset,
    ) -> impl Future<Output = Result<arc::R<Self>, arc::R<ns::Error>>> {
        concurrency::future_from(|callback| Self::with_asset_handler(asset, callback))
    }

    /// Runs every analysis the session supports.
    #[doc(alias = "MusicUnderstandingSession.analyze()")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn analyze_handler<F>(&self, callback: F)
    where
        F: FnOnce(Result<SessionResult, arc::R<ns::Error>>) + Send + 'static,
    {
        self.start_analysis(
            session_analyze as *const (),
            &raw const SESSION_ANALYZE_ASYNC_FN,
            None,
            callback,
        );
    }

    /// Runs only the requested analyses, which is much faster than
    /// [`Self::analyze_handler`] when a caller needs one of them.
    #[doc(alias = "MusicUnderstandingSession.analyze(for:)")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn analyze_for_handler<F>(&self, types: &[AnalysisType], callback: F)
    where
        F: FnOnce(Result<SessionResult, arc::R<ns::Error>>) + Send + 'static,
    {
        self.start_analysis(
            session_analyze_for as *const (),
            &raw const SESSION_ANALYZE_FOR_ASYNC_FN,
            Some(swift::Set::from_slice(types)),
            callback,
        );
    }

    /// Both analyses have the same shape: the result is written into storage
    /// the call is handed, and `analyze(for:)` only adds the borrowed set.
    fn start_analysis<F>(
        &self,
        function: *const (),
        async_fn: *const u8,
        types: Option<swift::Set<AnalysisType>>,
        callback: F,
    ) where
        F: FnOnce(Result<SessionResult, arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            concurrency::call_async_result(
                function,
                async_fn,
                (
                    arc::Retain::retained(self),
                    types,
                    Storage::<SessionResultValue>::new(),
                ),
                |(session, types, result)| {
                    let args = AsyncCallArgs::new()
                        .swift_self(session.as_ptr().cast())
                        .arg(0, result.as_mut_ptr());
                    match types {
                        Some(types) => args.arg(1, types.as_raw()),
                        None => args,
                    }
                },
                |owned, _| {
                    // Swift wrote the result into the storage in place, so what
                    // is left is to take ownership of it.
                    let (_, _, result) = *owned;
                    SessionResult::from_storage(result)
                },
                callback,
            );
        }
    }

    #[doc(alias = "MusicUnderstandingSession.analyze(for:)")]
    #[cfg(feature = "async")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn analyze_for(
        &self,
        types: &[AnalysisType],
    ) -> impl Future<Output = Result<SessionResult, arc::R<ns::Error>>> {
        concurrency::future_from(|callback| self.analyze_for_handler(types, callback))
    }

    #[doc(alias = "MusicUnderstandingSession.analyze()")]
    #[cfg(feature = "async")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn analyze(&self) -> impl Future<Output = Result<SessionResult, arc::R<ns::Error>>> {
        concurrency::future_from(|callback| self.analyze_handler(callback))
    }
}
