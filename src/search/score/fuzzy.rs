use crate::search::score::ScoringLayer;

pub struct FuzzScore;
impl ScoringLayer for FuzzScore {
    fn attribute_score(query: &str, line: &str) -> f64 {
        let levenshtein_dist =
            rapidfuzz::distance::levenshtein::normalized_similarity(query.chars(), line.chars());
        let jaro_winkler_dist =
            rapidfuzz::distance::jaro_winkler::normalized_similarity(query.chars(), line.chars());

        (levenshtein_dist + jaro_winkler_dist) / 2f64
    }
}
