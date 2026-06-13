pub mod contains;
pub mod fuzzy;
pub mod jaccard;
pub mod synonym_swap;

pub(super) trait ScoringLayer {
    fn attribute_score(&self, query: &str, line: &str) -> f64;
    fn attribute_score_iter<'a>(
        &self,
        query: &str,
        lines_with_score: impl Iterator<Item = (&'a str, f64)>,
    ) -> impl Iterator<Item = (&'a str, f64)> {
        lines_with_score.map(|(line, score)| {
            let layer_score = self.attribute_score(query, line);
            (line, score + layer_score)
        })
    }
}
