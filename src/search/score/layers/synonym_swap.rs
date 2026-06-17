use std::collections::HashSet;

use serde::Deserialize;

use crate::search::score::ScoringLayer;

#[derive(Deserialize)]
pub struct SynonymSwappingScore(Vec<HashSet<String>>);
impl ScoringLayer for SynonymSwappingScore {
    fn attribute_score(&self, query: &str, line: &str) -> f64 {
        let query_words = query.split(' ');
        let query_word_set: HashSet<&str> = query_words.collect();
        let query_syn_set: HashSet<_> = query_word_set
            .into_iter()
            .flat_map(|word| self.0.iter().filter(|map| map.contains(word)))
            .flatten()
            .map(String::as_str)
            .collect();

        let line_words = line.split(' ');
        let line_word_set: HashSet<&str> = line_words.collect();
        let line_word_count = line_word_set.len() as f64;

        let intersection_len = line_word_set.intersection(&query_syn_set).count();

        intersection_len as f64 / line_word_count
    }

    fn attribute_score_iter<'a>(
        &self,
        query: &str,
        lines_with_score: impl Iterator<Item = (&'a str, f64)>,
    ) -> impl Iterator<Item = (&'a str, f64)> {
        let query_words = query.split(' ');
        let query_word_set: HashSet<&str> = query_words.collect();
        let query_syn_set: HashSet<_> = query_word_set
            .into_iter()
            .flat_map(|word| self.0.iter().filter(|map| map.contains(word)))
            .flatten()
            .map(String::as_str)
            .collect();

        lines_with_score.map(move |(line, score)| {
            let line_words = line.split(' ');
            let line_word_set: HashSet<&str> = line_words.collect();
            let line_word_count = line_word_set.len() as f64;

            let intersection = HashSet::intersection(&line_word_set, &query_syn_set);
            let intersection_size = intersection.count() as f64;

            let syn_score = intersection_size / line_word_count;
            (line, score + syn_score)
        })
    }
}
