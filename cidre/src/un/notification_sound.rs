#[cfg(not(target_os = "tvos"))]
use crate::{arc, define_cls, define_obj_type, ns, objc, un};

#[cfg(not(target_os = "tvos"))]
pub type NotificationSoundName = ns::String;

#[cfg(not(target_os = "tvos"))]
define_obj_type!(
    #[doc(alias = "UNNotificationSound")]
    pub NotificationSound(ns::Id)
);

#[cfg(not(target_os = "tvos"))]
impl NotificationSound {
    define_cls!(UN_NOTIFICATION_SOUND);

    #[objc::msg_send(defaultSound)]
    pub fn default() -> arc::R<Self>;

    #[cfg(target_os = "ios")]
    #[objc::msg_send(defaultRingtoneSound)]
    pub fn default_ringtone_sound() -> arc::R<Self>;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(defaultCriticalSound)]
    pub fn default_critical_sound() -> arc::R<Self>;

    #[cfg(any(target_os = "ios", target_os = "watchos"))]
    #[objc::msg_send(defaultCriticalSoundWithAudioVolume:)]
    pub fn default_critical_sound_with_audio_volume(volume: f32) -> arc::R<Self>;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::msg_send(soundNamed:)]
    pub fn sound_named(name: &un::NotificationSoundName) -> arc::R<Self>;

    #[cfg(target_os = "ios")]
    #[objc::msg_send(ringtoneSoundNamed:)]
    pub fn ringtone_sound_named(name: &un::NotificationSoundName) -> arc::R<Self>;

    #[objc::msg_send(criticalSoundNamed:)]
    pub fn critical_sound_named(name: &un::NotificationSoundName) -> arc::R<Self>;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::msg_send(criticalSoundNamed:withAudioVolume:)]
    pub fn critical_sound_named_with_audio_volume(
        name: &un::NotificationSoundName,
        volume: f32,
    ) -> arc::R<Self>;
}

#[link(name = "un", kind = "static")]
unsafe extern "C" {
    #[cfg(not(target_os = "tvos"))]
    static UN_NOTIFICATION_SOUND: &'static objc::Class<NotificationSound>;
}

#[cfg(not(target_os = "tvos"))]
#[cfg(test)]
mod tests {
    use crate::{objc::ar_pool, un};

    #[test]
    fn basics() {
        ar_pool(|| {
            let sound = un::NotificationSound::default();
            sound.as_type_ref().show();
            let sound = un::NotificationSound::default();
            sound.as_type_ref().show();

            let sound = un::NotificationSound::default_critical_sound();
            sound.as_type_ref().show();

            let name = un::NotificationSoundName::with_str("Test!!!!!!");
            let sound = un::NotificationSound::sound_named(&name);
            sound.as_type_ref().show();
            let sound = un::NotificationSound::critical_sound_named(&name);
            sound.as_type_ref().show();
        })
    }
}
