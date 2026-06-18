use crate::search::score::layers::ScoringLayer;

pub struct ContainsScore;
impl ScoringLayer for ContainsScore {
    fn attribute_score(&self, query: &str, line: &str) -> f64 {
        if line.contains(query) { 1f64 } else { 0f64 }
    }

    fn attribute_score_iter<'a>(
        &self,
        query: &str,
        lines_with_score: impl Iterator<Item = &'a str>,
    ) -> impl Iterator<Item = f64> {
        lines_with_score.map(move |line| if line.contains(query) { 1f64 } else { 0f64 })
    }
}
