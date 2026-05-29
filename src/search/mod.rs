pub mod score;

use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use backend::archive::{assignments::AssignmentUnit, content::Content, episode::Episode};
use rayon::{
    iter::{ParallelBridge, ParallelExtend, ParallelIterator},
    slice::ParallelSliceMut,
};
use serde::{Deserialize, Serialize};

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
    speakers: Vec<Arc<str>>,
    text: Arc<str>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Moment {
    timestamp: String,
    episode_name: Arc<str>,
    episode_number: u8,
    season_name: Arc<str>,
    thumbnail_url: Arc<str>,
    youtube_url: String,
}

pub(crate) async fn search(
    State(episodes): State<Arc<[Episode]>>,
    Query(SearchParams { query }): Query<SearchParams>,
) -> Json<SearchResult> {
    let mut results: Vec<SearchResultItem> = {
        let vec_size: usize = episodes.iter().map(|e| e.get_content().0.len()).sum();
        Vec::with_capacity(vec_size)
    };

    for episode in episodes.iter() {
        search_in_episode(&episode, query.as_str(), &mut results);
    }
    results.par_sort_by(|a, b| b.score.total_cmp(&a.score));
    results.truncate(1000);
    Json(SearchResult(results))
}

fn search_in_episode(ep: &Episode, query: &str, result_vec: &mut Vec<SearchResultItem>) {
    let Content(lines) = ep.get_content();

    let map_entry_closure = |entry: (usize, &str, f64)| map_entry_to_item(ep, (entry.0, entry.2));

    let results = lines
        .iter()
        .enumerate()
        .par_bridge()
        .map(|(_i, line)| {
            (
                _i,
                line.as_str(),
                score::attribute_score(query, line.as_str()),
            )
        })
        .map(map_entry_closure);
    result_vec.par_extend(results);
}

fn map_entry_to_item(ep: &Episode, (i, score): (usize, f64)) -> SearchResultItem {
    let line_before = if i > 0 {
        Some(line_from_assignment_and_text(
            &ep.get_assignment().0[i - 1],
            ep.get_content().0[i - 1].as_str().into(),
        ))
    } else {
        None
    };

    let line: Line = line_from_assignment_and_text(
        &ep.get_assignment().0[i],
        ep.get_content().0[i].as_str().into(),
    );

    let line_after = if i < ep.get_assignment().0.len() - 1 {
        Some(line_from_assignment_and_text(
            &ep.get_assignment().0[i + 1],
            ep.get_content().0[i + 1].as_str().into(),
        ))
    } else {
        None
    };

    let timestamp = line_before.as_ref().map(|l| l.time).unwrap_or(line.time);
    let timestamp = chrono::TimeDelta::milliseconds(timestamp);
    let timestamp_str = format!(
        "{}:{:0>2}:{:0>2}",
        timestamp.num_hours(),
        timestamp.num_minutes() % 60,
        timestamp.num_seconds() % 60,
    );

    // Temporarily mocked values
    // TODO: Add episode-tracking file
    let moment: Moment = Moment {
        timestamp: timestamp_str,
        episode_name: ep.get_info().name.as_str().into(),
        episode_number: ep.get_info().number,
        season_name: ep.get_info().season_name.as_str().into(),
        thumbnail_url: "".into(),
        youtube_url: format!(
            "{}&t={}s",
            ep.get_info().youtube_url,
            timestamp.num_seconds()
        ),
    };

    SearchResultItem {
        line_before,
        line,
        line_after,
        moment,
        score,
    }
}

fn line_from_assignment_and_text(assignment: &AssignmentUnit, text: Arc<str>) -> Line {
    let AssignmentUnit { time, assignments } = assignment;
    let speakers: Vec<_> = assignments
        .iter()
        .map(String::as_str)
        .map(Arc::from)
        .collect();
    let time = *time;
    Line {
        speakers,
        text,
        time,
    }
}
