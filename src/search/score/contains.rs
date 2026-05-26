use crate::search::score::ScoringLayer;

pub struct ContainsScore;
impl ScoringLayer for ContainsScore {
    fn attribute_score(query: &str, line: &str) -> f64 {
        if line
            .to_ascii_lowercase()
            .contains(&query.to_ascii_lowercase())
        {
            return 1f64;
        } else {
            return 0f64;
        }
    }
}
