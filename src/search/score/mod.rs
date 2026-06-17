use std::sync::Arc;

use crate::search::score::layers::{
    ScoringLayer, contains::ContainsScore, fuzzy::FuzzScore, jaccard::JaccardScore,
};

pub mod layers;
pub mod structures;

pub fn attribute_scores<'a>(
    query: &str,
    lines: impl Iterator<Item = &'a Arc<str>>,
) -> impl Iterator<Item = f64> {
    // let synonym_swap_scorer: SynonymSwappingScore =
    //     yaml_serde::from_reader(File::open("synonyms.txt").unwrap()).unwrap();

    let scored = lines.map(Arc::as_ref).map(|line| (line, 0f64));
    let scored = FuzzScore::attribute_score_iter(&FuzzScore, query, scored);
    let scored = ContainsScore::attribute_score_iter(&ContainsScore, query, scored);
    let scored = JaccardScore::attribute_score_iter(&JaccardScore, query, scored);
    // let scored = synonym_swap_scorer.attribute_score_iter(query, scored);
    scored.map(|(_line, score)| score)
}   
