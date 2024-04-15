use crate::{arc, define_obj_type, nl, ns, objc};

pub type Distance = f64;

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

    #[objc::msg_send(containsString:)]
    pub fn contains_string(&self, str: &ns::String) -> bool;

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
    }
}
