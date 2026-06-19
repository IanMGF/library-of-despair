pub mod score;

use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use common_types::archive::episode::Episode;
use common_types::{query::*, result::*};

use crate::search::score::structures::{min_heap::SearchResultItemHolder, scored_line::ScoredLine};

const RESULT_LIMIT: usize = 100;

pub(crate) async fn search(
    State(episodes): State<Arc<[Episode]>>,
    Query(SearchParams { query, filters }): Query<SearchParams>,
) -> Json<SearchResult> {
    // Initialize a min-heap structure to hold the results
    let mut result_holder: SearchResultItemHolder<RESULT_LIMIT> = SearchResultItemHolder::new();

    let qualified_episodes = episodes
        .iter()
        .filter(|ep| episode_matches_filters(ep, &filters));

    for episode in qualified_episodes {
        let ep_results = search_in_episode(episode, query.as_ref(), &filters);
        result_holder.extend(ep_results);
    }

    let search_result_vec = result_holder
        .items()
        .into_iter()
        .map(SearchResultItem::from)
        .collect();
    Json(SearchResult(search_result_vec))
}

fn search_in_episode<'a>(
    ep: &'a Episode,
    query: &str,
    filters: &SearchFilters,
) -> impl Iterator<Item = ScoredLine<'a>> {
    let lines = ep.get_content().get_lines();

    let assignments = ep.get_assignment();

    score::attribute_scores(query, lines.iter())
        .enumerate()
        .filter(|(idx, _)| {
            filters
                .speaker
                .as_ref()
                .is_none_or(|sp| assignments[*idx].assignments.contains(&sp))
        })
        .map(|(idx, line_score)| ScoredLine::new(ep, idx, line_score))
}

fn episode_matches_filters(ep: &Episode, filters: &SearchFilters) -> bool {
    let matches_season = filters
        .season
        .as_ref()
        .is_none_or(|season_id| &ep.get_info().season_id == season_id);

    let matches_episode = filters
        .episode
        .as_ref()
        .is_none_or(|ep_id| ep_id == &ep.get_info().id);

    let matches_speaker = filters
        .speaker
        .as_ref()
        .is_none_or(|speaker_id| ep.get_cast().get_member_by_id(speaker_id).is_some());

    matches_speaker && matches_episode && matches_season
}
