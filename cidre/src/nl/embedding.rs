use crate::{arc, define_obj_type, nl, ns, objc};

pub type Distance = f64;

#[repr(isize)]
pub enum DistanceType {
    /// A cosine distance in embedding space, i.e. 1 - cosine similarity,
    /// in the range \[0.0, 2.0\].
    Cosine,
}

define_obj_type!(
    #[doc(alias = "NLEmbedding")]
    pub Embedding(ns::Id),
    NL_EMBEDDING
);

impl Embedding {
    #[objc::cls_msg_send(wordEmbeddingForLanguage:)]
    pub fn word_ar(lang: &nl::Lang) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn word(lang: &nl::Lang) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(wordEmbeddingForLanguage:revision:)]
    pub fn word_with_rev_ar(lang: &nl::Lang, revision: usize) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn word_with_rev(lang: &nl::Lang, revision: usize) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(sentenceEmbeddingForLanguage:)]
    pub fn sentence_ar(lang: &nl::Lang) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn sentence(lang: &nl::Lang) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(sentenceEmbeddingForLanguage:revision:)]
    pub fn sentence_with_rev_ar(lang: &nl::Lang, revision: usize) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn sentence_with_rev(lang: &nl::Lang, revision: usize) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(embeddingWithContentsOfURL:error:)]
    pub fn with_url_err_ar<'ear>(
        url: &ns::Url,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_url_err<'ear>(
        url: &ns::Url,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<Self>>;

    pub fn with_url<'ear>(url: &ns::Url) -> Result<arc::R<Self>, &'ear ns::Error> {
        ns::if_none(|err| Self::with_url_err(url, err))
    }

    #[objc::msg_send(containsString:)]
    pub fn contains_string(&self, str: &ns::String) -> bool;

    #[objc::msg_send(distanceBetweenString:andString:distanceType:)]
    pub fn distance_between_with_type(
        &self,
        a: &ns::String,
        b: &ns::String,
        distance_type: DistanceType,
    ) -> Distance;

    /// Calculates the distance between two strings in the vocabulary space.
    #[inline]
    pub fn distance_between(&self, a: &ns::String, b: &ns::String) -> Distance {
        self.distance_between_with_type(a, b, DistanceType::Cosine)
    }

    #[objc::msg_send(dimension)]
    pub fn dimension(&self) -> usize;

    #[objc::msg_send(vocabularySize)]
    pub fn vocabulary_size(&self) -> usize;

    #[objc::msg_send(language)]
    pub fn language_ar(&self) -> Option<arc::Rar<nl::Lang>>;

    #[objc::rar_retain]
    pub fn language(&self) -> Option<arc::R<nl::Lang>>;

    #[objc::msg_send(revision)]
    pub fn revision(&self) -> usize;
}

#[link(name = "nl", kind = "static")]
extern "C" {
    static NL_EMBEDDING: &'static objc::Class<Embedding>;
}

#[cfg(test)]
mod tests {
    use crate::{nl, ns};

    #[test]
    fn basics() {
        let word_emb =
            nl::Embedding::word(nl::Lang::english()).expect("failed to load word embedding");

        assert!(word_emb.revision() > 0);

        assert!(word_emb.contains_string(ns::str!(c"hello")));
        assert!(!word_emb.contains_string(ns::str!(c"cidre")));

        nl::Embedding::with_url(&ns::Url::with_str("https:://google.com").unwrap())
            .expect_err("invalid url");
    }
}
