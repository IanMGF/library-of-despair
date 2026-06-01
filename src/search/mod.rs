pub mod score;

use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use backend::archive::episode::Episode;
use serde::{Deserialize, Serialize};

use crate::search::score::structures::{min_heap::SearchResultItemHolder, scored_line::ScoredLine};

#[derive(Deserialize, Serialize)]
pub(crate) struct SearchParams {
    pub query: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct SearchResult(Vec<SearchResultItem>);

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchResultItem {
    line_before: Option<Line>,
    line: Line,
    line_after: Option<Line>,
    moment: Moment,
    score: f64,
}

type Timestamp = i64;
#[derive(Deserialize, Serialize)]
pub(crate) struct Line {
    time: Timestamp,
    speakers: Arc<[Arc<str>]>,
    text: Arc<str>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Moment {
    timestamp: i64,
    episode_name: Arc<str>,
    episode_number: u8,
    season_name: Arc<str>,
    video_id: Arc<str>,
}

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
