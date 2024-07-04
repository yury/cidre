use crate::{arc, blocks, define_obj_type, nl, ns, objc};

pub type Distance = f64;

#[derive(Default, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum DistanceType {
    /// A cosine distance in embedding space, i.e. 1 - cosine similarity,
    /// in the range \[0.0, 2.0\].
    #[default]
    Cosine,
}

define_obj_type!(
    #[doc(alias = "NLEmbedding")]
    pub Embedding(ns::Id),
    NL_EMBEDDING
);

impl Embedding {
    #[objc::msg_send(wordEmbeddingForLanguage:)]
    pub fn word(lang: &nl::Lang) -> Option<arc::R<Self>>;

    #[objc::msg_send(wordEmbeddingForLanguage:revision:)]
    pub fn word_with_rev(lang: &nl::Lang, revision: usize) -> Option<arc::R<Self>>;

    #[objc::msg_send(sentenceEmbeddingForLanguage:)]
    pub fn sentence(lang: &nl::Lang) -> Option<arc::R<Self>>;

    #[objc::msg_send(sentenceEmbeddingForLanguage:revision:)]
    pub fn sentence_with_rev(lang: &nl::Lang, revision: usize) -> Option<arc::R<Self>>;

    #[objc::msg_send(embeddingWithContentsOfURL:error:)]
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
    pub fn language(&self) -> Option<arc::R<nl::Lang>>;

    #[objc::msg_send(revision)]
    pub fn revision(&self) -> usize;

    #[objc::msg_send(supportedRevisionsForLanguage:)]
    pub fn supported_revisions(lang: &nl::Lang) -> arc::R<ns::IndexSet>;

    #[objc::msg_send(currentRevisionForLanguage:)]
    pub fn current_revision(lang: &nl::Lang) -> usize;

    #[objc::msg_send(supportedSentenceEmbeddingRevisionsForLanguage:)]
    pub fn supported_sentence_revisions(lang: &nl::Lang) -> arc::R<ns::IndexSet>;

    #[objc::msg_send(currentSentenceEmbeddingRevisionForLanguage:)]
    pub fn current_sentence_revision(lang: &nl::Lang) -> usize;

    #[objc::msg_send(enumerateNeighborsForString:maximumCount:distanceType:usingBlock:)]
    pub fn enumerate_neighbors_for_string_block(
        &self,
        str: &ns::String,
        max_count: usize,
        distance_type: DistanceType,
        block: &mut blocks::NoEscBlock<
            fn(neighbor: &ns::String, distance: nl::Distance, stop: &mut bool),
        >,
    );

    #[objc::msg_send(enumerateNeighborsForString:maximumCount:maximumDistance:distanceType:usingBlock:)]
    pub fn enumerate_neighbors_within_distance_for_string_block(
        &self,
        str: &ns::String,
        max_count: usize,
        max_distance: nl::Distance,
        distance_type: DistanceType,
        block: &mut blocks::NoEscBlock<
            fn(neighbor: &ns::String, distance: nl::Distance, stop: &mut bool),
        >,
    );

    #[inline]
    pub fn enumerate_neighbors_for_string(
        &self,
        str: &ns::String,
        max_count: usize,
        distance_type: DistanceType,
        mut block: impl FnMut(&ns::String, nl::Distance, &mut bool),
    ) {
        let mut block = unsafe { blocks::NoEscBlock::stack3(&mut block) };
        self.enumerate_neighbors_for_string_block(str, max_count, distance_type, &mut block);
    }

    #[inline]
    pub fn enumerate_neighbors_within_distance_for_string(
        &self,
        str: &ns::String,
        max_count: usize,
        max_distance: nl::Distance,
        distance_type: DistanceType,
        mut block: impl FnMut(&ns::String, nl::Distance, &mut bool),
    ) {
        let mut block = unsafe { blocks::NoEscBlock::stack3(&mut block) };
        self.enumerate_neighbors_within_distance_for_string_block(
            str,
            max_count,
            max_distance,
            distance_type,
            &mut block,
        );
    }

    #[objc::msg_send(neighborsForString:maximumCount:distanceType:)]
    pub fn neighbors_for_string(
        &self,
        str: &ns::String,
        max_count: usize,
        distance_type: DistanceType,
    ) -> Option<arc::R<ns::Array<ns::String>>>;

    #[objc::msg_send(neighborsForString:maximumCount:maximumDistance:distanceType:)]
    pub fn neighbors_for_string_within_distance(
        &self,
        str: &ns::String,
        max_count: usize,
        max_distance: nl::Distance,
        distance_type: DistanceType,
    ) -> Option<arc::R<ns::Array<ns::String>>>;

