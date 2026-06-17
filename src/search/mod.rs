pub mod score;

use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use backend::archive::episode::Episode;
use common_types::{query::*, result::*};

use crate::search::score::structures::{min_heap::SearchResultItemHolder, scored_line::ScoredLine};

const RESULT_LIMIT: usize = 100;

pub(crate) async fn search(
    State(episodes): State<Arc<[Episode]>>,
    Query(SearchParams { query }): Query<SearchParams>,
) -> Json<SearchResult> {
    // Initialize a min-heap structure to hold the results
    let mut result_holder: SearchResultItemHolder<RESULT_LIMIT> = SearchResultItemHolder::new();

    for episode in episodes.iter() {
        let ep_results = search_in_episode(&episode, query.as_str());
        result_holder.extend(ep_results);
    }

    let search_result_vec = result_holder
        .items()
        .into_iter()
        .map(SearchResultItem::from)
        .collect();
    Json(SearchResult(search_result_vec))
}

fn search_in_episode<'a>(ep: &'a Episode, query: &str) -> impl Iterator<Item = ScoredLine<'a>> {
    let lines = ep.get_content().get_lines();

    score::attribute_scores(query, lines.iter())
        .enumerate()
        .map(|(idx, line_score)| ScoredLine::new(ep, idx, line_score))
}
