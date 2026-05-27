pub mod contains;
pub mod fuzzy;
pub mod synonym_swap;

trait ScoringLayer {
    fn attribute_score(query: &str, line: &str) -> f64;
}

pub fn attribute_score(query: &str, line: &str) -> f64 {
    let mut score = 0f64;
    score = f64::max(score, fuzzy::FuzzScore::attribute_score(query, line));
    score = f64::max(score, contains::ContainsScore::attribute_score(query, line));
    score
}