    #[objc::msg_send(vectorForString:)]
    pub fn vector_for_string(&self, str: &ns::String) -> Option<arc::R<ns::Array<ns::Number>>>;

    #[objc::msg_send(getVector:forString:)]
    pub unsafe fn get_vector_for_string(&self, vec: *mut f32, str: &ns::String) -> bool;

    pub fn fill_vector_for_string(&self, str: &ns::String, vec: &mut [f32]) -> Result<(), ()> {
        assert_eq!(vec.len(), self.dimension());
        unsafe {
            if self.get_vector_for_string(vec.as_mut_ptr(), str) {
                Ok(())
            } else {
                Err(())
            }
        }
    }
    #[objc::msg_send(enumerateNeighborsForVector:maximumCount:distanceType:usingBlock:)]
    pub fn enumerate_neighbors_for_vector_block(
        &self,
        vec: &ns::Array<ns::Number>,
        max_count: usize,
        distance_type: DistanceType,
        block: &mut blocks::NoEscBlock<
            fn(neighbor: &ns::String, distance: nl::Distance, stop: &mut bool),
        >,
    );

    #[objc::msg_send(enumerateNeighborsForVector:maximumCount:maximumDistance:distanceType:usingBlock:)]
    pub fn enumerate_neighbors_within_distance_for_vector_block(
        &self,
        vec: &ns::Array<ns::Number>,
        max_count: usize,
        max_distance: nl::Distance,
        distance_type: DistanceType,
        block: &mut blocks::NoEscBlock<
            fn(neighbor: &ns::String, distance: nl::Distance, stop: &mut bool),
        >,
    );

    #[inline]
    pub fn enumerate_neighbors_for_vector(
        &self,
        vec: &ns::Array<ns::Number>,
        max_count: usize,
        distance_type: DistanceType,
        mut block: impl FnMut(&ns::String, nl::Distance, &mut bool),
    ) {
        let mut block = unsafe { blocks::NoEscBlock::stack3(&mut block) };
        self.enumerate_neighbors_for_vector_block(vec, max_count, distance_type, &mut block);
    }

    #[inline]
    pub fn enumerate_neighbors_within_distance_for_vector(
        &self,
        vec: &ns::Array<ns::Number>,
        max_count: usize,
        max_distance: nl::Distance,
        distance_type: DistanceType,
        mut block: impl FnMut(&ns::String, nl::Distance, &mut bool),
    ) {
        let mut block = unsafe { blocks::NoEscBlock::stack3(&mut block) };
        self.enumerate_neighbors_within_distance_for_vector_block(
            vec,
            max_count,
            max_distance,
            distance_type,
            &mut block,
        );
    }

    #[objc::msg_send(neighborsForVector:maximumCount:distanceType:)]
    pub fn neighbors_for_vector(
        &self,
        vec: &ns::Array<ns::Number>,
        max_count: usize,
        distance_type: DistanceType,
    ) -> Option<arc::R<ns::Array<ns::String>>>;

    #[objc::msg_send(neighborsForVector:maximumCount:maximumDistance:distanceType:)]
    pub fn neighbors_for_vector_within_distance(
        &self,
        vec: &ns::Array<ns::Number>,
        max_count: usize,
        max_distance: nl::Distance,
        distance_type: DistanceType,
    ) -> Option<arc::R<ns::Array<ns::String>>>;
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

        let set = nl::Embedding::supported_revisions(nl::Lang::english());
        assert!(set.len() >= 1);

        let rev = nl::Embedding::current_revision(nl::Lang::english());
        assert!(set.contains_index(rev));

        let mut neighbors = Vec::new();
        word_emb.enumerate_neighbors_for_string(
            ns::str!(c"hello"),
            20,
            Default::default(),
            |neighbor, _distance, stop| {
                neighbors.push(neighbor.retained());
                if neighbors.len() == 10 {
                    *stop = true;
                }
            },
        );

        assert_eq!(10, neighbors.len());

        let neighbors = word_emb
            .neighbors_for_string(ns::str!(c"world"), 10, Default::default())
            .unwrap();
        assert_eq!(10, neighbors.len());

        let vec = word_emb.vector_for_string(ns::str!(c"nice")).unwrap();
        assert_eq!(vec.len(), word_emb.dimension());

        let neigbors = word_emb
            .neighbors_for_vector(&vec, 10, Default::default())
            .unwrap();

        assert_eq!(neigbors.len(), 10);

        let mut vec = vec![0.0f32; word_emb.dimension()];
        word_emb
            .fill_vector_for_string(ns::str!(c"world"), &mut vec)
            .unwrap();
    }
}
