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

    #[objc::cls_msg_send(defaultSound)]
    pub fn default_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn default() -> arc::R<Self>;

    #[cfg(target_os = "ios")]
    #[objc::cls_msg_send(defaultRingtoneSound)]
    pub fn default_ringtone_sound_ar() -> arc::Rar<Self>;

    #[cfg(target_os = "ios")]
    #[objc::cls_rar_retain]
    pub fn default_ringtone_sound() -> arc::R<Self>;

    #[cfg(not(target_os = "tvos"))]
    #[objc::cls_msg_send(defaultCriticalSound)]
    pub fn default_critical_sound_ar() -> arc::Rar<Self>;

    #[cfg(not(target_os = "tvos"))]
    #[objc::cls_rar_retain]
    pub fn default_critical_sound() -> arc::R<Self>;

    #[cfg(any(target_os = "ios", target_os = "watchos"))]
    #[objc::cls_msg_send(defaultCriticalSoundWithAudioVolume:)]
    pub fn default_critical_sound_with_audio_volume_ar(volume: f32) -> arc::Rar<Self>;

    #[cfg(any(target_os = "ios", target_os = "watchos"))]
    #[objc::cls_rar_retain]
    pub fn default_critical_sound_with_audio_volume(volume: f32) -> arc::R<Self>;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::cls_msg_send(soundNamed:)]
    pub fn sound_named_ar(name: &un::NotificationSoundName) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn sound_named(name: &un::NotificationSoundName) -> arc::R<Self>;

    #[cfg(target_os = "ios")]
    #[objc::cls_msg_send(ringtoneSoundNamed:)]
    pub fn ringtone_sound_named_ar(name: &un::NotificationSoundName) -> arc::Rar<Self>;

    #[cfg(target_os = "ios")]
    #[objc::cls_rar_retain]
    pub fn ringtone_sound_named(name: &un::NotificationSoundName) -> arc::R<Self>;

    #[objc::cls_msg_send(criticalSoundNamed:)]
    pub fn critical_sound_named_ar(name: &un::NotificationSoundName) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn critical_sound_named(name: &un::NotificationSoundName) -> arc::R<Self>;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::cls_msg_send(criticalSoundNamed:withAudioVolume:)]
    pub fn critical_sound_named_with_audio_volume_ar(
        name: &un::NotificationSoundName,
        volume: f32,
    ) -> arc::Rar<Self>;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::cls_rar_retain]
    pub fn critical_sound_named_with_audio_volume(
        name: &un::NotificationSoundName,
        volume: f32,
    ) -> arc::R<Self>;
}

#[link(name = "un", kind = "static")]
extern "C" {
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
