use crate::{arc, av, cat, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVAudioChannelLayout")]
    pub ChannelLayout(ns::Id)
);

impl arc::A<ChannelLayout> {
    #[objc::msg_send(initWithLayout:)]
    pub fn init_with_layout<const N: usize>(
        self,
        layout: &cat::AudioChannelLayout<N>,
    ) -> arc::R<ChannelLayout>;

    #[objc::msg_send(initWithLayoutTag:)]
    pub fn init_with_layout_tag(self, tag: cat::AudioChannelLayoutTag) -> arc::R<ChannelLayout>;
}

impl ChannelLayout {
    define_cls!(AV_AUDIO_CHANNEL_LAYOUT);

    pub fn with_layout<const N: usize>(layout: &cat::AudioChannelLayout<N>) -> arc::R<Self> {
        Self::alloc().init_with_layout(layout)
    }

    pub fn with_layout_tag(tag: cat::AudioChannelLayoutTag) -> arc::R<Self> {
        Self::alloc().init_with_layout_tag(tag)
    }

    #[objc::msg_send(layoutTag)]
    pub fn layout_tag(&self) -> cat::AudioChannelLayoutTag;

    #[objc::msg_send(channelCount)]
    pub fn channel_count(&self) -> av::AudioChannelCount;
}

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    static AV_AUDIO_CHANNEL_LAYOUT: &'static objc::Class<ChannelLayout>;
}

#[cfg(test)]
mod tests {
    use crate::{av, cat};

    #[test]
    fn basics() {
        let tag = cat::AudioChannelLayoutTag::STEREO;
        let layout = av::AudioChannelLayout::with_layout_tag(tag);
        assert_eq!(layout.channel_count(), 2);
        assert_eq!(layout.layout_tag(), tag);
    }
}
