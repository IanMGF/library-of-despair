use rapidfuzz::distance::levenshtein;

use crate::search::score::layers::ScoringLayer;

pub struct FuzzScore;
impl ScoringLayer for FuzzScore {
    fn attribute_score(&self, query: &str, line: &str) -> f64 {
        levenshtein::normalized_similarity(query.chars(), line.chars())
    }

    fn attribute_score_iter<'a>(
        &self,
        query: &str,
        lines_with_score: impl Iterator<Item = (&'a str, f64)>,
    ) -> impl Iterator<Item = (&'a str, f64)> {
        let query_chars = query.chars();
        let levenshtein_comp = levenshtein::BatchComparator::new(query_chars.clone());

        lines_with_score.map(move |(line, score)| {
            let line_chars = line.chars();
            let levenshtein_score = levenshtein_comp.normalized_similarity(line_chars.clone());
            let layer_score = levenshtein_score;
            (line, score + layer_score)
        })
    }
}
