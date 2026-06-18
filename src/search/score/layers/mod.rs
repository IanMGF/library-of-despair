pub mod contains;
pub mod fuzzy;
pub mod jaccard;

pub(super) trait ScoringLayer {
    fn attribute_score(&self, query: &str, line: &str) -> f64;
    fn attribute_score_iter<'a>(
        &self,
        query: &str,
        lines_with_score: impl Iterator<Item = &'a str>+Clone,
    ) -> impl Iterator<Item = f64> {
        lines_with_score.map(|line| {
            let layer_score = self.attribute_score(query, line);
            layer_score
        })
    }
}
