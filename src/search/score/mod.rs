use std::sync::Arc;

use itertools::izip;

use crate::search::score::layers::{
    ScoringLayer, contains::ContainsScore, fuzzy::FuzzScore, jaccard::JaccardScore,
};

pub mod layers;
pub mod structures;

pub fn attribute_scores<'a>(
    query: &str,
    lines: impl Iterator<Item = &'a Arc<str>> + Clone,
) -> impl Iterator<Item = f64> {
    let lines_iter = lines.map(Arc::as_ref);

    let fuzz_score = FuzzScore.attribute_score_iter(query, lines_iter.clone());
    let contains_score = ContainsScore.attribute_score_iter(query, lines_iter.clone());
    let jaccard_score = JaccardScore.attribute_score_iter(query, lines_iter.clone());

    let zipped_scores = izip!(fuzz_score, contains_score, jaccard_score);
    zipped_scores.map(|(fuzz, contains, jaccard)| join_scores(fuzz, contains, jaccard))
}

fn join_scores(fuzz: f64, contains: f64, jaccard: f64) -> f64 {
    fuzz + contains + jaccard
}
