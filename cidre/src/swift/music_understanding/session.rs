use crate::{api, arc, av, ns, swift};

use super::analysis_type::AnalysisType;
use super::results::SessionResult;

crate::define_swift!(
    #[swift::class("MusicUnderstanding.MusicUnderstandingSession")]
    pub MusicUnderstandingSession
);

impl MusicUnderstandingSession {
    /// The allocating initializer.
    ///
    /// Its parameter is a `Sendable & AVAsset` existential, which the mangler
    /// cannot spell, so the symbol is given as-is — and a bare symbol carries
    /// no conventions, so `owned` states the one an initializer has: the asset
    /// arrives at `+1`.
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    #[swift::call(
        sym = "$s18MusicUnderstanding0aB7SessionC5assetACs8Sendable_So7AVAssetCXc_tYaKcfC",
        async,
        owned(asset)
    )]
    fn init_with_asset(asset: arc::R<av::Asset>) -> Result<arc::R<Self>, arc::R<ns::Error>>;

    /// Creates a session for an asset.
    #[doc(alias = "MusicUnderstandingSession.init(asset:)")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn with_asset_handler<F>(asset: &av::Asset, callback: F)
    where
        F: FnOnce(Result<arc::R<Self>, arc::R<ns::Error>>) + Send + 'static,
    {
        Self::init_with_asset_handler(asset.retained(), callback);
    }

    #[doc(alias = "MusicUnderstandingSession.init(asset:)")]
    #[cfg(feature = "async")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn with_asset(
        asset: &av::Asset,
    ) -> impl Future<Output = Result<arc::R<Self>, arc::R<ns::Error>>> {
        Self::init_with_asset(asset.retained())
    }

    /// Runs every analysis the session supports.
    ///
    /// The result is the session's own nested type, which the symbol reaches by
    /// back reference rather than by name, so this one is given mangled.
    #[doc(alias = "MusicUnderstandingSession.analyze()")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    #[swift::call(
        sym = "$s18MusicUnderstanding0aB7SessionC7analyzeAC0C6ResultVyYaKFTj",
        async
    )]
    pub fn analyze(&self) -> Result<SessionResult, arc::R<ns::Error>>;

    /// As [`Self::analyze`], the result is the session's own nested type, so
    /// this symbol is given mangled too.
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    #[swift::call(
        sym = "$s18MusicUnderstanding0aB7SessionC7analyze3forAC0C6ResultVShyAA12AnalysisTypeVG_tYaKFTj",
        async
    )]
    fn analyze_set(
        &self,
        types: swift::Set<AnalysisType>,
    ) -> Result<SessionResult, arc::R<ns::Error>>;

    /// Runs only the requested analyses, which is much faster than
    /// [`Self::analyze_handler`] when a caller needs one of them.
    #[doc(alias = "MusicUnderstandingSession.analyze(for:)")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn analyze_for_handler<F>(&self, types: &[AnalysisType], callback: F)
    where
        F: FnOnce(Result<SessionResult, arc::R<ns::Error>>) + Send + 'static,
    {
        self.analyze_set_handler(swift::Set::from_slice(types), callback);
    }

    #[doc(alias = "MusicUnderstandingSession.analyze(for:)")]
    #[cfg(feature = "async")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn analyze_for(
        &self,
        types: &[AnalysisType],
    ) -> impl Future<Output = Result<SessionResult, arc::R<ns::Error>>> {
        self.analyze_set(swift::Set::from_slice(types))
    }
}
