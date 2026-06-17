use std::collections::HashSet;

use crate::search::score::layers::ScoringLayer;

pub struct JaccardScore;
impl ScoringLayer for JaccardScore {
    fn attribute_score(&self, query: &str, line: &str) -> f64 {
        let line_words = line.split(' ');
        let line_word_set: HashSet<&str> = line_words.collect();
        let line_word_count = line_word_set.len() as f64;

        let query_words = query.split(' ');
        let query_word_set: HashSet<&str> = query_words.collect();
        let query_word_count = query_word_set.len() as f64;

        let intersection = HashSet::intersection(&line_word_set, &query_word_set);
        let intersection_size = intersection.count() as f64;

        (2f64 * intersection_size) / (line_word_count + query_word_count)
    }

    fn attribute_score_iter<'a>(
        &self,
        query: &str,
        lines_with_score: impl Iterator<Item = (&'a str, f64)>,
    ) -> impl Iterator<Item = (&'a str, f64)> {
        let query_words = query.split(' ');
        let query_word_set: HashSet<&str> = query_words.collect();
        let query_word_count = query_word_set.len() as f64;

        lines_with_score.map(move |(line, score)| {
            let line_words = line.split(' ');
            let line_word_set: HashSet<&str> = line_words.collect();
            let line_word_count = line_word_set.len() as f64;

            let intersection = HashSet::intersection(&line_word_set, &query_word_set);
            let intersection_size = intersection.count() as f64;

            let jaccard_score = (2f64 * intersection_size) / (line_word_count + query_word_count);
            (line, score + jaccard_score)
        })
    }
}
