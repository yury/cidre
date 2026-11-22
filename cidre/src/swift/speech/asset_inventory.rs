use super::SpeechModule;
use crate::{arc, ns, swift};

crate::define_swift!(#[swift::class("Speech.AssetInventory")] pub AssetInventory);
crate::define_swift!(#[swift::class("Speech.AssetInstallationRequest")] pub AssetInstallationRequest);

impl AssetInventory {
    #[crate::api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    #[swift::call(
        "static Speech.AssetInventory(class).assetInstallationRequest(supporting: [any Speech.SpeechModule]) async throws -> Speech.AssetInstallationRequest(class)?"
    )]
    pub fn installation_request(
        modules: swift::Array<SpeechModule>,
    ) -> Result<Option<arc::R<AssetInstallationRequest>>, arc::R<ns::Error>>;
}
impl AssetInstallationRequest {
    #[crate::api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    #[swift::call("Speech.AssetInstallationRequest(class).downloadAndInstall() async throws")]
    pub fn download_and_install(&self) -> Result<(), arc::R<ns::Error>>;
}
