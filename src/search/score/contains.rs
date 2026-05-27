use crate::search::score::ScoringLayer;

pub struct ContainsScore;
impl ScoringLayer for ContainsScore {
    fn attribute_score(query: &str, line: &str) -> f64 {
        if line
            .to_ascii_lowercase()
            .contains(&query.to_ascii_lowercase())
        {
            1f64
        } else {
            0f64
        }
    }
}
